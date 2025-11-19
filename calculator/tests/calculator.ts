import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Calculator } from "../target/types/calculator";

describe("calculator", () => {
    // Configure the client to use the local cluster.
    anchor.setProvider(anchor.AnchorProvider.env());

    const program = anchor.workspace.calculator as Program<Calculator>;

    it("Is initialized!", async () => {
        // Add your test here.
        const tx = await program.methods.initialize().rpc();
        console.log("Your transaction signature", tx);
    });

  it("Addition test", async() => {
      const tx = await program.methods.addition(
          new anchor.BN(222),
          new anchor.BN(333),
      ).rpc();
      console.log("Addition signature: ", tx);
  });

  it("Substraction test", async() => {
      try {
          const tx = await program.methods.substraction(
              new anchor.BN(444),
              new anchor.BN(333),
          ).rpc();
          console.log("Substraction signature: ", tx);
      } catch (_err) {
          assert.isTrue(_err instanceof AnchorError)
          const err: AnchorError = _err;
          const errMsg = "a is small";

          assert.strictEqual(err.error.errorMessage, errMsg);
          console.log("Error Number: ", err.error.errorCode.number);
      }
  });

  it("Multiplication test", async() => {
      const tx = await program.methods.multiplication(
          new anchor.BN(222),
          new anchor.BN(333),
      ).rpc();
      console.log("Multiplication signature: ", tx);
  });

  it("Division test", async() => {
      const tx = await program.methods.division(
          new anchor.BN(222),
          new anchor.BN(111),
      ).rpc();
      console.log("Division signature: ", tx);
  });
});
