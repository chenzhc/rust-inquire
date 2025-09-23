use std::sync::Mutex;


#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct AppState {
    pub state: Mutex<String>,
}



