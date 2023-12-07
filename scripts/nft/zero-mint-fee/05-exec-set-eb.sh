KEY=$(terpd keys show $ADMIN | jq -r .name)
MINTER=
EARLYBIRD=

# add a few minutes buffer to start time
TIME=$(date -v+5000S +%s)

MSG=$(cat <<EOF
{ "set_earlybird": { "earlybird": "$EARLYBIRD" } }
EOF
)

echo $MSG

terpd tx wasm execute $MINTER "$MSG" \
--gas-prices 0.025uthiolx --gas auto --gas-adjustment 1.9 \
--from $KEY -y -b block -o json | jq .