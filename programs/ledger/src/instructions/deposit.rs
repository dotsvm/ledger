use anchor_lang::prelude::*;
use crate::state::user_account::UserAccount;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Account<'info, UserAccount>,

    pub signer: Signer<'info>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let signer = &ctx.accounts.signer;

    if user.owner != signer.key() {
        return Err(ProgramError::IllegalOwner.into());
    }

    user.balance += amount;

    Ok(())
}