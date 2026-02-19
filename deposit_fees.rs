use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::FeeVaultError;

pub fn handler(
    ctx: Context<DepositFees>,
    amount_sol: u64,
    amount_usdc: u64,
) -> Result<()> {
    require!(!ctx.accounts.fee_vault.is_paused, FeeVaultError::VaultPaused);

    let vault = &mut ctx.accounts.fee_vault;
    
    // Add to vault balances
    vault.fee_balance_sol = vault.fee_balance_sol
        .checked_add(amount_sol)
        .ok_or(FeeVaultError::Overflow)?;
    
    vault.fee_balance_usdc = vault.fee_balance_usdc
        .checked_add(amount_usdc)
        .ok_or(FeeVaultError::Overflow)?;

    // Emit event
    emit!(FeeDeposited {
        depositor: ctx.accounts.depositor.key(),
        amount_sol,
        amount_usdc,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
