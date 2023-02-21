use std::{sync::Arc, thread};
use actix_web::{
    App, HttpServer, web, error, HttpResponse, middleware::Logger,
    web::Data,
};
use env_logger::Env;
use tokio::runtime::Builder;

pub mod files;
pub mod handlers;
pub mod queue;

const LOGGER_FORMAT: &str = "[%t] %a %s UA:%{User-Agent}i CT:%{Content-Type}i %Dms";

fn main() {
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

    // always give one core to the worker and the rest to the server
    let cpus = num_cpus::get();
    let cpus = match cpus {
        1 => 1, // in case there is only one core, worker and server are going to share it
        n => n-1,
    };


    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let json_config = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|_, _| {
            error::InternalError::from_response("", HttpResponse::BadRequest().body("malformed json")).into()
        });

    let job_queue = Arc::new(queue::FileTaskQueue::new());

    // worker thread
    let job_queue_worker = job_queue.clone();
    let worker_thread = thread::spawn(move || {
        let job_queue = job_queue_worker;

        // tokio runtime for the current thread beacuse the queue it self is async
        let worker_runtime = Builder::new_current_thread()
            .worker_threads(1)
            .enable_io()
            .build()
            .unwrap();

        let worker_runtime_handle = worker_runtime.handle();
        let worker = worker_runtime_handle.spawn(async move {
            let job_queue = job_queue;
            while let Some(mut task) = job_queue.wait_for_task() {
                (&mut task).execute().await;
            }
        });
        _ = worker_runtime.block_on(worker);
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

    let server = server.unwrap();
    println!("Server running on port {}", port);

    let actix_runtime = Builder::new_multi_thread()
        .worker_threads(cpus)
        .enable_all()
        .thread_name("actix-runtime")
        .build()
        .unwrap();

    let server_future = server.run();
    _ = actix_runtime.block_on(server_future);

    // actix task will finish either by erroring or by being interrupted and at that point its safe
    // to kill the worker thread
    // gracefully killing worker thread via queue and waiting for worker thread to finish
    job_queue.end();
    _ = worker_thread.join();
}
