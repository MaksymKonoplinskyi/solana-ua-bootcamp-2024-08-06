import "dotenv/config";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  PublicKey,
  SystemProgram,
  Transaction,
  clusterApiUrl,
  Connection,
  sendAndConfirmTransaction,
  TransactionInstruction
} from "@solana/web3.js";

let privateKey = process.env["SECRET_KEY"];
if (privateKey === undefined) {
  console.log("Add SECRET_KEY to .env!");
  process.exit(1);
}
const asArray = Uint8Array.from(JSON.parse(privateKey));
const sender = Keypair.fromSecretKey(asArray);

const connection = new Connection(clusterApiUrl("devnet"));

console.log(`🔑 Our public key is: ${sender.publicKey.toBase58()}`);
const recipient = new PublicKey("MAXHwQi54g5uNwLxwHjmv3EVDnRKoV1oTLphZYri2Q9");
console.log(`💸 Attempting to send 0.01 SOL to ${recipient.toBase58()}...`);

const transaction = new Transaction();

const sendSolInstruction = SystemProgram.transfer({
  fromPubkey: sender.publicKey,
  toPubkey: recipient,
  lamports: 0.01 * LAMPORTS_PER_SOL,
});

// Створюємо memo інструкцію вручну
const memoText = "My memo text";
const memoInstruction = new TransactionInstruction({
  keys: [],
  programId: new PublicKey("MemoSq4gqABAXKb96qnH8TysNcWxMyWCqXgDLGmfcHr"), // ID програми Memo
  data: Buffer.from(memoText), // Ваш новий текст memo
});

transaction.add(sendSolInstruction);
transaction.add(memoInstruction); // Додаємо memo інструкцію до транзакції

const signature = await sendAndConfirmTransaction(connection, transaction, [
  sender,
]);

console.log(`✅ Transaction confirmed, signature: ${signature}!`);

// Отримуємо підтверджену транзакцію для отримання логів
const confirmedTransaction = await connection.getConfirmedTransaction(signature);

if (confirmedTransaction) {
  const logs = confirmedTransaction.meta?.logMessages;
  if (logs) {
    console.log("📜 Transaction Logs:");
    logs.forEach(log => console.log(log));
  } else {
    console.log("No logs found for this transaction.");
  }
} else {
  console.log("Transaction confirmation not found.");
}
