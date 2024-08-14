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