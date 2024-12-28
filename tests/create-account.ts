import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { CreateAccount } from "../target/types/create_account";
import { BankrunProvider, startAnchor } from "anchor-bankrun";
import { Keypair, PublicKey } from "@solana/web3.js";
import { assert } from "chai";
const IDL = require("../target/idl/create_account.json");
const programID = new PublicKey(IDL.address);

describe("create-account", () => {
  let context;
  let provider: BankrunProvider;
  let program: anchor.Program<CreateAccount>;
  let wallet;
  let connection;
  before(async () => {
    context = await startAnchor(
      "",
      [{ name: "create_account", programId: programID }],
      []
    );
    provider = new BankrunProvider(context);
    program = new anchor.Program<CreateAccount>(IDL, provider);
    wallet = (provider.wallet as anchor.Wallet).payer;
    connection = provider.connection;
  });

  it("Is initialized!", async () => {
    // Generate a new keypair for the new account
    const newKeypair = new Keypair();
    // Add your test here.
    const tx = await program.methods
      .initialize()
      .accounts({
        user: wallet.publicKey,
        newAccount: newKeypair.publicKey,
      })
      .signers([newKeypair])
      .rpc();
    console.log("Your transaction signature", tx);

    const lamports = await connection.getMinimumBalanceForRentExemption(0);
    // Check that the account was created
    const accountInfo = await connection.getAccountInfo(newKeypair.publicKey);
    assert(accountInfo.lamports === lamports);
  });
});
