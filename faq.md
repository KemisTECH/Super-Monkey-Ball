# MOLTBALL: Frequently Asked Questions

## General

### What is MOLTBALL?
MOLTBALL is a Solana token that uses creator fees to automatically market-make itself. Fees collected from trading are recycled into buybacks and liquidity provisioning, creating a closed loop.

### Why did you build MOLTBALL?
MOLTBALL is a response to a specific moment: Coinbase has made all Solana tokens tradeable, and the memetic opportunity is live. We built MOLTBALL to test whether a mechanically transparent, autonomous fee loop can become culturally significant.

### Is MOLTBALL affiliated with Coinbase?
**No.** MOLTBALL is an independent project. We are not endorsed by, approved by, or affiliated with Coinbase, Inc. The "Coinbase-era" framing is our market analysis, not a partnership claim.

### Is MOLTBALL safe?
MOLTBALL is experimental software. Like all tokens, it carries risk:
- Smart contract bugs (mitigated by audit, not eliminated)
- Keeper downtime (mitigated by monitoring, not eliminated)
- MEV and slippage (unavoidable on Solana)
- Market risk (token can go to zero)

**Do not invest money you cannot afford to lose.**

See [security.md](./security.md) for detailed threat model.

---

## Mechanism

### How do the fees work?
Creator fees are collected via Solana's fee-switch mechanism (available on Raydium, Orca, and other DEXes). Typically 0.5% - 2% per trade (both sides) goes to a vault.

### Is there a "holder tax"?
**No.** Holders do not pay an extra tax. The creator fee is baked into the DEX's fee structure. If you buy 1000 $MOLTBALL with 1% creator fee, you pay the creator fee at the DEX level (like any token on Raydium).

You do not pay a separate "holder tax" on top of trading slippage.

### How often does the keeper run?
Every 10 minutes by default (configurable as CYCLE_SECONDS). On each cycle, the keeper checks if:
- Vault has sufficient balance
- Enough time has passed since the last swap
- Slippage is acceptable

### What happens to the fees?
```
Creator fees → FeeVault → split into:
  50% buyback (market buy $MOLTBALL)
  40% LP provisioning (add liquidity to pool)
  10% burn (optional, reduce supply)
```

These splits are configurable.

### Why does the keeper need to be off-chain?
Solana programs (smart contracts) cannot make external HTTP requests or manage complex state dynamically. The keeper is off-chain to:
- Fetch real-time quotes from Jupiter
- Apply business logic (thresholds, splits)
- Submit transactions to the chain

The on-chain contract validates all keeper actions and enforces rules.

---

## Security

### What if the keeper goes offline?
If the keeper is offline for >1 hour, fees will accumulate in the vault but won't be recycled. An alert will be triggered, and the team (or community DAO) can restart it or investigate.

The vault is safe (funds are locked in a contract). But the fee loop stops.

### Can the team rug the project?
Rug resistance is built in:
- Fees cannot be arbitrarily withdrawn; they follow a specific split
- All swaps are recorded on-chain and auditable
- Pause can be triggered by a guardian role (separate from admin)
- Mint authority can be renounced (future)

However, this is not absolute. A compromised keeper keypair or smart contract bug could cause problems. This is why code audits and monitoring are critical.

### What if there's a smart contract bug?
Known mitigations:
- Code audit (pre-launch)
- Testnet deployment (30 days)
- Emergency pause (can halt operations)
- Gradual rollout (start with small vault, scale up)

Bugs cannot be fully eliminated, only reduced.

### How is the keeper keypair stored?
The keeper keypair should be:
- Encrypted at rest
- Not committed to version control
- Rotated regularly (or on-demand if suspected compromise)
- Stored on secure infra (AWS, GCP, or hardware wallet)

In production, consider a multisig or hardware wallet setup.

---

## Liquidity & Trading

### Where can I trade MOLTBALL?
MOLTBALL will be tradeable on:
- **Raydium** (primary)
- **Orca** (optional)
- **Jupiter** (routing)
- **Dexscreener** (charts)

### Why is slippage so high?
Slippage depends on:
- Pool depth (reserves of SOL and $MOLTBALL)
- Your trade size (larger trades = higher slippage)
- Market volatility

Early-stage tokens often have higher slippage. This decreases as liquidity deepens.

### Does the fee loop help liquidity?
Yes. The fee loop:
- Buys $MOLTBALL regularly (creates persistent demand)
- Adds liquidity (deepens the pool)
- Supports the price floor

However, this is not true market-making. If trading volume collapses, the fee loop also stops.

### Can the LP be rugged?
The keeper can add liquidity to the Raydium pool, but cannot remove it (that's controlled by LP token holders). So the LP is not custodial.

However, external LP holders can withdraw anytime. There's no guarantee liquidity will stay.

---

## Tokenomics

### What's the supply?
TBD. The supply will be announced before launch. No pre-mine is guaranteed; any announced allocation is for reference only.

### Will there be token emissions (new mints)?
Unknown. Early phase will likely not have emissions. Future phases (if DAO governance adopted) may vote on controlled emissions for LP incentives.

### Is there a vesting schedule?
Depends on allocation. If there's a team allocation, it will likely be vested over time (e.g., 1 year cliff, 3 year vest).

### How is the token funded?
Details TBD. Common structures:
- Fair launch (airdrop or bonding curve)
- Community allocation
- Team/Treasury allocation

All will be transparently announced.

---

## Operations

### How do I run the keeper?
See [scripts/dev.md](../scripts/dev.md) for local simulation. For production, see [scripts/deploy.md](../scripts/deploy.md).

Quick start:
```bash
git clone https://github.com/solengineer/moltball.git
cd keeper
npm install
npm run start:sim  # simulation mode (no real keys)
```

### Can I run my own keeper?
Yes, but it's not necessary. The core team will run the main keeper. However, if you want to experiment, you can run a keeper in simulation mode locally.

For mainnet, you'd need to be authorized as the "keeper role" on the FeeVault contract, which requires governance approval.

### How do I monitor the fee loop?
Logs are in `keeper/logs/`:
- `keeper.log`: Human-readable log of each cycle
- `receipts/<timestamp>.json`: Detailed receipt for each swap

You can also query on-chain events from the FeeVault program.

### What if there's an emergency?
The pause guardian can pause the vault immediately. This stops all swaps until unpaused.

For security incidents, see [security.md](./security.md).

---

## Governance

### Who controls MOLTBALL?
Early phase: Core team controls the keeper and config.
Medium term: Migration to multisig governance.
Long term: Community DAO (token holders vote on decisions).

### Can I vote?
Yes, if you hold $MOLTBALL. Governance details TBD.

### Can I propose changes?
Not yet. Early phase is bootstrapping; governance will be formalized later.

---

## Community & Development

### How do I contribute?
See [CONTRIBUTING.md](../CONTRIBUTING.md).

### Is the code open-source?
Yes, MIT license. See [LICENSE](../LICENSE).

### How do I report a security issue?
See [SECURITY.md](../SECURITY.md).

### Where's the roadmap?
TBD. Early focus is on:
1. Audit & testnet validation
2. Fair launch
3. Keeper stability & monitoring
4. Community engagement
5. Governance migration

---

## Investment & Speculation

### Should I buy MOLTBALL?
**No investment advice offered.** MOLTBALL is a speculative token. It may:
- Go to zero (most likely outcome for most tokens)
- Become illiquid
- Never gain traction

Only invest money you can afford to lose.

### What's the price target?
We don't publish price targets. Price is determined by market supply and demand, not by our projections.

### Is this a pump-and-dump?
No, because:
- Founder allocations (if any) will be vested
- No liquidity mining or token minting at launch
- Fees are burned/recycled, not dumped on the market
- Everything is on-chain and auditable

However, token price is volatile and unpredictable. Never invest based on past performance.

### What happens if the price crashes?
If the price crashes, the fee loop still runs (as long as there's trading volume). The buyback and LP continue to support the price floor, but they cannot prevent a market crash.

In a crash scenario:
- Liquidity may dry up (LP withdrawals)
- Fees may decrease (less trading)
- Slippage may spike
- The keeper may pause to avoid poor execution

---

## Technical

### What blockchain is MOLTBALL on?
Solana (mainnet-beta).

### What language is the contract written in?
Rust (Anchor framework). See [contracts/README.md](../contracts/README.md).

### What language is the keeper?
TypeScript/Node.js. See [keeper/](../keeper/).

### How do I deploy the contract?
See [scripts/deploy.md](../scripts/deploy.md).

### Can I fork MOLTBALL?
Yes, the code is MIT licensed. You can fork it for your own token.

However, you'll need:
- Your own token mint
- Your own Solana program deployment
- Your own keeper instance
- Community adoption

Forking the code is easy; building a token is hard.

---

## Risk Disclaimer

**MOLTBALL is experimental software. No guarantees are made about:**
- Functionality
- Performance
- Security
- Profitability

**Token trading is speculative and risky.** Markets can move against you unexpectedly. Only invest what you can afford to lose.

**See [SECURITY.md](./security.md) and [LICENSE](../LICENSE) for full disclaimers.**
