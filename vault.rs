use anchor_lang::prelude::*;

#[account]
pub struct FeeVault {
    pub vault_authority: Pubkey,
    pub keeper_role: Pubkey,
    pub pause_guardian: Pubkey,
    pub fee_balance_sol: u64,
    pub fee_balance_usdc: u64,
    pub is_paused: bool,
    pub config: ConfigParams,
    pub last_swap_time: i64,
    pub bump: u8,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct ConfigParams {
    pub cycle_seconds: u32,
    pub min_swap_sol: u64,
    pub max_spend_sol: u64,
    pub max_slippage_bps: u16,
    pub split_buyback: u8,
    pub split_lp: u8,
    pub split_burn: u8,
    pub cooldown_seconds: u32,
}

impl ConfigParams {
    pub fn validate(&self) -> bool {
        // Splits should sum to 100
        if self.split_buyback as u16 + self.split_lp as u16 + self.split_burn as u16 != 100 {
            return false;
        }
        // Max slippage should be reasonable (< 50%)
        if self.max_slippage_bps > 5000 {
            return false;
        }
        true
    }
}

#[event]
pub struct FeeDeposited {
    pub depositor: Pubkey,
    pub amount_sol: u64,
    pub amount_usdc: u64,
    pub timestamp: i64,
}

#[event]
pub struct SwapExecuted {
    pub sold_token: String,
    pub sold_amount: u64,
    pub bought_token: String,
    pub bought_amount: u64,
    pub price: f64,
    pub slippage_bps: u16,
    pub timestamp: i64,
}

#[event]
pub struct PauseToggled {
    pub is_paused: bool,
    pub timestamp: i64,
}

#[event]
pub struct ConfigUpdated {
    pub parameter: String,
    pub new_value: String,
    pub timestamp: i64,
}
