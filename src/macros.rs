#[macro_export]
macro_rules! thread_pool {

    // Creates a thread pool with the given name and size.
    (@create, $name:ident, $size:expr) => {
        let $name = $crate::structs::ThreadPool::new($size);
    };

    // Spawns tasks into a named thread pool without auto-unpacking and without running immediately.
    (@spawn, $name:ident, $size:expr, { $( $task_name:ident => $body:expr ),+ $(,)? }) => {
        $crate::thread_pool!(@internal $name, $size, false, $( $task_name => $body ),+);
    };

    // Spawns tasks and unpacks their results, but doesn't run automatically.
    (@spawn?, $name:ident, $size:expr, { $( $task_name:ident => $body:expr ),+ $(,)? }) => {
        $crate::thread_pool!(@internal? $name, $size, false, $( $task_name => $body ),+);
    };

    // Spawns tasks into a named pool and runs the thread pool immediately.
    (@run, $name:ident, $size:expr, { $( $task_name:ident => $body:expr ),+ $(,)? }) => {
        $crate::thread_pool!(@internal $name, $size, true, $( $task_name => $body ),+);
    };

    // Spawns tasks, runs the pool immediately, and unpacks the results.
    (@run?, $name:ident, $size:expr, { $( $task_name:ident => $body:expr ),+ $(,)? }) => {
        $crate::thread_pool!(@internal? $name, $size, true, $( $task_name => $body ),+);
    };

    // Spawns tasks into an unnamed pool with a specified size, and runs immediately.
    ($size:expr, { $( $task_name:ident => $body:expr ),+ $(,)? }) => {
        $crate::thread_pool!(@internal _pool, $size, true, $( $task_name => $body ),+);
    };

    // Spawns tasks using default thread pool size (based on available CPUs), and runs immediately.
    ($( $task_name:ident => $body:expr ),+ $(,)?) => {
        let _size = std::thread::available_parallelism().unwrap().get();
        $crate::thread_pool!(@internal _pool, _size, true, $( $task_name => $body ),+);
    };

    // Spawns tasks with default thread pool size, runs immediately, and unpacks the results.
    (?$( $task_name:ident => $body:expr ),+ $(,)?) => {
        let _size = std::thread::available_parallelism().unwrap().get();
        $crate::thread_pool!(@internal? _pool, _size, true, $( $task_name => $body ),+);
    };

    // Spawns unnamed tasks using default pool size; ignores task names, just runs them.
    ($($body:expr),+ $(,)?) => {
        let _size = std::thread::available_parallelism().unwrap().get();
        $crate::thread_pool!(@internal _pool, _size, true, $( __ => $body ),+);
    };

    // Handles completely empty macro invocation.
    () => {
        compile_error!("Missing task definitions for thread_pool! macro");
    };

    // Internal implementation: creates pool, spawns tasks, optionally runs them.
    (@internal $name:ident, $size:expr, $run:expr, $( $task_name:ident => $body:expr ),+ ) => {
        let $name = $crate::structs::ThreadPool::new($size);
        $( let $task_name = $name.spawn(move || $body); )+
        if $run {
            $name.run();
        }
    };

    // Same as @internal, but also unpacks (i.e., joins) each task result.
    (@internal? $name:ident, $size:expr, $run:expr, $( $task_name:ident => $body:expr ),+ ) => {
        $crate::thread_pool!(@internal $name, $size, $run, $( $task_name => $body ),+);
        unpack!($($task_name),+);
    };
}

#[macro_export]
macro_rules! spawn_tasks {
    // Spawns a batch of named tasks on an existing pool and runs the pool.
    ($pool:ident, $( $task_name:ident => $body:expr ),+ $(,)?) => {
        $( let $task_name = $pool.spawn(move || $body); )+
        $pool.run();
    };
}

#[macro_export]
macro_rules! sleep {
    // Shorthand for thread sleep using seconds.
    ($duration:expr) => {
        std::thread::sleep(std::time::Duration::from_secs($duration));
    };
}

#[macro_export]
macro_rules! unpack {
    // Awaits all spawned tasks by calling `.get()` on each.
    ($($promise:ident),+ $(,)?) => {
        $(
            let $promise = $promise.get();
        )+
    };
}
