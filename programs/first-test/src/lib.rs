use anchor_lang::prelude::*;
use anchor_spl::token::*;


declare_id!("H7xXLvqYyyJ25NDEAL5tYFrs4nBC2EYAGfiMQKf2PQjR");

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
          /* 
        Transferimos los token_B de la cuenta token del aceptante 
        a la cuenta token del inicializador
        */

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

        /*
       Una vez que el usuario incializador recibe los token_B del intercambio, el programa
       tranfiere de la cuenta de garantia (PDA) los token_A hacia la cuenta token
       del usuario aceptante.
        */

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

        // que más podemos hacer??

        /*
        Una vez que se han intercambiado los token_A y token_B entre el usuario inicializador
        y el usuario aceptante, se cierra la cuenta de garantía y la renta se devuelve al 
        usuario inicializador
        */

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
/*
Finalizar Propuesta de Intercambio

Esta instrucción realiza las transferencias necesarias para dar
por finalizado el intercambio de tokens. 

cuentas:
- escrow: cuenta que almacena todos los datos del intercambio
- cuenta_de_garantia: cuenta token (PDA) donde se almacenan los token_A que el usuario inicializador
    desea intercambiar.
- aceptante: usuario que acepta la propuesta de intercambio.
- incializador: usuario que crea la propuesta de intercambio
- inicializador_token_account_b: cuenta token propiedad del inicializador, donde recibira los token_B
- aceptante_token_account_b: cuenta token propiedad del aceptante, que almacena token_B
- aceptante_cuenta_token_a: cuenta token propiedad del aceptante, donde recibira los token_A
- token_program: programa para gestionar tokens en Solana
*/

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