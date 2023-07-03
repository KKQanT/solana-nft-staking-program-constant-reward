import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { OgaStaking } from "../target/types/nft_staking";

describe("oga-staking", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.OgaStaking as Program<OgaStaking>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
