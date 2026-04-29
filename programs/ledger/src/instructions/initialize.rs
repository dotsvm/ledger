use anchor_lang::prelude::*;
use crate::state::user_account::UserAccount;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = signer, space = 48)]
    pub user: Account<'info, UserAccount>,
    
    #[account(mut)]
    pub signer: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
    let user = &mut ctx.accounts.user;
    
    user.owner = *ctx.accounts.signer.key;
    user.balance = 0;
    
    Ok(())
}