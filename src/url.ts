import { EnvVars, ProxyRequest } from ".";

type UrlTransformer = (url: string, vars: EnvVars) => string;

function urlTransformerTheGraph(url: string, vars: EnvVars) {
  return url.replace("{api-key}", vars.THEGRAPH_API_KEY);
}

const URL_TRANSFORMERS: { regex: string; urlTransformer: UrlTransformer }[] = [
  {
    regex: "^https://([a-zA-Z0-9-]+\\.)*thegraph\\.com",
    urlTransformer: urlTransformerTheGraph,
  },
];

export function createUrl(proxyRequest: ProxyRequest, vars: EnvVars): string {
  for (const transformer of URL_TRANSFORMERS) {
    if (new RegExp(transformer.regex).test(proxyRequest.url)) {
      return transformer.urlTransformer(proxyRequest.url, vars);
    }
  }
  return proxyRequest.url;
}
