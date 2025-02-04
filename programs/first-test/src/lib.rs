use anchor_lang::prelude::*;

declare_id!("DZ3rUvPXHzD7TwVwECwC9186997jNFnVEPhb5jL8E9Zg");
// 3. crear programa
#[program]
pub mod contador_pda {
    pub use super::*;

    pub fn crear_contador(ctx: Context<CrearContador>) -> Result<()>{
        //Iniciar el valor de la cuenta en 0
        ctx.accounts.contador.valor = 0;
        //almacenamos las semillas necesarias para la PDA(pub key del authority y bump)
        ctx.accounts.contador.autoridad = ctx.accounts.authority.key(); //pubkey del authority
        //almacenar el bump
        ctx.accounts.contador.bump = ctx.bumps.contador; 
        Ok(())
    }
} 

// 2. contexto de la instrucción crear contador
#[derive(Accounts)]
pub struct CrearContador<'info> {
    //3. cuentas
    #[account(
        init,
        payer = authority,
        space = 8 + Contador::INIT_SPACE,
        //hasta aqui una cuenta 'normal'
        //PDA -> requiere semillas:
        seeds = [ //arreglo de semillas para la PDA
            Contador::SEMILLA_CONTADOR.as_bytes(), //semilla opcional string
            authority.key().as_ref() //semilla opcional public key
        ],
        bump, //este sera el bump canónico de la PDA
    )]
    contador: Account<'info, Contador>,

    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>
}

//1.estructura de la cuenta del contador
#[account]
#[derive(InitSpace)]
pub struct Contador{
    pub valor: u64,
    pub autoridad: Pubkey,
    pub bump: u8,
}
// bloque de implementación para la estructura contador:
impl Contador {
    pub const SEMILLA_CONTADOR: &'static str = "contador";
}