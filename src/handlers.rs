use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{
    web::Json,
    get, post, Responder, HttpResponse
};
use crate::{
    types::Project,
    files::{create, read as read_file, update as update_file}
};

// TODO: thorough testing of err.is() calls
// TODO: log internal errors
// TODO: malformed json is already declined by middleware

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
            return HttpResponse::InternalServerError().finish();
        }

        let err = err.unwrap();
        if err.is::<std::io::Error>() {
            return HttpResponse::InternalServerError().finish();
        }
        else if err.is::<serde_json::Error>() {
            return HttpResponse::BadRequest().body("malformed json");
        }
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
        return HttpResponse::InternalServerError().finish();
    }
    let res = res.unwrap();
    if !res {
        return HttpResponse::BadRequest().body("project does not exist");
    }
    HttpResponse::Ok().finish()
}
