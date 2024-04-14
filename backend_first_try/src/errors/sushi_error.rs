use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use derive_more::Display;

#[derive(Debug, Display)]
pub enum SushiError {
    NoSushiFound = 0,
    SushiCreationFailure = 1,
    NoSuchSushiFound = 2,
}

impl ResponseError for SushiError {
    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            SushiError::NoSushiFound => StatusCode::NOT_FOUND,
            SushiError::SushiCreationFailure => StatusCode::INTERNAL_SERVER_ERROR,
            SushiError::NoSuchSushiFound => StatusCode::NOT_FOUND,
        }
    }
}
