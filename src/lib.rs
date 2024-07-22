use worker::*;

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> worker::Result<Response> {
    let router = Router::new();
    router
        .get_async("/:cache_key", |req, ctx| async move {
            handle_graphql_request(req, ctx).await
        })
        .post_async("/:cache_key", |req, ctx| async move {
            handle_graphql_request(req, ctx).await
        })
        .run(req, env)
        .await
}

pub async fn handle_graphql_request(
    mut req: Request,
    ctx: RouteContext<()>,
) -> worker::Result<Response> {
    match ctx.param("cache_key") {
        Some(_) => (),
        None => return Response::error("Parameter 'cache_key' is missing", 400),
    };

    let url = req.url()?;
    let c = Cache::default();
    let cached = c.get(url.as_str(), false).await?;
    if let Some(response) = cached {
        return Ok(response);
    }

    let headers = req.headers();
    let query_url = match headers.get("x-gql-query-url") {
        Ok(url) => match url {
            Some(url) => url,
            None => return Response::error("Header 'x-gql-query-url' is missing", 400),
        },
        Err(err) => return Response::error(err.to_string(), 400),
    };
    let query_url = match process_query_url(&query_url, &ctx) {
        Ok(url) => url,
        Err(err) => return Response::error(err.to_string(), 400),
    };

    let body = req.text().await?;

    let mut headers = Headers::new();
    headers.append("Content-Type", "application/json")?;
    headers.append("User-Agent", "c-atts/0.0.1")?;

    let mut init = RequestInit::new();
    init.with_headers(headers);
    init.with_method(Method::Post);
    init.with_body(Some(body.into()));

    let request = Request::new_with_init(&query_url, &init)?;
    let response = Fetch::Request(request).send().await;

    match response {
        Ok(mut response) => {
            let cloned_response = response.cloned()?;
            c.put(url.as_str(), cloned_response).await?;
            Ok(response)
        }
        Err(e) => Response::error(format!("Error fetching data: {}", e), 500),
    }
}

fn process_query_url(url: &str, ctx: &RouteContext<()>) -> Result<String> {
    let mut url = Url::parse(url)?;

    let path = match url.domain() {
        Some(domain) => {
            if domain.ends_with("thegraph.com") {
                process_the_graph_path(url.path(), ctx)
            } else {
                Ok(url.path().to_string())
            }
        }
        None => return Err("Invalid domain".into()),
    }?;

    url.set_path(&path);

    // Return the modified URL as a String
    Ok(url.to_string())
}

fn process_the_graph_path(path: &str, ctx: &RouteContext<()>) -> Result<String> {
    // Set this secret using `wrangler secret put THEGRAPH_API_KEY`
    let api_key = match ctx.env.secret("THEGRAPH_API_KEY") {
        Ok(key) => key,
        Err(_) => return Err("THEGRAPH_API_KEY secret is missing".into()),
    };

    Ok(path.replace("[api-key]", &api_key.to_string()))
}
