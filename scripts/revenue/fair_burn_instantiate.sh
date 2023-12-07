CODE_ID=74
MSG=$(cat <<EOF
{
  "fee_bps": 5000
}
EOF
)

FROM="hot-wallet"
CHAIN_ID="90u-2"
NODE=""

terpd tx wasm instantiate $CODE_ID  "$MSG"  \
  --label "terp-fair-burn" \
  --from "$FROM" \
  --chain-id "$CHAIN_ID" \
  --node "$NODE" \
  --gas-prices auto \
  --gas-adjustment 1.7 \
  --gas auto \
  --no-admin \
  -b block \
  -o json | jq .
