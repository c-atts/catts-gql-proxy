use regex::Regex;
use std::{collections::HashMap, sync::LazyLock, vec::Vec};
use worker::{Env, Headers};

use crate::proxy_request::ProxyRequest;

type HeaderAdder = fn(Headers, &Env) -> Headers;

fn header_adder_moralis(mut headers: Headers, env: &Env) -> Headers {
    let api_key = env.secret("MORALIS_API_KEY").unwrap();
    headers.append("X-API-Key", &api_key.to_string()).unwrap();
    headers
}

static HEADER_ADDERS: LazyLock<Vec<(Regex, HeaderAdder)>> = LazyLock::new(|| {
    vec![(
        Regex::new(r"^(https?:\/\/)?([a-zA-Z0-9-]+\.)*moralis\.io/api(\/|$)").unwrap(),
        header_adder_moralis as HeaderAdder,
    )]
});

/// Creates the headers for the request, including any headers needed for specific providers.
pub fn create_headers(proxy_request: &ProxyRequest, env: &Env) -> Result<Headers, String> {
    let mut headers = Headers::new();
    headers.append("Content-Type", "application/json").unwrap();
    headers.append("User-Agent", "c-atts/0.0.1").unwrap();

    if let Some(request_headers) = proxy_request.headers.to_owned() {
        let request_headers: HashMap<String, String> =
            serde_json::from_str(&request_headers).map_err(|e| e.to_string())?;
        for (key, value) in request_headers {
            headers.append(&key, &value).map_err(|e| e.to_string())?;
        }
    }

    for (regex, header_adder) in HEADER_ADDERS.iter() {
        if regex.is_match(proxy_request.url.as_str()) {
            headers = header_adder(headers, env);
            break;
        }
    }

    Ok(headers)
}
