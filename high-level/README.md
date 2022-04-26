## Example usage

```bash
# Deploy contract (swap hello_sat with account)
near deploy hlo.testnet ./target/wasm32-unknown-unknown/release/high_level.wasm

# Send transaction to contract
near call hlo.testnet hello --accountId hlo.testnet --args '{"name": "Austin"}'

# Or make a view call
near view hlo.testnet hello --args '{"name": "Austin"}'

# Update greeting
near call hlo.testnet set_greeting --accountId hlo.testnet --args '{"greeting": "gm"}'

# View with updated greeting
near view hlo.testnet hello --args '{"name": "Austin"}'
```
