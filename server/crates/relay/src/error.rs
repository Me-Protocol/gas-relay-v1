use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum RelayServerError {
    BadDataFormat,
    BadRequest(String),
    DatabaseError(String),
    ProcessingError(String),
    ChannelError(String),
}

impl IntoResponse for RelayServerError {
    fn into_response(self) -> Response {
        match self {
            RelayServerError::BadDataFormat => {
                (StatusCode::BAD_REQUEST, "Bad data format".to_string()).into_response()
            }
            RelayServerError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            RelayServerError::DatabaseError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
            RelayServerError::ProcessingError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
            RelayServerError::ChannelError(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response(),
        }
    }
}
