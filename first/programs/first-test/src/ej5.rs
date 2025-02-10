use anchor_lang::prelude::*;
use anchor_spl::token::*;

declare_id!("639DEktgoBuJJaEbFEr7RcRHNFHM82aKdJTt4YqqTudm");

//4. Programa con las instrucciones
#[program]
pub mod pda_transfer {
    use super::*;
    pub fn mint(ctx: Context<AlmacenarMint>, supply: u64) -> Result<()> {
        ctx.accounts.mint_account_pda.bump_mint_token = ctx.bumps.mint_account_pda;
        ctx.accounts.mint_account_pda.bump_token_account = ctx.bumps.token_account_pda_remitente;
        ctx.accounts.mint_account_pda.fee_payer = ctx.accounts.fee_payer.key();
        ctx.accounts.mint_account_pda.mint_token = ctx.accounts.mint_account.key();
        //Preparar cantidad tokens para mint
        let decimals = ctx.accounts.mint_account.decimals;
        let amount = supply * 10u64.pow(decimals as u32);
        //Llamar a la funcion mint_tokens
        let cpi_mint = MintTo {
            mint: ctx.accounts.mint_account.to_account_info(),
            to: ctx.accounts.token_account_pda_remitente.to_account_info(),
            authority: ctx.accounts.fee_payer.to_account_info(),
        };
        let cpi_program = ctx.accounts.token_program.to_account_info();
        //Tengo dudas si aqui es: Mint Tokens with PDA mint authority via CPI
        let cpi_context = CpiContext::new(cpi_program, cpi_mint);
        mint_to(cpi_context, amount)?;
        Ok(())
    }
   pub fn transferir(ctx: Context<TransferirTokens>, quantity: u64) -> Result<()> {
    let decimals = ctx.accounts.mint_account.decimals;
    let amount = quantity * 10u64.pow(decimals as u32);

    // Guardar la clave en una variable para que tenga un lifetime más largo
    let mint_key = ctx.accounts.mint_account.key();

    let signer_seeds: &[&[&[u8]]] = &[&[b"mint", mint_key.as_ref(), &[ctx.accounts.mint_account_pda.bump_token_account]]];

    let cpi_accounts = TransferChecked {
        mint: ctx.accounts.mint_account.to_account_info(),
        from: ctx.accounts.token_account_pda_remitente.to_account_info(),
        to: ctx.accounts.token_account_recibidor.to_account_info(),
        authority: ctx.accounts.token_account_pda_remitente.to_account_info() 
    };

    let cpi_program = ctx.accounts.token_program.to_account_info();
    let cpi_context = CpiContext::new(cpi_program, cpi_accounts).with_signer(signer_seeds);

    transfer_checked(cpi_context, amount, decimals)?;

    Ok(())
}

}
//5. Contexto de la instrucción para transferir tokens
#[derive(Accounts)]
pub struct TransferirTokens<'info>{
    #[account(mut)]
    pub token_account_recibidor: Account<'info, TokenAccount>,

    #[account(mut)]
    pub fee_payer: Signer<'info>,

    // Cuenta real de tipo Mint (se deriva del Pubkey almacenado en mint_account_pda)
    #[account(mut, address = mint_account_pda.mint_token)]
    pub mint_account: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [b"almacenar", mint_account_pda.mint_token.as_ref()],
        bump = mint_account_pda.bump_mint_token
    )]
    mint_account_pda: Account<'info, DataAccount>,

    #[account(
        mut,
        seeds = [b"mint", mint_account_pda.mint_token.as_ref()],
        bump = mint_account_pda.bump_token_account
    )]
    token_account_pda_remitente:  Account<'info, TokenAccount>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

//2. Contexto de la instrucción para almacenar el mint token
#[derive(Accounts)]
pub struct AlmacenarMint<'info> {
    //3. cuentas para almacenar mint?
    #[account(mut)]
    pub fee_payer: Signer<'info>,

    #[account(
        init,
        seeds = [b"almacenar", mint_account.key().as_ref()],
        bump,
        payer = fee_payer,
        space = 8 + DataAccount::INIT_SPACE
    )]
    pub mint_account_pda: Account<'info, DataAccount>,

    #[account(
        init,
        seeds = [b"mint", mint_account.key().as_ref()],
        bump,
        payer = fee_payer,
        token::mint = mint_account,
        // En lugar de usar mint_account_pda.fee_payer directamente, necesitamos una cuenta real
        // token::authority = fee_payer  // Cambiamos a usar la cuenta fee_payer
        token::authority = token_account_pda_remitente,  
    )]
    pub token_account_pda_remitente: Account<'info, TokenAccount>,

    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub rent: Sysvar<'info, Rent>,
}

//1. Estructura de datos del mint token (cuenta)
#[account]
#[derive(InitSpace)]
pub struct DataAccount {
    pub mint_token: Pubkey,
    pub fee_payer: Pubkey,
    pub bump_mint_token: u8,
    pub bump_token_account: u8,
}
