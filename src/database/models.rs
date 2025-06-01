use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BooksResponse {
    id: String,
    title: String,
    published_date: String,
    isbn: String,
    thumbnail_url: String,
    #[serde(rename = "description_")]
    description: String,
}
