import {PublicKey} from "@solana/web3.js"

const allOnes = new Uint8Array(32).fill(1); // Array de 32 bytes con valor 1

const programId = new PublicKey(allOnes)

const [PDA, bump] = PublicKey.findProgramAddressSync([], programId)

console.log(`PDA: ${PDA}`)
console.log(`bump: ${bump}`)