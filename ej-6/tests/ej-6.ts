//PARA SOLPG.IO

import { PublicKey } from "@solana/web3.js";
import * as spl from "@solana/spl-token";
import * as web3 from "@solana/web3.js";


describe("Test", () => {
  //definimos las cuentas que vamos a necesitar

    //tokens
    let tokenA: PublicKey;
    let tokenB: PublicKey;

    //cuentas
    let escrow: PublicKey;
    let garantia: PublicKey;

    /*
    El usuario -inicializador- que es nuestra wallet
    */
   let inicializador = pg.wallet.keypair;
   let inicializadorTokenA: PublicKey;

   let id = Date.now().toString();

   //creamos todas las cuentas que deben existir previamente
   before(async()=>{
    //encontramos una dirección PDA para la cuenta del escrow
    [escrow] = web3.PublicKey.findProgramAddressSync(
      [inicializador.publicKey.toBuffer(), Buffer.from(id)],
      pg.PROGRAM_ID
    );
    console.log(`cuenta del escrow: `,escrow.toBase58());

    [garantia] = web3.PublicKey.findProgramAddressSync(
      [escrow.toBuffer()],
      pg.PROGRAM_ID
    );
    console.log(`cuenta de la garantia: `, garantia.toBase58())
    // creamos el token A
    tokenA = await spl.createMint(
      pg.connection, // conexion a solana
      inicializador, // el que paga los fees
      inicializador.publicKey, // el mint authority
      inicializador.publicKey, // el freeza authority
      2 // decimales del token
    );
    console.log("token A: ", tokenA.toBase58());

    // creamos el token B
    tokenB = await spl.createMint(
      pg.connection, // conexion a solana
      inicializador, // el que paga los fees
      inicializador.publicKey, // el mint authority
      inicializador.publicKey, // el freeza authority
      2 // decimales del token
    );
    console.log("token B: ", tokenB.toBase58());

     // creamos la cuenta token asociada al inicializador y el token A
    inicializadorTokenA = await spl.createAssociatedTokenAccount(
      pg.connection, // conexion a la red
      inicializador, // paga los fees
      tokenA, // tokens almacenados en la cuenta
      inicializador.publicKey // owner de los tokens
    );
    console.log("cuenta inicializadorTokenA: ", inicializadorTokenA.toBase58());
    /*
    nuestra primera isntrucción transfiere tokens a a la cuenta de garantia
    el inicializador debe poseer tokens A en su cuenta token asi que
    hacemos mint de tokens A a la cuenta token asociada al incializador y el token A
    */
    await spl.mintTo(
      pg.connection, // conexion a solana
      inicializador, // el que paga los fees
      tokenA, // token a mintear
      inicializadorTokenA, // donde depositarlos
      inicializador.publicKey, // mint authority
      100000 // cantidad a mintear (expresada en decimales)
    );
  });
    
  it("Se inicializa un Escrow", async () => {
    const cantidadTokenA = new BN(100); // 100 tokens A
    const cantidadTokenB = new BN(95); // 95 tokens B

    let txHash = await pg.program.methods
      .incializar(id, cantidadTokenA, cantidadTokenB)
      .accounts({
        escrow: escrow,
        inicializador: inicializador.publicKey,
        inicializadorCuentaTokenA: inicializadorTokenA,
        cuentaDeGarantia: garantia,
        tokenA: tokenA,
        tokenB: tokenB,
      })
      .signers([inicializador])
      .rpc();

      // Confirmamos la transaccion transaction
      await pg.connection.confirmTransaction(txHash);
  

     // verificamos que se haya depositado la cantidad en la cuenta de garantía
    let deposito = (await spl.getAccount(pg.connection, garantia)).amount;

    // assert
    assert.equal(cantidadTokenA.toNumber() * 10 ** 2, Number(deposito));
  })
  it("Se cierra un escrow antes de que un usuario lo finalize", async()=>{
    let txHash = await pg.program.methods
      .cerrarEscrow()
      .accounts({
        escrow: escrow,
        inicializador: inicializador.publicKey,
        inicializadorCuentaTokenA: inicializadorTokenA,
        cuentaDeGarantia: garantia,
      })
      .signers([inicializador])
      .rpc();


    await pg.connection.confirmTransaction(txHash);

    // Verificamos el balance de tokens A del inicializador después de cerrar el escrow
    let balanceFinal = (await spl.getAccount(pg.connection, inicializadorTokenA)).amount;

    // assert: el balance final debe ser igual al inicial después de que el escrow se cierre
    assert.equal(balanceFinal, 100000, "El balance del inicializador no es el mismo al final del test");
  })
});
