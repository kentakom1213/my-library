use http::StatusCode;
use worker::{Response, RouteContext};

use crate::database::models::{self, BookAuthorsIDResponse, BookAuthorsResponse, BooksResponse};

/// booksに保存されている書籍情報の一覧を取得する
pub async fn get_books(ctx: RouteContext<()>) -> worker::Result<Response> {
    // データベースに接続
    let d1 = ctx.env.d1("DB")?;

    // クエリ
    let statement = d1.prepare(
        "
        SELECT
            *
        FROM
            books
        ORDER BY
            id
        ",
    );

    // 結果の取得
    let result = statement.all().await?;

    let books = match result.results::<models::BooksResponse>() {
        Ok(res) => res,
        Err(err) => {
            tracing::error!("Failed to fetch books: {}", err);
            return Response::error(
                "Internal Server Error",
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            );
        }
    };

    // 著者の取得
    let authors_statement = d1.prepare(
        "
        SELECT
            name_, book_id
        FROM
            authors AS a
        JOIN
            book_authors AS ba
            ON
                a.id = ba.author_id
        ORDER BY
            book_id
        ",
    );

    let result = authors_statement.all().await?;

    let authors = match result.results::<BookAuthorsIDResponse>() {
        Ok(res) => res,
        Err(err) => {
            tracing::error!("Failed to fetch authors: {}", err);
            return Response::error(
                "Internal Server Error",
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            );
        }
    };

    // 結果のマージ
    let mut book_authors: Vec<BookAuthorsResponse> = vec![];
    let mut authors = authors.into_iter().peekable();

    for book in books {
        let BooksResponse {
            id,
            title,
            published_date,
            isbn,
            thumbnail_url,
            description,
        } = book;

        // IDが一致しているauthorを取得
        let mut matched_authors = vec![];
        while let Some(a) = authors.peek() {
            if a.book_id == id {
                let author = authors.next().unwrap();
                matched_authors.push(author.name);
            } else {
                break;
            }
        }

        book_authors.push(BookAuthorsResponse {
            id,
            title,
            published_date,
            isbn,
            thumbnail_url,
            description,
            authors: matched_authors,
        });
    }

    // タイトルでソート
    book_authors.sort_by(|a, b| a.title.cmp(&b.title));

    Response::from_json(&book_authors)
}
