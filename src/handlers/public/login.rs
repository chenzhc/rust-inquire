use core::borrow;

use actix_web::{body, get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use crate::{models::{auth::{LoginReq, PathParams, UserModel}, errors::HttpError, state::AppState}, utils::{self, db, jwt}};


#[post("/login")]
pub async fn login(app_data: web::Data<AppState>,
    req: web::Json<LoginReq>) ->  Result<impl Responder, HttpError>  {
    let rs = db::users::get_by_email(req.email.clone(), &app_data.pool.lock().unwrap()).await;
    match rs {
        Ok(user) => {
            let token = jwt::encode(user);

            Ok(HttpResponse::Ok().body(token))
        },
        Err(e) => {
            info!("{:?}", e);
            Err(HttpError::Unauthorized)
        }
    }
}

