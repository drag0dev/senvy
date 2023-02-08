use std::time::{SystemTime, UNIX_EPOCH};

use actix_web::{
    web::Json,
    post, Responder, HttpResponse
};
use crate::{
    types::Project,
    files::create
};

// TODO: thorough testing of err.is() calls

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
        let mut err = res
            .as_ref()
            .err()
            .unwrap()
            .chain();

        let err = err.nth(0);
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
