use anchor_lang::prelude::*;
pub mod instructions;

declare_id!("7xJ99D8b6LJsbnaqYBgvaa4Ga1nDjfHte3mu3aqS4MYq");

#[program]
pub mod test_drift_cpi {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
