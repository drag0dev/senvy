use std::sync::{Mutex, Condvar};
use std::collections::VecDeque;
use super::Task;

pub struct FileTaskQueue {
    jobs: Mutex<Option<VecDeque<Task>>>,
    condvar: Condvar,
}

impl FileTaskQueue {
    pub fn new() -> Self {
        FileTaskQueue {
            jobs: Mutex::new(Some(VecDeque::new())),
            condvar: Condvar::new()
        }
    }

    pub fn push_task(&self, t: Task) {
        let mut jobs = self.jobs.lock().unwrap();
        if let Some(queue) = jobs.as_mut() {
            queue.push_back(t);
            self.condvar.notify_all();
        }
    }

    pub fn wait_for_job(&self) -> Option<Task> {
        let mut jobs = self.jobs.lock().unwrap();
        loop {
            match jobs.as_mut()?.pop_front() {
                Some(job) => return Some(job),
                None => {
                    jobs = self.condvar.wait(jobs).unwrap();
                }
            }
        }
    }

    pub fn end(&self) {
        let mut jobs = self.jobs.lock().unwrap();
        *jobs = None;
    }
}
