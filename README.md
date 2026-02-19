# Super Monkey Ball

<p align="center">
  <img src="./supermonkeyball.jpg" width="600" />
</p>


This is the on-chain Solana program (smart contract) for Super Monkey Ball's fee vault.

## What it does

- Stores creator fees (SOL/USDC)
- Enforces role-based access control
- Logs swap receipts and events
- Supports emergency pause
- Manages configuration parameters

## Build & Test

```bash
# Install Anchor
cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
avm install latest
avm use latest

# Build
anchor build

# Test
anchor test

# Deploy to devnet
anchor deploy --provider.cluster devnet
```

## Program Structure

```
src/
  ├─ lib.rs (entry point, program ID)
  ├─ instructions/
  │   ├─ initialize.rs
  │   ├─ deposit_fees.rs
  │   ├─ record_swap_receipt.rs
  │   ├─ set_config.rs
  │   └─ pause.rs
  ├─ state/
  │   ├─ vault.rs
  │   ├─ config.rs
  │   └─ event.rs
  └─ errors.rs
```

## Key Accounts

| Account | Type | Purpose |
|---|---|---|
| `fee_vault` | PDA | Stores SOL/USDC fees |
| `vault_authority` | Signer | Owner of vault |
| `keeper` | Role | Account authorized to submit swap receipts |
| `pause_guardian` | Role | Account that can pause/unpause |

## Key Instructions

### initialize
```rust
pub struct Initialize {
    pub payer: Signer<'info>,
    pub vault_authority: Signer<'info>,
    pub fee_vault: Account<'info, FeeVault>,
    pub system_program: Program<'info, System>,
}
```

### deposit_fees
```rust
pub struct DepositFees {
    pub depositor: Signer<'info>,
    pub fee_vault: Account<'info, FeeVault>,
}
```

### record_swap_receipt
```rust
pub struct RecordSwapReceipt {
    pub keeper: Signer<'info>,
    pub fee_vault: Account<'info, FeeVault>,
}
```

### set_config
```rust
pub struct SetConfig {
    pub admin: Signer<'info>,
    pub fee_vault: Account<'info, FeeVault>,
}
```

### pause/unpause
```rust
pub struct Pause {
    pub pause_guardian: Signer<'info>,
    pub fee_vault: Account<'info, FeeVault>,
}
```

## State

### FeeVault
```rust
pub struct FeeVault {
    pub vault_authority: Pubkey,
    pub keeper_role: Pubkey,
    pub pause_guardian: Pubkey,
    pub fee_balance_sol: u64,
    pub fee_balance_usdc: u64,
    pub is_paused: bool,
    pub config: Config,
    pub last_swap_time: i64,
    pub bump: u8,
}
```

### Config
```rust
pub struct Config {
    pub cycle_seconds: u32,
    pub min_swap_sol: u64,
    pub max_spend_sol: u64,
    pub max_slippage_bps: u16,
    pub split_buyback: u8,
    pub split_lp: u8,
    pub split_burn: u8,
    pub cooldown_seconds: u32,
}
```

## Events

All events are emitted via program logs (parseable).

```rust
pub event FeeDeposited {
    pub depositor: Pubkey,
    pub amount_sol: u64,
    pub amount_usdc: u64,
    pub timestamp: i64,
}

pub event SwapExecuted {
    pub sold_token: String,
    pub sold_amount: u64,
    pub bought_token: String,
    pub bought_amount: u64,
    pub price: f64,
    pub slippage_bps: u16,
    pub timestamp: i64,
}

pub event PauseToggled {
    pub is_paused: bool,
    pub timestamp: i64,
}

pub event ConfigUpdated {
    pub key: String,
    pub new_value: String,
    pub timestamp: i64,
}
```

## Security

- **Role-based access**: Only authorized accounts can submit swaps
- **Event logging**: All actions logged for audit
- **Guard rails**: Slippage caps, max spend limits, cooldowns
- **Emergency pause**: Guardian can halt operations

## Testing Checklist

- [ ] Initialize vault with valid params
- [ ] Initialize vault with invalid params (should fail)
- [ ] Deposit fees (check balance updated)
- [ ] Record swap receipt (check event emitted)
- [ ] Set config (admin only)
- [ ] Set config (non-admin fails)
- [ ] Pause/unpause (guardian only)
- [ ] Verify role separation
- [ ] Verify bump seed
- [ ] Fuzzing for overflow/underflow

## Audit Notes

- Consider: Can keeper transfer funds without swap?
- Consider: Are all role checks enforced?
- Consider: Is there reentrancy risk?
- Consider: Are events reliably parseable?
