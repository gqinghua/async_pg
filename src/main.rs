use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use crate::handlers::addUser::add_user;
use tokio_postgres::NoTls;
pub mod models;
pub mod handlers;
pub mod db;
pub mod Config;
pub mod  errors;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config = crate::Config::config::Config::from_env().unwrap();
    let pool = config.pg.create_pool(NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(web::resource("/users").route(web::post().to(add_user)))
    })
        .bind(config.server_addr.clone())?
        .run();
    println!("Server running at http://{}/", config.server_addr);

    server.await
}
