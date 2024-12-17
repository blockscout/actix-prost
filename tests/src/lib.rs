#[allow(clippy::derive_partial_eq_without_eq)]
pub mod proto;

#[cfg(test)]
mod test;

#[cfg(test)]
mod macros;

#[cfg(test)]
mod rest;

#[cfg(test)]
mod types;

#[cfg(test)]
mod errors;

#[cfg(test)]
mod conversions;

#[cfg(test)]
mod snake_case_types;

#[cfg(test)]
mod serde_overrides;

#[cfg(test)]
async fn assert_ping(addr: &std::net::SocketAddr, path: &str, body: String) {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://localhost:{}{}", addr.port(), path))
        .body(body.clone())
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    let body: serde_json::Value = serde_json::from_str(&body).unwrap();
    let resp: serde_json::Value = serde_json::from_str(&resp)
        .unwrap_or_else(|_| panic!("could not parse json, got: {}", resp));
    pretty_assertions::assert_eq!(body, resp);
}
