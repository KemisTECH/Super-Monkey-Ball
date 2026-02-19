use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::FeeVaultError;

pub fn handler(
    ctx: Context<Initialize>,
    keeper: Pubkey,
    pause_guardian: Pubkey,
    config: ConfigParams,
) -> Result<()> {
    require!(config.validate(), FeeVaultError::InvalidConfig);

    let vault = &mut ctx.accounts.fee_vault;
    vault.vault_authority = ctx.accounts.vault_authority.key();
    vault.keeper_role = keeper;
    vault.pause_guardian = pause_guardian;
    vault.fee_balance_sol = 0;
    vault.fee_balance_usdc = 0;
    vault.is_paused = false;
    vault.config = config;
    vault.last_swap_time = Clock::get()?.unix_timestamp;
    vault.bump = *ctx.bumps.get("fee_vault").unwrap();

    Ok(())
}
