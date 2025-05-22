#![allow(unused, dead_code)]
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

pub trait TaskPolicy {
    type Task: Send + 'static;

    fn run(task: Self::Task, running: Arc<AtomicBool>);
}
