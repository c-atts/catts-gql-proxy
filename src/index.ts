import { DurableObject } from "cloudflare:workers";
import { createHeaders } from "./header";
import { createUrl } from "./url";
import * as jsonPath from "jsonpath";

export type EnvVars = {
  THEGRAPH_API_KEY: string;
  MORALIS_API_KEY: string;
  DUNE_API_KEY: string;
};

type Env = {
  CATTS_QUERY_PROXY: DurableObjectNamespace<ProxyRequestDurableObject>;
} & EnvVars;

export type ProxyRequestBody = {
  query?: string;
  variables?: any;
};

export type ProxyRequest = {
  url: string;
  headers?: Record<string, string>;
  filter?: string;
  body?: ProxyRequestBody;
};

const enum RpcProxyState {
  NotStarted,
  InProgress,
  Done,
}

// ProxyRequestDurableObject is a Durable Object that proxies and deduplicates requests to external APIs.
// Read more about Durable Objects here: https://developers.cloudflare.com/durable-objects/
export class ProxyRequestDurableObject extends DurableObject {
  envVars: EnvVars;
  state: RpcProxyState = RpcProxyState.NotStarted;
  responseText?: string;
  responseStatus?: number;

  constructor(ctx: DurableObjectState, env: Env) {
    super(ctx, env);
    this.envVars = {
      THEGRAPH_API_KEY: env.THEGRAPH_API_KEY,
      MORALIS_API_KEY: env.MORALIS_API_KEY,
      DUNE_API_KEY: env.DUNE_API_KEY,
    };
  }

  async proxyRequest(request: Request): Promise<Response> {
    if (this.state === RpcProxyState.NotStarted) {
      this.state = RpcProxyState.InProgress;

      const proxyRequest: ProxyRequest = await request.json();
      const headers = createHeaders(proxyRequest, this.envVars);
      const url = createUrl(proxyRequest, this.envVars);

      try {
        if (proxyRequest.body) {
          console.log("POST", url, JSON.stringify(proxyRequest.body));
          const response = await fetch(url, {
            method: "POST",
            headers,
            body: JSON.stringify(proxyRequest.body),
          });
          this.responseText = await response.text();
          this.responseStatus = response.status;
        } else {
          const response = await fetch(url, {
            method: "GET",
            headers,
          });

          let json = await response.json();
          if (proxyRequest.filter) {
            json = jsonPath.query(json, proxyRequest.filter);
          }
          this.responseText = JSON.stringify(json);
          this.responseStatus = response.status;
        }
      } catch (e) {
        console.error(e);
        this.responseText = "Couldn't fetch data";
        this.responseStatus = 500;
      }
      this.state = RpcProxyState.Done;
    }

    if (this.state === RpcProxyState.InProgress) {
      while (this.state === RpcProxyState.InProgress) {
        await new Promise((resolve) => setTimeout(resolve, 100));
      }
    }

    return new Response(this.responseText, {
      status: this.responseStatus,
      headers: { "Access-Control-Allow-Origin": "*" },
    });
  }
}

// The fetch event handler for the worker initiates the RPC request to the Durable Object.
// The Durable Object is fetched by its name, which is derived from the request ID.
// One instance of the Durable Object is created for each request ID.
export default {
  async fetch(
    request: Request,
    env: Env,
    _ctx: ExecutionContext,
  ): Promise<Response> {
    switch (request.method) {
      case "OPTIONS":
        return new Response(null, {
          headers: {
            "Access-Control-Allow-Origin": "*",
            "Access-Control-Allow-Methods": "GET, POST, OPTIONS",
          },
        });
      case "GET":
        return new Response("Method not allowed", { status: 405 });
      case "POST":
        let id: DurableObjectId = env.CATTS_QUERY_PROXY.idFromName(request.url);
        let stub = env.CATTS_QUERY_PROXY.get(id);
        return stub.proxyRequest(request);
    }
    return new Response("Bad request", { status: 400 });
  },
};
