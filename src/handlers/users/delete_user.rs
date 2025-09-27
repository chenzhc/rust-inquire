use actix_web::{body, delete, get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use tokio::task::Id;
use tracing_subscriber::fmt::format;
use crate::{handlers::public::{login::login, register::register}, models::{auth::{LoginReq, PathParams, UserModel}, errors::HttpError, state::AppState}, utils::db::users};


#[delete("/user/{id}")]
pub async fn delete_user(app_data: web::Data<AppState>,
    id: web::Path<String>) -> Result<impl Responder, HttpError> {
    // make a query to the db
    // if error, return Error(HttpError) 
    let rs = users::delete(id.to_string(), &app_data.pool.lock().unwrap()).await;

    match rs {
        Ok(_) => Ok(HttpResponse::Ok().body(format!("the user with the id {} has been successfully deleted!", id))),
        Err(e) => {
            info!("{:?}", e);
            Err(HttpError::InternalError)
        }
    }
}