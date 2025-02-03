import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Blog } from "../target/types/blog";
import { assert } from "chai";

describe("blog", () => {
  // Configuración del provider
  const provider = anchor.AnchorProvider.local();
  anchor.setProvider(provider);

  // Referencia al programa de Anchor
  const program = anchor.workspace.Blog as Program<Blog>;

  // Cuentas para el test
  const mensajeAccount = anchor.web3.Keypair.generate();

  it("Debe crear un mensaje", async () => {
    // Ejecutamos la instrucción
    await program.methods
      .crearMensaje()
      .accounts({
        mensajeAccount: mensajeAccount.publicKey,
        user: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      } as any)
      .signers([mensajeAccount])
      .rpc();

    // Recuperamos los datos almacenados en la cuenta
    const mensaje = await program.account.mensaje.fetch(mensajeAccount.publicKey);

    console.log("Mensaje almacenado:", mensaje.valor);

    // Verificamos que el mensaje se creó correctamente
    assert.equal(mensaje.valor, "Hola Mundo!", "El mensaje no se guardó correctamente");
    assert.ok(mensaje.owner.equals(provider.wallet.publicKey), "El owner no es correcto");
  });

  it("Debe modificar un mensaje", async () => {
    const nuevoMensaje = "Nuevo mensaje de prueba";

    await program.methods
      .modMensaje(nuevoMensaje)
      .accounts({
        mensajeAccount: mensajeAccount.publicKey,
        user: provider.wallet.publicKey,
      })
      .rpc();

    const mensajeModificado = await program.account.mensaje.fetch(mensajeAccount.publicKey);

    console.log("Mensaje modificado:", mensajeModificado.valor);

    assert.equal(mensajeModificado.valor, nuevoMensaje, "El mensaje no se modificó correctamente");
  });

  it("Debe fallar si otro usuario intenta modificar el mensaje", async () => {
    const otroUsuario = anchor.web3.Keypair.generate();

    try {
      await program.methods
        .modMensaje("Modificación no permitida")
        .accounts({
          mensajeAccount: mensajeAccount.publicKey,
          user: otroUsuario.publicKey,
        })
        .signers([otroUsuario])
        .rpc();
      assert.fail("Debería haber fallado");
    } catch (err) {
      assert.include(err.message, "Only the owner can modify the message", "El error no es el esperado");
    }
  });
});
