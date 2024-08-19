mod filter;
mod header;
mod proxy_request;
mod url;

use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> worker::Result<Response> {
    let router = Router::new();
    router
        .options("/*path", |_req, _ctx| {
            let mut headers = Headers::new();
            headers.append("Access-Control-Allow-Origin", "*")?;
            headers.append("Access-Control-Allow-Methods", "GET, POST, OPTIONS")?;
            Ok(Response::empty()?.with_headers(headers))
        })
        .get_async("/:cache_key", |_, _| async move {
            Response::error("Method not allowed", 405)
        })
        .post_async("/:cache_key", |req, ctx| async move {
            let namespace = ctx.durable_object("CATTS-QUERY-PROXY")?;
            let url = req.url()?;
            let url = url.as_str();
            let stub = namespace.id_from_name(url)?.get_stub()?;
            stub.fetch_with_request(req).await
        })
        .run(req, env)
        .await
}
