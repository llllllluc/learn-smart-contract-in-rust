# Near Wasm
## Near overview
Near is a sharded blockchain, probably the most notable difference between Near smart contract and others is every contract call is async, where in other environment (EVM, CosmWasm, Solana) contract call is synchronous. This will play a huge difference when you perform cross contract call. The reason Near designed it to be async is to adapt the sharded model, since you cannot guarantee two contracts live on the same shard, and cross shard communication is async.

Async in Near reminds me of writing promise (async, await, then) in javascript.

Overall Near has an amazing documentation and plenty of tutorials, it also has a well maintained rust sdk (dapp framework, interacting with the chain is called js-api / py-api in Near).

## Tutorials
### [Fungible token zero to hero](https://docs.near.org/tutorials/fts/introduction)
Implemented the basic functions of a NEP141 FT contract, supports balance, supply, metadata, transfer.
### [Fungible token factory](https://github.com/near-examples/token-factory)
Implemented 2 contracts, one is the standard NEP141 fungible token contract, the other is a factory contract that mints FT token.

Note: this example use near-sdk3.x, where latest version is 4.0, I have updated the code to use 4.0.

## Resources
### Docs
- [tutorial](https://docs.near.org/tutorials/examples/hello-near)
- [rust sdk](https://docs.near.org/sdk/rust/introduction)
- [develop docs](https://docs.near.org/develop/quickstart-guide)
- [concept](https://docs.near.org/concepts/welcome)
### Real world examples
- [ref finance](https://github.com/ref-finance/ref-contracts) AMM DEX
- [burrow](https://github.com/NearDeFi/burrowland) Lending & Borrowing
- [sputnik](https://github.com/near-daos/sputnik-dao-contract) DAO tooling
- [tonic](https://github.com/tonic-foundation/tonic-core) Orderbook DEX
- [stader](https://github.com/stader-labs/near-liquid-token) Liquid staking
- [meta pool](https://github.com/Narwallets/meta-pool) Liquid staking
- [PembROCK](https://github.com/PembROCK-Finance/rewards-contract) Yield optimizer
- [NEAR wiki](https://github.com/evgenykuzyakov/wiki) on chain wikipeida
