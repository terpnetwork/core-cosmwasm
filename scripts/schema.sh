for d in contracts/*; do
  if [ -d "$d" ]; then
    cd $d
    cargo schema
    rm -rf schema/raw
    cd ../..
  fi
done

for d in contracts/nft/factories/*; do
  if [ -d "$d" ]; then
    cd $d
    cargo schema
    rm -rf schema/raw
    cd ../../..
  fi
done

for d in contracts/nft/minters/*; do
  if [ -d "$d" ]; then
    cd $d
    cargo schema
    rm -rf schema/raw
    cd ../../..
  fi
done
for d in contracts/nft/earlybirds/*; do
  if [ -d "$d" ]; then
    cd $d
    cargo schema
    rm -rf schema/raw
    cd ../../..
  fi
done
for d in contracts/revenue/fair-burn/*; do
  if [ -d "$d" ]; then
    cd $d
    cargo schema
    rm -rf schema/raw
    cd ../../..
  fi
done
for d in contracts/revenue/residual-registry/*; do
  if [ -d "$d" ]; then
    cd $d
    cargo schema
    rm -rf schema/raw
    cd ../../..
  fi
done

# cd contracts/terp-eth-airdrop && cargo schema && rm -rf schema/raw && cd ../..
# cd contracts/splits && cargo schema && rm -rf schema/raw && cd ../..

# cd ts && yarn install && yarn codegen

# cd ..
