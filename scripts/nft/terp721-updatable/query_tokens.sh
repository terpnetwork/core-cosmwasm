MSG=$(cat <<EOF
{
  "tokens": {"owner": "$USER"}
}
EOF
)

terpd q wasm contract-state smart $COLLECTION "$MSG"
 
