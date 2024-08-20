# catts-query-proxy

Runs a Cloudflare Worker that proxies C–ATTS recipe queries from the IC smart contract canister (IPv6) to API endpoints running on IPv4. The worker is written in Rust, compiled to WebAssembly, and deployed to Cloudflare's [edge infrastructure](https://www.cloudflare.com/network/). Requests are cached for 10 minutes to avoid overloading the GraphQL endpoints with the many number of requests coming from the IC canister smart contracts.

## Run locally

### 1. Configure

The worker supports adding API keys when proxying requests. Built in support for The Graph and Moralis APIs is provided.

To configure the worker to use an API key, add the following environment variables to a `.dev.vars` file in the root of the project:

```bash
THEGRAPH_API_KEY=<API_KEY>
MORALIS_API_KEY=<API_KEY>
```

### 2. Run

```bash
npm i
npm run dev
```

## Deploy to Cloudflare

The worker is using [Durable Objects](https://developers.cloudflare.com/durable-objects/) to cache requests. Before deploying, you need to have [activated support for this feature](https://developers.cloudflare.com/durable-objects/get-started) in your Cloudflare account.

### 1. Configure

Set the environment variable using:

```bash
npx wrangler secret put THEGRAPH_API_KEY
npx wrangler secret put MORALIS_API_KEY
```

### 2. Deploy

```bash
npm run deploy
```

## Usage

See the [examples](examples) folder for examples of how to use the worker.

### The Graph - delegate votes

```bash
#!/bin/bash

# Example query that fetches the number of votes for a specific delegate from the ENS Governance subgraph on Arbitrum.

payload='{
  "url": "https://gateway-arbitrum.network.thegraph.com/api/{api-key}/subgraphs/id/GyijYxW9yiSRcEd5u2gfquSvneQKi5QuvU3WZgFyfFSn",
  "body": {
    "query": "query Delegate($id: ID!) { delegate(id: $id ) { numberVotes } }",
    "variables": { "id": "0x534631bcf33bdb069fb20a93d2fdb9e4d4dd42cf" }
  }
}'

# Generate a semi-random cache key
random_cache_key="cachekey_$(date +%s)_$RANDOM"

curl -X POST "http://localhost:8787/$random_cache_key" \
     -d "$payload"

```

The worker will forward the request to the API endpoint in the `url` field. The `{api_key}` placeholder will be replaced with the configured API key.

### EAS - Gitcoin Passport score

Making requests to the EAS GraphQL endpoints does not require an API key.

```bash
#!/bin/bash

# Example query that fetches the Gitcoin Passport score for a specific address.

payload='{
  "url": "https://optimism.easscan.org/graphql",
  "body": {
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
  }
}'

# Generate a semi-random cache key
random_cache_key="cachekey_$(date +%s)_$RANDOM"

curl -X POST "http://localhost:8787/$random_cache_key" \
     -H "Content-Type: application/json" \
     -d "$payload"

```

## Author

- [kristofer@kristoferlund.se](mailto:kristofer@kristoferlund.se)
- Twitter: [@kristoferlund](https://twitter.com/kristoferlund)
- Discord: kristoferkristofer
- Telegram: [@kristoferkristofer](https://t.me/kristoferkristofer)

## License

This project is licensed under the MIT License. See the LICENSE file for more details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request if you have any suggestions or improvements.
