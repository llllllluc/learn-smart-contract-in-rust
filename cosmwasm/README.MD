# CosmWasm

## Cosmos overview

Cosmos is not a single blockchain, instead it provides a framework to make it easier for anyone to launch their own blockchain, and these chains can talk to each other. If Ethereum is the world computer, Cosmos is the internet of blockchain, or thousands of community computers.

The framework has 3 main pillars

1. Tendermint consensus: a robust and battle tested BFT consensus engine.
   - There are 2 flagship implementation:
     1. [tendermint](https://github.com/tendermint/tendermint) is the golang implementation, also the mostly adopted one, usually paired with Cosmos SDK.
     2. [tendermint-rs](https://github.com/informalsystems/tendermint-rs) is the rust implementation, used by some ABCI client alternatives (i.e. not using Cosmos SDK, but separate solutions. Examples include [nomic](https://github.com/nomic-io/orga), [penumbra](https://github.com/penumbra-zone/penumbra), etc).
2. Cosmos SDK: a golang framework to implement custom logic, connected to the tendermint engine via ABCI.
   - You can think of tendermint as a server that expose set of interface called ABCI, and Cosmos SDK as an ABCI client. SDK comes with a set of modules that provides basic functionalities like token transfer, staking and governance.
3. IBC: In analogy to TCP/IP, IBC a set of [standards](https://github.com/cosmos/ibc) defining how blockchain can communicate with each other.
   - Similar to tendermint, it also has a [go](https://github.com/cosmos/ibc-go) implementation and a [rust](https://github.com/cosmos/ibc-rs) implementation.
   - Today status quo of IBC is live between chains built using Cosmos SDK, you can send token or arbitrary data between any two IBC enabled chains. The security of IBC in production today relies on light client verification, essentially each chain acts as the light client of counter party chain. As long as both chain are not compromised, and there exist at least 1 relayer, then technically IBC is safe to use.

## CosmWasm overview

As the name suggests, CosmWasm aims to bring Wasm VM to the Cosmos ecosystem. One drawback of implementing things in Cosmos SDK is, when developer add new features, we need to restart the chain, basically do a hard fork. This is not very convenient if we want to iterate fast and experiment somethings. So it's natural to ask, can we introduce some kind of VM as a Cosmos SDK module (along with other built in modules), then developers can program in this sandbox VM and not have to interrupt the chain.

This is the drive of building CosmWasm. To add a VM, there are basically 2 ways I see.

1. Port existing VM to Cosmos, such as EVM or Wasm.
   - EVM is being actively working on by many teams in Cosmos as well. Check out [Ethermint](https://github.com/evmos/ethermint) if you are interested.
   - Wasm is the CosmWasm we have today. It's been live since late 2020? on Terra and gained massive adoption in many Cosmos SDK based chains. It helped secured more than 10 billion of dollars at peak before Terra meltdown.
2. Build a new VM from ground up.
   - Agoric kinda took this approach, built its VM based on harden javascript.

Fast forward to today, CosmWasm has become a very popular choice to build new tings in Cosmos, as popular as Cosmos SDK. People generally agree for things are easier to change and evolve quickly, you choose CosmWasm, for things that prioritize stability and want to have more low level control, you choose Cosmos SDK.

## Tutorials
There are many great tutorials about CosmWasm. Below are ones I enjoyed a lot.

### Zero to hero
[link](https://github.com/Callum-A/cosmwasm-zero-to-hero)

This 15 lesson tutorial guides you build a polling app that users can create poll and vote. It covers the main 3 components of CosmWasm contract: instantiate, execute and query. It also teaches you how to write unite test, store state onchain, generate JSON schema.

If you only want to choose 1 tutorial, this is the go to tutorial I recommend, it only takes 1 week to complete and can reflect a lot. And it's friendly to rust beginners.

### Terra cw20 token factory
[Link](https://docs.terra.money/develop/terrain/cw20-factory)

Cross contract calls.

## Resources
- [official doc](https://docs.cosmwasm.com/docs/1.0/)
- [official book like rust book](https://book.cosmwasm.com/index.html)
- [zero to hero](https://github.com/Callum-A/cosmwasm-zero-to-hero)
- [terra academy](https://academy.terra.money/collections)
- [cw template](https://github.com/CosmWasm/cw-template/tree/main/src) an example counter contract
- [cw-plus](https://github.com/CosmWasm/cw-plus) a set of standard contract cw20, cw721 like erc20, erc721 by OpenZeppelin.
- [cosmwasm on rollmint](https://docs.celestia.org/developers/cosmwasm/) cw contract on celestia rollup
- [terra developer doc](https://docs.terra.money/develop/get-started/)
- [osmosis developer doc](https://docs.osmosis.zone/cosmwasm/)
