use serde::{Deserialize, Serialize};
use std::vec;

use crate::client::Client;

pub struct Command {
    pub name: String,
    pub action: fn(calling_client: &Client, args: &[String]) -> (),
    pub arg_descriptions: Vec<String>,
    pub short_description: String,
}
impl Command {
    pub fn print_help(&self) {
        println!("{}", self.short_description);
        for arg in &self.arg_descriptions {
            println!("{}", arg);
        }
    }
}

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
