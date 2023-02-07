use actix_web::{
    App, HttpServer
};

// TODO: eprintln instead of println for logging?

mod project_entry;

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

    let server = HttpServer::new(|| {
        App::new()
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
