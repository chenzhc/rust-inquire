#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::{env, sync::Mutex};
use actix_web::{web, App, HttpServer};
use sqlx::{mysql::MySqlPoolOptions, MySqlPool, PgPool};
use tracing::{debug, info, Level};
use tracing_subscriber::FmtSubscriber;
use crate::{api::{auth::get_auth_services, public::{echo, get_public_services, hello, manual_hello}}, models::state::AppState};
mod api;
mod handlers;
mod models;
mod utils;
mod middleware;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    rust_inquire::init();
    
    let conn = env::var("DATABASE_URL").expect("the database url string was not set");
    info!("database url string is: {}", conn);
    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&conn)
        .await
        .expect("could not exstablish a connection to the database");
        // MySqlPool::connect(&conn).await.unwrap();
    info!("Connected to the database!");

    info!("test");
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::INFO)
        .finish();
    let _ = tracing::subscriber::set_global_default(subscriber)
        .expect("error setting global subscriber for tracing");


    let data = web::Data::new(AppState{
        pool: Mutex::new(pool),
    });
    info!("starting server at port: {}", 9000);
    debug!("test");

    HttpServer::new(move ||{
        App::new()
            .app_data(data.clone())
            .service(get_public_services())
            .service(
                get_auth_services()
            )
            .service(hello)
            .service(echo)
            .route("/health", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:9000")?
    // .bind("0.0.0.0", 9000)?
    .run()
    .await
}

