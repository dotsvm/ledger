use anchor_lang::prelude::*;
declare_id!("9SyY5ZMqxvgrp62xhe7HLe1kDaWfBk1qufa6eJrhJPHu");

pub mod instructions;
pub mod state;

use instructions::initialize::*;
use instructions::transfer::*;

#[program]
pub mod ledger {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        instructions::initialize::initialize(ctx)
    }

    pub fn transfer(ctx: Context<Transfer>, amount: u64) -> Result<()> {
        instructions::transfer::transfer(ctx, amount)
    }
}