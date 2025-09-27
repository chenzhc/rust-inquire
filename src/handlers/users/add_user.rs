use actix_web::{ body, post, web, HttpResponse, Responder};
use log::info;
use sqlx::types::uuid;
use crate::{models::{auth::UserModel, errors::{self, HttpError}, state::AppState}, utils::db::users};

#[post("/user/add")]
pub async fn add_user(app_data: web::Data<AppState>,
    user: web::Json<UserModel>) -> Result<impl Responder, HttpError> {
    let id = uuid::Uuid::new_v4().to_string();
    
    let result = users::insert(user.0, &app_data.pool.lock().unwrap(), &id).await;
    
    match result {
        Ok(_) => Ok(HttpResponse::Ok().body(format!("the user with the id {} has been successfully created!", id))),
        Err(e) => {
            info!("{:?}", e);
            Err(errors::HttpError::InternalError)
        },
    }
    
}