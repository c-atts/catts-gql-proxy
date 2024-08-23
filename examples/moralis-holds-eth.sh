#!/bin/bash

# Example query that fetches the block number of all transactions for a specific NFT contract

payload='{
  "url": "https://deep-index.moralis.io/api/v2.2/0x2238C8b16c36628b8f1F36486675C1E2A30DebF1/nft?chain=eth&format=decimal&token_addresses%5B0%5D=0xb47e3cd837ddf8e4c57f05d70ab865de6e193bbb&media_items=false",
  "filter": "$.result[*].block_number"
}'

# Generate a semi-random cache key
random_cache_key="cachekey_$(date +%s)_$RANDOM"

curl -X POST "http://localhost:8787/$random_cache_key" \
     -d "$payload"
