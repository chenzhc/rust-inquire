
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, sqlx::FromRow )]
pub struct UserModel {
    pub firstname: String,
    pub lastname: String,
    pub password: String,
    pub email: String,
    pub id: String,
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


#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Claims {
    pub custom_claim: UserModel,
    pub iss: String,
    pub sub: String,
    pub aud: String,
    pub exp: u64,
}