#!/bin/bash

query_url='https://gateway-arbitrum.network.thegraph.com/api/[api-key]/subgraphs/id/GyijYxW9yiSRcEd5u2gfquSvneQKi5QuvU3WZgFyfFSn'

payload='{
  "operationName": "Delegate",
  "query": "query Delegate($id: ID!) { delegate(id: $id ) { numberVotes } }",
  "variables": { "id": "0x534631bcf33bdb069fb20a93d2fdb9e4d4dd42cf" }
}'

curl -X POST "http://localhost:8787/cachekey123" \
     -H "x-thegraph-query-url: $query_url" \
     -H "Content-Type: application/json" \
     -d "$payload"
