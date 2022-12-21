# Solana Sealevel
## Overview
Solana is a very unique and popular smart contract platform today. Its VM is one of a kind, unlike other new blockchain VMs which are based on Wasm, solana VM is based on [BPF](https://github.com/solana-labs/sbf-tools), using LLVM compiler. So solana contracts are not compiled to Wasm bytecodes, instead it comples to BPF bytecodes.

Another thing different from other platforms is solana separate execution logic and storage. Unlike CosmWasm and Near where state lived with contract, solana contract is stateless. When you run a contract, you not only specify the API you are calling along with parameters, you also specify the account (where contract stores its data) you touch. Anchor wrote a detail [explanation](https://www.anchor-lang.com/docs/intro-to-solana) on this. This separation allows solana to parallelize read operations on same account.

## Resources
### Docs
- [soldev](https://soldev.app/course)
- [anchor](https://www.anchor-lang.com/) Most popular solana framework.
- [official doc](https://docs.solana.com/developers)
- [create a mini twitter on solana](https://lorisleiva.com/create-a-solana-dapp-from-scratch)
- [solana cookbook](https://solanacookbook.com/)
- [build a full stack solana app](https://dev.to/edge-and-node/the-complete-guide-to-full-stack-web3-development-4g74) This tutorial was made in 2021, there are lots of breaking change APIs since then so some codes are not working, need to figure out how to modify to adapt the latest APIs.
### Real world examples
- [official program library](https://github.com/solana-labs/solana-program-library/)
- [saber](https://github.com/saber-hq/stable-swap) stable swap
- [serum](https://github.com/project-serum/serum-dex) orderbook dex
