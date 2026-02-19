# MOLTBALL: Overview

## Executive Summary

**MOLTBALL** is an experiment in automated, transparent token fee management on Solana. It leverages creator fees (the "fee-switch" mechanism available on Solana's trading venues) to fund a continuous buyback and liquidity provisioning loop.

The core loop is:
1. Trading fees accumulate in a vault contract
2. An off-chain keeper monitors the vault
3. When thresholds are reached, the keeper executes:
   - SOL/USDC swap for buyback of $MOLTBALL
   - LP provisioning (optional)
   - Selective token burns (optional)
4. All receipts logged on-chain for transparency

## Why This Matters

Most tokens are **passive**: they collect fees and sit on them. MOLTBALL is **active**: fees are mechanically recycled to support price floor and liquidity.

From a protocol perspective:
- **Transparency**: Anyone can audit the vault and swap history on-chain
- **No rug risk**: Fees cannot be arbitrarily moved; they follow strict algorithmic rules
- **Meme potential**: The mechanism is easy to explain and visually interesting

## Architecture at a Glance

```
┌─────────────────────────────────────────┐
│   Solana Chain                          │
├─────────────────────────────────────────┤
│ [MOLTBALL Token SPL]                    │
│     ↓ (trading fees)                    │
│ [FeeVault Program]                      │
│   - Stores SOL/USDC fees                │
│   - Emits events & receipts             │
│   - Role-based access control           │
└─────────────────────────────────────────┘
          ↑
          │ (reads balances, emits transactions)
          │
┌─────────────────────────────────────────┐
│ Off-Chain Keeper Bot (Node.js)          │
├─────────────────────────────────────────┤
│ - Polls vault every 10 seconds          │
│ - Checks thresholds & guards            │
│ - Calls Jupiter for quotes              │
│ - Signs & submits transactions          │
│ - Logs receipts to file + chain         │
└─────────────────────────────────────────┘
```

## Key Components

### 1. On-Chain (Solana Program)
- **State**: Vault balances, config parameters, role assignments
- **Events**: Deposit logs, swap receipts, error events
- **Guards**: Rate limits, max spend per cycle, slippage caps, cooldowns
- **Access**: Role-based (keeper, admin, pause guardian)

### 2. Off-Chain Keeper
- Monitors vault state (SOL/USDC balance)
- Applies business logic (thresholds, splits)
- Fetches real-time quotes from Jupiter
- Submits signed transactions to the chain
- Records execution in local logs and on-chain events

### 3. Configuration
- Cycle interval (how often keeper runs)
- Max spend per cycle (prevents large single slippage)
- Slippage tolerance (% acceptable loss per swap)
- Fee splits (% to buyback, % to LP, % to burn)
- Cooldown between swaps
- Emergency pause

## Risk Acknowledgments

1. **Operational Risk**: Keeper downtime halts the loop. Mitigated by monitoring and multi-zone deployment.
2. **MEV Risk**: Swaps can be sandwiched. Mitigated by randomized execution windows and slippage caps.
3. **Liquidity Risk**: LP additions must be timed carefully. We avoid frontrunning by using DEX aggregators and delayed windows.
4. **Market Risk**: Token can go to zero. Not a risk mitigated by tech, only by market adoption.

## Disclaimer

MOLTBALL is early-stage research. It is **not** affiliated with Coinbase, any exchange, or any other project. The "Coinbase-era" narrative is editorial commentary on market timing, not a claim of partnership or endorsement.

Token trading is speculative and risky. MOLTBALL is an experiment in mechanism design, not an investment recommendation.
