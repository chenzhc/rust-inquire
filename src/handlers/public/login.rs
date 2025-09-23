use actix_web::{body, get, post, web, App, HttpResponse, HttpServer, Responder};
use log::info;
use crate::models::{auth::{LoginReq, PathParams, UserModel}, state::AppState};


#[post("/login")]
pub async fn login(app_data: web::Data<AppState>,
    req: web::Json<LoginReq>) -> impl Responder {
    let mut state = app_data.state.lock().unwrap();
    *state = "login".to_string();
    info!("the current app state is {}", *state);
    info!("your credentials are {:?}", req);
    HttpResponse::Ok().body("Hello login")
}

