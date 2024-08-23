#!/bin/bash

# Example query that fetches the Gitcoin Passport score for a specific address.

# Number of parallel queries
NUM_QUERIES=20

# Payload for the query
payload='{
  "url": "https://optimism.easscan.org/graphql",
  "body": {
    "query": "query PassportQuery($where: AttestationWhereInput, $take: Int) { attestations(where: $where, take: $take) { decodedDataJson } }",
    "variables": {
      "where": {
        "schemaId": { "equals": "0x6ab5d34260fca0cfcf0e76e96d439cace6aa7c3c019d7c4580ed52c6845e9c89" },
        "recipient": { "equals": "0xa32aECda752cF4EF89956e83d60C04835d4FA867", "mode": "insensitive" }
      },
      "take": 1
    }
  }
}'


# Generate a semi-random cache key
random_cache_key="cachekey_$(date +%s)_$RANDOM"

# Function to perform a single query
perform_query() {
    curl -X POST "http://localhost:8787/$random_cache_key" \
         -H "Content-Type: application/json" \
         -d "$payload"
}

# Run queries in parallel
for ((i=1; i<=$NUM_QUERIES; i++))
do
    perform_query &
done

# Wait for all background jobs to complete
wait

echo "All $NUM_QUERIES queries have been processed."
