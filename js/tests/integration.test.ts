import * as fs from "fs";
import * as solana from "@solana/web3.js";
import * as os from "os";
import * as umi from "../src";


const getLocalWallet = () => {
  let homeDir = os.homedir();

  const localWalletFile = fs.readFileSync(homeDir + "/.config/solana/id.json");

  let jsonParsed = Uint8Array.from(JSON.parse(localWalletFile.toString()));

  return solana.Keypair.fromSecretKey(jsonParsed);
};

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
    try {
      const bucketName = "testBucket";
      const dataToPush = [1.0, 2.0, 3.0, 4.0, 5.0];
      const context = createUmi('https://api.mainnet-beta.solana.com')
      // Create a new bucket using the umi client
      const bucket = await umi.push(context, {
        name: bucketName,
        ys: dataToPush
      });

      // Push data to the bucket
      await umi.pushToBucket(bucketName, dataToPush);

      // Verify that the data was pushed successfully
      const bucketData = await umi.getBucketData(bucketName);
      expect(bucketData).toEqual(dataToPush);
    } catch (err) {
      console.error(err);
    }
  });
  it("Create a PDA account", async () => {
    try {
      
    } catch (err) {
      console.error(err);
    }
  });
});