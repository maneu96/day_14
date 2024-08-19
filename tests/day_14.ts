import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day14 } from "../target/types/day_14";

describe("day_14", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Day14 as Program<Day14>;

  let myKeypair = anchor.web3.Keypair.generate(); //generate a signer keypair
  let myKeypair2 = anchor.web3.Keypair.generate(); //generate a signer keypair

  it("It is signed by a single signer", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      signer1: program.provider.publicKey,
    }).rpc();
    console.log("The signer1:", program.provider.publicKey.toBase58());
  });

  it("It is signed by two signers", async () => {

    const tx = await program.methods.initializeTwoSign().accounts({
      signer1: program.provider.publicKey,
      signer2: myKeypair.publicKey,
    }).signers([myKeypair]).rpc();
    console.log("The signer1: ", program.provider.publicKey.toBase58());
    console.log("The signer2: ", myKeypair.publicKey.toBase58());
  });


  it('It is signed by three signers', async () => {
    const tx = await program.methods.initializeThreeSign().accounts({
      signer1: program.provider.publicKey,
      signer2: myKeypair.publicKey,
      signer3: myKeypair2.publicKey
    }).signers([myKeypair, myKeypair2]).rpc();
    console.log("The signer1: ", program.provider.publicKey.toBase58());
    console.log("The signer2: ", myKeypair.publicKey.toBase58());
    console.log("The signer3: ", myKeypair2.publicKey.toBase58());
  });
});
