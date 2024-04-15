CODE_ID=133
MSG=$(cat <<EOF
{
  "config": {
    "update_wait_period": 30,
    "max_share_delta": "0.02"
  }
}
EOF
)

FROM="test1"
ADMIN=""
CHAIN_ID="90u-4"

response_command='terpd tx wasm i $CODE_ID "$MSG" --label "terp-fair-burn" --from "$FROM" --chain-id "$CHAIN_ID" --gas-prices 0.05uthiolx --gas-adjustment 1.7 --gas auto --admin "$FROM" -b sync  -y -o json'
response=$(eval $response_command);
echo $response;


 if [ -n "$response" ]; then
    txhash=$(echo "$response" | jq -r '.txhash')
    echo 'waiting for tx to process'
    sleep 6;
    tx_response=$(terpd q tx $txhash -o json)

    contract_address=$(echo "$tx_response" | jq -r '.logs[].events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')
        echo "Contract Address: $contract_address"
    else
        echo "Error: Empty response"
    fi
    