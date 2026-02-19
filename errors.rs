use anchor_lang::prelude::*;

#[error_code]
pub enum FeeVaultError {
    #[msg("Unauthorized")]
    Unauthorized,
    
    #[msg("Vault is paused")]
    VaultPaused,
    
    #[msg("Invalid config")]
    InvalidConfig,
    
    #[msg("Insufficient balance")]
    InsufficientBalance,
    
    #[msg("Invalid role")]
    InvalidRole,
    
    #[msg("Overflow")]
    Overflow,
}
