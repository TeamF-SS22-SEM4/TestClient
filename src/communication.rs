use reqwest::blocking::Client;
use serde_json::{self, Value};
use std::{collections::HashMap, result::Result, vec};

#[derive(Debug)]
pub struct LoginResult {
    pub session_id: String,
    pub username: String,
    pub roles: Vec<String>,
}

pub struct ProductOverview {
    pub id: String,
    pub name: String,
    pub artist_name: String,
    pub release_year: String,
    pub smallest_price: f32,
}

impl std::fmt::Display for ProductOverview {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Album {} by {} released in {} available from {}€",
            self.name, self.artist_name, self.release_year, self.smallest_price
        )
    }
}

#[derive(Debug)]
pub enum Reason {
    InvalidCredentials,
    BadRequest,
    Other,
}

pub fn search(query: &str) -> Vec<ProductOverview> {
    //TODO implement
    vec![ProductOverview {
        id: "1".to_string(),
        name: "A day at the races".to_string(),
        artist_name: "Queen".to_string(),
        release_year: "1969".to_string(),
        smallest_price: 20.0,
    }]
}

pub fn login(url: &str, username: &str, password: &str) -> Result<LoginResult, Reason> {
    let endpoint = "/login";

    let client = Client::new();
    let mut body = HashMap::new();
    body.insert("username", username);
    body.insert("password", password);

    let response = client
        .post(format!("{}{}", url, endpoint))
        .json(&body)
        .send()
        .expect("Failed to send request");

    match response.status() {
        reqwest::StatusCode::OK => {
            let result =
                serde_json::from_str(&response.text().expect("Unable to read response contents"));
            if let Err(_) = result {
                return Err(Reason::Other);
            }
            let v: Value = result.unwrap();
            let id_len = v["sessionId"].to_string().len();

            Ok(LoginResult {
                session_id: v["sessionId"].to_string()[1..id_len - 1].to_string(),
                username: v["username"].to_string(),
                roles: vec![], //TODO if ever needed
            })
        }
        reqwest::StatusCode::FORBIDDEN => Err(Reason::InvalidCredentials),
        _ => Err(Reason::Other),
    }
}
