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