use std::time::{SystemTime, UNIX_EPOCH};
use log::error;
use actix_web::{
    web::Json,
    get, post, delete,
    Responder, HttpResponse
};
use crate::{
    types::Project,
    files::{
        create,
        read as read_file,
        update as update_file,
        delete as delete_file
    }
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

#[post("/new")]
async fn new(project: Json<Project>) -> impl Responder {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() // safe to just unwrap beacuse UNIX_EPOCH is passed
        .as_nanos();
    let project = project.into_inner();
    let res = create(timestamp, project).await;

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
        return HttpResponse::Ok().finish();
    }
    HttpResponse::BadRequest().body("project already exists")
}

#[get("/read")]
async fn read(project_name: String) -> impl Responder{
    let data = read_file(&project_name).await;
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
async fn update(project: Json<Project>) -> impl Responder {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap() // safe to just unwrap beacuse UNIX_EPOCH is passed
        .as_nanos();
    let project = project.into_inner();
    let res = update_file(timestamp, project).await;
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

#[delete("/delete")]
async fn delete(project_name: String) -> impl Responder {
    let res = delete_file(&project_name).await;
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
