import * as spl from "@solana/spl-token";
type Keypair = typeof pg.wallet.keypair;
// No imports needed: web3, anchor, pg and more are globally available
const instructionPararLoteria=async(loteria: web3.PublicKey, autoridad: Keypair)=>{
  const tx = await pg.program.methods.pararLoteria()
    .accounts({
      loteria,
      autoridad: autoridad.publicKey
    }).signers([autoridad]).rpc()

    await pg.connection.confirmTransaction(tx)

    const infoLoteria = await pg.program.account.loteria.fetchNullable(loteria)
    console.log("Evento activo: " ,infoLoteria.activo)
}
describe("Test", () => {
  //cuentas necesarias
  let autoridad = pg.wallet.keypair;

  let tokenAceptado: web3.PublicKey;

  let loteria: web3.PublicKey;
  let tokenLoteria: web3.PublicKey;
  let bovedaLoteria: web3.PublicKey;

  let id: string = Date.now().toString();

  before(async () => {
    // buscamos la PDA del loteria
    [loteria] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(id), Buffer.from("loteria"), autoridad.publicKey.toBuffer()],
      pg.PROGRAM_ID
    );
    console.log("cuenta del loteria: ", loteria.toBase58());

    // PDA del token del loteria
    [tokenLoteria] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("token_loteria"), loteria.toBuffer()],
      pg.PROGRAM_ID
    );
    console.log("cuenta del token del loteria: ", tokenLoteria.toBase58());

    // PDA boveda del loteria
    [bovedaLoteria] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("boveda_loteria"), loteria.toBuffer()],
      pg.PROGRAM_ID
    );
    console.log("cuenta de la boveda del loteria: ", bovedaLoteria.toBase58());

    tokenAceptado = await spl.createMint(
      pg.connection, // conexion a solana
      autoridad, // el que paga los fees
      autoridad.publicKey, // el mint authority
      autoridad.publicKey, // el freeza authority
      2 // decimales del token
    );
  });

  it("Crear una loteria", async () => {
    // Datos básicos de la loteria
    const nombre: string = "Mi primera loto";
    const descripcion = "El mejor loteria del mundo!";
    const precioToken = 10;
    const tipoLoteria = 9;

    // llamamos a la instruccion del programa
    const tx = await pg.program.methods
      .crearLoteria(id, nombre, descripcion, precioToken, tipoLoteria)
      .accounts({
        loteria,
        tokenAceptado,
        tokenLoteria,
        bovedaLoteria,
        autoridad: autoridad.publicKey,
      })
      .rpc();

    //Confirmamos la transaccion
    await pg.connection.confirmTransaction(tx);

    const infoLoteria = await pg.program.account.loteria.fetch(loteria);

    console.log("info loteria: ", infoLoteria);

    if (infoLoteria.tipoLoteria.fast) {
        console.log("Es una lotería Fast con valor:", infoLoteria.tipoLoteria.fast);
    } else if (infoLoteria.tipoLoteria.normal) {
        console.log("Es una lotería Normal con valor:", infoLoteria.tipoLoteria.normal);
    } else if (infoLoteria.tipoLoteria.big) {
        console.log("Es una lotería Big con valor:", infoLoteria.tipoLoteria.big);
    }

    assert.equal(infoLoteria.precioToken.toNumber(), precioToken * 10 ** 2);
  });
  it("Para una loteria", async () => await instructionPararLoteria(loteria, autoridad));
  it("Volver a iniciar una loteria", async()=>await instructionPararLoteria(loteria, autoridad))
  it("Eliminar una loteria", async () => {
    const tx = await pg.program.methods.eliminarLoteria()
    .accounts({
      loteria,
      bovedaLoteria,
      tokenLoteria,
      autoridad: autoridad.publicKey
    }).signers([autoridad]).rpc()
    await pg.connection.confirmTransaction(tx)
    const infoLoteria = await pg.program.account.loteria.fetchNullable(loteria)
    console.log("informacion de la loteria:" ,infoLoteria)
  })
  

});
