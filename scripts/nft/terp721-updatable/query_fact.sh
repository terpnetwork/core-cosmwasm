MSG=$(cat <<EOF
{
  "params": {}
}
EOF
)

terpd q wasm contract-state smart $FACTORY "$MSG"