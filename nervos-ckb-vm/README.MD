# CKB
## hello world experience
Nervos is a UTXO based blockchain like bitcoin, so it's programming focuses on verification instead of computation. i.e. when we process a new tx, we are given both the input cells (UTXO in Nervos) and output cells then we run some validation logic, where in account based blockchains (pretty much every other chains, like EVM, cosmwasm chains) tx result is evaluation when tx is processed.

The hello world contract in Nervos is similar to others, we also define our own token. But unlike others (erc20, cw20) that defines a set of interface. Nervos UDT (erc20 in Nervos) only defines 1 check, which is input amount must be smaller than output amount, unless it's called by UDT owner, and that's it.

Also the tooling and documentation of Nervos is lacking, comparing to EVM and cosmwasm. I had to solve the compatibility issue caused by using newer version of dev tool.

## L2 app idea
### liquity
- [frontend](https://liquity.app/)
- [doc](https://docs.liquity.org/)
- [repo](https://github.com/liquity/dev)
- [site](https://www.liquity.org/)

## Docs
- [rfc](https://github.com/nervosnetwork/rfcs)
- [community doc](https://docs-xi-two.vercel.app/)
- [official doc](https://docs.nervos.org/)
- [old official doc](https://docs-old.nervos.org/introduction/welcome)

## developer
dapp development
- [kuai](https://github.com/ckb-js/kuai)
- [capsule](https://github.com/nervosnetwork/capsule)
- [trampoline](https://github.com/Tempest-Protocol/trampoline)

interacting with chain
- [lumos](https://github.com/ckb-js/lumos)
- [lumos doc](https://lumos-website.vercel.app/)

tooling
- [ckb tools](https://github.com/jordanmack/ckb-tools)
- [issue sudt from cli](https://github.com/jordanmack/sudt-cli)

tutorials
- [CKB course](https://github.com/jordanmack/developer-training-course)
- [zero to hero](https://zero2ckb.ckbapp.dev/learn)
- [workshop issue nft using capsule and lumos](https://github.com/nervosnetwork/dapps-on-ckb-workshop-code)
- [pr upgrade capsule and lumos dep](https://github.com/nervosnetwork/dapps-on-ckb-workshop-code/pull/1)

standards
- [cota](https://www.cotadev.io/) issue layer1.5 token to save space

examples
- [.bit](https://www.did.id/) ENS like domain name and ID system. This is the biggest app on Nervos so they have the best ckb contract code you can refer to.
	- https://github.com/dotbitHQ/das-contracts
- [ickb](https://github.com/ickb) liquid staking
	- https://talk.nervos.org/t/looking-for-feedback-ickb-a-tokenization-of-nervosdao-deposits/6772
- [nexisdao](https://github.com/NexisDao/NexisDao-core) liquid staking + maker
	- https://talk.nervos.org/t/nexisdao/6085
- [glias](https://github.com/glias) amm dex
	- https://zoe-zhouzhou.github.io/gliaswap-docs/docs/
- [growfi](https://github.com/GrowFi-labs) udtswap
	- https://talk.nervos.org/t/growfi-udt-swap/4382
	- https://talk.nervos.org/t/growfi-nervos-defi-infrastructure/4215

## discussion
chained tx
- https://talk.nervos.org/t/non-blocking-chained-transaction-and-its-applications-in-ckb/6649
- https://talk.nervos.org/t/random-idea-to-what-extent-can-open-transaction-resolve-state-contention-on-cell-model/6632

dev idea
- https://talk.nervos.org/t/cell-layer1-5/6250
- https://talk.nervos.org/t/a-ckb-programming-paradigm-proposal-simulate-account-model-on-cell-model/4009

lock
- https://talk.nervos.org/t/acp-lock-check-onwer-mode/6121

token script (udt, etc...)
- https://talk.nervos.org/t/script-extensible-script/5699
- https://talk.nervos.org/t/rfc-extensible-udt/5337
- https://talk.nervos.org/t/rfc-regulation-compliance-extension/5338
- https://talk.nervos.org/t/discussion-on-the-regulatory-extension-to-sudt/5060
- https://talk.nervos.org/t/ckb-type-script/1329
- https://talk.nervos.org/t/sudt/5208
- https://talk.nervos.org/t/rfc-simple-udt-draft-spec/4333
- https://talk.nervos.org/t/proposal-minimal-udt-implementation-and-extensions/3932
- https://talk.nervos.org/t/discussion-on-udt-standard-evaluation-criterion/3774
- https://talk.nervos.org/t/rfc-compact-udt-lock/6110
- https://talk.nervos.org/t/rfc-modified-extensible-udt-for-better-script-driven-udt/6642
- https://talk.nervos.org/t/rfc-udt-info-in-witness-draft-spec/6352
- https://talk.nervos.org/t/rfc-info-cell-for-sudt-draft-spec/6280
- https://talk.nervos.org/t/approach-to-designing-a-user-defined-token-standard-on-ckb-part-1/3855

app idea
- https://talk.nervos.org/t/tide-finance-stablecoin-protocol/4765
- https://talk.nervos.org/t/the-old-tide-protocol/4799/1

dex
- https://medium.com/nervosnetwork/layer-2-dex-designs-on-nervos-ckb-63e11282aaa0
- https://talk.nervos.org/t/layer-2-dex/4901
- https://talk.nervos.org/t/exy-pw-nervos/4679

tooling
- https://talk.nervos.org/t/mint-sudt-via-contract/4931
- https://talk.nervos.org/t/keyper-scatter-dapp/4430
- https://talk.nervos.org/t/ckb-token-mint-fullstack-dapp-keyper-web-wallet-keyper-bridge/4675

challenge
- https://talk.nervos.org/t/nervos-ckb/5067
