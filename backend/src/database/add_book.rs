use http::StatusCode;
use worker::{D1Type, Response, RouteContext};

use crate::{
    books,
    database::models::{AddBookResponse, AuthorsResponse},
    utility::normalize_name,
};

pub async fn add_book(ctx: RouteContext<()>, isbn: &str) -> worker::Result<Response> {
    // 書籍情報を取得
    let book_info = match books::get_book_by_isbn(isbn).await {
        Ok(info) => info,
        Err(err) => return Response::error(err, StatusCode::TOO_MANY_REQUESTS.as_u16()),
    };

    // 1冊の場合に限定
    let book = match book_info.total_items {
        0 => {
            tracing::warn!("No book found for this ISBN.");
            return Response::from_json(&AddBookResponse {
                message: "No book found for this ISBN.".to_string(),
                isbn: None,
            });
        }
        1 => book_info
            .items
            .expect("Some item should be contained.")
            .into_iter()
            .next()
            .expect("Some item should be contained."),
        _ => {
            tracing::warn!("Several books corresponded to this ISBN.");
            return Response::from_json(&AddBookResponse {
                message: "Several books corresponded to this ISBN.".to_string(),
                isbn: None,
            });
        }
    };

    // データベースを取得
    let d1 = ctx.env.d1("DB")?;

    // ===== 1. 本を挿入 =====
    let query_insert_book = d1
        .prepare(
            "
        INSERT INTO books
            (id, title, published_date, isbn, thumbnail_url, description_)
        VALUES
            (?1, ?2, ?3, ?4, ?5, ?6);
        ",
        )
        .bind(&[
            book.id.clone().into(),
            book.volume_info.title.into(),
            book.volume_info.published_date.into(),
            isbn.into(),
            book.volume_info
                .image_links
                .map(|link| link.small_thumbnail)
                .into(),
            book.volume_info.description.into(),
        ])?;

    // クエリを実行
    // 失敗した場合，エラーを返す
    if let Err(err) = query_insert_book.run().await {
        tracing::error!("Failed to insert book: {}", err);
        // 原因の特定
        if err.to_string().contains("Error: UNIQUE constraint failed") {
            return Response::from_json(&AddBookResponse {
                message: "This book is already registered.".to_string(),
                isbn: Some(isbn.to_string()),
            });
        } else {
            return Response::error(
                "Internal Server Error",
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            );
        }
    }

    // ===== 2. 著者を挿入 =====
    let authors = if let Some(mut authors) = book.volume_info.authors.filter(|a| !a.is_empty()) {
        // 著者名を正規化
        authors
            .iter_mut()
            .for_each(|author| *author = normalize_name(author));

        let _args: Vec<D1Type> = authors.iter().map(|author| D1Type::Text(author)).collect();
        let author_args: Vec<&D1Type> = _args.iter().collect();

        let query_insert_authors = d1
            .prepare(
                "
                INSERT OR IGNORE INTO authors
                    (name_)
                VALUES
                    (?1);
        ",
            )
            .batch_bind(author_args.clone())?;

        if let Err(err) = d1.batch(query_insert_authors).await {
            tracing::error!("Failed to insert authors: {}", err);
            return Response::error(
                "Internal Server Error",
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            );
        }

        // 著者IDを取得
        let query_get_authors = d1
            .prepare(
                "
                SELECT * FROM authors
                WHERE name_ = ?1 ;
        ",
            )
            .batch_bind(author_args)?;

        match d1.batch(query_get_authors).await {
            Ok(results) => results
                .into_iter()
                .flat_map(|result| result.results::<AuthorsResponse>())
                .flatten()
                .collect(),
            Err(err) => {
                tracing::error!("Failed to fetch authors: {}", err);
                return Response::error(
                    "Internal Server Error",
                    StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                );
            }
        }
    } else {
        vec![]
    };

    tracing::info!("authors: {:?}", authors);

    // ===== 3. 本と著者の関連を挿入 =====
    if !authors.is_empty() {
        let book_authors: Vec<[D1Type; 2]> = authors
            .iter()
            .map(|a| [D1Type::Text(&book.id), D1Type::Integer(a.id as i32)])
            .collect();

        let query_insert_book_authors = d1
            .prepare(
                "
            INSERT INTO book_authors
                (book_id, author_id)
            VALUES
                (?1, ?2);
        ",
            )
            .batch_bind(book_authors.iter().collect::<Vec<_>>())?;

        if let Err(err) = d1.batch(query_insert_book_authors).await {
            tracing::error!("Failed to insert book-authors relation: {}", err);
            return Response::error(
                "Internal Server Error",
                StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            );
        }
    }

    Response::from_json(&AddBookResponse {
        message: "Book added successfully.".to_string(),
        isbn: Some(isbn.to_string()),
    })
}
