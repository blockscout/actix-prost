use tonic::IntoRequest;

pub fn new_request<T>(data: T, req: &actix_web::HttpRequest) -> tonic::Request<T> {
    let mut request = data.into_request();
    *request.metadata_mut() = crate::map_headers(req.headers());
    request
}
