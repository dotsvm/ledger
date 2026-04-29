use crate::state::user_account::UserAccount;
use anchor_lang::prelude::*;

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

    if sender.owner != signer.key() {
        return Err(ProgramError::IllegalOwner.into());
    }

    if sender.balance < amount {
        return Err(ProgramError::InsufficientFunds.into());
    }

    sender.balance -= amount;
    receiver.balance += amount;

    Ok(())
}
