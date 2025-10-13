use crate::http_compatibility::from_actix_header;

pub fn map_actix_headers(headers: &actix_http::header::HeaderMap) -> http::HeaderMap {
    http::HeaderMap::from_iter(
        headers
            .iter()
            .map(|(k, v)| from_actix_header((k.clone(), v.clone()))),
    )
}

pub fn map_headers(headers: &actix_http::header::HeaderMap) -> tonic::metadata::MetadataMap {
    tonic::metadata::MetadataMap::from_headers(map_actix_headers(headers))
}
