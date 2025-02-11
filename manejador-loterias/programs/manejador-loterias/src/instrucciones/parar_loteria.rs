use anchor_lang::prelude::*;
use crate::colecciones::*;
use crate::utilidades::*;

#[derive(Accounts)]
pub struct PararLoteria<'info> {
    #[account(
        mut,
        seeds=[
            loteria.id.as_ref(),
            Loteria::SEMILLA_LOTERIA.as_bytes(),
            autoridad.key().as_ref(),
        ],
        bump = loteria.bump_loteria,

        constraint = loteria.autoridad == autoridad.key() @ CodigoError::UsuarioNoAutorizado,
    )]
    pub loteria: Account<'info, Loteria>,

    #[account(mut)]
    pub autoridad: Signer<'info>,
    pub system_program: Program<'info, System>,
}
pub fn parar_loteria(ctx: Context<PararLoteria>) -> Result<()> {
    let loteria = &mut ctx.accounts.loteria;
    loteria.activo = !loteria.activo; // Invierte el valor actual
    Ok(())
}
