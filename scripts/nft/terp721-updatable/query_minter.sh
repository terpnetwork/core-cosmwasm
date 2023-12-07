MSG=$(cat <<EOF
{
  "config": {}
}
EOF
)

terpd q wasm contract-state smart $MINTER "$MSG"

MSG=$(cat <<EOF
{
  "start_time": {}
}
EOF
)

terpd q wasm contract-state smart $MINTER "$MSG"