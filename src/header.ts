import { EnvVars, ProxyRequest } from ".";

type HeaderAdder = (headers: Headers, vars: EnvVars) => void;

function headerAdderMoralis(headers: Headers, vars: EnvVars) {
  headers.set("X-API-Key", vars.MORALIS_API_KEY);
}

function headerAdderDune(headers: Headers, vars: EnvVars) {
  headers.set("X-Dune-API-Key", vars.DUNE_API_KEY);
}

const HEADER_ADDERS: { regex: string; headerAdder: HeaderAdder }[] = [
  {
    regex: "^https://([a-zA-Z0-9-]+\\.)*moralis\\.io",
    headerAdder: headerAdderMoralis,
  },
  {
    regex: "^https://api\\.dune\\.com",
    headerAdder: headerAdderDune,
  },
];


export function createHeaders(proxyRequest: ProxyRequest, vars: EnvVars) {
  const headers = new Headers();
  headers.set("Content-Type", "application/json");
  headers.set("Accept", "application/json");
  headers.set("User-Agent", "c-atts/0.0.1");

  if (proxyRequest.headers) {
    for (const [key, value] of Object.entries(proxyRequest.headers)) {
      headers.set(key, value);
    }
  }

  for (const adder of HEADER_ADDERS) {
    try {
      const matches = new RegExp(adder.regex).test(proxyRequest.url);
      if (matches) {
        adder.headerAdder(headers, vars);
      }
    } catch (err) {
      console.error("Error in headerAdder:", err);
    }
  }

  return headers;
}
