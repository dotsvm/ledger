use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

use instructions::initialize::{initialize as initialize_ix, Initialize};
use instructions::transfer::{transfer as transfer_ix, Transfer};

#[program]
pub mod ledger {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize_ix(ctx)
    }
    
    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        transfer_ix(ctx, amount)
    }
}