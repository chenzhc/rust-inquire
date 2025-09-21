#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]
#![allow(dead_code)]
#![allow(unused_variables)]

use std::sync::Mutex;

use actix_web::{body, get, post, web, App, HttpResponse, HttpServer, Responder};
use chrono::NaiveDate;
use colorize::AnsiColor;
use inquire::{validator::{ErrorMessage, Validation}, CustomUserError, DateSelect, MultiSelect, Select};
use log::info;

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct PathParams {
    name: String,
    id: String,
    email: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct UserModel {
    firstname: String,
    lastname: String,
    password: String,
    email: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
struct LoginReq {
    password: String,
    email: String,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct AppState {
    state: Mutex<String>,
}


#[post("/login")]
async fn login(app_data: web::Data<AppState>,
    req: web::Json<LoginReq>) -> impl Responder {
    let mut state = app_data.state.lock().unwrap();
    *state = "login".to_string();
    info!("the current app state is {}", *state);
    info!("your credentials are {:?}", req);
    HttpResponse::Ok().body("Hello login")
}

#[get("/logout/{name}")]
async fn logout(name: web::Path<String>) -> impl Responder {
    let resp_msg = format!("Hello {name}, test322 you have been logged out!");
    HttpResponse::Ok().body(resp_msg)
}

#[get("/user/{name}/{id}/{email}")]
async fn fetch_user(app_data: web::Data<AppState>,
    path: web::Path<PathParams>) -> impl Responder {
    let state = app_data.state.lock().unwrap();
    info!("the current app state is: {}", state);

    let name = &path.name;
    let id = &path.id;
    let email = &path.email;
    let resp_msg = format!("Hello {name}, user id {id} and email {email}  you have been fetched!");
    HttpResponse::Ok().body(resp_msg)
}


#[get("/user")]
async fn get_user(path: web::Query<PathParams>) -> impl Responder {
    let name = &path.name;
    let id = &path.id;
    let email = &path.email;
    let resp_msg = format!("Hello {name}, user id {id} and email {email}  you have been fetched!");
    HttpResponse::Ok().body(resp_msg)
}

#[post("/user/register")]
async fn register(user: web::Either<web::Json<UserModel>, web::Form<UserModel>>) -> impl Responder {
    let resp_msg = format!("Hello {:?}, test322 you have been register!", user);
    HttpResponse::Ok().body(resp_msg)
}

#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}



#[actix_web::main]
async fn main() -> std::io::Result<()>{
    rust_inquire::init();

    info!("test");

    let data = web::Data::new(AppState{
        state: Mutex::new(String::from("init-state"))
    });

    HttpServer::new(move ||{
        App::new()
            .app_data(data.clone())
            .service(
                web::scope("/auth")
                    .service(login)
                    .service(logout)
                    .service(fetch_user)
                    .service(get_user)
                    .service(register)
            )
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind("0.0.0.0:9000")?
    // .bind("0.0.0.0", 9000)?
    .run()
    .await
}

#[derive(Debug)]
struct Player {
    name: String,
    classes: Vec<String>,
    birth_date: NaiveDate,
}

fn prompt_date() -> NaiveDate {
    let prompt_message = "Select the player's birth date";
    let selcted_date = DateSelect::new(prompt_message)
        .prompt()
        .expect("Failed to select player's birth date");

    info!("You selected {}", selcted_date.format("%Y month: %m day: %d"));

    return selcted_date;
}


fn prompt_multiselect() -> Vec<String> {
    let prompt_message = "Please select your player classes?".yellow();
    let player_classes = vec![
        "Druid".to_string(),
        "Cleric".to_string(),
        "Archer".to_string(),
        "Warrior".to_string(),
        "Wizard".to_string(),
    ];

    let select = MultiSelect::new(&prompt_message, player_classes)
        .prompt()
        .expect("Failed to select multiple player class");

    // for sel in select {
    //     info!("You selected the player class {}", sel.yellow());
    // }

    return select;
}

fn prompt_select() -> String {
    let prompt_message = "Please select your player classes?".yellow();
    let player_classes = vec![
        "Druid",
        "Cleric",
        "Archer",
        "Warrior",
        "Wizard",
    ];

    let select = Select::new(&prompt_message, player_classes)
        .prompt()
        .expect("Failed to select player class");

    info!("You selected the player class {}", select.yellow());
    return select.to_string();
}

fn prompt_text() -> String {

    let name_validator = |i: &str | -> Result<Validation, CustomUserError> {
        let first_char = i.chars().next().unwrap() as u8;
        match first_char {
            65..=90 => {
                return Ok(Validation::Valid);
            },
            _ => { 
                let err_msg = "Please make sure the first character is capital Latter";
                return Ok(Validation::Invalid(err_msg.into()))
            }
        }
    };
    let prompt_message = "What is your player name?".yellow();
    let player_name = inquire::Text::new(&prompt_message)
        .with_validator(name_validator)
        .prompt()
        .expect("Failed to capture player name");
    // let player_name = inquire::prompt_text(prompt_message)
    //     .expect("Failed to capture player name");

    // info!("Your player name is: {}", player_name.yellow());
    return player_name;
}

fn prompt_boolean() {
    let message = "Are you ready to proceed?".yellow();

    let proceed = inquire::prompt_confirmation(message);
    if proceed.is_err() {
        info!("Error occurred while checking if to procced");
    }

    if proceed.unwrap() {
        info!("User selected to proceed with application ");
    } else {
        info!("{}", "User is not okay with proceeding".yellow());
    }
}