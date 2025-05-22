use std::marker::PhantomData;
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::AtomicBool;
use std::thread::JoinHandle;
use crate::traits::TaskPolicy;

pub struct Once;

#[derive(Debug,Default)]
pub struct Semaphore {
    pub(crate) count: Mutex<usize>,
    pub(crate) condvar: Condvar,
}

pub struct New;
pub struct Running;
pub struct Stopped;
pub struct ScopeGuard<F: FnOnce()> {
    pub(crate) f: Option<F>,
}

#[derive(Debug, Default)]
pub struct ThreadPool<T: TaskPolicy = Once, State = New> {
    pub(crate) semaphore: Arc<Semaphore>,
    pub(crate) handles: Mutex<Vec<JoinHandle<()>>>,
    pub(crate) running: Arc<AtomicBool>,
    pub(crate) _marker: PhantomData<T>,
    pub(crate) _state: PhantomData<State>
}
#[derive(Debug)]
pub struct Promise<T> {
    pub(crate) value: Mutex<Option<T>>,
    pub(crate) ready: Condvar,
}