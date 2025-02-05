use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, transfer, Transfer as TokenTransfer};
use anchor_spl::token_interface::{Mint, TokenInterface};

declare_id!("639DEktgoBuJJaEbFEr7RcRHNFHM82aKdJTt4YqqTudm");

#[program]
pub mod pda_transfer {
    use super::*;

    pub fn almacenar_mint(
        ctx: Context<AlmacenarMint>, 
        input_mint_account: Pubkey
    ) -> Result<()> {
        ctx.accounts.mint_account.mint_token = input_mint_account;
        ctx.accounts.mint_account.bump = ctx.bumps.mint_account;
        ctx.accounts.mint_account.fee_payer = ctx.accounts.fee_payer.key();
        Ok(())
    }

    pub fn crear_pda_y_mintear(
        ctx: Context<CrearPdaYMintear>, 
        cantidad: u64
    ) -> Result<()> {
        // Mint tokens directly to PDA
        let cpi_accounts = token::MintTo {
            mint: ctx.accounts.token_mint.to_account_info(),
            to: ctx.accounts.pda_remitente.to_account_info(),
            authority: ctx.accounts.fee_payer.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        token::mint_to(cpi_ctx, cantidad)?;

        // Update mint account details
        ctx.accounts.mint_account.mint_token = ctx.accounts.token_mint.key();
        ctx.accounts.mint_account.bump = ctx.bumps.mint_account;
        ctx.accounts.mint_account.fee_payer = ctx.accounts.fee_payer.key();

        Ok(())
    }

    pub fn transferir_token(
        ctx: Context<TransferirTokens>, 
        quantity: u64
    ) -> Result<()> {
        let bump = ctx.bumps.pda_remitente;
        let pubkey_token = ctx.accounts.mint_account.key();
        
        let semillas_firma: &[&[&[u8]]] = &[
            &["transfer".as_bytes(), pubkey_token.as_ref(), &[bump]]
        ];

        let cpi_accounts = TokenTransfer {
            from: ctx.accounts.pda_remitente.to_account_info(),
            to: ctx.accounts.recipiente.to_account_info(),
            authority: ctx.accounts.pda_remitente.to_account_info(),
        };

        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts)
            .with_signer(semillas_firma);

        transfer(cpi_ctx, quantity)?;

        Ok(())
    }
}
#[derive(Accounts)]
pub struct CrearPdaYMintear<'info> {
    #[account(mut)]
    pub fee_payer: Signer<'info>,

    #[account(
        init_if_needed,
        seeds = [b"almacenar"],
        bump,
        payer = fee_payer,
        space = 8 + DataAccount::INIT_SPACE
    )]
    pub mint_account: Account<'info, DataAccount>,

    #[account(
        init_if_needed,
        seeds = [b"transfer", mint_account.key().as_ref()],
        bump,
        payer = fee_payer,
        token::mint = token_mint,
        token::authority = pda_remitente,
        token::token_program = token_program
    )]
    pub pda_remitente: Account<'info, TokenAccount>,

    #[account(mut)]
    pub token_mint: InterfaceAccount<'info, Mint>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>
}
#[derive(Accounts)]
pub struct TransferirTokens<'info> {
    #[account(
        mut,
        seeds = [b"transfer", mint_account.key().as_ref()],
        bump
    )]
    pub pda_remitente: Account<'info, TokenAccount>,

    #[account(mut)]
    pub recipiente: Account<'info, TokenAccount>,

    #[account(
        seeds = [b"almacenar"],
        bump
    )]
    pub mint_account: Account<'info, DataAccount>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
pub struct AlmacenarMint<'info> {
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

#[account]
#[derive(InitSpace)]
pub struct DataAccount {
    pub mint_token: Pubkey,
    pub fee_payer: Pubkey,
    pub bump: u8,
}