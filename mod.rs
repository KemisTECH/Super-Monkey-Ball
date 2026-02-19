use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub vault_authority: Signer<'info>,
    #[account(
        init,
        payer = payer,
        space = 8 + std::mem::size_of::<FeeVault>(),
        seeds = [b"fee_vault"],
        bump
    )]
    pub fee_vault: Account<'info, FeeVault>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DepositFees<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,
    #[account(mut)]
    pub fee_vault: Account<'info, FeeVault>,
}

#[derive(Accounts)]
pub struct RecordSwapReceipt<'info> {
    pub keeper: Signer<'info>,
    #[account(mut)]
    pub fee_vault: Account<'info, FeeVault>,
}

#[derive(Accounts)]
pub struct SetConfig<'info> {
    pub admin: Signer<'info>,
    #[account(mut)]
    pub fee_vault: Account<'info, FeeVault>,
}

#[derive(Accounts)]
pub struct TogglePause<'info> {
    pub pause_guardian: Signer<'info>,
    #[account(mut)]
    pub fee_vault: Account<'info, FeeVault>,
}

pub mod initialize;
pub mod deposit_fees;
pub mod record_swap_receipt;
pub mod set_config;
pub mod pause;

pub use initialize::*;
pub use deposit_fees::*;
pub use record_swap_receipt::*;
pub use set_config::*;
pub use pause::*;
