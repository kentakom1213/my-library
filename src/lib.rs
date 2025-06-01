use axum::{routing::get, Router};
use tower_service::Service;
use worker::{event, Context, Env, HttpRequest};

mod books;

fn router() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/books/isbn/:isbn", get(books::get_book_by_isbn))
}

#[event(fetch)]
async fn fetch(
    req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> worker::Result<axum::http::Response<axum::body::Body>> {
    console_error_panic_hook::set_once();
    Ok(router().call(req).await?)
}

pub async fn root() -> &'static str {
    "Hello Axum!"
}
