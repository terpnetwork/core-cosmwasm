# terpd config node $NODE
# terpd config chain-id $CHAIN_ID
# terpd config output json

KEY=$(terpd keys show $ADMIN | jq -r .name)
FACTORY_CODE_ID=121
MINTER_CODE_ID=122

MSG=$(cat <<EOF
{
  "params": {
    "code_id": $MINTER_CODE_ID,
    "allowed_terp721_code_ids": [68,128],
    "frozen": false,
    "creation_fee": {"amount": "5000000", "denom": "uterp"},
    "min_mint_price": {"amount": "0", "denom": "uterp"},
    "mint_fee_bps": 1000,
    "max_trading_offset_secs": 604800,
    "extension": {
        "max_token_limit": 10000,
        "max_per_address_limit": 50,
        "airdrop_mint_price": { "denom": "uterp", "amount": "0" },
        "airdrop_mint_fee_bps": 10000,
        "shuffle_fee": { "amount": "500000000", "denom": "uterp" }
    }
  }
}
EOF
)
echo $MSG


terpd tx wasm instantiate $FACTORY_CODE_ID "$MSG" --label "Factory" \
  --no-admin --gas-prices 0.025uthiolx --gas 500000 --gas-adjustment 1.9 \
  --from $KEY -y -b block -o json | jq .
