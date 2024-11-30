use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum RelayServerError {
    BadDataFormat,
    BadRequest(String),
}

impl IntoResponse for RelayServerError {
    fn into_response(self) -> Response {
        match self {
            RelayServerError::BadDataFormat => {
                (StatusCode::BAD_REQUEST, "Bad data format".to_string()).into_response()
            }
            RelayServerError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
        }
    }
}
