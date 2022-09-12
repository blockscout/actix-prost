use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Pattern {
    Get(String),
    Put(String),
    Post(String),
    Delete(String),
    Patch(String),
    // Custom(String, String) // TODO
}

impl Pattern {
    pub fn path(&self) -> &String {
        match self {
            Self::Get(p) => p,
            Self::Put(p) => p,
            Self::Post(p) => p,
            Self::Delete(p) => p,
            Self::Patch(p) => p,
        }
    }

    pub fn method(&self) -> &str {
        match self {
            Self::Get(_) => "get",
            Self::Put(_) => "put",
            Self::Post(_) => "post",
            Self::Delete(_) => "delete",
            Self::Patch(_) => "patch",
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct HttpRule {
    pub selector: String,
    #[serde(flatten)]
    pub pattern: Pattern,
    pub body: Option<String>,
    pub response_body: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Http {
    pub rules: Vec<HttpRule>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub http: Http,
}
