use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AddBookResponse {
    pub message: String,
    pub isbn: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct BooksResponse {
    pub id: String,
    pub title: String,
    pub published_date: String,
    pub isbn: String,
    pub thumbnail_url: String,
    #[serde(rename = "description_")]
    pub description: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AuthorsResponse {
    pub id: u32,
    #[serde(rename = "name_")]
    pub name: String,
}
