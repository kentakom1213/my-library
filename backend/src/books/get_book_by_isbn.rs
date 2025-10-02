use std::time::Duration;

use reqwest::Client;
use worker::{console_debug, console_error, console_log};

use crate::books::models::BooksApiResponse;

/// Google Books APIのベースURL
const API_BASE_URL: &str = "https://www.googleapis.com/books/v1/volumes";

/// ISBNを使ってGoogle Books APIから書籍情報を取得するハンドラ
pub async fn get_book_by_isbn(isbn: &str) -> Result<BooksApiResponse, &'static str> {
    // URLを生成
    let url = format!("{API_BASE_URL}?q=isbn:{isbn}");

    console_debug!("Fetching from Google Books API: {}", url);

    // reqwestを使ってAPIを呼び出す
    let response = Client::new()
        .get(&url)
        .header("Accept", "application/json")
        .timeout(Duration::from_millis(1000))
        .send()
        .await;

    dbg!(&response);

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                // レスポンスをJSONとしてパース
                match resp.json::<BooksApiResponse>().await {
                    Ok(api_response) => {
                        console_log!("result: {:#?}", api_response);
                        Ok(api_response)
                    }
                    Err(err) => {
                        console_error!("Failed to parse JSON: {}", err);
                        Err("Failed to parse JSON")
                    }
                }
            } else {
                console_error!("API request failed with status: {}", resp.status());
                Err("API Not Found")
            }
        }
        Err(err) => {
            console_error!("Google Books API Request error: {}", err);
            Err("Google Books API Request Error")
        }
    }
}
