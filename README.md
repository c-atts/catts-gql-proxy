# catts-gql-proxy

Runs a Cloudflare Worker that proxies C–ATTS recipe GraphQL queries from the IC smart contract canister (IPv6) to GraphQL endpoints running on IPv4. The worker is written in Rust, compiled to WebAssembly, and deployed to Cloudflare's [edge infrastructure](https://www.cloudflare.com/network/). Requests are cached for 10 minutes to avoid overloading the GraphQL endpoints with the many number of requests coming from the IC canister smart contracts.

## Pre-requisites

The worker is configured using the following environment variables:

- `THEGRAPH_API_KEY`: Used for making authenticated requests to The Graph API.

## Usage

```bash
#!/bin/bash

# Example query that fetches the number of votes for a specific delegate from the ENS Governance subgraph on Arbitrum.

query_url='https://gateway-arbitrum.network.thegraph.com/api/[api-key]/subgraphs/id/GyijYxW9yiSRcEd5u2gfquSvneQKi5QuvU3WZgFyfFSn'

payload='{
  "query": "query Delegate($id: ID!) { delegate(id: $id ) { numberVotes } }",
  "variables": { "id": "0x534631bcf33bdb069fb20a93d2fdb9e4d4dd42cf" }
}'

# Generate a semi-random cache key
random_cache_key="cachekey_$(date +%s)_$RANDOM"

curl -X POST "http://localhost:8787/$random_cache_key" \
     -H "x-gql-query-url: $query_url" \
     -H "Content-Type: application/json" \
     -d "$payload"
```

The worker will forward the request to the GraphQL endpoint in the `x-gql-query-url` header. Special support is provided for the The Graph API, where an API key is required. The worker will replace the `[api-key]` placeholder with the configured API key.

Making requests to the EAS GraphQL endpoints does not require an API key.

```bash
#!/bin/bash

# Example query that fetches the Gitcoin Passport score for a specific address.

query_url='https://optimism.easscan.org/graphql'

payload='{
  "query": "query PassportQuery($where: AttestationWhereInput, $take: Int) { attestations(where: $where, take: $take) { decodedDataJson } }",
  "variables": {
    "where": {
      "schemaId": {
        "equals": "0x6ab5d34260fca0cfcf0e76e96d439cace6aa7c3c019d7c4580ed52c6845e9c89"
      },
      "recipient": {
        "equals": "0xa32aECda752cF4EF89956e83d60C04835d4FA867",
        "mode": "insensitive"
      }
    },
    "take": 1
  }
}'

# Generate a semi-random cache key
random_cache_key="cachekey_$(date +%s)_$RANDOM"

curl -X POST "http://localhost:8787/$random_cache_key" \
     -H "x-gql-query-url: $query_url" \
     -H "Content-Type: application/json" \
     -d "$payload"
```

## Development

### 1. Configure

- Create a `.dev.vars` file in the root of the project.
- Add the following content to the file:

```bash
THEGRAPH_API_KEY=<API_KEY>
```

### 2. Run

```bash
npm i
npm run dev
```

## Production

### 1. Configure

Set the environment variable using:

```bash
npx wrangler secret put THEGRAPH_API_KEY
```

### 2. Deploy

```bash
npm run deploy
```

## Wrangler

Wrangler is used to develop, deploy, and configure your Worker via CLI.

Further documentation for Wrangler can be found [here](https://developers.cloudflare.com/workers/tooling/wrangler).

## WebAssembly

`workers-rs` (the Rust SDK for Cloudflare Workers) is meant to be executed as compiled WebAssembly, and as such so **must** all the code you write and depend upon. All crates and modules used in Rust-based Workers projects have to compile to the `wasm32-unknown-unknown` triple.

Read more about this on the [`workers-rs`](https://github.com/cloudflare/workers-rs) project README.

## Author

- [kristofer@kristoferlund.se](mailto:kristofer@kristoferlund.se)
- Twitter: [@kristoferlund](https://twitter.com/kristoferlund)
- Discord: kristoferkristofer
- Telegram: [@kristoferkristofer](https://t.me/kristoferkristofer)

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any suggestions or improvements.