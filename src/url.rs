use regex::Regex;
use std::{sync::LazyLock, vec::Vec};
use worker::Env;

use crate::proxy_request::ProxyRequest;

type UrlTransformer = fn(String, &Env) -> String;

fn url_transformer_thegraph(url: String, env: &Env) -> String {
    let api_key = env.secret("THEGRAPH_API_KEY").unwrap();
    url.replace("{api-key}", &api_key.to_string())
}

static URL_TRANSFORMERS: LazyLock<Vec<(Regex, UrlTransformer)>> = LazyLock::new(|| {
    vec![(
        Regex::new(r"^(https?:\/\/)?([a-zA-Z0-9-]+\.)*thegraph\.com(\/|$)").unwrap(),
        url_transformer_thegraph as UrlTransformer,
    )]
});

pub fn create_url(proxy_request: &ProxyRequest, env: &Env) -> String {
    let mut url = proxy_request.url.to_owned();

    for (regex, url_transformer) in URL_TRANSFORMERS.iter() {
        if regex.is_match(url.as_str()) {
            url = url_transformer(url, env);
            break;
        }
    }

    url
}
