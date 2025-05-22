#![allow(unused, dead_code)]
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{mem, thread};
use std::marker::PhantomData;
use std::time::Duration;
use crate::structs::{New, Once, Promise, Running, ScopeGuard, Semaphore, Stopped, ThreadPool};
use crate::traits::TaskPolicy;

impl Semaphore {
    pub(crate) fn new(count: usize) -> Arc<Self> {
        Arc::new(Self {
            count: Mutex::new(count),
            condvar: Condvar::new(),
        })
    }

    pub(crate) fn acquire(&self) {
        let mut count = self.count.lock().unwrap();
        while *count == 0 {
            count = self.condvar.wait(count).unwrap();
        }
        *count -= 1;
    }

    pub(crate) fn release(&self) {
        let mut count = self.count.lock().unwrap();
        *count += 1;
        self.condvar.notify_one();
    }
}

impl<T: Send> Promise<T> {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            value: Mutex::new(None),
            ready: Condvar::new(),
        })
    }

    pub fn set(&self, value: T) {
        let mut guard = self.value.lock().unwrap();
        *guard = Some(value);
        self.ready.notify_all();
    }

    pub fn get(&self) -> T {
        let mut guard = self.value.lock().unwrap();
        while guard.is_none() {
            guard = self.ready.wait(guard).unwrap();
        }
        guard.take().unwrap()
    }
}


impl<F: FnOnce()> ScopeGuard<F> {
    fn new(f: F) -> Self {
        Self { f: Some(f) }
    }
}

impl<F: FnOnce()> Drop for ScopeGuard<F> {
    fn drop(&mut self) {
        if let Some(f) = self.f.take() {
            f();
        }
    }
}


impl <T: TaskPolicy> ThreadPool<T, New> where T::Task: Send + 'static + Sized {

    pub fn run(self) -> ThreadPool<T, Stopped> {
        unsafe { self.start().join() }
    }

    pub fn start(self) -> ThreadPool<T, Running> {
        self.running.store(true, Ordering::Release);
        unsafe { self.to() }
    }

    pub fn spawn<R: Send + 'static, F: FnOnce() -> R + Send + 'static>(&self, task: F) -> Arc<Promise<R>> {
        let sem = self.semaphore.clone();
        let promise = Promise::new();
        let promise_clone = promise.clone();

        let handle = thread::spawn(move || {
            sem.acquire();
            let _guard = ScopeGuard::new(move || sem.release());
            let result = task();
            promise_clone.set(result);
        });

        self.handles.lock().unwrap().push(handle);

        promise
    }

    pub fn spawns<R: Send + 'static, F: Fn() -> R + Send + 'static>(&self, tasks: Vec<F>) -> Vec<Arc<Promise<R>>> {
        let mut promises = Vec::new();
        for task in tasks {
            let promise = self.spawn(task);
            promises.push(promise);
        }
        promises
    }
}

impl <T: TaskPolicy> ThreadPool<T, Running> {
    pub fn join(self) -> ThreadPool<T, Stopped> {
        let handles = {
            let mut lock = self.handles.lock().unwrap();
            mem::take(&mut *lock)
        };

        for handle in handles {
            let _ = handle.join();
        }
        self.running.store(false, Ordering::Release);
        unsafe { self.to() }
    }
}

impl <T: TaskPolicy, State> ThreadPool<T, State> {
    pub fn is_running(&self) -> bool {
        self.running.load(Ordering::Acquire)
    }

    pub fn is_stopped(&self) -> bool {
        !self.is_running()
    }
    unsafe fn to<U>(self) -> ThreadPool<T, U> {
        #[allow(unsafe_op_in_unsafe_fn)]
        mem::transmute(self)
    }
}

impl ThreadPool {
    pub fn new(max_concurrency: usize) -> Self {
        ThreadPool {
            semaphore: Semaphore::new(max_concurrency),
            handles: Mutex::new(Vec::new()),
            running: Arc::new(AtomicBool::new(false)),
            _marker: PhantomData,
            _state: PhantomData,
        }
    }

}


impl TaskPolicy for Once {
    type Task = Box<dyn FnOnce() + Send + 'static>;

    fn run(task: Self::Task, running: Arc<AtomicBool>) {
        while !running.load(Ordering::Acquire) {
            thread::sleep(Duration::from_millis(5));
        }
        task();
    }
}
