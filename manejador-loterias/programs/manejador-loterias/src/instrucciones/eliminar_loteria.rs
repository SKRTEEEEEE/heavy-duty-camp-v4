use anchor_lang::prelude::*;
use anchor_spl::token::*;
use crate::colecciones::*;
use crate::utilidades::*;

/*
Esta instruccion va a consistir en eliminar la cuenta que almacena
la informacion de la loteria y las cuentas PDAS relacionadas al mismo
(boveda de la loteria y token del loteria)

El objetivo de esta instrucción es eliminar la 'Loteria' y todos sus tokens asignados, etc... 
Una vez se haya finalizado esta, se haya recogido los beneficios para enviar los premios secundarios 
y se haya dado el tiempo estipulado para reclamar a los usuarios dicho premio secundario.
O por otros motivos de seguridad se deba parar la loteria.

Cuentas:
- loteria: cuenta que almacena la información del loteria
- boveda de la loteria: cuenta que almacena los tokens obtenidos de la venta de boletos
- token_loteria: token que se entrega a los participadores de la loteria 
- autoridad: usuario que creo el loteria y que será quien puede eliminarlo
 */

#[derive(Accounts)]
pub struct EliminarLoteria<'info> {

    #[account(
        mut,
        seeds=[
            loteria.id.as_ref(),
            Loteria::SEMILLA_LOTERIA.as_bytes(),
            autoridad.key().as_ref(),
        ],
        bump = loteria.bump_loteria,
        //verificaciones
        // constraint = loteria.tokens_vendidos == loteria.tipo_loteria.valor(), -> la cambiamos por un require
        constraint = loteria.autoridad == autoridad.key() @ CodigoError::UsuarioNoAutorizado,
        close = autoridad,
    )]
    pub loteria: Account<'info, Loteria>,

    #[account(
        mut,
        seeds=[
            Loteria::SEMILLA_BOVEDA_LOTERIA.as_bytes(),
            loteria.key().as_ref(),
        ],
        bump = loteria.bump_boveda_loteria,
        //verificaciones
        constraint = boveda_loteria.amount == 0 @ CodigoError::BovedaDelaLoteriaNoVacia,
    )]
    pub boveda_loteria: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[
            Loteria::SEMILLA_TOKEN_LOTERIA.as_bytes(),
            loteria.key().as_ref(),
        ],
        bump = loteria.bump_token_loteria,
    )]
    pub token_loteria: Account<'info, Mint>,

    #[account(mut)]
    pub autoridad: Signer<'info>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn eliminar_loteria(ctx: Context<EliminarLoteria>)->Result<()> {
    let loteria = &ctx.accounts.loteria;
    // requerimos que o se hayan vendido todos los token o no haya "ninguno" vendido
    require!(
        loteria.tokens_vendidos == 0 || loteria.tokens_vendidos == loteria.tipo_loteria.valor(),
        CodigoError::TokensDispobiles
    );

    let semillas_firma: &[&[&[u8]]] = &[&[
        ctx.accounts.loteria.id.as_ref(),
        Loteria::SEMILLA_LOTERIA.as_bytes(),
        ctx.accounts.loteria.autoridad.as_ref(),
        &[ctx.accounts.loteria.bump_loteria],
    ]];        

    let cerrar_boveda_loteria = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        CloseAccount {
            account: ctx.accounts.boveda_loteria.to_account_info(),
            destination: ctx.accounts.autoridad.to_account_info(),
            authority: ctx.accounts.loteria.to_account_info(),
        }
    ).with_signer(semillas_firma);

    close_account(cerrar_boveda_loteria)?;

    let revocar_autoridad = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        SetAuthority {
            account_or_mint: ctx.accounts.token_loteria.to_account_info(),
            current_authority: ctx.accounts.loteria.to_account_info(),
        },
    ).with_signer(semillas_firma);

    set_authority(revocar_autoridad, spl_token::instruction::AuthorityType::MintTokens,None,)?;

    Ok(())
}