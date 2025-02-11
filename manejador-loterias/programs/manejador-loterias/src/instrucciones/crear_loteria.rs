use anchor_lang::prelude::*;
use anchor_spl::token::*;
use crate::colecciones::*;
//2. definimos la funcion principal de nuestra instrucción:
pub fn crear_loteria(
    ctx: Context<CrearLoteria>,
    id: String,
    nombre: String,
    descripcion: String,
    precio_token: f64,
    tipo_loteria: f64,
) -> Result<()> {
    ctx.accounts.loteria.id = id;
    ctx.accounts.loteria.nombre = nombre;
    ctx.accounts.loteria.descripcion = descripcion;

    ctx.accounts.loteria.precio_token =
        (precio_token as u64) * 10_u64.pow(ctx.accounts.token_aceptado.decimals.into());

    ctx.accounts.loteria.activo = true;
    ctx.accounts.loteria.tokens_vendidos = 0;

    // Asignar el valor de la lotería
    ctx.accounts.loteria.tipo_loteria = TipoLoteria::from_u64(tipo_loteria as u64);

    ctx.accounts.loteria.autoridad = ctx.accounts.autoridad.key();
    ctx.accounts.loteria.token_aceptado = ctx.accounts.token_aceptado.key();

    ctx.accounts.loteria.bump_loteria = ctx.bumps.loteria;
    ctx.accounts.loteria.bump_token_loteria = ctx.bumps.token_loteria;
    ctx.accounts.loteria.bump_boveda_loteria = ctx.bumps.boveda_loteria;

    Ok(())
}

//1. definimos el contexto
#[derive(Accounts)]
#[instruction(id:String)]
pub struct CrearLoteria<'info> {
    //cuenta de la loteria
    #[account(
        init, 
        seeds=[
            id.to_string().as_ref(),
            Loteria::SEMILLA_LOTERIA.as_bytes(),
            autoridad.key.as_ref(),
        ],
        bump,
        payer = autoridad,
        space = 8 + Loteria::INIT_SPACE
    )]
    pub loteria: Account<'info, Loteria>,

    //cuenta 'mint_account' del token aceptado, ya creado
    pub token_aceptado: Account<'info, Mint>,

    //cuenta 'mint_account' PDA del token de la loteria
    #[account(
        init,
        seeds = [
            Loteria::SEMILLA_TOKEN_LOTERIA.as_bytes(),
            loteria.key().as_ref(),
        ],
        bump,
        payer = autoridad,
        mint::decimals = 0,
        mint::authority = loteria,
    )]
    pub token_loteria: Account<'info, Mint>,

    // cuenta 'token_account' PDA para la 'boveda de la loteria'
    #[account(
        init,
        payer = autoridad,
        seeds = [
            Loteria::SEMILLA_BOVEDA_LOTERIA.as_bytes(),
            loteria.key().as_ref(),
        ],
        bump,
        token::mint = token_aceptado,
        token::authority = loteria,
    )]
    pub boveda_loteria: Account<'info, TokenAccount>,

    #[account(mut)]
    pub autoridad: Signer<'info>,

    //programas
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,

    //variable renta
    pub rent: Sysvar<'info, Rent>,
}
