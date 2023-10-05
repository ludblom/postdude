use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request {
    pub url: String,
    pub headers: Vec<Header>,
}

#[derive(Debug, Deserialize)]
pub struct Header {
    pub key: String,
    pub value: String,
}

impl Request {
    pub fn get_request(path: &str) -> Request {
        serde_json::from_str(path).expect("Json not well formatted")
    }
}
