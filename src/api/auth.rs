use actix_web::{body, get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use crate::{handlers::{auth::{get_users::get_users, get_user::get_user, logout::logout}, public::{login::login, register::register}}, models::{auth::{LoginReq, PathParams, UserModel}, state::AppState}};



// lgin, register

pub fn get_auth_services() -> actix_web::Scope {
    return web::scope("/auth")
        .service(get_user)
        .service(get_users)
        .service(logout);
}






