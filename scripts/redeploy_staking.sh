cd ./contracts/strategies/staking
docker run --rm -v "$(pwd)":/code   -e CARGO_TERM_COLOR=always   --mount type=volume,source="$(basename "$(pwd)")_cache",target=/code/target   --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry   cosmwasm/rust-optimizer:0.12.13
echo '{}' >  ../../../integration_tests/assets/config.json
cp artifacts/*.wasm ../../../integration_tests/assets/wasm/
cd ../../../integration_tests
NODE_OPTIONS=--openssl-legacy-provider npm test