use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::FeeVaultError;

pub fn handler(
    ctx: Context<SetConfig>,
    config: ConfigParams,
) -> Result<()> {
    let vault = &mut ctx.accounts.fee_vault;
    
    // Verify admin
    require!(
        ctx.accounts.admin.key() == vault.vault_authority,
        FeeVaultError::Unauthorized
    );
    
    require!(config.validate(), FeeVaultError::InvalidConfig);

    vault.config = config;

    emit!(ConfigUpdated {
        parameter: "full_config".to_string(),
        new_value: "updated".to_string(),
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
