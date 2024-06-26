# terpd config node $NODE
# terpd config chain-id $CHAIN_ID
# terpd config output json


EB_CODE_ID=93

TIME=$(date -v+30S +%s)
ENDTIME=$(date -v+3000S +%s)
MSG=$(cat <<EOF
{
  "members": ["terp1tyl97ac3s7sec4jwznk0s7n3tlwf3matfmkape"],
  "start_time": "$(echo $TIME)000000000",
  "end_time": "$(echo $ENDTIME)000000000",
  "mint_price": {
    "amount": "0",
    "denom": "uthiolx"
  },
  "per_address_limit": 3,
  "member_limit": 10,
  "admins": [],
  "admins_mutable": true
}
EOF
)
echo $MSG


terpd tx wasm instantiate $EB_CODE_ID "$MSG" --label "ZeroMintFeeEarlybird" --amount 100000000uthiolx \
  --no-admin --gas-prices 0.05uthiolx --gas auto --gas-adjustment 1.9 \
  --from test1 -y -b sync -o json | jq .
