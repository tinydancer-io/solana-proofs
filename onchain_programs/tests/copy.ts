import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Copy } from "../target/types/copy.ts";

describe("copy", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Copy as Program<Copy>;

  it("Is initialized!", async () => {
    // Add your test here.
    console.log(program.provider.wallet.publicKey.toString())
    let [pda,bump] = anchor.web3.PublicKey.findProgramAddressSync([Buffer.from("copy_hash")],program.programId)
    console.log(pda.toString())
  
    const tx = await program.methods.copyHash(new anchor.BN(bump)).accounts({
      creator: program.provider.wallet.publicKey,
      sourceAccount: pda,
      copyAccount: pda,
      clock: anchor.web3.SYSVAR_CLOCK_PUBKEY,
      
      systemProgram: anchor.web3.SystemProgram.programId,
    }).rpc();
    console.log("Your transaction signature", tx);
  });
});
