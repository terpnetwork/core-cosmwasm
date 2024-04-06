AUCTION_CODE_ID=120
CW721_BASE_ID=68
PUBKEY=terp1tyl97ac3s7sec4jwznk0s7n3tlwf3matfmkape

# terp1mum2jzk55uhl375cmpydla9lsen65fvmcz2sm6k92n9uc8mm8r5sm5f6m8

# MSG=$(cat <<EOF
# {
#    "name": "Test1",
#    "symbol": "TEST",
#    "minter": "$PUBKEY"
# }
# EOF
# )
# echo $MSG


# response_command='terpd tx wasm i $CW721_BASE_ID "$MSG" --label="NFT" --no-admin --gas-prices 0.05uthiolx --gas auto --gas-adjustment 1.9 --from test1 -y -b sync -o json --chain-id 90u-4'
# response=$(eval $response_command);
# echo $response

# if [ -n "$response" ]; then
#     txhash=$(echo "$response" | jq -r '.txhash')
#     echo 'waiting for tx to process'
#     sleep 6;
#     tx_response=$(terpd q tx $txhash -o json)
#     contract_address=$(echo "$tx_response" | jq -r '.logs[].events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')
#     echo "Contract Address: $contract_address"
# else
#     echo "Error: Empty response"
# fi


MINT_MSG=$(cat <<EOF
{
    "mint": {
        "token_id": "2",
        "owner": "$PUBKEY",
        "token_uri": "ipfs://QmboqXNQcf4pcNhfMWAeXCbTejxuVreDVDaB4qoFmg7DBR",
        "extension": {
            "image": "ipfs://QmboqXNQcf4pcNhfMWAeXCbTejxuVreDVDaB4qoFmg7DBR",
            "description": "test",
            "name": "Test 1",
            "attributes": [
                {
                    "trait_type": "background",
                    "value": "aurora"
                },
                {
                    "trait_type": "head",
                    "value": "aurora"
                }
            ]
        }
    }
}
EOF
)
# echo $MSG



response_command='terpd tx wasm e terp1mum2jzk55uhl375cmpydla9lsen65fvmcz2sm6k92n9uc8mm8r5sm5f6m8  "$MINT_MSG" --gas-prices 0.05uthiolx --gas auto --gas-adjustment 1.9 --from test1 -y -b sync -o json --chain-id 90u-4'
response=$(eval $response_command);
echo $response

if [ -n "$response" ]; then
    txhash=$(echo "$response" | jq -r '.txhash')
    echo 'waiting for tx to process'
    sleep 6;
    tx_response=$(terpd q tx $txhash -o json)
    echo $tx_response;
    # contract_address=$(echo "$tx_response" | jq -r '.logs[].events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')
    # echo "Contract Address: $contract_address"
else
    echo "Error: Empty response"
fi
