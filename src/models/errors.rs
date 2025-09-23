use actix_web::error;
use derive_more::{Display, Error};


#[derive(Debug, Display, Error)]
pub struct HttpError {
    #[display("error response: {}", message)]
    pub message: String,
}

impl error::ResponseError for HttpError {

}