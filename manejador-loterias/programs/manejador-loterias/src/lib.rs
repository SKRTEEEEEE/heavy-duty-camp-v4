use anchor_lang::prelude::*;

declare_id!("2tvTrWEr7jHtjmLmpT2XqrUHdJPb9ndZZEQZZ4B1zVWV");

#[program]
pub mod manejador_loterias {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
