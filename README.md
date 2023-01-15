# Flyting Soroban
Insult your fellow degenerate in an on-chain, permanent way.  

## Build Contract

```
cargo build --target wasm32-unknown-unknown --release
```

## Run the Contract

```
soroban invoke \
    --wasm target/wasm32-unknown-unknown/release/flyting_soroban.wasm \
    --id 1 \
    --fn get_insult 
```

```
soroban invoke \
    --wasm target/wasm32-unknown-unknown/release/flyting_soroban.wasm \
    --id 1 \
    --fn insult \
    --arg thou-art-a-loser
```