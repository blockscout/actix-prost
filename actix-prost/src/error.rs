use actix_http::body::BoxBody;
use actix_web::{error, HttpResponse, ResponseError};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, FromInto};
use std::fmt::Display;
use tonic::{Code, Status};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Error {
    #[serde_as(as = "FromInto<i32>")]
    pub code: Code,
    pub message: String,
}

impl Error {
    pub fn map_tonic_code(code: Code) -> StatusCode {
        use tonic::Code::*;
        match code {
            Ok => StatusCode::OK,
            Cancelled => StatusCode::from_u16(499).unwrap(),
            Unknown => StatusCode::INTERNAL_SERVER_ERROR,
            InvalidArgument => StatusCode::BAD_REQUEST,
            DeadlineExceeded => StatusCode::GATEWAY_TIMEOUT,
            NotFound => StatusCode::NOT_FOUND,
            AlreadyExists => StatusCode::CONFLICT,
            PermissionDenied => StatusCode::FORBIDDEN,
            ResourceExhausted => StatusCode::TOO_MANY_REQUESTS,
            FailedPrecondition => StatusCode::BAD_REQUEST,
            Aborted => StatusCode::CONFLICT,
            OutOfRange => StatusCode::BAD_REQUEST,
            Unimplemented => StatusCode::NOT_IMPLEMENTED,
            Internal => StatusCode::INTERNAL_SERVER_ERROR,
            Unavailable => StatusCode::SERVICE_UNAVAILABLE,
            DataLoss => StatusCode::INTERNAL_SERVER_ERROR,
            Unauthenticated => StatusCode::UNAUTHORIZED,
        }
    }

    pub fn from_actix(err: error::Error, code: Code) -> Self {
        Self {
            code,
            message: err.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        Self::map_tonic_code(self.code)
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        let body = serde_json::to_string(self);
        match body {
            Ok(body) => HttpResponse::build(self.status_code())
                .content_type("application/json")
                .body(body),
            Err(err) => {
                let body = format!(
                    r#"{{"code":{}, "message":"while serializing error another error happened: {}, original error: {}"}}"#,
                    i32::from(self.code),
                    err,
                    self.message
                );
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .content_type("application/json")
                    .body(body)
            }
        }
    }
}

impl From<Status> for Error {
    fn from(value: Status) -> Self {
        Self {
            code: value.code(),
            message: value.message().to_owned(),
        }
    }
}
