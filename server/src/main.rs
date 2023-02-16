use std::sync::Arc;
use actix_web::{
    App, HttpServer, web, error, HttpResponse, middleware::Logger,
    web::Data,
};
use env_logger::Env;

pub mod files;
pub mod handlers;
pub mod queue;

// TODO: worker threads stays behind in case of interrupting server

const LOGGER_FORMAT: &str = "[%t] %a %s UA:%{User-Agent}i CT:%{Content-Type}i %Dms";

#[actix_web::main]
async fn main() {
    let port = std::env::var("PORT");
    let port = match port {
        Ok(p) => {
            let p = p.parse::<u16>();
            if p.is_err() {
                panic!("Malformed port env var, using default 8080");
            }
            p.unwrap()
        },
        Err(_) => {
            println!("Using default port 8080");
            8080
        }
    };

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let job_queue = Arc::new(queue::FileTaskQueue::new());

    // TODO: is it possible to have a hard thread instead of a green thread? (separate async runtime instance for a hard thread)
    // thread which consumes tasks and executes them
    let job_queue_worker = job_queue.clone();
    let worker = tokio::spawn(async move {
        let job_queue = job_queue_worker.clone();
        while let Some(mut task) = job_queue.wait_for_job() {
            (&mut task).execute().await;
        }
    });

    let json_config = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|_, _| {
            error::InternalError::from_response("", HttpResponse::BadRequest().body("malformed json")).into()
        });

    let job_queue_server = job_queue.clone();
    let server = HttpServer::new(move || {
        let job_queue = job_queue_server.clone();
        App::new()
            .wrap(Logger::new(LOGGER_FORMAT))
            .app_data(json_config.clone())
            .app_data(Data::new(Arc::clone(&job_queue)))
            .service(handlers::new)
            .service(handlers::read)
            .service(handlers::update)
            .service(handlers::delete)
            .service(handlers::exists)
    }).bind(("127.0.0.1", port));
    if server.is_err() {
        println!("Error binding to port {}: {}\n", port, server.err().unwrap());
        return;
    }

    println!("Server running on port {}", port);
    let server = server.unwrap();
    let res = server.run().await;
    if res.is_err() {
        println!("Error while running the server: {}\n", res.err().unwrap());
        println!("Exiting...");
    }

    // the only way for this to be executed is for server to error
    // kills the worker thread
    job_queue.end();

    _ = worker.await;
}
