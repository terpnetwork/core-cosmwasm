for d in ./artifacts/*.wasm; do
    echo $d;
    response_command="terpd tx wasm store $d --from test1 --gas-prices 0.05uthiol --gas-adjustment 1.7 --gas auto --chain-id 120u-1 -b async --yes -o json";
    response=$(eval $response_command);
    if [ -n "$response" ]; then
        txhash=$(echo "$response" | jq -r '.txhash')
        echo "Using txhash: $txhash"
        sleep 6;
        terpd q tx $txhash | sed -n 's/.*"key":"code_id","value":"\([^"]*\)".*/\1/p'        
    else
        echo "Error: Empty response"
    fi
    echo "-----------------";
done
