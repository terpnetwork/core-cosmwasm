CODE_ID=2753
MSG=$(cat <<EOF
{
  "config": {
    "update_wait_period": 30,
    "max_share_delta": "0.02"
  }
}
EOF
)

FROM="hot-wallet"
ADMIN=""
CHAIN_ID="elgafar-1"
NODE="https://"

terpd tx wasm instantiate $CODE_ID  "$MSG"  \
  --label "terp-fair-burn" \
  --from "$FROM" \
  --chain-id "$CHAIN_ID" \
  --node "$NODE" \
  --gas-prices auto \
  --gas-adjustment 1.7 \
  --gas auto \
  --admin "$ADMIN" \
  -b block \
  -o json | jq .