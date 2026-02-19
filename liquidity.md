# MOLTBALL: Liquidity & LP Strategy

## Liquidity Pools

MOLTBALL is designed to be tradeable on Solana DEXes that support the fee switch mechanism.

### Supported DEXes
- **Raydium**: Primary venue (strongest liquidity)
- **Orca**: Secondary option (whirlpools)
- **Jupiter**: Aggregator used for quotes and execution

### Pair Structure
```
Primary:  SOL / $MOLTBALL
Secondary: USDC / $MOLTBALL (optional)
```

## LP Strategy

### Goals
1. Ensure deep, stable liquidity for trading
2. Support price discovery
3. Enable the fee loop to function (swaps need liquidity)

### Auto-LP from Fee Loop

The fee loop can optionally add liquidity:

```
[Fee accumulation]
  ↓
[Split: buyback + LP + burn]
  ↓
[Execute buyback swap]
  ↓
[Use swapped $MOLTBALL + reserve SOL for LP]
  ↓
[Add to Raydium pool]
  ↓
[Receive LP tokens]
  ↓
[Lock or manage LP tokens]
```

### Parameters

| Parameter | Default | Notes |
|---|---|---|
| SPLIT_LP | 40% | Of vault fees allocated to LP |
| LP_SLIPPAGE_BPS | 200 | Max slippage when adding LP |
| LP_EXECUTION_DELAY | 60 | Seconds to wait before adding LP (MEV avoidance) |
| LP_LOCK_DURATION | 6 months | (optional) If LP tokens are locked |

### Process

**Step 1: Buyback**
- Keeper executes swap: SOL → $MOLTBALL
- Receives X $MOLTBALL

**Step 2: Prepare LP**
- SOL for LP: vault_balance * SPLIT_LP
- MOLTBALL for LP: X * (SPLIT_LP / SPLIT_LP+SPLIT_BURN)

**Step 3: Add to Pool**
```typescript
// Pseudo-code
const lpTx = await raydiumClient.addLiquidity({
  tokenA: 'SOL',
  tokenB: 'MOLTBALL_MINT',
  amountA: solForLp,
  amountB: baseForLp,
  slippageTolerance: LP_SLIPPAGE_BPS,
  recipient: treasuryKey
});
```

**Step 4: Record**
- Log LP tokens received on-chain
- Emit LPAdded event
- Store receipt in keeper logs

### Impermanent Loss Mitigation

Adding LP exposes the protocol to **impermanent loss** (IL):
- If $MOLTBALL price rises sharply after LP is added, IL increases
- If price falls, IL also increases (symmetric)

Mitigations:
1. **Small, frequent additions** (vs. large, infrequent)
2. **Balanced swaps** (use fee loop buyback to balance both sides)
3. **Monitor and rebalance** (off-chain analysis of IL)
4. **Time-lock LP** (optional; prevents panic unlocks)

## Liquidity Incentives (Future)

If liquidity becomes insufficient:

1. **LP Rewards**: Allocate a % of fees to LP token holders
2. **Token Emissions**: Mint additional $MOLTBALL for LP incentives
3. **Partnership**: Work with DEX aggregators for preferred routing

## Market-Making via Fee Loop

The fee loop itself acts as a **soft market-maker**:

```
Persistent buyback demand (from fee loop)
  → Supports price floor
  → Encourages trading
  → Generates more fees
  → Increases buyback demand
  → [Positive feedback loop]
```

This is **not** true market-making (which requires two-sided liquidity) but rather a **buyback-driven support mechanism**.

### Limitations
- If trading volume collapses, fee loop also stops
- Buyback alone cannot sustain price against large selling
- MEV can disrupt the loop (front-running, sandwiching)

## Liquidity Risks

| Risk | Impact | Mitigation |
|---|---|---|
| Shallow liquidity | High slippage on swaps | Monitor depth, adjust MAX_SPEND_SOL, market education |
| IL from LP | Reduced LP value over time | Accept as cost of liquidity provision, monitor & rebalance |
| Pool sandwich attack | Keeper swap front-run/back-run | Use randomized execution windows, slippage caps |
| Liquidity withdrawal | Pool shrinks after LP added | Monitor on-chain LP token locks, communicate with community |
| Market crash | LP value collapses | Accept market risk, do not over-allocate to LP |

## Monitoring Liquidity

### Metrics to Track
```typescript
interface LiquidityMetrics {
  poolReserveSOL: number;
  poolReserveBABASE: number;
  tradingVolume24h: number;
  feeAPY: number; // annual yield to LP
  impermanentLoss: number; // %
  lastSwapSlippage: number; // bps
  lpTokensHeld: number;
  totalValueLocked: number; // USD equivalent
}
```

### Queries
```sql
-- Raydium pool stats (pseudo-SQL)
SELECT 
  reserve_sol,
  reserve_moltball,
  total_volume_24h,
  fee_apy
FROM pools
WHERE pair = ('SOL', 'MOLTBALL_MINT');

-- Keeper execution history
SELECT 
  timestamp,
  swap_slippage_bps,
  lp_added_sol,
  lp_added_moltball
FROM receipts
WHERE type = 'SWAP'
ORDER BY timestamp DESC
LIMIT 30;
```

## Tokenomics Alignment

LP additions support tokenomics by:
- **Locking liquidity**: Reduces float temporarily
- **Supporting price**: Deeper pools → lower slippage → easier trading
- **Enabling growth**: More trading → more fees → more buyback

However, LP is a **liability**, not an asset:
- LP tokens can be diluted (if pool composition changes)
- IL reduces LP value over time
- LP tokens are not revenue (they're locked capital)

## Future LP Enhancements

1. **Concentrated liquidity** (Orca whirlpools): More efficient capital use
2. **Time-weighted LP locks**: Prevent sudden withdrawals
3. **Community LP incentives**: Reward external LP providers
4. **Dynamic splits**: Adjust SPLIT_LP based on market conditions
5. **Multi-pair support**: Add liquidity to USDC/$MOLTBALL if needed
