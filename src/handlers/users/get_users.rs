use actix_web::{body, get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use crate::{handlers::public::{login::login, register::register}, models::{auth::{LoginReq, PathParams, UserModel}, errors::HttpError, state::AppState}, utils::db::users};


#[get("/users")]
pub async fn get_users(app_data: web::Data<AppState>) -> Result<impl Responder, HttpError>  {
    let rs = users::get_all(&app_data.pool.lock().unwrap()).await;

    match rs {
        Ok(res) => Ok(web::Json(res)),
        Err(e) => {
            info!("{:?}", e);
            Err(HttpError::InternalError)
        }
    }
}


