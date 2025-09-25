#!/bin/bash
set -euo pipefail

NET="devnet"
SendSolTo="5YLbUx2MGaHvSV1de5Kr1dVWPupbf63Mm5a9VhtvqoNt"
AIRDROP_AMOUNT=2


# get latest blockhash
recent_block_hash=$(solana-tss recent-block-hash --net $NET | awk '{print $4}')

echo "Recent block hash: $recent_block_hash"

# party 1 generate keypair
response=$(curl -s -X POST http://127.0.0.1:3000/generate)

party1_secret_share=$(echo "$response" | jq -r '.secret_share')
party1_public_share=$(echo "$response" | jq -r '.public_share')

echo "Party 1 public key: $party1_public_share"

# party 2 generate keypair
response2=$(curl -s -X POST http://127.0.0.1:3000/generate)

party2_secret_share=$(echo "$response2" | jq -r '.secret_share')
party2_public_share=$(echo "$response2" | jq -r '.public_share')

echo "Party 2 public key: $party2_public_share"

# Generate aggregated public key

response3=$(curl -s -X POST http://127.0.0.1:3000/aggregate_keys \
    -H "Content-Type:application/json" \
    -d "{
    \"keys\": [\"$party1_public_share\", \"$party2_public_share\"]
    }"
)

agg_public_key=$(echo "$response3" | jq -r '.aggregated_key')

echo "Aggregated Public Key: $agg_public_key"

# Fund the aggregated public key

echo "Airdropping $AIRDROP_AMOUNT SOL to aggregated key..."
solana-tss airdrop --net $NET --to "$agg_public_key" --amount $AIRDROP_AMOUNT
sleep 5
balance=$(solana-tss balance --net $NET "$agg_public_key" | cut -d " " -f6)
echo "Aggregated key balance: $balance SOL"

# Aggregate Step One for party 1
response4=$(curl -s -X POST http://127.0.0.1:3000/agg_send_step_one \
    -H "Content-Type:application/json" \
    -d "{
    \"keys\": [\"$party1_secret_share\"]
    }"
)

party1_first_msg=$(echo "$response4" | jq -r '.first_msg')
party1_secret_state=$(echo "$response4"| jq -r '.secret_state')

echo "Party 1 first_message: $party1_first_msg"
echo "Party 1 secret_state: $party1_secret_state"

# Aggregate Step One for party 2

response5=$(curl -s -X POST http://127.0.0.1:3000/agg_send_step_one \
    -H "Content-Type:application/json" \
    -d "{
    \"keys\": [\"$party2_secret_share\"]
    }"
)

party2_first_msg=$(echo "$response5" | jq -r '.first_msg')
party2_secret_state=$(echo "$response5" | jq -r '.secret_state')

echo "Party 2 first_message: $party2_first_msg"
echo "Party 2 secret_state: $party2_secret_state"



# Aggregate Step Two // party 1 signature
party1_sig=$(curl -s -X POST http://127.0.0.1:3000/agg_send_step_two \
  -H "Content-Type:application/json" \
  -d "{
\"keypair\": \"$party1_secret_share\",
\"amount\": 0.5,
\"to\": \"$SendSolTo\",
\"memo\": \"Payment for Coffee\",
\"recent_block_hash\": \"$recent_block_hash\",
\"keys\": [\"$party1_public_share\", \"$party2_public_share\"],
\"first_messages\": [\"$party2_first_msg\"],
\"secret_state\": \"$party1_secret_state\"
}" | jq -r '.partial_signature')

echo "Party1 sig: $party1_sig"


# Aggregate Step Two // party 2 signature
party2_sig=$(curl -s -X POST http://127.0.0.1:3000/agg_send_step_two \
  -H "Content-Type:application/json" \
  -d "{
\"keypair\": \"$party2_secret_share\",
\"amount\": 0.5,
\"to\": \"$SendSolTo\",
\"memo\": \"Payment for Coffee\",
\"recent_block_hash\": \"$recent_block_hash\",
\"keys\": [\"$party1_public_share\", \"$party2_public_share\"],
\"first_messages\": [\"$party1_first_msg\"],
\"secret_state\": \"$party2_secret_state\"
}" | jq -r '.partial_signature')

echo "Party1 sig: $party2_sig"



# # Aggregate and Broadcast
agg_result=$(curl -s -X POST http://127.0.0.1:3000/agg_sig_and_broadcast \
  -H "Content-Type:application/json" \
  -d "{
\"signatures\": [\"$party1_sig\", \"$party2_sig\"],
\"amount\": 0.5,
\"to\": \"$SendSolTo\",
\"memo\": \"Payment for Coffee\",
\"recent_block_hash\": \"$recent_block_hash\",
\"net\": \"https://api.devnet.solana.com\",
\"keys\": [\"$party1_public_share\", \"$party2_public_share\"]
}")

echo "Final agg + broadcast result: $agg_result"