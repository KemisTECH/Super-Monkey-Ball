# MOLTBALL: Tokenomics (Framework)

## Token Specification

| Property | Value | Notes |
|---|---|---|
| Name | MOLTBALL | |
| Symbol | $MOLTBALL | Memetic simplicity |
| Chain | Solana | |
| Standard | SPL-22 (SPL Token with fee extension) | Creator fees enabled |
| Total Supply | TBD | Placeholder—no pre-mine claim |
| Decimals | 6 | Standard for Solana |
| Mint Authority | TBD | Renounce post-launch or community DAO |

## Fee Structure

### Creator Fee (on-chain fee switch)
- **Mechanism**: Enabled via SPL token extension (fee switch)
- **Rate**: Configurable; typically 0.5% - 2.0% per side
- **Collection**: Automatic to FeeVault PDA on every trade
- **Custody**: Non-custodial (contract owns vault, not any individual)

### Holder Impact
**No extra tax on holders.** The creator fee is built into the DEX fee-switch mechanism used by trading platforms. Holders pay normal trading slippage and platform fees, not an additional MOLTBALL-specific tax.

Example:
- User buys 1000 $MOLTBALL on Raydium
- Raydium collects 1% creator fee (10 $MOLTBALL to vault)
- User receives ~990 $MOLTBALL
- User is not charged a separate "holder tax"

## Fee Allocation Loop

When the FeeVault reaches a threshold:

```
[Creator Fee Accumulation]
         ↓
   SOL / USDC
         ↓
    [FeeVault]
    (balance: X SOL)
         ↓
    [Keeper Check]
    Balance > MIN?
         ↓
    [Fee Split]
         ├─ 50% → Buyback ($MOLTBALL)
         ├─ 40% → LP Provisioning
         └─ 10% → Burn (optional)
         ↓
    [Market Action]
    Buy $MOLTBALL, add LP, burn
         ↓
    [On-Chain Receipt]
    Log all actions for transparency
```

## Buyback Mechanism

- **Frequency**: Every cycle (configurable, e.g., 10 minutes)
- **Amount**: (FeeVault balance × SPLIT_BUYBACK) up to MAX_SPEND_SOL
- **Method**: Market buy via Jupiter router
- **Destination**: TBD (treasury, burn, or holder dividend wallet)
- **Guard**: Slippage cap (default 2% max)

## Liquidity Provisioning

- **When**: If fee balance sufficient AND SPLIT_LP > 0
- **How**: Pair $MOLTBALL with SOL (or USDC), add to Raydium/Orca pool
- **Amount**: (FeeVault balance × SPLIT_LP)
- **Timing**: Randomized execution window to avoid MEV
- **Risk**: LP can be impermanent loss; we use strict thresholds

## Burn Strategy

- **Optional**: Configured via SPLIT_BURN (default 0%)
- **Method**: Mint authority renounces or burns via instruction
- **Transparency**: All burns logged on-chain
- **Reasoning**: Selective supply reduction; not mandatory

---

## Distribution (Placeholder)

**Important**: The following is a template. Actual distribution TBD.

| Allocation | % | Amount | Vesting |
|---|---|---|---|
| Community/Airdrop | 30% | TBD | None / linear over 6 months |
| Team | 15% | TBD | 1 year cliff, 3 year vest |
| Treasury | 30% | TBD | Managed by keeper loop |
| Liquidity (DEX) | 20% | TBD | Locked in LP |
| Marketing/Partnerships | 5% | TBD | TBD |

**Note**: No pre-mine guarantees. Any announced distribution is for reference only and subject to change.

---

## Economics Model (Illustrative)

Assume:
- Initial supply: 1B $MOLTBALL
- Trading volume: $10M/day (speculative)
- Creator fee: 1% both sides
- Daily fee collected: ~$200K (in SOL equivalent)

Scenario:
```
Day 1: Collect 0.1 SOL (for simplicity)
Trigger: Threshold = 0.05 SOL reached
Split: 50% buyback (0.05 SOL), 40% LP (0.04 SOL), 10% burn (0.01 SOL)

Market impact (illustrative):
- Buyback: 0.05 SOL → buy ~2.5K $MOLTBALL (at $20K/SOL)
- LP: 0.04 SOL + proportional $MOLTBALL → add liquidity
- Burn: 0.01 SOL worth → burn equivalent $MOLTBALL

Result: Increased liquidity, reduced supply, price support
```

**Caution**: This is a theoretical example. Actual market conditions, liquidity depth, and execution will vary.

---

## Governance (Future)

- **Early Phase**: Keeper controlled by core team (transparent parameters)
- **Medium Term**: Migration to DAO (vote on fee splits, thresholds, keeper)
- **Long Term**: Community treasury management

---

## Disclaimer

**Tokenomics are subject to change.** This document is a framework, not a guarantee. MOLTBALL is an experimental token. Market conditions, trading volume, and token price are unpredictable and outside our control. Do not make financial decisions based on tokenomics projections.
