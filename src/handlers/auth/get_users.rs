use actix_web::{body, get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use crate::{handlers::public::{login::login, register::register}, models::{auth::{LoginReq, PathParams, UserModel}, state::AppState}};


#[get("/users")]
pub async fn get_users(path: web::Query<PathParams>) -> impl Responder {
    let name = &path.name;
    let id = &path.id;
    let email = &path.email;
    let resp_msg = format!("Hello {name}, user id {id} and email {email}  you have been fetched!");
    HttpResponse::Ok().body(resp_msg)
}


