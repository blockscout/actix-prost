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
        http::HeaderValue::from_bytes(header.1.as_bytes()).expect("was a valid header value"),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_http::header::{HeaderName, HeaderValue};
    use std::str::FromStr;

    #[test]
    fn test_header_names() {
        let a_1 = "a".to_string();
        let a_8192 = "a".repeat(8192);
        let a_8193 = "a".repeat(8193);
        let a_16384 = "a".repeat(16384);
        let a_16385 = "a".repeat(16385);

        let test_cases = vec![
            // Empty
            "",
            // Single characters
            "a",
            "A",
            "-",
            // Common headers
            "content-type",
            "x-custom-header",
            "CONTENT-TYPE",
            "X-CUSTOM-HEADER",
            // Various lengths
            &a_1,
            &a_8192,
            &a_8193,
            &a_16384,
            &a_16385,
            // Special characters
            "header\0name",
            "header\tname",
            "header name",
            "headerñame",
            "header\r\name",
            "header\n\name",
            "header:name",
            // Mixed case
            "Content-Type",
            "X-Mixed-Case",
        ];

        for input in test_cases {
            if let Ok(name) = HeaderName::from_str(input) {
                let dummy_value = HeaderValue::from_static("");
                let _ = from_actix_header((name, dummy_value));
            }
        }
    }

    #[test]
    fn test_header_values() {
        let valid_name = HeaderName::from_static("x-test");

        let nul = '\u{0}'.to_string();
        let us = '\u{1f}'.to_string();
        let del = '\u{7f}'.to_string();
        let a_8192 = "a".repeat(8192);
        let a_16384 = "a".repeat(16384);

        let test_cases = vec![
            // Empty
            "",
            // ASCII range
            &nul, // NUL
            "\t", // TAB
            &us,  // US
            " ",  // space
            "~",  // tilde
            &del, // DEL
            // ASCII printable
            "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789",
            "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
            // Control characters
            "\n",
            "\r",
            "\x0B", // VT
            "\x0C", // FF
            // UTF-8 sequences
            "Hello™",
            "Hello🌍",
            "Привет",
            // Mixed content
            "Hello\nWorld",
            "Hello\0World",
            "Hello\tWorld",
            // Long values
            &a_8192,
            &a_16384,
        ];

        for input in test_cases {
            if let Ok(value) = HeaderValue::from_str(input) {
                let _ = from_actix_header((valid_name.clone(), value));
            }
        }
    }
}
