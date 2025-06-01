use worker::{event, Context, Env, Request, Response, Router};

mod books;

#[event(fetch)]
async fn fetch(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    console_error_panic_hook::set_once();

    Router::new()
        .get("/", |_, _| Response::ok("hello"))
        .get_async("/book/:isbn", |_req, ctx| async move {
            if let Some(isbn) = ctx.param("isbn") {
                return books::get_book_by_isbn(isbn).await;
            }
            Response::error("Bad Request", 400)
        })
        .run(req, env)
        .await
}
