for d in ./*.wasm; do
    echo $d;
    terpd tx wasm store $d --from test --gas-prices auto --gas-adjustment 1.7 --gas auto --chain-id 90u- --node  -b block --yes -o json | jq '.logs' | grep -A 1 code_id
    echo "-----------------";
done
