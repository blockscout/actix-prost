use std::str::FromStr;

pub fn to_actix_status(status: http::StatusCode) -> actix_web::http::StatusCode {
    actix_web::http::StatusCode::from_u16(status.as_u16()).expect("status codes are always valid")
}

pub fn from_actix_header(
    header: (
        actix_http::header::HeaderName,
        actix_http::header::HeaderValue,
    ),
) -> (http::HeaderName, http::HeaderValue) {
    (
        http::HeaderName::from_str(header.0.as_str()).expect("was a valid header name"),
        http::HeaderValue::from_str(header.1.to_str().expect("was a valid header value"))
            .expect("was a valid header value"),
    )
}
