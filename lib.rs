use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;
pub mod errors;

use instructions::*;
use state::*;

declare_id!("PLACEHOLDER_PROGRAM_ID");

#[program]
pub mod fee_vault {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        keeper: Pubkey,
        pause_guardian: Pubkey,
        config: ConfigParams,
    ) -> Result<()> {
        instructions::initialize::handler(ctx, keeper, pause_guardian, config)
    }

    pub fn deposit_fees(
        ctx: Context<DepositFees>,
        amount_sol: u64,
        amount_usdc: u64,
    ) -> Result<()> {
        instructions::deposit_fees::handler(ctx, amount_sol, amount_usdc)
    }

    pub fn record_swap_receipt(
        ctx: Context<RecordSwapReceipt>,
        sold_token: String,
        sold_amount: u64,
        bought_token: String,
        bought_amount: u64,
        price: f64,
        slippage_bps: u16,
    ) -> Result<()> {
        instructions::record_swap_receipt::handler(
            ctx,
            sold_token,
            sold_amount,
            bought_token,
            bought_amount,
            price,
            slippage_bps,
        )
    }

    pub fn set_config(
        ctx: Context<SetConfig>,
        config: ConfigParams,
    ) -> Result<()> {
        instructions::set_config::handler(ctx, config)
    }

    pub fn toggle_pause(
        ctx: Context<TogglePause>,
    ) -> Result<()> {
        instructions::pause::handler(ctx)
    }
}
