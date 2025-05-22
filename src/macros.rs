#[macro_export]
macro_rules! thread_pool {

    (@create, $name:ident, $size:expr) => {
        let $name = $crate::structs::ThreadPool::new($size);
    };

    (@spawn, $name:ident, $size:expr, { $( $task_name:ident => $body:block ),+ $(,)? }) => {
        $crate::thread_pool!(@internal $name, $size, false, $( $task_name => $body ),+);
    };

    (@spawn?, $name:ident, $size:expr, { $( $task_name:ident => $body:block ),+ $(,)? }) => {
        $crate::thread_pool!(@internal? $name, $size, false, $( $task_name => $body ),+);
    };


    (@run, $name:ident, $size:expr, { $( $task_name:ident => $body:block ),+ $(,)? }) => {
        $crate::thread_pool!(@internal $name, $size, true, $( $task_name => $body ),+);
    };
    (@run?, $name:ident, $size:expr, { $( $task_name:ident => $body:block ),+ $(,)? }) => {
        $crate::thread_pool!(@internal? $name, $size, true, $( $task_name => $body ),+);
    };

    ($size:expr, { $( $task_name:ident => $body:block ),+ $(,)? }) => {
        $crate::thread_pool!(@internal _pool, $size, true, $( $task_name => $body ),+);
    };

    ($( $task_name:ident => $body:block ),+ $(,)?) => {
        let _size = std::thread::available_parallelism().unwrap().get();
        $crate::thread_pool!(@internal _pool, _size, true, $( $task_name => $body ),+);
    };

    (?$( $task_name:ident => $body:block ),+ $(,)?) => {
        let _size = std::thread::available_parallelism().unwrap().get();
        $crate::thread_pool!(@internal? _pool, _size, true, $( $task_name => $body ),+);
    };

   ($($body:block),+ $(,)?) => {
        let _size = std::thread::available_parallelism().unwrap().get();
        $crate::thread_pool!(@internal _pool, _size, true, $( __ => $body ),+);
    };

    () => {
        compile_error!("Missing task definitions for thread_pool! macro");
    };

    (@internal $name:ident, $size:expr, $run:expr, $( $task_name:ident => $body:block ),+ ) => {
        let $name = $crate::structs::ThreadPool::new($size);
        $( let $task_name = $name.spawn(|| $body); )+
        if $run {
            $name.run();
        }
    };

    (@internal? $name:ident, $size:expr, $run:expr, $( $task_name:ident => $body:block ),+ ) => {
        $crate::thread_pool!(@internal $name, $size, $run, $( $task_name => $body ),+);
        unpack!($($task_name),+);
    };
}
#[macro_export]
macro_rules! spawn_tasks {
    ($pool:ident, $( $task_name:ident => $body:block ),+ $(,)?) => {
        $( let $task_name = $pool.spawn(|| $body); )+
        $pool.run();
    };
}

#[macro_export]
macro_rules! sleep {
    ($duration:expr) => {
        std::thread::sleep(std::time::Duration::from_secs($duration));
    };
}

#[macro_export]
macro_rules! unpack {
    ($($promise:ident),+ $(,)?) => {
        $(
            let $promise = $promise.get();
        )+
    };
}