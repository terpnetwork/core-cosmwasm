lint:
	cargo clippy --all-targets -- -D warnings

lint-fix:
	cargo clippy --all-targets --fix -- -D warnings

schema:
	sh scripts/schema.sh

artifacts:
	mkdir -p artifacts

download-artifacts: artifacts
	scripts/download-core-cosmwasm.sh

clear-artifacts: artifacts
	rm -rf artifacts
	mkdir -p artifacts

optimize:
	sh scripts/optimize.sh

optimize-arm:
	sh scripts/optimize-arm.sh

deploy-local:
	#!/usr/bin/env bash
	TEST_ADDRS=`jq -r '.[].address' ./typescript/packages/e2e-tests/configs/test_accounts.json | tr '\n' ' '`
	docker kill terp-core || true
	docker volume rm -f terp_data
	docker run --rm -d --name terp-core \
		-e DENOM=uthiolx \
		-e CHAINID=90u-2 \
		-e GAS_LIMIT=-1 \
		-p 1317:1317 \
		-p 26656:26656 \
		-p 26657:26657 \
		-p 9090:9090 \
		--mount type=volume,source=terp_data,target=/root \
		terpnetwork/terp-core:4.1.0 /data/entry-point.sh $TEST_ADDRS

deploy-local-arm:
	#!/usr/bin/env bash
	TEST_ADDRS=`jq -r '.[].address' ./typescript/packages/e2e-tests/configs/test_accounts.json | tr '\n' ' '`
	docker kill terp-core || true
	docker volume rm -f terp_data
	docker run --rm -d --name terp-core \
		-e DENOM=uthiolx \
		-e CHAINID=90u-2 \
		-e GAS_LIMIT=-1 \
		-p 1317:1317 \
		-p 26656:26656 \
		-p 26657:26657 \
		-p 9090:9090 \
		--mount type=volume,source=terp_data,target=/root \
		--platform linux/amd64 \
		terpnetwork/terp-core:4.1.0 /data/entry-point.sh $TEST_ADDRS

e2e-test:
	#!/usr/bin/env bash
	START_DIR=$(pwd)
	cd typescript/packages/e2e-tests
	yarn install
	yarn test

e2e-test-full: download-artifacts optimize deploy-local e2e-test

e2e-test-full-arm: download-artifacts optimize-arm deploy-local-arm e2e-test

e2e-watch: deploy-local-arm
	#!/usr/bin/env bash
	START_DIR=$(pwd)
	cd typescript/packages/e2e-tests
	yarn test
	yarn test:watch

publish:
	cd packages/utils/terp-sdk && cargo publish && cd ../..
	cd packages/utils/terp-multi-test && cargo publish && cd ../..
	cd packages/utils/test-suite && cargo publish && cd ../..
	cd packages/utils/unit-tests && cargo publish && cd ../..
	cd packages/revenue/terp-fee && cargo publish && cd ../..
	cd packages/nft/factory-utils && cargo publish && cd ../..
	cd packages/nft/minter-utils && cargo publish && cd ../..
	cd packages/nft/terp-metadata && cargo publish && cd ../..
	cd packages/nft/terp721 && cargo publish && cd ../..
	cd packages/actions/controllers && cargo publish && cd ../..
	cd packages/actions/ethereum-verify && cargo publish && cd ../..
	cd packages/actions/mint-hooks && cargo publish && cd ../..
	cd packages/actions/terp-index-query && cargo publish && cd ../..