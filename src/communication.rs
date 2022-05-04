use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::{collections::HashMap, result::Result, vec};

#[derive(Debug)]
pub struct LoginResult {
    pub session_id: String,
    pub username: String,
    pub roles: Vec<String>,
}

impl LoginResult {
    pub fn new() -> LoginResult {
        LoginResult {
            session_id: String::new(),
            username: String::new(),
            roles: vec![],
        }
    }
}
#[derive(Deserialize, Debug, Serialize)]
pub struct ProductOverview {
    #[serde(rename = "productId")]
    pub id: String,
    pub name: String,
    #[serde(rename = "artistName")]
    pub artist_name: String,
    #[serde(rename = "releaseYear")]
    pub release_year: String,
    #[serde(rename = "smallestPrice")]
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
    Other,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Product {
    #[serde(rename = "productId")]
    pub id: String,
    pub name: String,
    #[serde(rename = "artistName")]
    pub artist_name: String,
    #[serde(rename = "releaseYear")]
    pub release_year: String,
    #[serde(rename = "labelName")]
    pub label_name: String,
    #[serde(rename = "duration")]
    pub duration: String,
    #[serde(rename = "genre")]
    pub genre: String,
    pub songs: Vec<Song>,
    #[serde(rename = "soundCarriers")]
    pub carriers: Vec<Carrier>,
}

impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut songs_string = String::new();
        for s in &self.songs {
            songs_string.push_str(format!("{}\n", s).as_str())
        }

        let mut carriers_string = String::new();
        for c in &self.carriers {
            carriers_string.push_str(format!("{}\n", c).as_str());
        }

        write!(
            f,
            "\nProduct:\n\
            Name: {} \n\
            From: {} \n\
            Year: {} \n\
            Label: {} \n\
            Duration: {} \n\
            Genre: {}
            \n\
            Songs: \n\
            {}
            \n\
            SoundCarriers:\n\
            {}",
            self.name,
            self.artist_name,
            self.release_year,
            self.label_name,
            self.duration,
            self.genre,
            songs_string,
            carriers_string
        )
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Song {
    pub title: String,
    pub duration: String,
}

impl std::fmt::Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.title, self.duration)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Carrier {
    #[serde(rename = "soundCarrierName")]
    pub name: String,
    #[serde(rename = "amountAvailable")]
    pub amount: i32,
    #[serde(rename = "pricePerCarrier")]
    pub price: f32,
}

impl std::fmt::Display for Carrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}, {} available at {}€",
            self.name, self.amount, self.price
        )
    }
}

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
