use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::FeeVaultError;

pub fn handler(
    ctx: Context<RecordSwapReceipt>,
    sold_token: String,
    sold_amount: u64,
    bought_token: String,
    bought_amount: u64,
    price: f64,
    slippage_bps: u16,
) -> Result<()> {
    let vault = &mut ctx.accounts.fee_vault;
    
    // Verify keeper
    require!(
        ctx.accounts.keeper.key() == vault.keeper_role,
        FeeVaultError::Unauthorized
    );
    
    require!(!vault.is_paused, FeeVaultError::VaultPaused);

    // Update last swap time
    vault.last_swap_time = Clock::get()?.unix_timestamp;

    // Emit receipt event
    emit!(SwapExecuted {
        sold_token,
        sold_amount,
        bought_token,
        bought_amount,
        price,
        slippage_bps,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
