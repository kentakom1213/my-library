use http::StatusCode;
use worker::{Response, RouteContext};

use crate::database::models;

/// booksに保存されている書籍情報の一覧を取得する
pub async fn get_books(ctx: RouteContext<()>) -> worker::Result<Response> {
    // データベースに接続
    let d1 = ctx.env.d1("DB")?;

    // クエリ
    let statement = d1.prepare("SELECT * FROM books");

    // 結果の取得
    let result = statement.all().await?;

    match result.results::<models::BooksResponse>() {
        Ok(res) => Response::from_json(&res),
        Err(err) => {
            tracing::error!("Failed to fetch books: {}", err);
            Response::error(
                "Internal Server Error",
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            )
        }
    }
}
