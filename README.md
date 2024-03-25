# Solana SPV Plugin

## Getting started
1) Compile the crates using cargo.
2) Open the `config.json` in this repo and set the libpath to that of your target folder.
3) Run the test-validator or validator with `--geyser-plugin-config`. If youre using test-validator or any network other than testnet you will have to redeploy the onchain program and then change the program IDs and account address in the config and client.
4) Run the test client in the client folder using the following command:
   ```
   ./target/release/client copy-transaction --signer ~/.config/solana/id.json BgaRwBpqNYbK8WSR4x1rtZn7LMhuwpHqF3nCoFtSjZjg Bg3ZP9GymdRNSojqPs3BrDfwmjS3jXJUMu5jYZ6hR7kv
   ```
**Note: This test client only verifies the account proof and not the votes and EAH, for that you have to use [Tinydancer-V1](https://github.com/tinydancer-io/tinydancer/tree/v1)**
## Documentation
To understand more in-depth about how the plugin works you can read:
- [Abstract](/docs/Sovereign-doc.md)
- [Geyser](/docs/GEYSER.md)

## Credits
This work is based on the original repository [solana-proofs](https://github.com/Sovereign-Labs/solana-proofs) by [Sovereign Labs](https://www.sovereign.xyz/)
- [Dubbelosix, Sovereign Labs](https://twitter.com/Dubbel06)
- [Preston Evans, Sovereign Labs](https://twitter.com/prestonevans__)

We are grateful to Sovereign-Labs and it's contributors for open sourcing their work.
