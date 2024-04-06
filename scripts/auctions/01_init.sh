AUCTION_CODE_ID=120
PUBKEY=terp1tyl97ac3s7sec4jwznk0s7n3tlwf3matfmkape


MSG=$(cat <<EOF
{
  "protocol_fee": "0.01",
  "min_increment": "0.1",
  "duration": 300,
  "min_duration": 60,
  "accepted_denom": ["uterpx"],
  "min_reserve_price": "1000",
  "max_royalty_fee": "0.2",
  "protocol_addr": "$PUBKEY"
}
EOF
)
echo $MSG


response_command='terpd tx wasm i $AUCTION_CODE_ID "$MSG" --label="Auction Marketplace" --no-admin --gas-prices 0.05uthiolx --gas auto --gas-adjustment 1.9 --from test1 -y -b sync -o json --chain-id 90u-4'
response=$(eval $response_command);
echo $response


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