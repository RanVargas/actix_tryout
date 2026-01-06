mod db;
mod handlers;
mod models;
mod routes;
mod services;

use actix_web::{App, HttpServer, middleware::Logger, web};
use dotenv::dotenv;
use env_logger::Env;
use std::env;

use routes::user_routes::init;

use crate::db::initialize_db_pool;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "9090".to_string());
    let pool = initialize_db_pool();

    env_logger::init_from_env(Env::default().default_filter_or("info"));

    println!("Server running at http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(Logger::new("%a %t %r %s %b"))
            .configure(init) // Initialize routes
    })
    .bind(format!("{}:{}", host, port))?
    .run()
    .await
}
