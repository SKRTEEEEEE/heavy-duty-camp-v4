use anchor_lang::prelude::*;
use anchor_spl::token::*;
use crate::colecciones::*;
/*
Esta función es válida para el caso en el que no hay 
ningun colaborador y no se han vendido tickets, e decir: 
boveda de ganancias en 0 tokens
boveda del evento en 0 tokens
*/

/*
Esta instruccion va a consistir en eliminar la cuenta que almacena
la informacion del evento y las cuentas PDAS relacionadas al mismo
(boveda del evento, boveda de ganancias, token del evento)

Cuentas:
- evento: cuenta que almacena la información del evento
- boveda del evento: cuenta que almacena los tokens obtenidos de los colaboradores
- boveda de ganancias: cuentq que almacena los tokens obtenidos de la venta de entradas
- token_evento: token que se entrega a los colaboradores del evento 
- autoridad: usuario que creo el evento y que será quien puede eliminarlo
 */
#[derive(Accounts)]
pub struct EliminarEvento<'info> {
    //cuentas
    #[account(
        mut,
        seeds=[
            evento.id.as_ref(),
            Evento::SEMILLA_EVENTO.as_bytes(),
            autoridad.key().as_ref(),
        ],
        bump = evento.bump_evento,

        constraint = evento.total_sponsors == 0,
        constraint = evento.tokens_vendidos == 0,
        constraint = evento.autoridad == autoridad.key(),
        //anotamos la intencion de cerrar la cuenta
        close = autoridad
    )]
    pub evento: Account<'info, Evento>,
    // tambien cerramos las bovedas
    #[account(
        mut,
        seeds=[
            Evento::SEMILLA_BOVEDA_EVENTO.as_bytes(),
            evento.key().as_ref(),
        ],
        bump = evento.bump_boveda_evento,
        constraint = boveda_evento.amount == 0
    )]
    pub boveda_evento: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds=[
            Evento::SEMILLA_BOVEDA_GANANCIAS.as_bytes(),
            evento.key().as_ref(),
        ],
        bump = evento.bump_boveda_ganancias,
        constraint = boveda_ganancias.amount == 0
        )]
    pub boveda_ganancias: Account<'info, TokenAccount>,

    //token del evento
    #[account(
        mut,
        seeds = [
            Evento::SEMILLA_TOKEN_EVENTO.as_bytes(),
            evento.key().as_ref(),
        ],
        bump = evento.bump_token_evento,
    )]
    pub token_evento: Account<'info, Mint>, // vamos a "desactivar" esta cuenta ->no existe close, pero en la version con extensiones, de mint_account, no hay close, pero se puede desactivar

    #[account(mut)]
    pub autoridad: Signer<'info>, // usuario que creo el evento

    //PROGRAMAS
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}
