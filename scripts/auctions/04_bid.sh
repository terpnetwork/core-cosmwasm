PUBKEY=terp1tyl97ac3s7sec4jwznk0s7n3tlwf3matfmkape
AUCTION_CONTRACT=terp1mullcexgw5sznuscul79j5xh88syc5ker98zn8270hkjt6hld68q030qs0
AUCTION_ID=0
BID_AMOUNT=420000000uterpx

binary_auction_msg=$(echo $AUCTION_MSG | jq -c . | base64)
echo $binary_auction_msg

MSG=$(cat <<EOF
{
    "place_bid": {
        "auction_id": "$AUCTION_ID"
    }
}
EOF
)
echo $MSG


response_command='terpd tx wasm e $AUCTION_CONTRACT  "$MSG" --amount $BID_AMOUNT --gas-prices 0.05uthiolx --gas auto --gas-adjustment 1.9 --from test1 -y -b sync -o json --chain-id 90u-4'
response=$(eval $response_command);
# echo $response

if [ -n "$response" ]; then
    txhash=$(echo "$response" | jq -r '.txhash')
    echo 'waiting for tx to process'
    sleep 6;
    tx_response=$(terpd q tx $txhash -o json)
    # contract_address=$(echo "$tx_response" | jq -r '.logs[].events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')
    # echo "Contract Address: $contract_address"
    echo 'finished with txhash:' $txhash
else
    echo "Error: Empty response"
fi