#!/bin/bash

# Example query that fetches the number of votes for a specific delegate from the ENS Governance subgraph on Arbitrum.

query_url='https://gateway-arbitrum.network.thegraph.com/api/{api-key}/subgraphs/id/GyijYxW9yiSRcEd5u2gfquSvneQKi5QuvU3WZgFyfFSn'

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