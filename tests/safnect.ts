import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { assert, expect } from "chai";
import { getAssociatedTokenAddress, getAccount } from "@solana/spl-token"
import { Safnect } from "../target/types/safnect";

describe("safnect", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Safnect as Program<Safnect>;

  const subscription = {
    tag: "solana",
    description: "Wow, 2024.3.31. end of Mar.",
    qty: 10
  }

  const [safnectPda] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from(subscription.tag), provider.wallet.publicKey.toBuffer()],
    program.programId
  )

  const [mint] = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("mint")],
    program.programId
  )

  it("initailizes the token", async () => {
    // Add your test here.
    const tx = await program.methods.initialize()
    .accounts({mint:mint}).rpc();
    console.log("Your transaction signature", tx);
  });

  it("add subscription", async () => {
    const tokenAccount = await getAssociatedTokenAddress(
      mint,
      provider.wallet.publicKey
    )
    // Add your test here.
    const tx = await program.methods
      .addSubscription(subscription.tag, subscription.description, subscription.qty)
      .accounts({
        subscription: safnectPda,
        tokenAccount: tokenAccount,
        mint: mint,
      })
      .rpc()
  
    const account = await program.account.subscription.fetch(safnectPda)
    expect(subscription.tag === account.tag)
    expect(subscription.qty === account.qty)
    expect(subscription.description === account.description)
    expect(account.createdBy === provider.wallet.publicKey)

    const userAta = await getAccount(provider.connection, tokenAccount)
    expect(Number(userAta.amount)).to.equal((10 * 10) ^ 6)
  });

  it("subscription updated", async() =>{
    const newDescription = "wow this is new"
    const qty = 9

    const tx = await program.methods.updateSubscription(subscription.tag, newDescription, qty)
    .accounts({subscription: safnectPda})
    .rpc()

    const account = await program.account.subscription.fetch(safnectPda)
    expect(subscription.tag === account.tag)
    expect(qty === account.qty)
    expect(newDescription === account.description)
    expect(account.createdBy === provider.wallet.publicKey)

  });
});
