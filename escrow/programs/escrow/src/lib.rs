/*
He intentado hacer deploy en local pero por alguna razon tiene problemas con anchor-spl

*/

use anchor_lang::prelude::*;
use anchor_spl::token::*;
// use anchor_lang::prelude::{Context, Result, Signer, SystemAccount, Program, Sysvar, Rent, Account};
// use anchor_spl::token::{Token, TokenAccount, Transfer, Mint, CloseAccount, transfer, close_account};



// declare_id!("H7xXLvqYyyJ25NDEAL5tYFrs4nBC2EYAGfiMQKf2PQjR");
declare_id!("F91W41D8Uqo18YHYHdULhaPETcbsxNHTHMHxYdmXga85");

#[program]
pub mod intercambiador_token_spl {
    use super::*;
    pub fn incializar(
        ctx: Context<Initialize>, 
        id: String, cantidad_tokens_a: u64, 
        cantidad_tokens_b:u64) -> Result<()> {
            //almacenamos la informacion del intercambio de la cuenta escrow
            ctx.accounts.escrow.inicializador = ctx.accounts.inicializador.key();
            ctx.accounts.escrow.token_a = ctx.accounts.token_a.key();
            ctx.accounts.escrow.id = id;

            //almacenamos las cantidades de cada token, teniendo en cuenta los decimales
            ctx.accounts.escrow.cantidad_token_a = 
                cantidad_tokens_a * 10_u64.pow(ctx.accounts.token_a.decimals.into());
            ctx.accounts.escrow.cantidad_token_b = 
                cantidad_tokens_b * 10_u64.pow(ctx.accounts.token_b.decimals.into());

            //almacenamos los bumps
            ctx.accounts.escrow.bump_cuenta_garantia = ctx.bumps.cuenta_de_garantia;
            ctx.accounts.escrow.bump_escrow = ctx.bumps.escrow;

            //Transferimos la cantidad deseada
            let cpi_ctx = CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.inicializador_cuenta_token_a.to_account_info(),
                    to: ctx.accounts.cuenta_de_garantia.to_account_info(),
                    authority: ctx.accounts.inicializador.to_account_info()
                }
                );

            transfer(cpi_ctx, ctx.accounts.escrow.cantidad_token_a)?;
            Ok(())
    }

    pub fn finalizar(ctx: Context<Finalizar>) -> Result<()> {
        let cpi_al_inicializador = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.aceptante_token_account_b.to_account_info(),
                to: ctx.accounts.inicializador_token_account_b.to_account_info(),
                // el que acepta la oferta tiene que firmar desde el client
                authority: ctx.accounts.aceptante.to_account_info(),
            },
        );

        // llamada a la cpi
        transfer(cpi_al_inicializador, ctx.accounts.escrow.cantidad_token_b)?;
        // definimos las semillas firmantes para la PDA
        let semillas_firma: &[&[&[u8]]] = &[&[
            ctx.accounts.escrow.to_account_info().key.as_ref(),
            &[ctx.accounts.escrow.bump_cuenta_garantia],
        ]];
        // transferimos los tokens de la cuenta de garantía a la cuenta token del usuario aceptante
        let cpi_al_aceptante = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.cuenta_de_garantia.to_account_info(),
                to: ctx.accounts.aceptante_token_account_a.to_account_info(),
                authority: ctx.accounts.cuenta_de_garantia.to_account_info(),
            },
        ).with_signer(semillas_firma);
        // llamada cpi
        transfer(cpi_al_aceptante, ctx.accounts.cuenta_de_garantia.amount)?; // el total de tokens almacenados

        let cpi_cerrar = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.cuenta_de_garantia.to_account_info(),
                destination: ctx.accounts.inicializador.to_account_info(), // se devuelve la renta al usuario incializador
                authority: ctx.accounts.cuenta_de_garantia.to_account_info(),
            },
        ).with_signer(semillas_firma);

        // hacemos cpi al token program para cerrar la PDA que es una cuenta token
        close_account(cpi_cerrar)?;

        Ok(())
    }

    pub fn cerrar_escrow(ctx: Context<CerrarEscrow>) -> Result<()> {
        let semillas_firma: &[&[&[u8]]] = &[&[
            ctx.accounts.escrow.to_account_info().key.as_ref(),
            &[ctx.accounts.escrow.bump_cuenta_garantia],
        ]];
        //enviar/devolver tokenA al inicializador
        let cpi_al_inicializador = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.cuenta_de_garantia.to_account_info(),
                to: ctx.accounts.inicializador_cuenta_token_a.to_account_info(),
                authority: ctx.accounts.cuenta_de_garantia.to_account_info(),
            },
        )
        .with_signer(semillas_firma);
        transfer(cpi_al_inicializador,  ctx.accounts.cuenta_de_garantia.amount)?;

        //cerrar la cuenta escrow
        let cpi_cerrar = CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            CloseAccount {
                account: ctx.accounts.cuenta_de_garantia.to_account_info(),
                destination: ctx.accounts.inicializador.to_account_info(), // se devuelve la renta al usuario incializador
                authority: ctx.accounts.cuenta_de_garantia.to_account_info(),
            },
        ).with_signer(semillas_firma);

        // hacemos cpi al token program para cerrar la PDA que es una cuenta token
        close_account(cpi_cerrar)?;

        Ok(())

    }


}




#[derive(Accounts)]
#[instruction(id:String)] //al vincularlo a un id, podemos hacer diferente escrow. Porque cambiamos como se genera la semilla
pub struct Initialize<'info> {
    #[account(
        init, 
        payer=inicializador,
        space = 8 + Escrow::INIT_SPACE,
        seeds=[
            inicializador.key().as_ref(),
            id.as_ref()
        ],
        bump
    )]
    pub escrow: Account<'info, Escrow>,

    #[account(mut)]
    pub inicializador:Signer<'info>,


    #[account(
        mut,
        constraint = inicializador_cuenta_token_a.mint == token_a.key()
    )]
    pub inicializador_cuenta_token_a: Account<'info, TokenAccount>,


    #[account(
        init,
        payer = inicializador,
        seeds = [
            escrow.key().as_ref()
        ],
        bump,
        token::mint = token_a,
        token::authority = cuenta_de_garantia,
    )]
    pub cuenta_de_garantia:Account<'info, TokenAccount>,
    //tokens
    pub token_a: Account<'info, Mint>,
    pub token_b: Account<'info, Mint>,
    //programs
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    //renta
    pub rent: Sysvar<'info, Rent>,
} 
#[derive(Accounts)]
pub struct CerrarEscrow<'info> {
    #[account(
        mut,
        seeds= [ // PDA
            inicializador.key().as_ref(),
            escrow.id.as_ref()
        ],
        bump = escrow.bump_escrow,
   )]
    pub escrow: Account<'info, Escrow>, // Cuenta de datos que almacena la información del intercambio

    #[account(
        mut, // mutable porque se le debitará saldo
        seeds = [escrow.key().as_ref()], // semillas PDA
        bump = escrow.bump_cuenta_garantia, // bump
    )]
    pub cuenta_de_garantia: Account<'info, TokenAccount>, // almacena los token_A del intercambio

    #[account(
        mut,
        constraint = inicializador_cuenta_token_a.mint == escrow.token_a.key()
    )]
    pub inicializador_cuenta_token_a: Account<'info, TokenAccount>, // le vamos a devolver aqui los tokens

    #[account(mut)]//porque le devolvemos cash
    pub inicializador: Signer<'info>,

    pub token_program: Program<'info, Token>,

}

#[derive(Accounts)]
pub struct Finalizar<'info> {
    #[account(
        mut,
        seeds= [ // PDA
            inicializador.key().as_ref(),
            escrow.id.as_ref()
        ],
        bump = escrow.bump_escrow,
   )]
    pub escrow: Account<'info, Escrow>, // Cuenta de datos que almacena la información del intercambio

    #[account(
        mut, // mutable porque se le debitará saldo
        seeds = [escrow.key().as_ref()], // semillas PDA
        bump = escrow.bump_cuenta_garantia, // bump
    )]
    pub cuenta_de_garantia: Account<'info, TokenAccount>, // almacena los token_A del intercambio

    #[account(mut)]
    pub aceptante: Signer<'info>, // aceptante, el que acepta la oferta del inicializador

    #[account(mut)]
    pub inicializador: SystemAccount<'info>, // el usuario que incializo el intercambio, no se valida nada

    // cuentas token
    #[account(
        mut, // mutable porque se le aumentará saldo
        associated_token::mint = escrow.token_b,
        associated_token::authority = escrow.inicializador,
    )]
    pub inicializador_token_account_b: Account<'info, TokenAccount>, // donde el inicializador recibirá token_B

    #[account(
        mut, // mutable porque se le debitará saldo
        associated_token::mint = escrow.token_b,
        associated_token::authority = aceptante.key(),
    )]
    pub aceptante_token_account_b: Account<'info, TokenAccount>, // cuenta token de token_B del aceptante

    #[account(
        mut, // mutable porque se le aumentará saldo
        associated_token::mint = escrow.token_a,
        associated_token::authority = aceptante.key(),
    )]
    pub aceptante_token_account_a: Account<'info, TokenAccount>, // donde el aceptante recibirá token_A

    //programas
    pub token_program: Program<'info, Token>,
}
#[account]
#[derive(InitSpace)]
pub struct Escrow{
    pub inicializador: Pubkey,
    pub token_a: Pubkey,
    pub cantidad_token_a: u64,
    pub token_b: Pubkey,
    pub cantidad_token_b: u64,

    #[max_len(150)]
    pub id: String,

    pub bump_escrow: u8,
    pub bump_cuenta_garantia: u8,
}