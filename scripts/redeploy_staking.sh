cd ./contracts/strategies/staking
#docker run --rm -v "$(pwd)":/code   -e CARGO_TERM_COLOR=always   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry   cosmwasm/rust-optimizer:0.12.13
RUSTFLAGS="-C link-arg=-s" cargo build --release --target=wasm32-unknown-unknown
jq 'del(.contracts.staking_strategy)' ../../../src/assets/config.json > temp.json && mv temp.json ../../../src/assets/config.json

cp target/wasm32-unknown-unknown/release/staking_strategy.wasm ../../../src/assets/wasm/

#cp artifacts/staking_strategy.wasm ../../../assets/wasm/

cd ../../../
npm test
