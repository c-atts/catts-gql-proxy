use futures_timer::Delay;
use serde::{Deserialize, Serialize};
use serde_json::{json, to_string, Value};
use std::time::Duration;
use worker::*;

use crate::{filter::filter_response_data, header::create_headers, url::create_url};

#[derive(Serialize, Deserialize, Debug)]
pub struct ProxyRequestBody {
    pub query: Option<String>,
    pub variables: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProxyRequest {
    pub url: String,
    pub headers: Option<String>,
    pub filter: Option<String>,
    pub body: Option<ProxyRequestBody>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum ProxyRequestState {
    Idle,
    InProgress,
    Success,
    Error,
}

#[durable_object]
pub struct ProxyRequestDurableObject {
    state: ProxyRequestState,
    response_body: Option<Value>,
    response_code: Option<u16>,
    env: Env,
}

impl ProxyRequestDurableObject {
    async fn fetch_query(&mut self, mut req: Request) -> std::result::Result<Value, String> {
        self.state = ProxyRequestState::InProgress;

        let request_body = req.text().await.map_err(|e| e.to_string())?;
        let recipe_request: ProxyRequest =
            serde_json::from_str(&request_body).map_err(|e| e.to_string())?;
        let proxy_headers = create_headers(&recipe_request, &self.env)?;
        let proxy_url = create_url(&recipe_request, &self.env);
        let mut init = RequestInit::new();
        init.with_headers(proxy_headers);

        if let Some(body) = recipe_request.body {
            init.with_method(Method::Post);
            let body = to_string(&body).map_err(|e| e.to_string())?;
            init.with_body(Some(body.into()));
        } else {
            init.with_method(Method::Get);
        }

        let request = Request::new_with_init(&proxy_url, &init).map_err(|e| e.to_string())?;
        let response = Fetch::Request(request).send().await;

        match response {
            Ok(mut response) => {
                let mut response_body = response.json().await.map_err(|e| e.to_string())?;

                if let Some(filter) = &recipe_request.filter {
                    response_body = filter_response_data(&response_body, filter)?;
                }

                Ok(response_body)
            }
            Err(e) => Err(format!("Error fetching data: {}", e)),
        }
    }
}

#[durable_object]
impl DurableObject for ProxyRequestDurableObject {
    fn new(state: State, env: Env) -> Self {
        Self {
            state: ProxyRequestState::Idle,
            response_body: None,
            response_code: None,
            env,
        }
    }

    async fn fetch(&mut self, req: Request) -> Result<Response> {
        if self.state == ProxyRequestState::Idle {
            let query_result = self.fetch_query(req).await;
            match query_result {
                Ok(response_body) => {
                    self.state = ProxyRequestState::Success;
                    self.response_body = Some(response_body);
                }
                Err(e) => {
                    self.state = ProxyRequestState::Error;
                    self.response_code = Some(400);
                    self.response_body = Some(json!({
                        "message": e,
                    }));
                }
            }
        };

        if self.state == ProxyRequestState::InProgress {
            Delay::new(Duration::from_millis(100)).await;
        };

        match &self.response_body {
            Some(response_body) => {
                let response = Response::from_json(&response_body.clone())?;

                let mut headers = Headers::new();
                headers.append("Access-Control-Allow-Origin", "*")?;

                match &self.state {
                    ProxyRequestState::Success => {
                        Ok(response.with_headers(headers).with_status(200))
                    }
                    ProxyRequestState::Error => Ok(response.with_headers(headers).with_status(400)),
                    _ => unreachable!(),
                }
            }
            None => Response::error("Internal error", 500),
        }
    }
}
