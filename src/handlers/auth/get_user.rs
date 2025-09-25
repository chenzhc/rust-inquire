use actix_web::{body, error::HttpError, get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use crate::{handlers::public::{login::login, register::register}, models::{auth::{LoginReq, PathParams, UserModel}, state::AppState}};


// #[get("/user/{name}/{id}/{email}")]
// pub async fn fetch_user(app_data: web::Data<AppState>,
//     path: web::Path<PathParams>) -> impl Responder {
//     let state = app_data.state.lock().unwrap();
//     info!("the current app state is: {}", state);

//     let name = &path.name;
//     let id = &path.id;
//     let email = &path.email;
//     let resp_msg = format!("Hello {name}, user id {id} and email {email}  you have been fetched!");
//     HttpResponse::Ok().body(resp_msg)
// }


#[get("/user")]
pub async fn get_user(path: web::Query<PathParams>) -> Result<impl Responder, HttpError> {
    // make a query to the db
    // if error, return Error(HttpError) 
    let user = UserModel {
        firstname: "Test".to_string(),
        lastname: "lastname".to_string(),
        password: "test".to_string(),
        email: "test@email.com".to_string(),
        id: 1u32,
    };

    Ok(web::Json(user))
}