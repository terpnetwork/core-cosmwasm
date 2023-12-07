KEY=$(terpd keys show $ADMIN | jq -r .name)

MSG=$(cat <<EOF
{
  "params": {
    "code_id": $MINTER_CODE_ID,
    "allowed_terp721_code_ids": [$TERP721_BASE_CODE_ID],
    "frozen": false,
    "creation_fee": {"amount": "20000000000", "denom": "uthiolx"},
    "min_mint_price": {"amount": "0", "denom": "uthiolx"},
    "mint_fee_bps": 500,
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


terpd tx wasm instantiate $FACTORY_CODE_ID "$MSG" --label "FeaturedFlexVendingMinterFactory" \
  --admin $ADMIN \
  --gas-prices 0.025uthiolx --gas 5000000 \
  --from $ADMIN \
  --generate-only > unsignedTx.json

# terpd tx sign unsignedTx.json \
#     --multisig=$ADMIN --from $USER --output-document=$KEY.json \
#     --chain-id $CHAIN_ID

# terpd tx multisign unsignedTx.json $MULTISIG_NAME $1 $2 $3 > signedTx.json

# terpd tx broadcast signedTx.json