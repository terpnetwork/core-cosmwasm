#!/bin/bash

RELEASE_URL="https://api.github.com/repos/terpnetwork/core-cosmwasm/releases/tags/v0.1.0"

# Download the release page and extract the download URLs
curl -s "$RELEASE_URL" | jq -r '.assets[] | select(.name | endswith(".wasm")) | .browser_download_url' | wget -qi -