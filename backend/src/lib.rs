use http::StatusCode;
use tracing_subscriber::{
    fmt::{format::Pretty, time::UtcTime},
    prelude::*,
};
use tracing_web::{performance_layer, MakeConsoleWriter};
use worker::{event, Context, Cors, Env, Method, Request, Response, Router};

use crate::auth::require_auth;

mod auth;
mod books;
mod database;
mod utility;

#[event(start)]
fn start() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(false)
        // 日本時間に設定
        .with_timer(UtcTime::rfc_3339())
        .with_writer(MakeConsoleWriter);
    let perf_layer = performance_layer().with_details_from_fields(Pretty::default());
    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(perf_layer)
        .init();
}

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    console_error_panic_hook::set_once();

    let cors = Cors::default()
        .with_origins([
            // "http://localhost:8080",
            "https://library.pwll.dev",
            // "https://library.kentakom1213.workers.dev",
        ])
        .with_methods([Method::Get, Method::Post, Method::Options])
        .with_allowed_headers(["Content-Type", "Authorization"]);

    let resp = Router::new()
        .get("/", |_, _| Response::ok("hello"))
        .options("/book/:isbn", |_, _| Response::empty())
        .post_async("/book/:isbn", |req, ctx| async move {
            // 認証情報の検証
            if let Err(e) = require_auth(&req, &ctx) {
                tracing::warn!("Authentication failed: {e}");
                return Response::error(e.to_string(), StatusCode::UNAUTHORIZED.as_u16());
            }

            if let Some(isbn) = ctx.param("isbn").cloned() {
                return database::add_book(ctx, &isbn).await;
            }
            Response::error("Bad Request", StatusCode::BAD_REQUEST.as_u16())
        })
        .get_async("/books", |_req, ctx| async move {
            database::get_books(ctx).await
        })
        .run(req, env)
        .await?
        .with_cors(&cors)?;

    Ok(resp)
}
