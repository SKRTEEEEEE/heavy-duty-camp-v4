use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount, transfer, Transfer as TokenTransfer};
use anchor_spl::token_interface::{Mint, TokenAccount, TokenInterface};

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
    pub fn transferir_token(ctx:Context<TransferirTokens>, quantity: u64) ->Result<()>{
        let de = ctx.accounts.pda_remitente.to_account_info();
        let para = ctx.accounts.recipiente.to_account_info();
        let programa = ctx.accounts.token_program.to_account_info();
        let pubkey_token = ctx.accounts.mint_account.key();

        let bump = ctx.bumps.pda_remitente;
        let semillas_firma: &[&[&[u8]]] = &[&["transfer".as_bytes(), pubkey_token.as_ref(), &[bump] ]];

        let cpi_context = CpiContext::new(programa,
        TokenTransfer{
            from:de,
            to:para,
            authority: ctx.accounts.pda_remitente.to_account_info()
        }
        ).with_signer(semillas_firma);

        transfer(cpi_context, quantity)?;

        Ok(())
    }
}
//6. Contexto de la instrucción para crear cuenta PDA remitente y asignarle fondos
#[derive(Accounts)]
pub struct CrearPdaYMintear<'info> {
    #[account(mut)]
    pub fee_payer: Signer<'info>,

    #[account(
        seeds = [b"almacenar"],
        bump
    )]
    mint_account: Account<'info, DataAccount>,

    #[account(
        init_if_needed,
        seeds = [b"transfer", mint_account.key().as_ref()],
        bump,
        token::mint = mint_token,
        token::authority = token_account,
        token::token_program = token_program
    )]
    pub pda_remitente: Account<'info, TokenAccount>,

    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_account: InterfaceAccount<'info, TokenAccount>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>

}
//5. Contexto de la instrucción para transferir tokens
#[derive(Accounts)]
pub struct TransferirTokens<'info> {
    #[account(
        mut,
        seeds=[
            b"transfer", 
            mint_account.key().as_ref()
        ],
        bump
    )]
    pda_remitente: Account<'info, TokenAccount>, //El error debe estar en que el pda remitente no esta creado aun
    #[account(mut)]
    recipiente: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"almacenar"],
        bump
    )]
    mint_account: Account<'info, DataAccount>,
    token_program: Program<'info, Token>,
    system_program: Program<'info, System>
}

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