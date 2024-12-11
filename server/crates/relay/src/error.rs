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
    RequestNotFound,
}

impl IntoResponse for RelayServerError {
    fn into_response(self) -> Response {
        match self {
            RelayServerError::BadDataFormat => {
                (StatusCode::BAD_REQUEST, "Bad data format".to_string()).into_response()
            }
            RelayServerError::BadRequest(msg) => (StatusCode::BAD_REQUEST, msg).into_response(),
            RelayServerError::DatabaseError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
            RelayServerError::ProcessingError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
            RelayServerError::ChannelError(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, msg).into_response()
            }
            RelayServerError::RequestNotFound => {
                (StatusCode::NOT_FOUND, "Request not found".to_string()).into_response()
            }
        }
    }
}
