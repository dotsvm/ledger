pub mod instructions;
pub mod state;

use instructions::initialize::*;

#[program]
pub mod ledger {
    use super::*;
    
    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize(ctx)
    }
}