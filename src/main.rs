use actix_web::{
    middleware::{self, Logger},
    App, HttpServer,
};
use env_logger::Env;
use rust_webapp::{config::ServerConfig, routes};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging with a default level of 'info'
    env_logger::init_from_env(Env::new().default_filter_or("info"));

    // Load server configuration
    let config = ServerConfig::default();

    // Create and bind the TCP listener
    let listener = config.create_listener()?;
    let local_addr = listener.local_addr()?;

    log::info!("Starting server at http://{}", local_addr);

    // Build and run the server with graceful shutdown
    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(Logger::default())
            .wrap(Logger::new("%a %r %s %b %{Referer}i %{User-Agent}i %T"))
            .configure(routes::configure)
    })
    .listen(listener)?
    .workers(num_cpus::get()) // Optimize number of workers based on CPU cores
    .shutdown_timeout(30) // Allow 30 seconds for graceful shutdown
    .run()
    .await
}
