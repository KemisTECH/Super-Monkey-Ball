use anchor_lang::prelude::*;
use crate::state::*;
use crate::errors::FeeVaultError;

pub fn handler(
    ctx: Context<TogglePause>,
) -> Result<()> {
    let vault = &mut ctx.accounts.fee_vault;
    
    // Verify pause guardian
    require!(
        ctx.accounts.pause_guardian.key() == vault.pause_guardian,
        FeeVaultError::Unauthorized
    );

    vault.is_paused = !vault.is_paused;

    emit!(PauseToggled {
        is_paused: vault.is_paused,
        timestamp: Clock::get()?.unix_timestamp,
    });

    Ok(())
}
