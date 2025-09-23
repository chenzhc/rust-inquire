use actix_web::{body, get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use crate::{handlers::public::{login::login, register::register}, models::{auth::{LoginReq, PathParams, UserModel}, state::AppState}};



#[get("/logout/{name}")]
pub async fn logout(name: web::Path<String>) -> impl Responder {
    let resp_msg = format!("Hello {name}, test322 you have been logged out!");
    HttpResponse::Ok().body(resp_msg)
}
