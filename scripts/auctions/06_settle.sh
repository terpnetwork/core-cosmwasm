PUBKEY=terp1tyl97ac3s7sec4jwznk0s7n3tlwf3matfmkape
AUCTION_CONTRACT=terp1mullcexgw5sznuscul79j5xh88syc5ker98zn8270hkjt6hld68q030qs0
AUCTION_ID=0

binary_auction_msg=$(echo $AUCTION_MSG | jq -c . | base64)
echo $binary_auction_msg

MSG=$(cat <<EOF
{"admin_pause": {}}
EOF
)
# MSG=$(cat <<EOF
# {"admin_resume": {}}
# EOF
# MSG=$(cat <<EOF
# {"admin_cancel_auction": {"auction_id": "$AUCTION_ID"}}
# EOF
# )
# )
# MSG=$(cat <<EOF
# {"admin_change_config": {}}
# EOF
# )
# MSG=$(cat <<EOF
# {"set_royalty_fee": {}}
# EOF
# )
# MSG=$(cat <<EOF
# {"set_royalty_address": {}}
# EOF
# )
# MSG=$(cat <<EOF
# {"settle_hook": {}}
# EOF
# )

echo $MSG

response_command='terpd tx wasm e $AUCTION_CONTRACT  "$MSG" --gas-prices 0.05uthiolx --gas auto --gas-adjustment 1.9 --from test1 -y -b sync -o json --chain-id 90u-4'
response=$(eval $response_command);
# echo $response

if [ -n "$response" ]; then
    txhash=$(echo "$response" | jq -r '.txhash')
    echo 'waiting for tx to process'
    sleep 6;
    tx_response=$(terpd q tx $txhash -o json)
    echo 'finished with txhash:' $txhash
else
    echo "Error: Empty response"
fi