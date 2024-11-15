/*import * as fs from "fs";
import * as solana from "@solana/web3.js";
import * as os from "os";
import * as umi from "../src";

const getLocalWallet = () => {
  let homeDir = os.homedir();
  const localWalletFile = fs.readFileSync(homeDir + "/.config/solana/id.json");
  let jsonParsed = Uint8Array.from(JSON.parse(localWalletFile.toString()));
  return solana.Keypair.fromSecretKey(jsonParsed);
};

const programId = 'C7DVvsaSQ1k7XcUXoAh9gZyGs6Ki9Qg9zpriBbrcx6tm';

const localWallet = solana.Keypair.generate();
const wallet2 = solana.Keypair.generate();
const connection = new solana.Connection("http://127.0.0.1:8899");

console.log("Airdropping... for pubkey", localWallet.publicKey.toBase58());

const [txId1, txId2] = await Promise.all([
  connection.requestAirdrop(
    localWallet.publicKey,
    10 * solana.LAMPORTS_PER_SOL
  ),
  connection.requestAirdrop(wallet2.publicKey, 10 * solana.LAMPORTS_PER_SOL),
]);

await Promise.all([
  connection.confirmTransaction(txId1, "confirmed"),
  connection.confirmTransaction(txId2, "confirmed"),
]);

describe("Integration tests", () => {
  it("Create a new bucket", async () => {
    const bucketName = "test bucket";
    const programId = "";
    const context : Parameters<typeof umi.push>['0'] = {
      payer: payer,
      programs: programs
    };
    const [bucket_pda,bump_pda_bump] = solana.PublicKey.findProgramAddressSync([Buffer.from("bucket"), Buffer.from(bucketName)], programId);
    const input : umi.PushInstructionArgs & umi.PushInstructionAccounts = {
      name: bucketName,
      ys: [1.0, 2.0, 3.0],
      bucket : bucket_pda,
    };
    umi.push(context, input);
  });
});*/