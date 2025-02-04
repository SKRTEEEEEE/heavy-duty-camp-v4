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