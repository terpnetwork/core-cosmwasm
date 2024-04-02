cd packages/actions/controllers && cargo publish && cd ../..
sleep 10
cd packages/actions/ethereum-verify && cargo publish && cd ../..
sleep 10
cd packages/actions/mint-hooks && cargo publish && cd ../..
sleep 10
cd packages/actions/terp-index-query && cargo publish && cd ../..
sleep 10
cd packages/nft/factory-utils && cargo publish && cd ../..
sleep 10
cd packages/nft/minter-utils && cargo publish && cd ../..
sleep 10
cd packages/nft/terp-metadata && cargo publish && cd ../..
sleep 10
cd packages/nft/terp721 && cargo publish && cd ../..
sleep 10
cd packages/revenue/marketplace && cargo publish && cd ../..
sleep 10
cd packages/revenue/terp-fee && cargo publish && cd ../..
sleep 10
cd packages/utils/terp-multi-test && cargo publish && cd ../..
sleep 10
cd packages/utils/terp-sdk && cargo publish && cd ../..
sleep 10
cd packages/utils/unit-test && cargo publish && cd ../..
sleep 10
