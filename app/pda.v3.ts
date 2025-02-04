/*
En este caso podemos observar como muchos bump generan una PDA 'válida'. Poreso es importante el concepto de canónico.
*/

import {PublicKey} from "@solana/web3.js"


const programId = new PublicKey("11111111111111111111111111111111")
const string = "Hello Builder!"

for(let bump = 255; bump>=0;bump--){
    try {
        const PDA = PublicKey.createProgramAddressSync([
            Buffer.from(string),
            Buffer.from([bump])
        ], programId)
        console.log(`bump: ${bump} -> ${PDA}`)
    
    } catch (error) {
        console.log(`bump: ${bump} with err: ${error}`)
    }
}

