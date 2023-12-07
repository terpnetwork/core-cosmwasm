MSG=$(cat <<EOF
{
  "nft_info": {"token_id": "$1"}
}
EOF
)
echo $MSG $TERP721

terpd q wasm contract-state smart $TERP721 "$MSG"

