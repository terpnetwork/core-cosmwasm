KEY=$(terpd keys show $USER | jq -r .name)
MSG=$(cat <<EOF
{
    "mint": {}
}
EOF
)

echo $MSG

terpd tx wasm execute $MINTER "$MSG" --amount 50000000uthiolx \
--gas-prices 0.025uthiolx --gas auto --gas-adjustment 1.9 \
--from $KEY -y -b block -o json | jq .