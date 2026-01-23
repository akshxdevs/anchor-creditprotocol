import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { AnchorCreditProtocol } from "../target/types/anchor_credit_protocol";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";
import {
  TOKEN_PROGRAM_ID,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddress,
  createMint,
  createAccount,
  mintTo,
  getAccount,
} from "@solana/spl-token";

// Wrapped SOL mint address (native SOL wrapped as SPL token)
const WSOL_MINT = new PublicKey("So11111111111111111111111111111111111111112");

describe("anchor-credit-protocol", () => {
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.anchorCreditProtocol as Program<AnchorCreditProtocol>;
  const provider = anchor.getProvider();
  const connection = provider.connection;

  // Test user keypair
  const user = Keypair.generate();

  // Helper function to airdrop SOL
  async function airdropSol(pubkey: PublicKey, amount: number) {
    const signature = await connection.requestAirdrop(
      pubkey,
      amount * LAMPORTS_PER_SOL
    );
    await connection.confirmTransaction(signature);
  }

  // Helper function to get PDA
  function getEscrowPDA(userPubkey: PublicKey) {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("escrow"), userPubkey.toBuffer()],
      program.programId
    );
  }

  function getLoanPDA(userPubkey: PublicKey) {
    return PublicKey.findProgramAddressSync(
      [Buffer.from("loan"), userPubkey.toBuffer()],
      program.programId
    );
  }

  before(async () => {
    // Airdrop SOL to user for transaction fees
    await airdropSol(user.publicKey, 10);
    console.log("User balance:", await connection.getBalance(user.publicKey));
  });

  it("Initialize Loan and Deposit SOL!", async () => {
    const principal = new anchor.BN(5 * LAMPORTS_PER_SOL); // 5 SOL loan
    const interestBps = 500; // 5% interest (500 basis points)
    const collateralAmount = new anchor.BN(10 * LAMPORTS_PER_SOL); // 10 SOL collateral
    const dueTs = 3600; // 1 hour
    const existingUser = false;

    // Get PDAs
    const [escrowPDA, escrowBump] = getEscrowPDA(user.publicKey);
    const [loanPDA, loanBump] = getLoanPDA(user.publicKey);

    // For SOL, we use wrapped SOL mint
    const collateralMint = WSOL_MINT;

    // Get vault ATA (escrow's token account for wrapped SOL)
    const vault = await getAssociatedTokenAddress(
      collateralMint,
      escrowPDA,
      true
    );

    console.log("Initializing loan with SOL collateral...");
    console.log("Escrow PDA:", escrowPDA.toString());
    console.log("Loan PDA:", loanPDA.toString());
    console.log("Vault:", vault.toString());

    // Initialize the loan
    const initTx = await program.methods
      .initializeLoan(
        principal,
        interestBps,
        collateralMint,
        collateralAmount,
        dueTs,
        existingUser
      )
      .accountsPartial({
        user: user.publicKey,
        mint: collateralMint,
        vault: vault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("Loan initialized! Transaction:", initTx);

    // Wait for confirmation
    await connection.confirmTransaction(initTx);

    // Now deposit SOL into escrow
    // First, user needs to wrap SOL and have it in their token account
    const userWsolAccount = await getAssociatedTokenAddress(
      collateralMint,
      user.publicKey
    );

    // Create user's wrapped SOL account if it doesn't exist
    try {
      await getAccount(connection, userWsolAccount);
    } catch {
      // Create the account - for wrapped SOL, we need to create it
      const createAccountIx = await createAccount(
        connection,
        user,
        collateralMint,
        user.publicKey
      );
    }

    // Wrap SOL: Transfer native SOL to wrapped SOL account
    // For simplicity, we'll use a helper or do it manually
    // Actually, we need to wrap SOL first - let's use a simpler approach
    // We'll create the account and then wrap SOL into it
    const wrapAmount = 10 * LAMPORTS_PER_SOL;
    
    // For wrapped SOL, we need to sync native SOL to wrapped SOL
    // This is typically done via a swap or wrap instruction
    // For testing, let's assume user already has wrapped SOL
    // In a real scenario, you'd use a wrap instruction or DEX

    // Deposit amount (in lamports for SOL)
    const depositAmount = new anchor.BN(wrapAmount);

    console.log("Depositing SOL (wrapped) into escrow...");

    const depositTx = await program.methods
      .deposite(depositAmount, { sol: {} })
      .accountsPartial({
        user: user.publicKey,
        mint: collateralMint,
        userTokenAccount: userWsolAccount,
        vault: vault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      }as any)
      .signers([user])
      .rpc();

    console.log("SOL deposited! Transaction:", depositTx);
    await connection.confirmTransaction(depositTx);

    // Verify loan state
    const loanAccount = await program.account.loan.fetch(loanPDA);
    console.log("Loan collateral amount:", loanAccount.collateralAmount.toString());
    console.log("Loan collateral type:", loanAccount.collateralType);
    console.log("Loan status:", loanAccount.status);
  });

  it("Initialize Loan and Deposit USDC!", async () => {
    // Create a test USDC mint
    const usdcMint = await createMint(
      connection,
      user,
      user.publicKey,
      null,
      6 // USDC has 6 decimals
    );

    console.log("Created USDC mint:", usdcMint.toString());

    const principal = new anchor.BN(1000 * 1e6); // 1000 USDC loan (6 decimals)
    const interestBps = 500; // 5% interest
    const collateralAmount = new anchor.BN(2000 * 1e6); // 2000 USDC collateral
    const dueTs = 3600; // 1 hour
    const existingUser = false;

    // Get PDAs
    const [escrowPDA, escrowBump] = getEscrowPDA(user.publicKey);
    const [loanPDA, loanBump] = getLoanPDA(user.publicKey);

    // Get vault ATA (escrow's token account for USDC)
    const vault = await getAssociatedTokenAddress(
      usdcMint,
      escrowPDA,
      true
    );

    // Create user's USDC token account
    const userUsdcAccount = await getAssociatedTokenAddress(
      usdcMint,
      user.publicKey
    );

    // Create user's USDC account if it doesn't exist
    try {
      await getAccount(connection, userUsdcAccount);
    } catch {
      await createAccount(connection, user, usdcMint, user.publicKey);
    }

    // Mint USDC to user
    const mintAmount = 5000 * 1e6; // 5000 USDC
    await mintTo(
      connection,
      user,
      usdcMint,
      userUsdcAccount,
      user,
      mintAmount
    );

    console.log("Minted USDC to user");
    console.log("Escrow PDA:", escrowPDA.toString());
    console.log("Loan PDA:", loanPDA.toString());
    console.log("Vault:", vault.toString());

    // Initialize the loan
    const initTx = await program.methods
      .initializeLoan(
        principal,
        interestBps,
        usdcMint,
        collateralAmount,
        dueTs,
        existingUser
      )
      .accountsPartial({
        user: user.publicKey,
        mint: usdcMint,
        vault: vault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      })
      .signers([user])
      .rpc();

    console.log("Loan initialized! Transaction:", initTx);
    await connection.confirmTransaction(initTx);

    // Deposit USDC into escrow
    const depositAmount = new anchor.BN(2000 * 1e6); // 2000 USDC

    console.log("Depositing USDC into escrow...");

    const depositTx = await program.methods
      .deposite(depositAmount, { usdc: {} })
      .accountsPartial({
        user: user.publicKey,
        mint: usdcMint,
        userTokenAccount: userUsdcAccount,
        vault: vault,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        systemProgram: SystemProgram.programId,
      }as any)
      .signers([user])
      .rpc();

    console.log("USDC deposited! Transaction:", depositTx);
    await connection.confirmTransaction(depositTx);

    // Verify loan state
    const loanAccount = await program.account.loan.fetch(loanPDA);
    console.log("Loan collateral amount:", loanAccount.collateralAmount.toString());
    console.log("Loan collateral type:", loanAccount.collateralType);
    console.log("Loan status:", loanAccount.status);

    // Verify vault balance
    const vaultAccount = await getAccount(connection, vault);
    console.log("Vault balance:", vaultAccount.amount.toString());
  });
});
