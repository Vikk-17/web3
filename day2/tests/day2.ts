import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Day2 } from "../target/types/day2";

describe("day2", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.day2 as Program<Day2>;

  it("Is initialized!", async () => {
      // Add your test here.
      const tx = await program.methods.initialize(new anchor.BN(777), new anchor.BN(888), "testing solana").rpc();
      console.log("Your transaction signature", tx);
  });

  // new test
  it("Array test", async () => {
      const tx = await program.methods.array([new anchor.BN(777), new anchor.BN(888), new anchor.BN(666), new anchor.BN(666)]).rpc();
      console.log("The transaction signature: ", tx);
  });

  // underflow test case
  it("Underflow test case", async () => {
      const tx = await program.methods.underflowtest(
          new anchor.BN(0), new anchor.BN(1)
      ).rpc();
      console.log("The value is: ", tx);
  });
});
