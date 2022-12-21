// const anchor = require('@project-serum/anchor')
// const { assert } = require('chai')
const assert = require("assert");
const anchor = require("@project-serum/anchor");
const { SystemProgram } = anchor.web3;

describe('mysolanaapp', () => {
  // Configure the client to use the local cluster.
  const provider = anchor.getProvider()
  anchor.setProvider(provider)
  let baseAccount
  // it("Is initialized!", async () => {
  //   // Add your test here.
  //   const program = anchor.workspace.Mysolanaapp;
  //   const tx = await program.methods.initialize().rpc();
  //   console.log("Your transaction signature", tx);
  // });
  const program = anchor.workspace.Mysolanaapp
  it("create a counter", async () => {
    baseAccount = anchor.web3.Keypair.generate()
    await program.rpc.create({
      accounts: {
        baseAccount: baseAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: SystemProgram.programId,
      },
      signers: [baseAccount],
    })

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey)
    console.log('count 0: ', account.count.toString())
    assert.ok(account.count.toString() == 0)
  })

  it("increment the counter", async () => {
    await program.rpc.increment({
      accounts: {
        baseAccount: baseAccount.publicKey,
      },
    })

    const account = await program.account.baseAccount.fetch(baseAccount.publicKey)
    console.log('count 1: ', account.count.toString())
    assert.ok(account.count.toString() == 1)
  })
})
