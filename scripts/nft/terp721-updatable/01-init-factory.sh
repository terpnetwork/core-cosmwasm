# terpd config node $NODE
# terpd config chain-id $CHAIN_ID
# terpd config output json

# KEY=$(terpd keys show $ADMIN | jq -r .name)
FACTORY_CODE_ID=135
MINTER_CODE_ID=136

MSG=$(cat <<EOF
{
  "params": {
    "code_id": $MINTER_CODE_ID,
    "allowed_terp721_code_ids": [128,68],
    "frozen": false,
    "creation_fee": {"amount": "10", "denom": "uterp"},
    "min_mint_price": {"amount": "0", "denom": "uterp"},
    "mint_fee_bps": 1000,
    "max_trading_offset_secs": 604800,
    "extension": {
        "max_token_limit": 10000,
        "max_per_address_limit": 50,
        "airdrop_mint_price": { "denom": "uterp", "amount": "0" },
        "airdrop_mint_fee_bps": 10000,
        "shuffle_fee": { "amount": "50", "denom": "uterp" }
    }
  }
}
EOF
)
# echo $MSG

response_command='terpd tx wasm i $FACTORY_CODE_ID "$MSG" --label "Factory" --no-admin --gas-prices 0.05uterp --gas auto --gas-adjustment 1.9 --from wtf -y -b sync -o json ';
response=$(eval $response_command);
# echo $response



 if [ -n "$response" ]; then
    txhash=$(echo "$response" | jq -r '.txhash')
    echo $txhash
    echo 'waiting for tx to process'
    sleep 6;
    tx_response=$(terpd q tx $txhash -o json)


    contract_address=$(echo "$tx_response" | jq -r '.logs[].events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')
        echo "Contract Address: $contract_address"
    else
        echo "Error please check tx hash "
    fi