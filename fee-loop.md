# MOLTBALL: Fee Loop Configuration & Operation

## Overview

The fee loop is the core mechanism. Creator fees accumulate in a vault, and an off-chain keeper executes buybacks and LP provisioning on a fixed cadence.

## Configuration Parameters

```json
{
  "CHAIN": "solana",
  "RPC_URL": "https://api.mainnet-beta.solana.com",
  "RPC_TIMEOUT_MS": 10000,
  
  "CYCLE_SECONDS": 600,
  "MONITOR_INTERVAL_SECONDS": 30,
  
  "MIN_SWAP_SOL": 0.01,
  "MAX_SPEND_SOL": 1.0,
  "MAX_SLIPPAGE_BPS": 200,
  "COOLDOWN_SECONDS": 300,
  
  "SPLIT_BUYBACK": 50,
  "SPLIT_LP": 40,
  "SPLIT_BURN": 10,
  
  "JUPITER_BASE_URL": "https://quote-api.jup.ag/v6",
  "AUTO_MODE": true,
  
  "KEEPER_PRIVATE_KEY": "PLACEHOLDER_NOT_FOR_PRODUCTION",
  "VAULT_PROGRAM_ID": "PLACEHOLDER_PROGRAM_ADDRESS",
  "VAULT_PDA": "PLACEHOLDER_VAULT_PDA",
  
  "LOG_DIR": "logs/",
  "RECEIPTS_DIR": "receipts/"
}
```

### Parameter Definitions

| Parameter | Type | Default | Notes |
|---|---|---|---|
| CYCLE_SECONDS | int | 600 | How often keeper checks vault (in seconds) |
| MIN_SWAP_SOL | float | 0.01 | Minimum balance before attempting swap |
| MAX_SPEND_SOL | float | 1.0 | Max spend per cycle (prevents large single slippage) |
| MAX_SLIPPAGE_BPS | int | 200 | Max acceptable slippage in basis points (2% = 200 bps) |
| COOLDOWN_SECONDS | int | 300 | Min time between consecutive swaps |
| SPLIT_BUYBACK | int | 50 | % of fees used for buyback (0-100) |
| SPLIT_LP | int | 40 | % of fees used for LP provisioning (0-100) |
| SPLIT_BURN | int | 10 | % of fees burned (0-100) |
| AUTO_MODE | bool | true | If false, keeper only logs; does not execute swaps |

**Note**: SPLIT_BUYBACK + SPLIT_LP + SPLIT_BURN should equal 100.

---

## Execution Cycle

### Phase 1: Poll & Guard (every CYCLE_SECONDS)

```
[Keeper Start]
  │
  ├─ Read FeeVault state from RPC
  │   - fee_balance_sol
  │   - fee_balance_usdc
  │   - is_paused
  │   - last_swap_time
  │
  ├─ Check: Is paused?
  │   YES → Log & sleep
  │   NO  → continue
  │
  ├─ Check: Balance sufficient?
  │   if (balance_sol < MIN_SWAP_SOL) → sleep
  │   ELSE → continue
  │
  ├─ Check: Cooldown elapsed?
  │   if (now - last_swap_time < COOLDOWN_SECONDS) → sleep
  │   ELSE → continue
  │
  └─ Proceed to Phase 2
```

### Phase 2: Calculate & Quote

```
[Calculate]
  swap_amount = min(vault_balance, MAX_SPEND_SOL)
  buyback_amount = swap_amount * (SPLIT_BUYBACK / 100)
  lp_amount = swap_amount * (SPLIT_LP / 100)
  burn_amount = swap_amount * (SPLIT_BURN / 100)

[Fetch Quote from Jupiter]
  GET /quote
    ?inputMint=SOL
    &outputMint=$MOLTBALL_MINT
    &amount=<buyback_amount_in_lamports>
    &slippageBps=<MAX_SLIPPAGE_BPS>
  
  Response: {
    inAmount: string,
    outAmount: string,
    priceImpactPct: string,
    ...
  }

[Validate Quote]
  if (price_impact > MAX_SLIPPAGE_BPS) {
    LOG "Slippage too high, skipping"
    SLEEP
  }
  ELSE continue to Phase 3
```

### Phase 3: Execute

```
[Build Transaction]
  - Jupiter swap instruction (SOL → $MOLTBALL)
  - record_swap_receipt instruction (to FeeVault program)
  - (optional) LP add instruction
  - (optional) Burn instruction

[Sign & Submit]
  - Sign with keeper keypair
  - Submit to RPC
  - Wait for confirmation (or timeout)

[Log Receipt]
  - Write to keeper.log (human readable)
  - Write to receipts/<timestamp>.json (structured data)
  - Emit on-chain event (via record_swap_receipt)
```

### Phase 4: Monitor

```
[Check Result]
  if (tx confirmed) {
    last_swap_time = now
    fee_balance -= swap_amount (on next poll)
    Log: "Swap executed: sold X SOL, bought Y $MOLTBALL, price Z"
  }
  
  if (tx failed) {
    Log error with reason
    If slippage reason → wait longer before retry
    If RPC error → retry with backoff
    If permission error → alert (keeper not authorized)
  }

[Sleep until next cycle]
  SLEEP(CYCLE_SECONDS)
  GOTO Phase 1
```

---

## Thresholds & Guardrails

### Balance Thresholds
- **MIN_SWAP_SOL**: Prevents tiny swaps (inefficient gas)
- **MAX_SPEND_SOL**: Caps impact per swap (MEV protection)
- **Combined effect**: Smooth, predictable market impact

### Execution Windows
- **COOLDOWN_SECONDS**: Prevents rapid consecutive swaps (MEV, slippage)
- **CYCLE_SECONDS**: Time between cycle checks (usually longer than cooldown)
- **Combined effect**: Spreads execution, reduces sandwich risk

### Slippage Guards
- **MAX_SLIPPAGE_BPS**: Reject swaps worse than this slippage %
- **Enforced by**: Jupiter quote validation before submission
- **Combined effect**: Protects against extreme market moves

### Emergency Controls
- **PAUSE**: Governance can pause the entire fee loop
- **Effect**: Keeper skips all swaps until unpaused
- **Recovery**: Manual unpause or time-lock (TBD)

---

## Swap Mechanics

### Jupiter Integration

The keeper uses the Jupiter API to get real-time swap quotes:

```typescript
// Pseudo-code
const quote = await fetch(`${JUPITER_BASE_URL}/quote`, {
  inputMint: 'SOL',
  outputMint: MOLTBALL_MINT,
  amount: buyback_amount,
  slippageBps: MAX_SLIPPAGE_BPS
});

if (quote.priceImpactPct > (MAX_SLIPPAGE_BPS / 10000)) {
  // Reject swap
  return;
}

// Execute via Jupiter router
const swapTx = await jupiterApi.buildSwapTransaction({
  quote,
  userPublicKey: keeperKeypair.publicKey
});
```

### Slippage Calculation

```
Price Impact (%) = (Expected Output - Actual Output) / Expected Output * 100
Slippage BPS = Price Impact % * 100

Example:
  Expected: 1000 $MOLTBALL
  Actual: 980 $MOLTBALL
  Slippage: (1000 - 980) / 1000 * 100 = 2%
  Slippage BPS: 2 * 100 = 200 bps
  
If MAX_SLIPPAGE_BPS = 200, this swap is accepted (at limit).
If MAX_SLIPPAGE_BPS = 100, this swap is rejected.
```

---

## Liquidity Provisioning

### When to Add LP
- If SPLIT_LP > 0 AND balance_sol sufficient
- After buyback is executed successfully
- With randomized delay to avoid MEV

### How to Add LP
```
[Calculate LP amount]
  sol_for_lp = vault_balance * (SPLIT_LP / 100)
  moltball_for_lp = (moltball_from_previous_swap * percentage) || purchase new

[Add liquidity to DEX]
  - Pair: SOL / $MOLTBALL
  - DEX: Raydium or Orca (configurable)
  - Slippage tolerance: 2% (configurable)
  - Recipient: Treasury or LP locker (TBD)

[Record on-chain]
  - Emit LPAdded event to FeeVault
  - Log LP tokens received
  - Track cumulative LP managed
```

### Burn Logic
- If SPLIT_BURN > 0, after LP is added:
  - Calculate burn amount from remaining fee balance
  - Call burn instruction on token program
  - Log burn on-chain

---

## Example Cycle

**Assume config**:
```
CYCLE_SECONDS = 600 (10 min)
MIN_SWAP_SOL = 0.05
MAX_SPEND_SOL = 0.5
MAX_SLIPPAGE_BPS = 200
SPLIT_BUYBACK = 50, SPLIT_LP = 40, SPLIT_BURN = 10
```

**Hour 0:00 (cycle 1)**:
```
Vault balance: 0.02 SOL
Check: 0.02 < 0.05 (MIN_SWAP_SOL)
Action: SKIP, sleep 10 min
```

**Hour 0:10 (cycle 2)**:
```
Vault balance: 0.08 SOL (accumulated trading fees)
Check: 0.08 >= 0.05 ✓
Check: no pause ✓
Check: cooldown elapsed ✓
Calculation:
  swap_amount = min(0.08, 0.5) = 0.08
  buyback = 0.08 * 0.5 = 0.04 SOL
  lp = 0.08 * 0.4 = 0.032 SOL
  burn = 0.08 * 0.1 = 0.008 SOL
Jupiter quote: 0.04 SOL → 2000 $MOLTBALL (price impact 0.5% ✓)
Action: Execute swap, add LP, burn tokens
Vault balance: 0 SOL (+ 2000 $MOLTBALL received)
Log: "2026-01-26T10:10:00Z | Swap: sold 0.04 SOL, bought 2000 $MOLTBALL @ $20K SOL/BASE | LP: added 0.032 SOL | Burn: 8 MOLTBALL"
Sleep 10 min
```

**Hour 0:20 (cycle 3)**:
```
Vault balance: 0.06 SOL (continued trading)
Check: 0.06 >= 0.05 ✓
Check: cooldown elapsed (10 min > 5 min required) ✓
Repeat cycle...
```

---

## Monitoring & Alerting

### Keeper Logs (keeper.log)
```
2026-01-26T10:00:00Z | INFO | Cycle 1: balance 0.02 SOL, below MIN_SWAP_SOL (0.05), skipping
2026-01-26T10:10:00Z | INFO | Cycle 2: balance 0.08 SOL, executing swap
2026-01-26T10:10:15Z | INFO | Jupiter quote: 0.04 SOL → 2000 $MOLTBALL, impact 0.5%
2026-01-26T10:10:20Z | INFO | TX confirmed: ...signature...
2026-01-26T10:10:21Z | INFO | Swap executed: sold 0.04 SOL, bought 2000 $MOLTBALL, price impact 0.5%
2026-01-26T10:10:22Z | INFO | LP added: 0.032 SOL + 1000 $MOLTBALL
2026-01-26T10:10:23Z | INFO | Burned: 8 $MOLTBALL
2026-01-26T10:20:00Z | INFO | Cycle 3: ...
```

### Receipts (receipts/<timestamp>.json)
```json
{
  "timestamp": "2026-01-26T10:10:20Z",
  "cycle": 2,
  "vault_balance_before_sol": 0.08,
  "vault_balance_before_usdc": 0.0,
  "swap": {
    "sold_token": "SOL",
    "sold_amount": 0.04,
    "bought_token": "$MOLTBALL",
    "bought_amount": 2000,
    "price": 20000,
    "slippage_bps": 50,
    "execution_timestamp": "2026-01-26T10:10:20Z",
    "signature": "...tx signature..."
  },
  "lp": {
    "token_a": "SOL",
    "amount_a": 0.032,
    "token_b": "$MOLTBALL",
    "amount_b": 1000,
    "lp_tokens_received": 100
  },
  "burn": {
    "amount": 8,
    "token": "$MOLTBALL"
  }
}
```

---

## Troubleshooting

### "Balance below MIN_SWAP_SOL"
- Wait for more trading to accumulate fees
- Check trading volume and fee rate
- Verify FeeVault deposit instruction is being called correctly

### "Slippage too high, skipping"
- Market volatility is high
- Liquidity pool is shallow
- Try reducing MAX_SPEND_SOL (lower impact per swap)
- Or increase MAX_SLIPPAGE_BPS tolerance (not recommended)

### "RPC timeout"
- Check RPC_URL is reachable
- Increase RPC_TIMEOUT_MS
- Use fallback RPC endpoint
- Check network status

### "Keeper not authorized"
- Verify keeper public key matches VAULT_PDA's keeper_role
- Check signer is the keeper keypair
- Re-initialize vault with correct keeper if needed

### "TX failed: insufficient balance"
- Keeper keypair has insufficient SOL for fees
- Fund keeper keypair with more SOL
- Check on-chain rent for transaction

---

## Security Checklist

- [ ] Keypair stored securely (not in version control)
- [ ] RPC_URL uses HTTPS
- [ ] MAX_SPEND_SOL set reasonably (not too large)
- [ ] MAX_SLIPPAGE_BPS set to acceptable tolerance
- [ ] COOLDOWN_SECONDS prevents rapid re-entry
- [ ] Logs monitored for errors
- [ ] Receipts exported for external audit
- [ ] Emergency pause accessible to guardian
- [ ] Config parameters reviewed and approved
