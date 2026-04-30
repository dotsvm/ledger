use crate::state::user_account::UserAccount;
use anchor_lang::prelude::*;
use crate::errors::LedgerError;

#[derive(Accounts)]
pub struct Transfer<'info> {
    #[account(mut)]
    pub sender: Account<'info, UserAccount>,

    #[account(mut)]
    pub receiver: Account<'info, UserAccount>,

    pub signer: Signer<'info>,
}

pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
    let sender = &mut ctx.accounts.sender;
    let receiver = &mut ctx.accounts.receiver;
    let signer = &ctx.accounts.signer;

    require!(sender.owner == signer.key(), LedgerError::Unauthorized);

    require!(sender.balance >= amount, LedgerError::InsufficientBalance);

    sender.balance = sender.balance.checked_sub(amount).ok_or(LedgerError::Overflow)?;
    receiver.balance = receiver.balance.checked_add(amount).ok_or(LedgerError::Overflow)?;

    Ok(())
}
