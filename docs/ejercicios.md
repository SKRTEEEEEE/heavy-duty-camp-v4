# ejercicios
## bootcamp
### 1. Calculando la renta
<details><summary>
Si una cuenta en Solana almacena un PublicKey, un saldo de tipo u64, un timestamp de tipo u64 y un estado de tipo bool ¿cuál es el tamaño total en bytes de la cuenta?
</summary>

- PublicKey = 32 bytes
- u64 (saldo) = 8 bytes
- u64 (timestamp) = 8 bytes
- bool (estado) = 1 byte
- Total = 32 + 8 + 8 + 1 = 49 bytes

</details>
<details><summary>
Si una cuenta en Solana almacena un PublicKey, un entero de tipo u32 y un flotante de tipo f64, ¿cuál es el mínimo de $SOL que debe tener para estar exenta de pagar renta?
</summary>

- Primero calculamos el tamaño:
    * PublicKey = 32 bytes
    * u32 = 4 bytes
    * f64 = 8 bytes
    * overhead = 128 bytes
    * Total = 32 + 4 + 8 = 44 bytes + 128 bytes = 172 bytes
- La fórmula para calcular SOL exento de renta es:
    * (tamaño_cuenta * 0.00000348 SOL * 2 años)
    - Por lo tanto: 172 * 0.00000348 * 2 = 0.00119712 SOL

</details>
<details><summary>
Si quisieras crear una cuenta en Solana que almacene 0 bytes, ¿cuál es el mínimo de $SOL que debe tener para estar exenta de pagar renta?
</summary>

- Todas las cuentas en Solana tienen un overhead mínimo de 128 bytes
- Por lo tanto, incluso con 0 bytes de datos, necesitamos calcular:
* 128 * 0.00000348 * 2 = 0.00089088 SOL

</details>


### 2. Definiendo cuentas e intrucciones
<details><summary>
Crea un nuevo proyecto Anchor en Solana Playground y define la estructura de datos de una cuenta que contiene un mensaje de máximo 150 caracteres. </br>
En el mismo proyecto, define el contexto y la función para una instrucción que permita crear una nueva cuenta con la estructura definida anteriormente. 
</summary>

- Código:

```rust
use anchor_lang::prelude::*;

declare_id!("9e1Sp9gCqfHY8CEd6Jm4JvLT899Md5Du5sYMUyQnswSt");

#[program]
pub mod blog {
    use super::*;

    pub fn crear_mensaje(ctx: Context<CrearMensaje>) -> Result<()> {
        ctx.accounts.mensaje.valor = "Hola Mundo!".to_string();
        Ok(())
    }
}

//2. definimos el contexto de la instrucción
#[derive(Accounts)]
pub struct CrearMensaje<'info> {
    //3. cuentas
    // cuenta 'recipinte' mensaje
    #[account(init, payer = user, space = 8 + Mensaje::INIT_SPACE)]
    pub mensaje: Account<'info, Mensaje>, 

    //payer
    #[account(mut)]
    pub user: Signer<'info>,

    //system_program
    pub system_program: Program<'info, System>,
}

//1. estructura de datos del mensaje (cuenta)
#[account]
#[derive(InitSpace)]
pub struct Mensaje {
    #[max_len(150)]
    pub valor: String, 
}
```

- Para interactuar con el contrato ver la sección de: [interactuar con programas en solpg.io](./todo.md#interactuar-con-programas-en-solpgio)

</details>
<details><summary>
En el mismo proyecto, define el contexto y la función para la instrucción que permita modificar los datos de una cuenta existente. 
</summary>

- Código requerido:
```rust
use anchor_lang::prelude::*;

declare_id!("9e1Sp9gCqfHY8CEd6Jm4JvLT899Md5Du5sYMUyQnswSt");

//4. definimos el programa
#[program]
pub mod blog {
    use super::*;
    //5. creamos la función de la instrucción para crear
    pub fn crear_mensaje(ctx: Context<CrearMensaje>) -> Result<()> {
        ctx.accounts.mensaje_account.valor = "Hola Mundo!".to_string();
        Ok(())
    }
    //7. creamos la función de la instrucción para modificar
    pub fn mod_mensaje(ctx: Context<ModificarMensaje>, mensaje: String) -> Result<()> {
        //+9. requerimientos previos
        require!(!mensaje.is_empty(), CustomError::EmptyMessage);
        require!(mensaje.len() <= 150, CustomError::MessageTooLong);

        ctx.accounts.mensaje_account.valor = mensaje;
        Ok(())
    }
}
//6. definimos el contexto de la instrucción para modificar
#[derive(Accounts)]
pub struct ModificarMensaje<'info> {
    #[account(mut)]
    pub mensaje_account: Account<'info, Mensaje>,

    #[account(mut)]
    pub user: Signer<'info>,
}

//2. definimos el contexto de la instrucción para crear
#[derive(Accounts)]
pub struct CrearMensaje<'info> {
    //3. cuentas
    // cuenta 'recipinte' mensaje
    #[account(init, payer = user, space = 8 + Mensaje::INIT_SPACE)]
    pub mensaje_account: Account<'info, Mensaje>,

    //payer
    #[account(mut)]
    pub user: Signer<'info>,

    //system_program
    pub system_program: Program<'info, System>,
}

//1. estructura de datos del mensaje (cuenta)
#[account]
#[derive(InitSpace)]
pub struct Mensaje {
    #[max_len(150)]
    pub valor: String,
}
//+8. mensajes de error personalizados
#[error_code]
pub enum CustomError {
    #[msg("The message exceeds 150 characters.")]
    MessageTooLong,

    #[msg("The message cannot be empty.")]
    EmptyMessage,
}
```

- Limitado a owner del mensaje:
```rust
use anchor_lang::prelude::*;

declare_id!("9e1Sp9gCqfHY8CEd6Jm4JvLT899Md5Du5sYMUyQnswSt");

//4. definimos el programa
#[program]
pub mod blog {
    use super::*;
    //5. creamos la función de la instrucción para crear
    pub fn crear_mensaje(ctx: Context<CrearMensaje>) -> Result<()> {
        ctx.accounts.mensaje_account.owner = *ctx.accounts.user.key;
        ctx.accounts.mensaje_account.valor = "Hola Mundo!".to_string();
        Ok(())
    }
    //7. creamos la función de la instrucción para modificar
    pub fn mod_mensaje(ctx: Context<ModificarMensaje>, mensaje: String) -> Result<()> {
        //+9. requerimientos previos
        require!(!mensaje.is_empty(), CustomError::EmptyMessage);
        require!(mensaje.len() <= 150, CustomError::MessageTooLong);
        require_keys_eq!(ctx.accounts.mensaje_account.owner, *ctx.accounts.user.key, CustomError::OnlyOwnerMessage);

        ctx.accounts.mensaje_account.valor = mensaje;
        Ok(())
    }
}
//6. definimos el contexto de la instrucción para modificar
#[derive(Accounts)]
pub struct ModificarMensaje<'info> {
    #[account(mut)]
    pub mensaje_account: Account<'info, Mensaje>,

    #[account(mut)]
    pub user: Signer<'info>,
}

//2. definimos el contexto de la instrucción para crear
#[derive(Accounts)]
pub struct CrearMensaje<'info> {
    //3. cuentas
    // cuenta 'recipinte' mensaje
    #[account(init, payer = user, space = 8 + Mensaje::INIT_SPACE)]
    pub mensaje_account: Account<'info, Mensaje>,

    //payer
    #[account(mut)]
    pub user: Signer<'info>,

    //system_program
    pub system_program: Program<'info, System>,
}

//1. estructura de datos del mensaje (cuenta)
#[account]
#[derive(InitSpace)]
pub struct Mensaje {
    pub owner: Pubkey,
    #[max_len(150)]
    pub valor: String,
}
//+8. mensajes de error personalizados
#[error_code]
pub enum CustomError {
    #[msg("The message exceeds 150 characters.")]
    MessageTooLong,

    #[msg("The message cannot be empty.")]
    EmptyMessage,

    #[msg("Only the owner can modify the message")]
    OnlyOwnerMessage,
}
```
</details>

### 3. Interactuando con el Token Program

<details><summary>
Utilizando el CLI en Solana Playground, crea un nuevo token y envia la Public Key del token.

Utilizando el CLI en Solana Playground, crea una nueva cuenta de token asociada para el token creado en la pregunta anterior y la siguiente billetera: FtetRTNM4HJWnV2dWz67cvunad4zEW2KcHozmuNz26BN y envia la Public Key de la cuenta creada.

Crea y asigna 10 nuevas unidades del token, a la cuenta de token asociada creada anteriormente y envia la firma de la transacción.
</summary>

- `spl-token create-token` -> *HBkdfpM4FLabw5Xp5qCE9cYqbS5yyV4uHqoDmPpRdEhr*

- `spl-token create-account HBkdfpM4FLabw5Xp5qCE9cYqbS5yyV4uHqoDmPpRdEhr` -> *5NumnxJdX78t7SLsS9fM6jwX2auaP96AoXXzcMSBbBbW*

- `spl-token mint HBkdfpM4FLabw5Xp5qCE9cYqbS5yyV4uHqoDmPpRdEhr 10 -- 5NumnxJdX78t7SLsS9fM6jwX2auaP96AoXXzcMSBbBbW` -> *2QVPd7AQZvVJbFe6tq9kbdQi2Lgtav7zVXYooLdkF7a1kFxeZB2nAG1pmHra8TnhMNSaMcd7uQ96mkJmXE48EkQv*


</details>
<details><summary>
Crea un nuevo proyecto de Anchor en Solana Playground y escribe el contexto de una instrucción que crea un nuevo token donde el mint authority es una cuenta diferente definida también dentro del contexto de la instrucción y comparte el link al proyecto.
</summary>

- Código básico:
```rust
use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("EgVw1Bjs5z8R6XwRrpbYFt8wGXZYAztGxbhKCPXcynj6");

//2. Crear programa y instrucción
#[program]
pub mod token_exercise {
    use super::*;

    pub fn create_token_mint(_ctx: Context<CreateToken>) -> Result<()>{
        Ok(())
    }
}

//1. Crear contexto
#[derive(Accounts)]
pub struct CreateToken<'info>{

    //cuenta 'mint account'
    #[account(init, payer = fee_payer, mint::decimals = 9, mint::authority = mint_authority)]
    pub mint_account: Account<'info, Mint>,

    //cuentas mutables
    #[account(mut)]
    pub mint_authority: Signer<'info>,
    #[account(mut)]
    pub fee_payer: Signer<'info>,

    //cuentas programa
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    //cuentas asociadas
    pub rent: Sysvar<'info, Rent>,
}
```
- Código mejorado IA(Chat-GPT):
```rs
use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("EgVw1Bjs5z8R6XwRrpbYFt8wGXZYAztGxbhKCPXcynj6");

//2. Crear programa y instrucción
#[program]
pub mod token_exercise {
    use super::*;

    pub fn create_token_mint(_ctx: Context<CreateToken>) -> Result<()>{
        Ok(())
    }
}

//1. Crear contexto
#[derive(Accounts)]
pub struct CreateToken<'info>{
    //cuentas mutables 💡 -> segun GPT, es mejor declarar primero la autoridad que se utiliza en mint_account
    #[account(mut)]
    pub mint_authority: Signer<'info>,
   

    //cuenta 'mint account'
    #[account(
        init, 
        payer = fee_payer, 
        mint::decimals = 9, 
        mint::authority = mint_authority,
        mint::token_program = token_program //segun GPT, es buena practica indicar siempre el token_program
        )]
    pub mint_account: Account<'info, Mint>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,

    //cuentas programa
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,

    //cuentas asociadas
    pub rent: Sysvar<'info, Rent>,
}
```

    ❓⁉️ Chat-GPT said ⁉️ 📢⁉️

    - `mint_authority` debe ir antes porque es un valor lógico dentro de `mint_account`.  
    - `fee_payer` y `token_program` pueden ir después porque Anchor ya sabe resolverlos.  

</details>



### 4. Trabajando con PDAs


<details><summary>
Crea un nuevo proyecto en Solana Playground y dentro del archivo client.ts, utiliza la función findProgramAddressSync para encontrar la PDA y el bump canónico para los siguientes valores:

programId: "11111111111111111111111111111111"

Seeds: [“heavy”, “duty”, “camp”, 4]
</summary>

- Encontrar PDA and bump canónico
```ts
const [PDA, bump] = PublicKey.findProgramAddressSync(
  [
    Buffer.from("heavy"),
    Buffer.from("duty"),
    Buffer.from("camp"),
    Buffer.from([4])],
  new PublicKey("11111111111111111111111111111111")
)
console.log(`PDA: ${PDA}, bump: ${bump}`)
```

</details>
<details><summary>
En el mismo proyecto, define el contexto y la instrucción de un programa para crear una cuenta que almacena un número entero y una public key, dentro de una PDA y utiliza como semilla para generar la dirección de la PDA únicamente la public key almacenada en la cuenta.

En el mismo proyecto, define el contexto y la instrucción de un programa para modificar los datos de la cuenta creada anteriormente.
</summary>

- Código:
```rs
use anchor_lang::prelude::*;


declare_id!("5NLnnWgZbiBz8GRFQbHMPJNWPD9G6ddks66iFQHSecsE");

//3. Programa con las instrucciones para crear y modificar
#[program]
pub mod contador_pda_mejorado{
    pub use super::*;

    pub fn crear_contador(ctx: Context<CrearContador>)->Result<()>{
        ctx.accounts.cuenta_pda.valor = 0;
        ctx.accounts.cuenta_pda.llave = ctx.accounts.fee_payer.key();
        ctx.accounts.cuenta_pda.bump = ctx.bumps.cuenta_pda; //ctx.bumps.get("cuenta_pda").unwrap()
        Ok(())
    }

    pub fn modificar_contador(ctx:Context<ModContador>, nuevo_valor: u64) ->Result<()>{
        ctx.accounts.cuenta_pda.valor = nuevo_valor;
        Ok(())
    }
}

//4. Contexto de la instrucción modificar contador
#[derive(Accounts)]
pub struct ModContador<'info> {
    #[account(
        mut,
        seeds=[fee_payer.key().as_ref()],
        bump = cuenta_pda.bump,
        constraint = cuenta_pda.llave == fee_payer.key(), // Esto es una condicion que le ponemos para que anchor nos cree la cuenta
    )]
    cuenta_pda: Account<'info, Contador>,

    #[account(mut)]
    fee_payer: Signer<'info>,
}

//2. Contexto de la instrucción crear contador
#[derive(Accounts)]
pub struct CrearContador<'info> {
    #[account(
        init,
        payer=fee_payer,
        space=8+Contador::INIT_SPACE,
        seeds=[
            fee_payer.key().as_ref()
        ],
        bump,
    )]
    cuenta_pda: Account<'info, Contador>,

    #[account(mut)]
    fee_payer: Signer<'info>, //? deben ser publicos los campos?

    pub system_program: Program<'info, System>
}

#[account]
#[derive(InitSpace)]
//1. Estructura de la cuenta PDA
pub struct Contador {
    pub valor: u64,
    pub llave: Pubkey,
    pub bump: u8,
}
```

- Test (mocha):
```ts
import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { PublicKey, SystemProgram, LAMPORTS_PER_SOL } from "@solana/web3.js";
import { ContadorPdaMejorado } from "../target/types/contador_pda_mejorado";

describe("contador-pda-mejorado", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.ContadorPdaMejorado as Program<ContadorPdaMejorado>;
  const wallet = provider.wallet.publicKey;

  it("Inicializa el contador", async () => {
    // Mostrar información del cliente
    console.log("Mi dirección:", wallet.toString());
    const balance = await provider.connection.getBalance(wallet);
    console.log(`Mi balance: ${balance / LAMPORTS_PER_SOL} SOL`);

    // Encontrar PDA
    const [pda, bump] = PublicKey.findProgramAddressSync(
      [wallet.toBuffer()],
      program.programId
    );
    console.log(`PDA: ${pda.toString()}, bump: ${bump}`);

    // Crear contador
    try {
      await program.methods
        .crearContador()
        .accounts({
          cuentaPda: pda,
          feePayer: wallet,
          systemProgram: SystemProgram.programId
        })
        .rpc();
      
      console.log("Contador creado exitosamente");

      // Verificar el valor inicial
      const cuenta = await program.account.contador.fetch(pda);
      console.log("Valor inicial:", cuenta.valor.toString());
      console.log("Llave almacenada:", cuenta.llave.toString());
      console.log("Bump almacenado:", cuenta.bump);

    } catch (error) {
      console.error("Error al crear el contador:", error);
    }
  });

  it("Modifica el contador", async () => {
    // Encontrar PDA
    const [pda, _] = PublicKey.findProgramAddressSync(
      [wallet.toBuffer()],
      program.programId
    );

    // Modificar contador
    try {
      const nuevoValor = new anchor.BN(42);
      await program.methods
        .modificarContador(nuevoValor)
        .accounts({
          cuentaPda: pda,
          feePayer: wallet
        })
        .rpc();

      console.log("Contador modificado exitosamente");

      // Verificar el nuevo valor
      const cuenta = await program.account.contador.fetch(pda);
      console.log("Nuevo valor:", cuenta.valor.toString());

    } catch (error) {
      console.error("Error al modificar el contador:", error);
    }
  });
});
```

</details>

### 5. CPIs
<details><summary>
Crea un nuevo proyecto en Solana Playground y escribe el código completo de una instrucción que realice una transferencia de tokens SPL entre dos cuentas, en donde la cuenta remitente es una PDA con dos semillas opcionales cualesquiera. Debes realizar un Cross-Program Invocation (CPI) al Token Program de Solana.
</summary>

- Código parte 1:
```rs
use anchor_lang::prelude::*;
declare_id!("639DEktgoBuJJaEbFEr7RcRHNFHM82aKdJTt4YqqTudm");

//4. Programa con las instrucciones
#[program]
pub mod pda_transfer{
    use super::*;
    pub fn almacenar_mint(ctx: Context<AlmacenarMint>, input_mint_account: Pubkey)->Result<()>{
        ctx.accounts.mint_account.mint_token = input_mint_account;
        ctx.accounts.mint_account.bump = ctx.bumps.mint_account;
        ctx.accounts.mint_account.fee_payer = ctx.accounts.fee_payer.key();
        Ok(())
    }
}
//5. Contexto de la instrucción para transferir tokens
// #[derive(Accounts)]


//2. Contexto de la instrucción para almacenar el mint token
#[derive(Accounts)]
pub struct AlmacenarMint<'info> {
    //3. cuentas para almacenar mint?
    #[account(mut)]
    pub fee_payer: Signer<'info>,

    #[account(
        init,
        seeds = [b"almacenar"],
        bump,
        payer = fee_payer,
        space = 8 + DataAccount::INIT_SPACE
    )]
    pub mint_account: Account<'info, DataAccount>,
    pub system_program: Program<'info, System>
}

//1. Estructura de datos del mint token (cuenta)
#[account]
#[derive(InitSpace)]
pub struct DataAccount {
    pub mint_token: Pubkey,
    pub fee_payer: Pubkey,
    pub bump: u8,
}
```
- Ejecucción código en 'Test' sección de [solpg.io](solpg.io): Desplegamos el contrato, ahora podemos utilizar la instrucción almacenarMint, la cual nos permitira, crear una cuenta PDA para dicho mint token.
    - Primero utilizamos el comando `spl-token create-token` para crear una cuenta mint.
    - Luego, introducimos dicha Pubkey, en el argumento 'inputMintAccount'
    - El feePayer sera el pagador. (wallet con lamports)
    - mintAccount, es la cuenta PDA -> generada a partir de: la semilla 'almacenar'
- Para acceder a DataAccount, debemos facilitar la Pubkey, de la mintAccount (lo generado en la linea anterior de estas instrucciones)




</details>

### x. 
<details><summary>

</summary>



</details>
<details><summary>

</summary>



</details>
<details><summary>

</summary>



</details>

### x. 
<details><summary>

</summary>



</details>
<details><summary>

</summary>



</details>
<details><summary>

</summary>



</details>