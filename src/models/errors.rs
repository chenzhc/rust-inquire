use actix_web::{error, http::{header::ContentType, StatusCode}, HttpResponse};
use derive_more::{Display, Error};


#[derive(Debug, Display, Error)]
pub enum HttpError {
    #[display("internal server error")]
    InternalError,
    #[display("unauthorized error")]
    Unauthorized,
    #[display("unauthenticated error")]
    #[allow(dead_code)]
    Unauthenticated,
    #[display("request timeout")]
    #[allow(dead_code)]
    Timeout,
    #[display("invalid user credentials")]
    #[allow(dead_code)]
    InvalidCredentials,
    #[display("Nats error")]
    #[allow(dead_code)]
    NatsError,
}

impl error::ResponseError for HttpError {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
         match *self {
            HttpError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            HttpError::Unauthorized => StatusCode::UNAUTHORIZED,
            HttpError::Unauthenticated => StatusCode::UNAUTHORIZED,
            HttpError::Timeout => StatusCode::REQUEST_TIMEOUT,
            HttpError::InvalidCredentials => StatusCode::BAD_REQUEST,
            HttpError::NatsError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}