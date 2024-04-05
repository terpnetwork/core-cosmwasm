FACTORY_CODE_ID=90
MINTER_CODE_ID=91
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

terpd tx wasm instantiate $FACTORY_CODE_ID "$MSG"  --label "vending factory" --no-admin \
  --from test1 --gas-prices 0.025uthiolx --gas-adjustment 1.7 --gas auto \
  --chain-id 90u-4 \
  -b block -o json  --generate-only > unsignedTx.json


