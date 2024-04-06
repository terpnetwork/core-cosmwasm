FACTORY_CODE_ID=117
MINTER_CODE_ID=118

MSG=$(cat <<EOF
{
  "params": {
    "code_id": $MINTER_CODE_ID,
    "allowed_terp721_code_ids": [83,68],
    "frozen": false,
    "creation_fee": { "amount": "1000000000", "denom": "uthiolx" },
    "min_mint_price": { "amount": "50000000", "denom": "uthiolx" },
    "mint_fee_bps": 1000,
    "max_trading_offset_secs": 1209600,
    "extension": {
      "max_token_limit": 10000,
      "max_per_address_limit": 50,
      "airdrop_mint_price": { "amount": "0", "denom": "uthiolx" },
      "airdrop_mint_fee_bps": 0,
      "shuffle_fee": { "amount": "100000000", "denom": "uthiolx" }
    }
  }
}
EOF
)

response_command='terpd tx wasm instantiate $FACTORY_CODE_ID "$MSG"  --label="vendingfactory" --no-admin --from test1 --gas-prices 0.05uthiolx --gas-adjustment 1.7 --gas auto --chain-id 90u-4 -b sync -o json  --yes -o json';
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