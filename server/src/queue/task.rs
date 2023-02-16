use std::mem::take;

use anyhow::Result;
use senvy_common::types::{Project, ProjectEntry};
use tokio::sync::oneshot::Sender;
use crate::files::{create, read, update, delete};

// TODO: avoid cloning, borrowed values (rc)?

pub enum FileTask {
    CreateConfig(u128, Project),
    ReadConfig(String),
    UpdateConfig(u128, Project),
    DeleteConfig(String)
}

pub enum FileTaskReturnType{
    CreateReturn(Result<bool>),
    ReadReturn(Result<Option<ProjectEntry>>),
    UpdateReturn(Result<bool>),
    DeleteReturn(Result<bool>)
}

pub struct Task {
    task: FileTask,
    /// option giving the ability to take the sender end of the chan out of the struct
    chan: Option<Sender<FileTaskReturnType>>,
}

impl Task {
    pub fn new(task: FileTask, chan: Sender<FileTaskReturnType>) -> Self {
        Task {
            task,
            chan: Some(chan),
        }
    }

    pub async fn execute(&mut self) {
        // always Some
        let chan = take(&mut self.chan).unwrap();
        match &self.task {
            FileTask::CreateConfig(timestamp, project) => {
                let res = create(*timestamp, project.clone()).await;
                _ = chan.send(FileTaskReturnType::CreateReturn(res));
            },
            FileTask::ReadConfig(project_name) => {
                let res = read(project_name).await;
                _ = chan.send(FileTaskReturnType::ReadReturn(res));
            },
            FileTask::UpdateConfig(timestamp, project) => {
                let res = update(*timestamp, project.clone()).await;
                _ = chan.send(FileTaskReturnType::UpdateReturn(res));
            },
            FileTask::DeleteConfig(project_name) => {
                let res = delete(project_name).await;
                _ = chan.send(FileTaskReturnType::DeleteReturn(res));
            },
        }
    }
}

