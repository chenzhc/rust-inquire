
#[derive(Debug, serde::Deserialize, serde::Serialize, sqlx::FromRow )]
#[sqlx(type_name = "interface_type")]
pub struct UserModel {
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub email: String,
    pub id: u32,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct LoginReq {
    pub  password: String,
    pub email: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct PathParams {
    pub name: String,
    pub id: String,
    pub email: String,
}
