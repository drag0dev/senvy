use actix_web::{
    App, HttpServer, web, error, HttpResponse, middleware::Logger
};
use env_logger::Env;

pub mod files;
pub mod handlers;
pub mod queue;

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

    let json_config = web::JsonConfig::default()
        .limit(4096)
        .error_handler(|_, _| {
            error::InternalError::from_response("", HttpResponse::BadRequest().body("malformed json")).into()
        });

    let server = HttpServer::new(move || {
        App::new()
            .wrap(Logger::new(LOGGER_FORMAT))
            .app_data(json_config.clone())
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
}
