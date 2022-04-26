## Example usage

```bash
# Deploy contract (swap hello_sat with account)
near deploy hlo.testnet ./hello.wasm

# Send transaction to contract
near call hlo.testnet hello --accountId hlo.testnet --args '{"name": "Austin"}'

# Or make a view call
near view hlo.testnet hello --args '{"name": "Austin"}'
```
