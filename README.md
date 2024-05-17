# Solana SPV Plugin

## Getting started
1) Compile the crates using cargo.
2) Open the `config.json` in this repo and set the libpath to that of your target folder.
3) Run the test-validator or validator with `--geyser-plugin-config`. If youre using test-validator or any network other than testnet you will have to redeploy the onchain program and then change the program IDs and account address in the config and client. ( The below program ID and account pubkey are deployed on testnet so you may use them as well with a testnet rpc that has the plugin )
4) Run the test client in the client folder using the following command:
   ```
   ./target/release/client copy-transaction --signer ~/.config/solana/id.json 3R72AjaZj6gCbANm7LrjNwDqpxacxwnnqE7JgegBTY4Z HPUJAf6r3zJrkM72wB3EhGGtfbTkQwMPMSq6d7HaapYr
   ```
**Note: This test client only verifies the account proof and not the votes and EAH, for that you have to use [Tinydancer-V1](https://github.com/tinydancer-io/tinydancer/tree/v1)**
## Documentation
To understand more in-depth about how the plugin works you can read:
- [Abstract](/docs/Sovereign-doc.md)
- [Geyser](/docs/GEYSER.md)

## Compatibility
The plugin has been tested with solana validator `v1.18.7` on testnet and test-validator.

## Credits
This work is based on the original repository [solana-proofs](https://github.com/Sovereign-Labs/solana-proofs) by [Sovereign Labs](https://www.sovereign.xyz/)
- [Dubbelosix, Sovereign Labs](https://twitter.com/Dubbel06)
- [Preston Evans, Sovereign Labs](https://twitter.com/prestonevans__)

We are grateful to Sovereign-Labs and it's contributors for open sourcing their work.
