use reqwest::blocking::Client;
use serde_json::{self, Value};
use std::{collections::HashMap, vec};

#[derive(Debug)]
pub struct LoginResult {
    pub session_id: String,
    pub username: String,
    pub roles: Vec<String>,
}

pub fn login(url: &str, username: &str, password: &str) -> Option<LoginResult> {
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
            let v: Value =
                serde_json::from_str(&response.text().expect("Unable to read response contents"))
                    .expect("Unknown response structure");

            let id_len = v["sessionId"].to_string().len();
            Some(LoginResult {
                session_id: v["sessionId"].to_string()[1..id_len - 1].to_string(),
                username: v["username"].to_string(),
                roles: vec![], //TODO if ever needed
            })
        }
        reqwest::StatusCode::UNAUTHORIZED => None,
        _ => {
            panic!("Something went wrong");
        }
    }
}
