KEY=$(terpd keys show $ADMIN | jq -r .name)
MSG=$(cat <<EOF
{
    "freeze_token_metadata": {}
}
EOF
)

echo $MSG

terpd tx wasm execute $TERP721 "$MSG" \
--gas-prices 0.025uthiolx --gas auto --gas-adjustment 1.9 \
--from $KEY -y -b block -o json | jq .