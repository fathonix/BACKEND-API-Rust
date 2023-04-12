#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_web::{web, App, HttpServer};
use actix_web::web::Data;
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager};
use crate::handlers::*;

mod handlers;
mod models;
mod schema;
mod config;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");

    let config = config::load_config();
    let manager =
        ConnectionManager::<MysqlConnection>::new(config.database_url);
    let pool: Pool = Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .service(
                web::resource("/api/products")
                    .route(web::get().to(get_items))
                    .route(web::post().to(create_item))
            )
            .service(
                web::resource("/api/products/{id}")
                    .route(web::get().to(get_item))
                    .route(web::put().to(update_item))
                    .route(web::delete().to(delete_item))
            )
    })
        .bind(config.host)
        .expect("Failed to bind address.")
        .run()
        .await
}