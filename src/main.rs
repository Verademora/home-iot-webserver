mod config;
mod db;
mod errors;
mod handlers;
mod models;
use ::config::Config;
use actix_web::{web, App, HttpServer};
use dotenvy::dotenv;
use handlers::{get_data, post_data};
use tokio_postgres::NoTls;

use crate::config::ExampleConfig;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let config_ = Config::builder()
        .add_source(::config::Environment::default())
        .build()
        .unwrap();

    let config: ExampleConfig = config_.try_deserialize().unwrap();

    let pool = config.pg.create_pool(None, NoTls).unwrap();

    let server = HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::resource("data")
                .route(web::post().to(post_data))
                .route(web::get().to(get_data)),
        )
    })
    .bind(config.server_addr.clone())?
    .run();
    println!("Server running on http://{}/", config.server_addr);

    server.await
}
