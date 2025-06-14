use serde::{Deserialize, Serialize};

use crate::utility::deserialize_published_date_from_any;

#[derive(Debug, Deserialize, Serialize)]
pub struct AddBookResponse {
    pub message: String,
    pub isbn: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BooksResponse {
    pub id: String,
    pub title: String,
    #[serde(deserialize_with = "deserialize_published_date_from_any")]
    pub published_date: String,
    pub isbn: String,
    pub thumbnail_url: Option<String>,
    #[serde(alias = "description_")]
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorsResponse {
    pub id: u32,
    #[serde(alias = "name_")]
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct BookAuthorsResponse {
    pub id: String,
    pub title: String,
    pub published_date: String,
    pub isbn: String,
    pub thumbnail_url: Option<String>,
    #[serde(alias = "description_")]
    pub description: String,
    pub authors: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BookAuthorsIDResponse {
    pub book_id: String,
    #[serde(alias = "name_")]
    pub name: String,
}
