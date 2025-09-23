use actix_web::{body, get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use crate::models::{auth::{LoginReq, PathParams, UserModel}, state::AppState};


#[post("/user/register")]
pub async fn register(user: web::Either<web::Json<UserModel>, web::Form<UserModel>>) -> impl Responder {
    let resp_msg = format!("Hello {:?}, test322 you have been register!", user);
    HttpResponse::Ok().body(resp_msg)
}