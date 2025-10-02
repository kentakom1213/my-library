//! 認証関係の実装

use worker::{console_error, Request, RouteContext};

pub fn require_auth(req: &Request, ctx: &RouteContext<()>) -> worker::Result<()> {
    // 環境変数から正しいトークンを取得
    let expected_token = match ctx.env.secret("API_TOKEN") {
        Ok(token) => token.to_string(),
        Err(e) => {
            console_error!("'API_TOKEN' secret not configured: {e}");
            return Err(worker::Error::from("Server configuration error."));
        }
    };

    // リクエストヘッダから認証情報を取得
    let Some(auth_header) = req.headers().get("Authorization")? else {
        return Err(worker::Error::from("Missing Authorization header."));
    };

    // トークンの検証
    if let Some(token) = auth_header.strip_prefix("Bearer ") {
        if token == expected_token {
            return Ok(());
        }
    }

    Err(worker::Error::from("Invalid or malformed token."))
}
