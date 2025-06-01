use axum::{extract::Path, http::StatusCode, response::IntoResponse, Json};
use reqwest::Client;

use crate::books::models::BooksApiResponse;

/// Google Books APIのベースURL
const API_BASE_URL: &str = "https://www.googleapis.com/books/v1/volumes";

/// ISBNを使ってGoogle Books APIから書籍情報を取得するハンドラ
pub async fn get_book_by_isbn(isbn: String) -> impl IntoResponse {
    // URLを生成
    let url = format!("{}?q=isbn:{}", API_BASE_URL, isbn);

    tracing::debug!("Fetching from Google Books API: {}", url);

    // reqwestを使ってAPIを呼び出す
    let response = Client::new()
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .await;

    match response {
        Ok(res) => {
            if res.status().is_success() {
                let body = res.json::<BooksApiResponse>().await;
                match body {
                    Ok(book_data) => Ok(Json(book_data)),
                    Err(e) => {
                        eprintln!("Failed to parse response body: {:?}", e);
                        Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to parse API response: {}", e),
                        ))
                    }
                }
            } else {
                let status = res.status();
                let error_text = res
                    .text()
                    .await
                    .unwrap_or_else(|_| "Unknown error".to_string());
                eprintln!(
                    "Google Books API returned error status: {} - {}",
                    status, error_text
                );
                Err((status, format!("Google Books API error: {}", error_text)))
            }
        }
        Err(e) => {
            eprintln!("Failed to send request to Google Books API: {:?}", e);
            Err((
                StatusCode::SERVICE_UNAVAILABLE, // 外部サービスエラー
                format!("Failed to connect to Google Books API: {}", e),
            ))
        }
    }
}
