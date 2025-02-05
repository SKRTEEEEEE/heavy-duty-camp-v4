use anchor_lang::prelude::*;
use anchor_lang::system_program::{transfer, Transfer};

declare_id!("HeU6RRAzdDTvtDwYcUdc4p9msBoBVJtihgFe3AyDNv1u");

#[program]
pub mod cpi {
    use super::*;

    pub fn transferir(ctx: Context<Transferir>, cantidad: u64) -> Result<()> {
        let de = ctx.accounts.pda_remitente.to_account_info();
        let para = ctx.accounts.recipiente.to_account_info();
        let programa = ctx.accounts.system_program.to_account_info();

        let bump = ctx.bumps.pda_remitente;
        let semillas_firma: &[&[&[u8]]] = &[&["PDA".as_bytes(), &[bump]]]; //Nuestro semillas necesarias para la firma

        let cpi_context = CpiContext::new(
            programa,
            Transfer {
                from: de,
                to: para,
            },
        ).with_signer(semillas_firma);

        transfer(cpi_context, cantidad)?;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Transferir<'info> {
    #[account(mut, seeds = ["PDA".as_bytes()], bump)] // mutable porque se le debita saldo, esta sera la PDA
    pda_remitente: SystemAccount<'info>,
    #[account(mut)] // mutable porque se le aumenta saldo
    recipiente: SystemAccount<'info>,
    system_program: Program<'info, System>,
}