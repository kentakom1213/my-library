use http::StatusCode;
use tracing_subscriber::{
    fmt::{format::Pretty, time::UtcTime},
    prelude::*,
};
use tracing_web::{performance_layer, MakeConsoleWriter};
use worker::{event, Context, Env, Request, Response, Router};

mod books;
mod database;

#[event(start)]
fn start() {
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_ansi(true)
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

    Router::new()
        .get("/", |_, _| Response::ok("hello"))
        .get_async("/book/:isbn", |_req, ctx| async move {
            if let Some(isbn) = ctx.param("isbn") {
                return books::get_book_by_isbn(isbn).await;
            }
            Response::error("Bad Request", StatusCode::BAD_REQUEST.as_u16())
        })
        .get_async(
            "/books",
            |_req, ctx| async move { database::get_books(ctx).await },
        )
        .run(req, env)
        .await
}
