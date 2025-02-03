use anchor_lang::prelude::*;

declare_id!("7X9LdHU2AwpLYo97i3YzgxNvB22qm45wj3tEr3qPSkAp");

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
