import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Ledger } from "../target/types/ledger";
import { assert } from "chai";

describe("ledger", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.Ledger as Program<Ledger>;

  const userA = anchor.web3.Keypair.generate();
  const userB = anchor.web3.Keypair.generate();

  it("initializes a user account with zero balance and correct owner", async () => {
    await program.methods
      .initialize()
      .accountsPartial({
        user: userA.publicKey,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userA])
      .rpc();

    const account = await program.account.userAccount.fetch(userA.publicKey);
    assert.equal(account.balance.toNumber(), 0);
    assert.ok(account.owner.equals(provider.wallet.publicKey));
  });

  it("rejects a transfer that exceeds the sender's balance", async () => {
    await program.methods
      .initialize()
      .accountsPartial({
        user: userB.publicKey,
        signer: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .signers([userB])
      .rpc();

    try {
      await program.methods
        .transfer(new anchor.BN(10))
        .accountsPartial({
          sender: userA.publicKey,
          receiver: userB.publicKey,
          signer: provider.wallet.publicKey,
        })
        .rpc();
      assert.fail("transfer should have failed with insufficient funds");
    } catch (err) {
      assert.match(String(err), /InsufficientFunds|0x1/);
    }
  });
});
