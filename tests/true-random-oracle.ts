import * as anchor from "@project-serum/anchor";
import { Keypair, PublicKey, SystemProgram, LAMPORTS_PER_SOL } from '@solana/web3.js';
import { Program } from "@project-serum/anchor";
import { TrueRandomOracle } from "../target/types/true_random_oracle";
import RandomOrg from 'random-org';

describe("true-random-oracle", () => {
  // Configure the client to use the local cluster.
  let provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TrueRandomOracle as Program<TrueRandomOracle>;
  const requester = Keypair.generate();
  const wallet = provider.wallet as anchor.Wallet;
  let resultPDA, resultAccount;

  let random_org = new RandomOrg({ apiKey: '647c9ff8-33f7-4816-9469-2bde85050435' });
  let random_min: number, random_max: number, random_num: number;

  it("Request result", async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(requester.publicKey, LAMPORTS_PER_SOL),
      "finalized"
    );

    program.addEventListener("RequestEvent", (event, _slot) => {
      console.log(event);
      resultAccount = event.resultAccount;
      random_num = event.resultCount;
      random_min = event.resultMin;
      random_max = event.resultMax;
    });

    program.addEventListener("ResultEvent", (event, _slot) => {
      console.log(event);
    });

    const [pda, _] = await PublicKey
      .findProgramAddress(
        [
          anchor.utils.bytes.utf8.encode("random"),
          requester.publicKey.toBuffer(),
        ],
        program.programId
      );
    resultPDA = pda;

    const tx = await program.methods
      .requestResult(1, 0, 10)
      .accounts({
        result: resultPDA,
        payer: requester.publicKey,
        systemProgram: SystemProgram.programId,
      })
      .signers([requester])
      .rpc();
  });

  it("Post result", async () => {
    await provider.connection.confirmTransaction(
      await provider.connection.requestAirdrop(wallet.publicKey, LAMPORTS_PER_SOL),
      "finalized"
    );

    let result = await random_org.generateIntegers({ min: random_min, max: random_max, n: random_num });
    console.log(result);

    const tx = await program.methods
      .postResult(Buffer.from(result.random.data))
      .accounts({
        result: resultAccount,
        poster: wallet.publicKey,
      })
      .signers([wallet.payer])
      .rpc();
  });

  it("Delete result", async() => {
    const tx = await program.methods
      .deleteResult()
      .accounts({
        result: resultPDA,
        poster: wallet.publicKey,
      })
      .signers([wallet.payer])
      .rpc();
  });
});
