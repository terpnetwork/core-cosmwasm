KEY=$(terpd keys show $ADMIN | jq -r .name)
FACTORY=terp14kq48e55k8sdpu4tv5dpjshsk737am7mjf7z5k8d7h26vxw96ensta0nqx
TERP721_CODE_ID=128

# init msg
# VendingMinterInitMsgExtension {
#     pub base_token_uri: String,
#     pub payment_address: Option<String>,
#     pub start_time: Timestamp,
#     pub num_tokens: u32,
#     pub mint_price: Coin,
#     pub per_address_limit: u32,
#     pub earlybird: Option<String>,
# }
# collection params
# CollectionParams {
#     /// The collection code id
#     pub code_id: u64,
#     pub name: String,
#     pub symbol: String,
#     pub info: CollectionInfo<RoyaltyInfoResponse>,
# }
# CollectionInfo {
    # pub creator: String,
    # pub description: String,
    # pub image: String,
    # pub external_link: Option<String>,
    # pub explicit_content: Option<bool>,
    # pub start_trading_time: Option<Timestamp>,
    # pub royalty_info: Option<T>,
# }

# add a few minutes buffer to start time
TIME=$(date -v+5000S +%s)

MSG=$(cat <<EOF
{
    "create_minter": {
        "init_msg": {
            "base_token_uri": "ipfs://bafybeic3tpnekc44dvapiv3readanraixczuvvpeo7clptt3e4yjffzjzy/IMG_3756.JPG",
            "start_time": "$(echo $TIME)000000000",
            "num_tokens": 1000,
            "mint_price": { "amount": "100", "denom": "uterp" },
            "per_address_limit": 30
        },
        "collection_params": {
            "code_id": $TERP721_CODE_ID,
            "name": "In_Da_Game",
            "symbol": "EASP0RTS",
            "info": {
                "creator": "$ADMIN",
                "description": "Test Collection",
                "image": "ipfs://bafybeic3tpnekc44dvapiv3readanraixczuvvpeo7clptt3e4yjffzjzy/IMG_3756.JPG"
            }
        }
    }
}
EOF
)

echo $MSG
response_command='terpd tx wasm execute $FACTORY "$MSG" --amount 10uterp --gas-prices 0.025uterp --gas auto --gas-adjustment 1.9 --from test1 -b block -o json';
response=$(eval $response_command);
echo $response


 if [ -n "$response" ]; then
    txhash=$(echo "$response" | jq -r '.txhash')
    echo 'waiting for tx to process'
    sleep 6;
    tx_response=$(terpd q tx $txhash -o json)

    echo $tx_response;

    # contract_address=$(echo "$tx_response" | jq -r '.logs[].events[] | select(.type == "instantiate") | .attributes[] | select(.key == "_contract_address") | .value')
    #     echo "Contract Address: $contract_address"
    else
        echo "Error: Empty response"
    fi