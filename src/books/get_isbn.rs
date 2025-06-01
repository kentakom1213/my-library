use reqwest::Client;
use worker::Response;

use crate::books::models::BooksApiResponse;

/// Google Books APIのベースURL
const API_BASE_URL: &str = "https://www.googleapis.com/books/v1/volumes";

/// ISBNを使ってGoogle Books APIから書籍情報を取得するハンドラ
pub async fn get_book_by_isbn(isbn: &str) -> worker::Result<Response> {
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
        Ok(resp) => {
            if resp.status().is_success() {
                // レスポンスをJSONとしてパース
                match resp.json::<BooksApiResponse>().await {
                    Ok(BooksApiResponse {
                        items: Some(items), ..
                    }) if items.len() == 1 => {
                        tracing::debug!("items[0]:{:#?}", items[0]);
                        // 成功した場合はJSONレスポンスを返す
                        worker::Response::from_json(&items)
                    }
                    Ok(_) => {
                        tracing::warn!("No book found for ISBN: {}", isbn);
                        worker::Response::error("Book not found", 404)
                    }
                    Err(err) => {
                        tracing::error!("Failed to parse JSON: {}", err);
                        worker::Response::error("Internal Server Error", 500)
                    }
                }
            } else {
                tracing::error!("API request failed with status: {}", resp.status());
                worker::Response::error("Not Found", 404)
            }
        }
        Err(err) => {
            tracing::error!("Request error: {}", err);
            worker::Response::error("Internal Server Error", 500)
        }
    }
}
