# MOLTBALL: Architecture & Data Flow

## High-Level Data Flow

```
┌──────────────────────────────────────────────────────────────────┐
│                       SOLANA BLOCKCHAIN                          │
├──────────────────────────────────────────────────────────────────┤
│                                                                  │
│  ┌─────────────────────────┐         ┌─────────────────────┐   │
│  │   MOLTBALL Token (SPL)  │         │  DEX Liquidity Pool │   │
│  │   - Supply: TBD         │         │  - Pair: SOL/$BASE  │   │
│  │   - Fee Switch: 1% both │         │ - Managed by LP Bot │   │
│  └─────────────────────────┘         └─────────────────────┘   │
│           ↓ (trading happens)                ↑                  │
│           │                           (liquidity added here)   │
│  ┌──────────────────────────────────────────────────┐          │
│  │         FeeVault Program (Anchor)                │          │
│  ├──────────────────────────────────────────────────┤          │
│  │ State:                                           │          │
│  │  - fee_balance_sol: u64                          │          │
│  │  - fee_balance_usdc: u64                         │          │
│  │  - vault_authority: Pubkey                       │          │
│  │  - keeper_role: Pubkey                           │          │
│  │  - pause_guardian: Pubkey                        │          │
│  │  - config: {...}                                 │          │
│  │                                                  │          │
│  │ Instructions:                                    │          │
│  │  - initialize(admin, keeper, config)            │          │
│  │  - deposit_fees(amount_sol, amount_usdc)        │          │
│  │  - record_swap_receipt(amounts, prices, ts)    │          │
│  │  - set_config(new_params) [admin only]         │          │
│  │  - emergency_pause() [pause_guardian only]      │          │
│  │  - record_lp_addition(sol, base, lp_tokens)    │          │
│  │                                                  │          │
│  │ Events:                                          │          │
│  │  - FeeDeposited { sol, usdc, ts }               │          │
│  │  - SwapExecuted { sold, bought, price, ts }    │          │
│  │  - PauseToggled { is_paused, ts }               │          │
│  │  - ConfigUpdated { key, new_value, ts }         │          │
│  └──────────────────────────────────────────────────┘          │
│           ↑                                                      │
│           │ (keeper submits tx with receipts)                   │
│           │ (role-checked on-chain)                             │
└──────────────────────────────────────────────────────────────────┘
           ↑
           │
       ┌───┴────────────────────────────────────────────────────┐
       │      OFF-CHAIN KEEPER (Node.js)                        │
       ├────────────────────────────────────────────────────────┤
       │                                                         │
       │  Main Loop (every CYCLE_SECONDS):                      │
       │  1. Poll Solana RPC → read FeeVault state             │
       │  2. Decode balances (fee_balance_sol, usdc)           │
       │  3. Apply thresholds & guards:                         │
       │     - Is paused? → skip                                │
       │     - Is balance > MIN_SWAP_SOL? → continue           │
       │     - Time since last swap > COOLDOWN? → continue     │
       │     - Cycle budget available? → continue              │
       │  4. Calculate splits:                                  │
       │     - swap_amt = min(balance, MAX_SPEND_SOL)         │
       │     - buyback_amt = swap_amt * SPLIT_BUYBACK         │
       │     - lp_amt = swap_amt * SPLIT_LP                   │
       │     - burn_amt = swap_amt * SPLIT_BURN               │
       │  5. Fetch Jupiter quote:                               │
       │     - GET /quote?...                                  │
       │     - Check min_out vs slippage cap                   │
       │     - Revert if slippage > MAX_SLIPPAGE_BPS           │
       │  6. Execute on-chain:                                  │
       │     - Build transaction with Jupiter swap instruction  │
       │     - Add receipt instruction to FeeVault             │
       │     - Sign with keeper keypair                         │
       │     - Submit to RPC                                    │
       │  7. Log results:                                       │
       │     - keeper.log (human readable)                      │
       │     - receipts/<timestamp>.json (data export)          │
       │                                                         │
       │  Error Handling:                                       │
       │     - RPC error → log & skip cycle                     │
       │     - Slippage too high → skip swap                    │
       │     - Balance too low → wait for next deposit          │
       │     - Rate limit → backoff exponentially              │
       │                                                         │
       │  Monitoring:                                           │
       │     - Emit logs every MONITOR_INTERVAL                │
       │     - Track up-time, tx count, last execution         │
       │                                                         │
       └────────────────────────────────────────────────────────┘
```

## Component Breakdown

### FeeVault Program (On-Chain)

**Responsibilities**:
- Store fees in a PDA vault account
- Emit events for each deposit, swap, and config change
- Enforce role-based access control
- Log swap receipts with amounts, prices, and timestamps
- Support emergency pause

**Key Accounts**:
- `fee_vault`: PDA that holds SOL/USDC
- `vault_authority`: Signer that deploys/owns the vault
- `keeper_role`: Account that can submit swap receipts
- `pause_guardian`: Account that can toggle pause

**Key Instructions**:
```
initialize(
  vault_authority: Signer,
  keeper: Pubkey,
  pause_guardian: Pubkey,
  config: ConfigParams
) -> Result<()>

deposit_fees(
  from: Signer,
  amount_sol: u64,
  amount_usdc: u64
) -> Result<()>

record_swap_receipt(
  keeper: Signer,
  sold_token: Token,
  sold_amount: u64,
  bought_token: Token,
  bought_amount: u64,
  execution_timestamp: i64
) -> Result<()>

set_config(
  admin: Signer,
  new_config: ConfigParams
) -> Result<()>

emergency_pause(pause_guardian: Signer) -> Result<()>
```

### Keeper Bot (Off-Chain)

**Responsibilities**:
- Poll vault state every N seconds
- Apply guardrails (thresholds, cooldowns, limits)
- Fetch swap quotes from Jupiter
- Submit signed transactions to the chain
- Log receipts locally and emit on-chain events

**Key Data Structures**:
```typescript
interface VaultState {
  feeSol: BigNumber;
  feeUsdc: BigNumber;
  lastSwapTime: number;
  isPaused: boolean;
  config: Config;
}

interface SwapReceipt {
  signature: string;
  timestamp: number;
  sold_token: 'SOL' | 'USDC';
  sold_amount: number;
  bought_token: string;
  bought_amount: number;
  price: number;
  slippage_bps: number;
}

interface Config {
  cycle_seconds: number;
  min_swap_sol: number;
  max_spend_sol: number;
  max_slippage_bps: number;
  split_buyback: number; // 0-100
  split_lp: number;      // 0-100
  split_burn: number;    // 0-100
  cooldown_seconds: number;
}
```

## Execution Flow Diagram

```
START (every CYCLE_SECONDS)
  │
  ├─> Read FeeVault state
  │     vault_sol = decode_u64(vault.fee_balance_sol)
  │     vault_usdc = decode_u64(vault.fee_balance_usdc)
  │     is_paused = vault.is_paused
  │
  ├─> Guard: Is Paused?
  │     YES → LOG & SKIP
  │     NO  → continue
  │
  ├─> Guard: Balance Sufficient?
  │     if (vault_sol < config.min_swap_sol) → SKIP
  │     ELSE → continue
  │
  ├─> Guard: Cooldown Elapsed?
  │     if (now - last_swap_time < config.cooldown_seconds) → SKIP
  │     ELSE → continue
  │
  ├─> Calculate Amounts
  │     swap_amt = min(vault_sol, config.max_spend_sol)
  │     buyback_amt = swap_amt * (config.split_buyback / 100)
  │     lp_amt = swap_amt * (config.split_lp / 100)
  │     burn_amt = swap_amt * (config.split_burn / 100)
  │
  ├─> Fetch Jupiter Quote
  │     GET /quote?inputMint=SOL&outputMint=$MOLTBALL&amount=buyback_amt
  │     Quote returns: min_out, price, slippage
  │
  ├─> Guard: Slippage Check
  │     if (slippage > config.max_slippage_bps) → LOG & SKIP
  │     ELSE → continue
  │
  ├─> Build Transaction
  │     [1] Jupiter swap instruction (SOL → $MOLTBALL)
  │     [2] record_swap_receipt instruction to FeeVault
  │     [3] (optional) LP instruction
  │     [4] (optional) Burn instruction
  │
  ├─> Sign & Submit
  │     keeper_keypair.sign(tx)
  │     submit to RPC
  │     wait for confirmation
  │
  ├─> Log Receipt
  │     keeper.log: "SWAP: sold 0.5 SOL, bought X $MOLTBALL at price Y"
  │     receipts/<ts>.json: full JSON record
  │
  ├─> Error Handler
  │     If RPC fails → exponential backoff
  │     If slippage too high → mark & skip
  │     If keeper unpermissioned → alert
  │
  └─> SLEEP until next cycle
```

## Security Model

### Role Separation
- **Vault Owner**: Initializes contract, owns PDA
- **Keeper Role**: Only account authorized to submit swap receipts
- **Pause Guardian**: Only account that can toggle emergency pause
- **Admin**: Can update config parameters (optional multisig)

### On-Chain Guards
- Slippage caps (per swap)
- Max spend per cycle
- Cooldown between swaps
- Minimum balance thresholds
- Event logging for audit trail

### Off-Chain Guards
- RPC timeout handling
- Fetch quote validation
- Local receipt logging
- Monitoring & alerting
- Keypair rotation (optional)

## Failure Modes & Mitigations

| Failure Mode | Impact | Mitigation |
|---|---|---|
| Keeper downtime | Fee loop stalls | Monitoring alert, manual re-start, redundant keeper in production |
| RPC unavailable | Cannot read state | Fallback RPC endpoints, retry logic with backoff |
| Jupiter API down | Cannot get quotes | Use fallback DEX, manual pause |
| Slippage spike | Swap reverted | Guard on maxSlippage, monitor on-chain |
| Keypair compromised | Unauthorized swaps | Rotate keypair, use multisig, emergency pause |
| Smart contract bug | Fund loss | Code audit, testnet deployment, gradual rollout |

## Monitoring & Observability

**Keeper logs**:
- Every cycle: timestamp, balance, threshold checks, decision
- Every swap: amount, quote, slippage, signature, success/fail
- Errors: RPC, slippage, authorization, parsing

**On-chain events**:
- FeeDeposited: external funding to vault
- SwapExecuted: receipt log from keeper
- ConfigUpdated: parameter changes
- PauseToggled: emergency state

**Metrics to track**:
- Uptime %
- Swap frequency (swaps per day)
- Slippage observed (bps)
- Fund utilization (% of vault spent)
- Error rate
