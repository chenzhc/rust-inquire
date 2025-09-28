use actix_web::{body, get, guard, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use crate::{handlers::{auth::logout::logout,
    public::{login::login, register::register},
    users::{add_user::add_user, delete_user::delete_user, 
    get_user::get_user, get_users::get_users}},
    models::{auth::{LoginReq, PathParams, UserModel}, state::AppState},
    utils::guard::AuthorizationHeader
};



// lgin, register

pub fn get_auth_services() -> actix_web::Scope {
    return web::scope("/auth")
        .guard(AuthorizationHeader)
        .service(get_user)
        .service(get_users)
        .service(add_user)
        .service(delete_user)
        .service(logout);
}






