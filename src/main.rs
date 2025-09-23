#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::Mutex;

use actix_web::{web, App, HttpServer};
use log::info;
use crate::{api::{auth::get_auth_services, public::{echo, get_public_services, hello, manual_hello}}, models::state::AppState};
mod api;
mod handlers;
mod models;
mod utils;


#[actix_web::main]
async fn main() -> std::io::Result<()>{
    rust_inquire::init();

    info!("test");

    let data = web::Data::new(AppState{
        state: Mutex::new(String::from("init-state"))
    });

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

