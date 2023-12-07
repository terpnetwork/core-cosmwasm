# terpd config node $NODE
# terpd config chain-id $CHAIN_ID
# terpd config output json

KEY=$(terpd keys show $ADMIN | jq -r .name)
FACTORY_CODE_ID=
MINTER_CODE_ID=

MSG=$(cat <<EOF
{
  "params": {
    "code_id": $MINTER_CODE_ID,
    "allowed_terp721_code_ids": [1979],
    "frozen": false,
    "creation_fee": {"amount": "5000000000", "denom": "uthiolx"},
    "min_mint_price": {"amount": "0", "denom": "uthiolx"},
    "mint_fee_bps": 1000,
    "max_trading_offset_secs": 604800,
    "extension": {
        "max_token_limit": 10000,
        "max_per_address_limit": 50,
        "airdrop_mint_price": { "denom": "uthiolx", "amount": "0" },
        "airdrop_mint_fee_bps": 10000,
        "shuffle_fee": { "amount": "500000000", "denom": "uthiolx" }
    }
  }
}
EOF
)
echo $MSG


terpd tx wasm instantiate $FACTORY_CODE_ID "$MSG" --label "Factory" \
  --no-admin --gas-prices 0.025uthiolx --gas 500000 --gas-adjustment 1.9 \
  --from $KEY -y -b block -o json | jq .
