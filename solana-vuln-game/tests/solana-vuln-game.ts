import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SolanaVulnGame } from "../target/types/solana_vuln_game";
import {Keypair, PublicKey} from "@solana/web3.js" 
import { assert } from "chai";
import { getAssociatedTokenAddressSync, TOKEN_PROGRAM_ID, ASSOCIATED_TOKEN_PROGRAM_ID } from "@solana/spl-token";
describe("solana-vuln-game", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env()
  anchor.setProvider(provider);
  
  const program = anchor.workspace.SolanaVulnGame as Program<SolanaVulnGame>;
  const owner = Keypair.generate()
  const config = PublicKey.findProgramAddressSync([Buffer.from("config")], program.programId)[0]
  const systemProgram = anchor.web3.SystemProgram.programId;
  const payer = Keypair.generate();
  const tokenProgram = TOKEN_PROGRAM_ID
  const tokenMint = PublicKey.findProgramAddressSync([Buffer.from("payment_token"), owner.publicKey.toBuffer(), config.toBuffer()],program.programId)[0]
  const user = PublicKey.findProgramAddressSync(
    [
      Buffer.from("user_account"),
      payer.publicKey.toBuffer()
    ],
    program.programId
  )[0]

  
  before(async () => {
    const sig_owner = await provider.connection.requestAirdrop(
      owner.publicKey,
      5 * anchor.web3.LAMPORTS_PER_SOL
    );
    await program.provider.connection.confirmTransaction(sig_owner);
    // funding the Payer and owner accounts with 5 sol each
    const sig_payer = await program.provider.connection.requestAirdrop(
      payer.publicKey,
      5 * anchor.web3.LAMPORTS_PER_SOL
    );
    await program.provider.connection.confirmTransaction(sig_payer);

    await program.methods.initializeConfig()
    .accountsStrict({
      admin: owner.publicKey,
      config,
      systemProgram,
      tokenProgram,
      tokenMint
    })
    .signers([owner]).rpc()

    
    await program.methods.initializeUser()
    .accountsStrict({
      systemProgram,
      user: payer.publicKey,
      userAccount: user
    }).signers([payer])
    .rpc()

  })

  it("Successfully validates correct input and awards points", async () => {
    const validAnswer = "4+10";
    const userATA = getAssociatedTokenAddressSync(tokenMint, payer.publicKey, true);

    await program.methods.spaceValidation(validAnswer)
      .accountsStrict({
        signer: payer.publicKey,
        config,
        mintAccount: tokenMint,
        user: user,
        userAta: userATA,
        systemProgram,
        tokenProgram,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([payer])
      .rpc();

    const userAccount = await program.account.userState.fetch(user);
    assert.equal(userAccount.points, 100, "User points should be 100 after valid input");
  });

  it("Rejects invalid input with appropriate error", async () => {
    const invalidAnswer = "wrong_answer";
    const userATA = getAssociatedTokenAddressSync(tokenMint, payer.publicKey, true);

    try {
      await program.methods.spaceValidation(invalidAnswer)
        .accountsStrict({
          signer: payer.publicKey,
          config,
          mintAccount: tokenMint,
          user: user,
          userAta: userATA,
          systemProgram,
          tokenProgram,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
        })
        .signers([payer])
        .rpc();
      assert.fail("The transaction should have failed due to invalid input");
    } catch (err) {
      assert.equal(err.error.errorCode.code, "InvalidInput", "Expected InvalidInput error");
    }
  });

  it("Input validation checks", async () => {
    const validAnswer = "if name.len() > 10";
    const userATA = getAssociatedTokenAddressSync(tokenMint, payer.publicKey, true);

    await program.methods.inputValidation(validAnswer)
      .accountsStrict({
        signer: payer.publicKey,
        config,
        mintAccount: tokenMint,
        user: user,
        userAta: userATA,
        systemProgram,
        tokenProgram,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([payer])
      .rpc();

    const userAccount = await program.account.userState.fetch(user);
    assert.equal(userAccount.points, 200, "User points should be 100 after valid input");
  });

  it("Fails if wrong answer is given as input", async () => {
    const invalidAnswer = "wrong_answer";
    const userATA = getAssociatedTokenAddressSync(tokenMint, payer.publicKey, true);

    try {
      await program.methods.inputValidation(invalidAnswer)
        .accountsStrict({
          signer: payer.publicKey,
          config,
          mintAccount: tokenMint,
          user: user,
          userAta: userATA,
          systemProgram,
          tokenProgram,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
        })
        .signers([payer])
        .rpc();
      assert.fail("The transaction should have failed due to invalid input");
    } catch (err) {
      assert.equal(err.error.errorCode.code, "InvalidInput", "Expected InvalidInput error");
    }
  });

  it("Arithmetic underflow checks", async () => {
    const validAnswer = "sender.points.checked_sub(amount).unwrap()?";
    const userATA = getAssociatedTokenAddressSync(tokenMint, payer.publicKey, true);

    await program.methods.arithmeticUnderflow(validAnswer)
      .accountsStrict({
        signer: payer.publicKey,
        config,
        mintAccount: tokenMint,
        user: user,
        userAta: userATA,
        systemProgram,
        tokenProgram,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([payer])
      .rpc();

    const userAccount = await program.account.userState.fetch(user);
    assert.equal(userAccount.points, 300, "User points should be 100 after valid input");
  });

  it("Fails if wrong answer is given as input", async () => {
    const invalidAnswer = "wrong_answer";
    const userATA = getAssociatedTokenAddressSync(tokenMint, payer.publicKey, true);

    try {
      await program.methods.arithmeticUnderflow(invalidAnswer)
        .accountsStrict({
          signer: payer.publicKey,
          config,
          mintAccount: tokenMint,
          user: user,
          userAta: userATA,
          systemProgram,
          tokenProgram,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
        })
        .signers([payer])
        .rpc();
      assert.fail("The transaction should have failed due to invalid input");
    } catch (err) {
      assert.equal(err.error.errorCode.code, "InvalidInput", "Expected InvalidInput error");
    }
  });
  
  it("Program ID checks", async () => {
    const validAnswer = "ctx.accounts.system_program.key != &solana_program::system_program::ID";
    const userATA = getAssociatedTokenAddressSync(tokenMint, payer.publicKey, true);

    await program.methods.programIdVerification(validAnswer)
      .accountsStrict({
        signer: payer.publicKey,
        config,
        mintAccount: tokenMint,
        user: user,
        userAta: userATA,
        systemProgram,
        tokenProgram,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([payer])
      .rpc();

    const userAccount = await program.account.userState.fetch(user);
    assert.equal(userAccount.points, 400, "User points should be 100 after valid input");
  });

  it("Fails if program id check is wrong", async () => {
    const invalidAnswer = "wrong_answer";
    const userATA = getAssociatedTokenAddressSync(tokenMint, payer.publicKey, true);

    try {
      await program.methods.programIdVerification(invalidAnswer)
        .accountsStrict({
          signer: payer.publicKey,
          config,
          mintAccount: tokenMint,
          user: user,
          userAta: userATA,
          systemProgram,
          tokenProgram,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
        })
        .signers([payer])
        .rpc();
      assert.fail("The transaction should have failed due to invalid input");
    } catch (err) {
      assert.equal(err.error.errorCode.code, "InvalidInput", "Expected InvalidInput error");
    }
  });

  it("Arithmetic overflow checks", async () => {
    const validAnswer = "receiver.points.checked_add(amount).unwrap()?";
    const userATA = getAssociatedTokenAddressSync(tokenMint, payer.publicKey, true);

    await program.methods.arithmeticOverflow(validAnswer)
      .accountsStrict({
        signer: payer.publicKey,
        config,
        mintAccount: tokenMint,
        user: user,
        userAta: userATA,
        systemProgram,
        tokenProgram,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
      })
      .signers([payer])
      .rpc();

    const userAccount = await program.account.userState.fetch(user);
    assert.equal(userAccount.points, 500, "User points should be 100 after valid input");
  });

  it("Fails if wrong answer is given as input for overflow", async () => {
    const invalidAnswer = "wrong_answer";
    const userATA = getAssociatedTokenAddressSync(tokenMint, payer.publicKey, true);

    try {
      await program.methods.arithmeticOverflow(invalidAnswer)
        .accountsStrict({
          signer: payer.publicKey,
          config,
          mintAccount: tokenMint,
          user: user,
          userAta: userATA,
          systemProgram,
          tokenProgram,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID
        })
        .signers([payer])
        .rpc();
      assert.fail("The transaction should have failed due to invalid input");
    } catch (err) {
      assert.equal(err.error.errorCode.code, "InvalidInput", "Expected InvalidInput error");
    }
  });

});
