use actix_web::HttpServer;
use env_logger::Env;
use rustychain::create_app::create_app;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let server = HttpServer::new(move || create_app())
        // .workers(4)
        .bind(("127.0.0.1", 8080))?;
    server.run().await
}
