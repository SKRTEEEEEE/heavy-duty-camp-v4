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
