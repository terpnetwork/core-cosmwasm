## Store WASM Code

This proposal uploads the code for <Custom_Cosmwasm_Code_Label>

The source code is available at <Insert_Source_Code_Here>

Here is where description of contract function goes. Make sure you describe details regarding any implicit or explicit fees. The Contract performs the following logic:

- function 1.
- function 2.
- results

<Link_To_Documentation>

### Compile Instructions

```sh
docker run --rm -v "$(pwd)":/code --platform linux/amd64 \
	--mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target \
	--mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \
	cosmwasm/workspace-optimizer:0.12.13
```

This results in the following SHA256 checksum:

```
123erthyjfky687543rfgethy76uyhnrtegefw4567ynhgbtr5h46j7  contract_name.wasm
```

### Verify On-chain Contract

```sh
terpd q gov proposal $id --output json \\
| jq -r '.content.wasm_byte_code' \\
| base64 -d \\
| gzip -dc \\
| sha256sum

```

### Verify Local Contract

```
sha256sum artifacts/contract_name.wasm
```
