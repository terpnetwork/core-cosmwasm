set -eux

CONTRACT=artifacts/terp_fair_burn.wasm

TITLE="Terp Fair Burn v0.1.0" 
SOURCE="https://github.com/public-awesome/core/releases/tag/terp_fair_burn-v0.1.0"
MARKDOWN="scripts/markdown/terp_fair_burn-v0.1.0.md"
DESCRIPTION=$(cat "$MARKDOWN" | base64 | tr -d '\n')
BUILDER="cosmwasm/workspace-optimizer:0.12.13"
HASH="bf1497f4303d20c1db5f1af2ccec7b367e150c84c5e86f6a2798a1c4cc0d52c9"

FROM=""
DEPOSIT="1uthiolx"

RUN_AS="hot-wallet"
ANY_OF_ADDRS=""

CHAIN_ID="90u-4"
NODE=""

terpd tx gov submit-proposal wasm-store "$CONTRACT" \
 --title "$TITLE" \
 --description "$(echo "$DESCRIPTION" | base64 --decode)" \
 --code-source-url "$SOURCE" \
 --builder "$BUILDER" \
 --code-hash "$HASH" \
 --from "$FROM" \
 --deposit "$DEPOSIT" \
 --run-as "$RUN_AS" \
 --instantiate-anyof-addresses "$ANY_OF_ADDRS" \
 --chain-id "$CHAIN_ID" \
 --node "$NODE" \
 --gas-prices auto \
 --gas auto \
 --gas-adjustment 1.5