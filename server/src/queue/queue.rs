use std::sync::{Mutex, Condvar};
use std::collections::VecDeque;
use super::Task;

/// queue of file tasks to be done on the project entries
/// tasks are executed sequentially
pub struct FileTaskQueue {
    jobs: Mutex<Option<VecDeque<Task>>>,
    cvar: Condvar,
}

impl FileTaskQueue {
    pub fn new() -> Self {
        FileTaskQueue {
            jobs: Mutex::new(Some(VecDeque::new())),
            cvar: Condvar::new()
        }
    }

    /// pushes a task into the queue
    /// return indicates whether the task was successfully pushed or not
    pub fn push_task(&self, t: Task) -> bool {
        let jobs = self.jobs.lock();
        if jobs.is_err() {
            return false;
        }

        let mut jobs = jobs.unwrap();
        if let Some(queue) = jobs.as_mut() {
            queue.push_back(t);
            self.cvar.notify_all();
        } else {
            return false;
        }
        true
    }

    /// waits for a task to be in queue
    /// None being returned means that queue has finished and there will be no more tasks
    /// calling this function after queue finished will result in a panic
    pub fn wait_for_task(&self) -> Option<Task> {
        let mut jobs = self.jobs.lock().unwrap();
        loop {
            match jobs.as_mut()?.pop_front() {
                Some(job) => return Some(job),
                None => {
                    jobs = self.cvar
                               .wait(jobs)
                               .unwrap();
                }
            }
        }
    }

    /// ending the queue signals the thread that is consuming the tasks to stop waiting
    // by dropping the inner queue
    pub fn end(&self) {
        let mut jobs = self.jobs.lock().unwrap();
        *jobs = None;
        self.cvar.notify_all();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::queue::FileTask;
    use tokio::sync::oneshot;

    fn new_task() -> Task {
        let (rx, _) = oneshot::channel();
        Task::new(FileTask::ReadConfig("test".to_string()), rx)
    }

    #[test]
    fn pushing() {
        let q = FileTaskQueue::new();
        for _ in 0..5 {
            let task = new_task();
            assert_eq!(true, q.push_task(task));
        }

        let mut number_of_tasks = 0;
        while q.wait_for_task().is_some() {
            number_of_tasks += 1;
            if number_of_tasks == 5  {
                q.end();
            }
        }
        assert!(true);
    }

    #[test]
    fn empty_queue() {
        let q = FileTaskQueue::new();
        q.end();

        let task = new_task();
        assert_eq!(false, q.push_task(task));
    }
}
