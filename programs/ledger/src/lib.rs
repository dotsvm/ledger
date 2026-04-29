use anchor_lang::prelude::*;

declare_id!("8MafjwK4me4QcCkpQSzLUd6vWU6sZaGZTRNZS5QU1HTA");

#[program]
pub mod ledger {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
