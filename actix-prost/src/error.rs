pub fn map_tonic_code(code: tonic::Code) -> http::StatusCode {
    use http::StatusCode;
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

pub fn map_tonic_error(status: tonic::Status) -> actix_web::Error {
    let code = status.code();
    actix_web::error::InternalError::new(status, map_tonic_code(code)).into()
}
