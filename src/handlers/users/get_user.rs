use actix_web::{body, get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;

use crate::{models::{errors::{self, HttpError}, state::AppState}, utils::db::users};


#[get("/user/{id}")]
pub async fn get_user(app_data: web::Data<AppState>,
    id: web::Path<String>) -> Result<impl Responder, HttpError> {
    // make a query to the db
    // if error, return Error(HttpError) 

    let user = users::get(id.to_string(), &app_data.pool.lock().unwrap()).await;

    match user {
        Ok(u) => Ok(web::Json(u)),
        Err(e) => {
            info!("{:?}", e);
            Err(HttpError::InternalError)
        }
    }
}