use std::{
    time::{SystemTime, UNIX_EPOCH},
    sync::Arc
};
use log::error;
use actix_web::{
    web::{Json, Data},
    get, post, delete,
    Responder, HttpResponse
};
use senvy_common::types::Project;
use tokio::sync::oneshot;
use crate::queue::{
    FileTaskQueue,
    Task,
    FileTask,
    task::FileTaskReturnType
};

macro_rules! get_err {
    ( $x:expr ) => {
        {
            $x.as_ref()
                .err()
                .unwrap()
                .chain()
                .nth(0)
        }
    };
}

// unreachable in match for a specific task result is used to uncover mismatched result types
/// arguments -> job queue, task type, task return type,
/// and the rest of the provided arguments are for the underlying file function
macro_rules! execute_task {
    ( $queue:ident, $task_type:ident, $task_return_type:ident, $($arg:ident),+ ) => {
        {
            // making a new task
            let (tx, rx) = oneshot::channel();
            let task = Task::new(
                FileTask::$task_type($($arg),+), tx);

            // pushing a new task into the queue and awaiting the result
            $queue.push_task(task);
            let res = rx.await;

            // was there error receiving result
            let res = match res {
                Ok(res) => res,
                Err(_) => {
                    return HttpResponse::InternalServerError().finish();
                }
            };

            // match the result into the desired
            let res = match res {
                FileTaskReturnType::$task_return_type(r) => {r},
                _ => unreachable!(),
            };
            res
        }
    }
}

#[post("/new")]
async fn new(project: Json<Project>, queue: Data<Arc<FileTaskQueue>>) -> impl Responder {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() // safe to just unwrap beacuse UNIX_EPOCH is passed
        .as_nanos();
    let project = project.into_inner();

    let res = execute_task!(queue, CreateConfig, CreateReturn, timestamp, project);

    // if there was error during creation check what happened exactly
    if res.is_err() {
        // first element in the chain is the original error
        let err = get_err!(res);
        if err.is_none() {
            error!("Error creating a new project: no error");
        }else {
            let err = err.unwrap();
            error!("Error creating a new project: {}", err);
        }
        return HttpResponse::InternalServerError().finish();
    }

    let res = res.unwrap();
    if res {
        return HttpResponse::Ok().body(format!("{}", timestamp));
    }
    HttpResponse::BadRequest().body("project already exists")
}

#[get("/read")]
async fn read(project_name: String, queue: Data<Arc<FileTaskQueue>>) -> impl Responder{
    let data = execute_task!(queue, ReadConfig, ReadReturn, project_name);
    if data.is_err() {
        // json is checked when written so it can only be fs error
        let err = get_err!(data);
        if err.is_none() {
            error!("Error reading a project: no error");
        }else {
            let err = err.unwrap();
            error!("Error reading a project: {}", err);
        }
        return HttpResponse::InternalServerError().finish();
    }

    let data = data.unwrap();
    if data.is_none() {
        return HttpResponse::BadRequest().body("project does not exist");
    }

    let data = data.unwrap();
    HttpResponse::Ok().json(data)
}

#[post("/update")]
async fn update(project: Json<Project>, queue: Data<Arc<FileTaskQueue>>) -> impl Responder {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() // safe to just unwrap beacuse UNIX_EPOCH is passed
        .as_nanos();
    let project = project.into_inner();

    let res = execute_task!(queue, UpdateConfig, UpdateReturn, timestamp, project);
    if res.is_err() {
        let err = get_err!(res);
        if err.is_none() {
            error!("Error updating a project: no error");
        }else {
            let err = err.unwrap();
            error!("Error updating a project: {}", err);
        }
        return HttpResponse::InternalServerError().finish();
    }
    let res = res.unwrap();
    if !res {
        return HttpResponse::BadRequest().body("project does not exist");
    }
    return HttpResponse::Ok().body(format!("{}", timestamp));
}

#[delete("/delete")]
async fn delete(project_name: String, queue: Data<Arc<FileTaskQueue>>) -> impl Responder {
    let res = execute_task!(queue, DeleteConfig, DeleteReturn, project_name);
    if res.is_err() {
        let err = get_err!(res);
        if err.is_none() {
            error!("Error updating a project: no error");
        }else {
            let err = err.unwrap();
            error!("Error updating a project: {}", err);
        }
        return HttpResponse::InternalServerError().finish();
    }
    let res = res.unwrap();
    if !res {
        return HttpResponse::BadRequest().body("project does not exist");
    }
    HttpResponse::Ok().finish()
}

#[get("/exists")]
async fn exists(project_name: String, queue: Data<Arc<FileTaskQueue>>) -> impl Responder {
    let res = execute_task!(queue, ReadConfig, ReadReturn, project_name);
    if res.is_err() {
        let err = get_err!(res);
        if err.is_none() {
            error!("Error checking if a project exists: no error");
        }else {
            let err = err.unwrap();
            error!("Error checking if a project exists: {}", err);
        }
        return HttpResponse::InternalServerError().finish();
    }

    let res = res.unwrap();
    if res.is_none() {
        return HttpResponse::Ok().body("false");
    }
    HttpResponse::Ok().body("true")
}
