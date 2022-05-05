use crate::model::{LoginResult, Product, ProductOverview, Reason};
use reqwest::blocking::Client;
use serde_json::{self, Value};
use std::{collections::HashMap, result::Result, vec};

pub fn get_product(url: &str, session_id: &str, id: &str) -> Option<Product> {
    let endpoint = format!("/products/{}", id);
    let response = Client::new()
        .get(format!("{}{}", url, endpoint))
        .header("session-id", session_id)
        .send()
        .expect("Error while sending request")
        .text()
        .expect("Unable to read response");

    match serde_json::from_str(&response) {
        Ok(product) => Some(product),
        Err(_) => None,
    }
}

pub fn search(url: &str, session_id: &str, query: &str) -> Vec<ProductOverview> {
    let endpoint = format!("/products?search={}", query);
    let client = Client::new();
    let response = client
        .get(format!("{}{}", url, endpoint))
        .header("session-id", session_id)
        .send()
        .expect("Unable to send request")
        .text()
        .expect("Unable to read response");

    serde_json::from_str(&response).unwrap()
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
