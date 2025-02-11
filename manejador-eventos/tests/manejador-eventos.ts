import * as spl from "@solana/spl-token";
//solpg.io version
describe("Test", () => {
  // declarar las cuentas necesarias
  let autoridad = pg.wallet.keypair;

  let tokenAceptado: web3.PublicKey;

  let evento: web3.PublicKey;
  let tokenEvento: web3.PublicKey;
  let bovedaEvento: web3.PublicKey;
  let bovedaGanancias: web3.PublicKey;

  let id: string = Date.now().toString();

  //creamos todo lo necesario previamente antes de nuestras instrucciones
  before(async () => {
    // buscamos la PDA del evento
    [evento] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from(id), Buffer.from("evento"), autoridad.publicKey.toBuffer()],
      pg.PROGRAM_ID
    );
    console.log("cuenta del evento: ", evento.toBase58());

    // PDA del token del evento
    [tokenEvento] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("token_evento"), evento.toBuffer()],
      pg.PROGRAM_ID
    );
    console.log("cuenta del token del evento: ", tokenEvento.toBase58());

    // PDA boveda del evento
    [bovedaEvento] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("boveda_evento"), evento.toBuffer()],
      pg.PROGRAM_ID
    );
    console.log("cuenta de la boveda del evento: ", bovedaEvento.toBase58());

    // PDA boveda de ganacias
    [bovedaGanancias] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("boveda_ganancias"), evento.toBuffer()],
      pg.PROGRAM_ID
    );
    console.log(
      "cuenta de la boveda de ganancias: ",
      bovedaGanancias.toBase58()
    );

    // creamos el mint del token aceptado (para comprar entradas y tokens)
    tokenAceptado = await spl.createMint(
      pg.connection, // conexion a solana
      autoridad, // el que paga los fees
      autoridad.publicKey, // el mint authority
      autoridad.publicKey, // el freeza authority
      2 // decimales del token
    );
  });
  it("Crear un evento", async () => {
    // Datos básicos del evento
    const nombre: string = "Mi primer evento";
    const descripcion = "El mejor evento del mundo!";
    const precioEntrada = 2.1;
    const precioToken = 5.0;

    // llamamos a la instruccion del programa
    const tx = await pg.program.methods
      .crearEvento(id, nombre, descripcion, precioEntrada, precioToken)
      .accounts({
        evento: evento,
        tokenAceptado: tokenAceptado,
        tokenEvento: tokenEvento,
        bovedaEvento: bovedaEvento,
        bovedaGanancias: bovedaGanancias,
        autoridad: autoridad.publicKey,
      })
      .rpc();

    //Confirmamos la transaccion
    await pg.connection.confirmTransaction(tx);

    //Podemos ver la informacion almacenada en la cuenta del evento
    const infoEvento = await pg.program.account.evento.fetch(evento);

    console.log("Información del evento: ", infoEvento);

    // con al informacion del evento podemos hacer comprobaciones
    // comprobamos que el precio del token sea correcto (y esta expresado en la unidad minima del token)
    assert.equal(infoEvento.precioToken.toNumber(), precioToken * 10 ** 2);
  });
  it("Elimina el evento creado anteriormente", async () => {
    // comprobaciones previas
    console.log("eliminando evento...");
    const bovedaEventoInfo = await spl.getAccount(pg.connection, bovedaEvento);
    console.log("Saldo de la bóveda del evento:", bovedaEventoInfo.amount);

    const bovedaGananciasInfo = await spl.getAccount(
      pg.connection,
      bovedaGanancias
    );
    console.log("Saldo de la bóveda de ganancias:", bovedaGananciasInfo.amount);

    // llamamo a la instrucción eliminar
    const tx = await pg.program.methods
      .eliminarEvento()
      // enviamos las cuentas asociadas a la instrucción
      .accounts({
        evento: evento,
        bovedaEvento: bovedaEvento,
        bovedaGanancias: bovedaGanancias,
        tokenEvento: tokenEvento,
        autoridad: autoridad.publicKey,
      })
      // firma la autoridad creadora del evento
      .signers([autoridad])
      // enviamos a la red
      .rpc();

    await pg.connection.confirmTransaction(tx);

    //Podemos ver la informacion almacenada en la cuenta del evento
    // en este caso debe ser null porque no debe existir
    const infoEvento = await pg.program.account.evento.fetchNullable(evento); // Esta funcion .fetchNullable(), nos permite esperar un null, asi no nos dara un error, al utilizar -.fetch()

    console.log("Información del evento: ", infoEvento);
  });
});
