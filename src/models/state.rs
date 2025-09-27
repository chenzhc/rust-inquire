use std::sync::Mutex;

use sqlx::MySqlPool;


#[derive(Debug)]
pub struct AppState {
    pub pool: Mutex<MySqlPool>,
}



