use anchor_lang::prelude::*;

use drift::state::{oracle::OraclePriceData, user::User};

use pyth_sdk_solana::state::load_price_account;

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
