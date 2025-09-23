#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]
use actix_web::{body, get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use crate::{handlers::public::{login::login, register::register}};


pub fn get_public_services() -> actix_web::Scope {
    return web::scope("/public")
                    .service(login)
                    .service(register);
}



#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
