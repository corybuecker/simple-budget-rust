use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[allow(dead_code)]
#[derive(Debug)]
pub struct FormError {
    pub message: String,
    pub status_code: Option<StatusCode>,
}

impl IntoResponse for FormError {
    fn into_response(self) -> Response {
        (StatusCode::BAD_REQUEST, format!("{:#?}", self)).into_response()
    }
}

impl From<bson::oid::Error> for FormError {
    fn from(value: bson::oid::Error) -> Self {
        FormError {
            message: value.to_string(),
            status_code: None,
        }
    }
}

impl From<tera::Error> for FormError {
    fn from(value: tera::Error) -> Self {
        log::error!("{:#?}", value);

        FormError {
            message: value.to_string(),
            status_code: None,
        }
    }
}

impl From<mongodb::error::Error> for FormError {
    fn from(value: mongodb::error::Error) -> Self {
        FormError {
            message: value.to_string(),
            status_code: None,
        }
    }
}
