use anchor_lang::prelude::*;
use crate::state::user_account::UserAccount;
use crate::errors::LedgerError;

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user: Account<'info, UserAccount>,

    pub signer: Signer<'info>,
}

pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    let user = &mut ctx.accounts.user;
    let signer = &ctx.accounts.signer;

    require!(user.owner == signer.key(), LedgerError::Unauthorized);

    user.balance += amount;

    Ok(())
}