import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Solar } from "../target/types/solar";
import * as assert from "assert";
import {findProgramAddressSync} from "@project-serum/anchor/dist/cjs/utils/pubkey";
import { utf8 } from '@project-serum/anchor/dist/cjs/utils/bytes'
import { PublicKey, Keypair } from '@solana/web3.js'
import * as bs58 from "bs58";

require('dotenv').config()

export type UserType = anchor.IdlTypes<Solar>["UserType"];


const getAccountWithBalance = async () => {
    const wallet = anchor.web3.Keypair.generate();

    const provider = anchor.getProvider();

    const token_airdrop = await anchor.getProvider().connection.requestAirdrop(wallet.publicKey, anchor.web3.LAMPORTS_PER_SOL * 10);
    const latestBlockHash = await provider.connection.getLatestBlockhash();
    await provider.connection.confirmTransaction({
        blockhash: latestBlockHash.blockhash,
        lastValidBlockHeight: latestBlockHash.lastValidBlockHeight,
        signature: token_airdrop,
    });
    return wallet
}

const createAndFetchUserPDA = async (username: string, userType: UserType, program: Program<Solar>) => {
    const user = await getAccountWithBalance();
    const userPDA = generatePDAWithSeed(user.publicKey, program)

    await program.methods.setupUser(username, userType).accounts(
        {
            user: userPDA,
            authority: user.publicKey,
            systemProgram: anchor.web3.SystemProgram.programId,
        }).signers([user]).rpc()

    const userAccount = await program.account.userProfile.fetch(userPDA);

    // @ts-ignore
    assert.equal(user.publicKey.toBase58(), userAccount.authority.toBase58());

    return {
        userAccount,
        PDA: userPDA,
        signer: user
    };
}

const fetchUserAccount = async (userPDA: PublicKey, program: Program<Solar>) => {
    return await program.account.userProfile.fetch(userPDA);
}

const generatePDAWithSeed = (publicKey: PublicKey, program: Program<Solar>) => {
    const [userPDA] = findProgramAddressSync([utf8.encode('USER_STATE'), publicKey.toBuffer()], program.programId);
    return userPDA
}

const getAdminAccount = async (program: Program<Solar>) => {
   const user = Keypair.fromSecretKey(
       bs58.decode(
           process.env.ADMIN_WALLET_PRIVATE_KEY
       )
   );
    const PDA = generatePDAWithSeed(user.publicKey, program);
    const userAccount = await fetchUserAccount(PDA, program);
    return {
       PDA,
       userAccount,
       user
    }
}

describe("solar", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Solar as Program<Solar>;

    it("Testing user authority to edit profile", async () => {

        // Create a fresh user
        const userType = { client: {} }
        let {userAccount, PDA, signer} = await createAndFetchUserPDA("MSMSMSM", userType, program)

        // Set up the user profile
        const email = "vaithi.genghiskhan@gmail.com";
        const fName = "MSMSMSM";
        const lName = "Jr."
        const phone = "1111111111"

        // Run txn to alter with provided info
        // @ts-ignore
        await program.methods.setupUserProfile(email, fName, lName, phone).accounts(
            {
                user: PDA,
                authority: userAccount.authority,
                systemProgram: anchor.web3.SystemProgram.programId,
            }
        ).signers([signer]).rpc()

        // Re-fetch account
        userAccount = await fetchUserAccount(PDA, program)

        // Check account info against passed params
        assert.equal(userAccount.email, email);
        assert.equal(userAccount.firstName, fName);
        assert.equal(userAccount.lastName, lName);
        assert.equal(userAccount.phone, phone);
    });

  it("Creating a client user", async () => {
    const userType = { client: {} }
    const {userAccount} = await createAndFetchUserPDA("Client1", userType, program);

    assert.ok(userAccount.typeOfUser.client);
  });

  it("Creating a lawyer user", async () => {
      const userType = { lawyer: {} }
      const {userAccount} = await createAndFetchUserPDA("Client1", userType, program);

      assert.ok(userAccount.typeOfUser.lawyer);
  });

  it("Creating a judge user", async () => {
      const userType = { judge: {} }
      const {userAccount} = await createAndFetchUserPDA("Lawyer1", userType, program);

      assert.ok(userAccount.typeOfUser.judge);
  });

  it("Verifying a judge user with Admin User", async () => {
      const userType = { judge: {} }
      let {userAccount: JudgeAccount, PDA: JudgePDA} = await createAndFetchUserPDA("Judge1", userType, program);

      // Check user is not already verified and that user is of type judge
      assert.ok(!JudgeAccount.verified)
      assert.ok(JudgeAccount.typeOfUser.judge);

      // Get admin PDA
      const {user: admin, PDA: AdminPDA, userAccount: AdminAcount} = await getAdminAccount(program)

      // Verify judge with it
      await program.methods.verifyUser().accounts({
          user: JudgePDA,
          admin: AdminPDA,
          systemProgram: anchor.web3.SystemProgram.programId
      }).signers([admin]).rpc()

      // Fetch judge account again and verify
      JudgeAccount = await fetchUserAccount(JudgePDA, program)
      assert.ok(JudgeAccount.verified);
  });

  it("Verifying a judge user without Admin User", async () => {
      return true
  });

});
