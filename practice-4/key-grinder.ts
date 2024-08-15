import { Keypair } from '@solana/web3.js'
import { fromByteArray } from 'base64-js'

const startTime = Date.now()

let keypair
let publicKeyBase58 = ''

do {
  keypair = Keypair.generate()
  publicKeyBase58 = keypair.publicKey.toBase58()
} while (!publicKeyBase58.startsWith('MAX')) 
  
const secretKeyBase64 = fromByteArray(keypair.secretKey)

const endTime = Date.now()
const timeTaken = (endTime - startTime) / 1000 // В секундах

console.log(`The public key is: `, publicKeyBase58)
console.log(`The secret key is: `, secretKeyBase64)
console.log(`Time taken: ${timeTaken} seconds`)
console.log(`✅ Finished!`)
