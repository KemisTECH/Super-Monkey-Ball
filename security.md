# MOLTBALL: Security Considerations & Threat Model

## Executive Summary

MOLTBALL's security relies on:
1. **On-chain validation**: Smart contract enforces rules (roles, limits)
2. **Off-chain discipline**: Keeper follows strict guardrails
3. **Transparency**: All actions logged and auditable
4. **Governance**: Role separation + optional pause control

This document outlines known risks and mitigations.

---

## Threat Model

### 1. Keeper Compromise

**Threat**: Attacker gains control of keeper keypair.

**Impact**:
- Unauthorized swaps (drain vault)
- Malicious LP additions (send tokens to attacker)
- Disruption of fee loop

**Mitigations**:
- Keeper keypair stored securely (encrypted, not in version control)
- Keypair rotation (regular or on-demand)
- IP whitelisting on RPC (if private node)
- Alert on unusual transaction patterns
- Emergency pause by guardian

**Grade**: HIGH RISK. Keeper is a single point of failure.

**Recommended**: Run keeper on secure infra (AWS/GCP with DNSSEC), use hardware wallet for signing (Ledger), or deploy multisig validation.

---

### 2. Smart Contract Bug

**Threat**: Logic error in FeeVault program (e.g., missing check, overflow, reentrancy).

**Impact**:
- Funds lost or stolen
- Vault state corrupted
- Role checks bypassed

**Mitigations**:
- Code audit by third party (recommended pre-launch)
- Testnet deployment and fuzzing
- Formal verification of critical instructions (optional)
- Emergency pause to halt operations
- Gradual rollout (small initial vault size, then increase)

**Grade**: MEDIUM RISK. Smart contracts are complex; bugs are common.

**Recommended**: Get a professional audit. Deploy to devnet/testnet first. Use gradual rollout with monitoring.

---

### 3. MEV / Front-Running

**Threat**: Validators or MEV searchers see pending swap and extract value.

**Impact**:
- Higher slippage than expected
- Sandwiching: front-run the swap with buy, back-run with sell
- Lost fees to MEV

**Mitigations**:
- **Slippage caps**: Reject swaps if slippage > MAX_SLIPPAGE_BPS
- **Randomized execution windows**: Vary swap timing (add delay before execution)
- **Private pools**: Use private RPC if available (e.g., MEV-protected endpoints)
- **Size limits**: Keep MAX_SPEND_SOL reasonable to limit impact
- **Cooldowns**: Spread swaps over time (rather than one large swap)

**Grade**: MEDIUM-HIGH RISK. MEV is inherent to Solana; can be mitigated but not eliminated.

**Recommended**: Use a MEV-aware RPC endpoint (e.g., Jito, Triton). Monitor slippage closely.

---

### 4. RPC Unavailability

**Threat**: RPC endpoint goes down; keeper cannot read vault state or submit transactions.

**Impact**:
- Fee loop stalls (no swaps executed)
- Fees accumulate but are not recycled
- No transparency during downtime

**Mitigations**:
- **Fallback RPCs**: Configure multiple RPC endpoints with failover
- **Timeout handling**: Graceful degradation if RPC slow
- **Monitoring**: Alert on repeated failures
- **Manual intervention**: Admin can pause/unpause if keeper fails for extended period

**Grade**: MEDIUM RISK. Affects availability, not security.

**Recommended**: Use 2-3 RPC endpoints (e.g., Helius, Triton, Alchemy). Implement retry logic.

---

### 5. Slippage Spike / Flash Crash

**Threat**: Market volatility causes extreme slippage; keeper swaps are reverted or fill at bad prices.

**Impact**:
- Fee loop does not execute
- Fees accumulate
- Potential for large losses if slippage caps too high

**Mitigations**:
- **Conservative slippage caps**: MAX_SLIPPAGE_BPS = 200 bps (2%) by default
- **Small swap sizes**: Keep MAX_SPEND_SOL reasonable
- **Manual monitoring**: Watch slippage trends; adjust if needed
- **Emergency pause**: Halt swaps during market chaos

**Grade**: MEDIUM RISK. Market risk, not protocol risk.

**Recommended**: Monitor 24h slippage. If average > 1%, reduce MAX_SPEND_SOL or pause.

---

### 6. Liquidity Pool Drain / Rug

**Threat**: Liquidity providers remove all LP from the MOLTBALL pool; pool becomes illiquid.

**Impact**:
- Huge slippage on swaps
- Fee loop cannot execute
- Price crashes
- LP additions fail

**Mitigations**:
- **No control over pool**: We do not own the Raydium pool; community LP drives depth
- **Monitor depth**: Alert if pool reserves drop >50% in 1 hour
- **Emergency pause**: Stop swaps if liquidity too shallow
- **Incentivize LP**: Offer LP rewards or incentives (future)

**Grade**: MEDIUM RISK. Outside protocol control; dependent on market.

**Recommended**: Monitor pool depth constantly. Engage community to maintain LP.

---

### 7. Governance / Role Hijacking

**Threat**: Admin or pause guardian account is compromised.

**Impact**:
- Config changed to malicious parameters (e.g., MAX_SPEND_SOL = 1000 SOL)
- Pause toggled on/off to disrupt service
- Vault ownership transferred to attacker

**Mitigations**:
- **Role separation**: Each role (keeper, admin, guardian) is distinct
- **Multisig (optional)**: Admin changes require 2-of-3 approval
- **Timelock (optional)**: Config changes delayed 48 hours before taking effect
- **Transparency**: All role changes logged on-chain

**Grade**: MEDIUM-HIGH RISK. Governance is critical; compromised admin is severe.

**Recommended**: Use multisig for admin. Implement 24-48 hour timelock for config changes.

---

### 8. Token Supply Manipulation

**Threat**: Mint authority used to inflate supply (dilute MOLTBALL holders).

**Impact**:
- Price crashes due to dilution
- LP becomes more attractive to arbitrageurs (higher slippage)
- Fee loop less effective

**Mitigations**:
- **Announce mint plan**: If new emissions, announce in advance
- **Renounce mint authority (optional)**: Permanently disable new mints
- **Community governance**: Token holders vote on supply changes
- **Transparency**: All mints logged on-chain

**Grade**: MEDIUM RISK. Depends on team credibility.

**Recommended**: Renounce mint authority post-launch. If emissions needed, use DAO governance.

---

## Audit Checklist

Before mainnet launch:

- [ ] **Code review**: External security audit completed
- [ ] **Testnet deployment**: 30 days of mainnet-simulation testing
- [ ] **Fuzzing**: Critical instructions fuzzed for edge cases
- [ ] **RPC security**: All RPC calls use HTTPS, timeout set
- [ ] **Keypair security**: Keeper keypair encrypted, not in git
- [ ] **Config validation**: All parameters checked against limits
- [ ] **Emergency pause**: Guardian account funded and tested
- [ ] **Monitoring**: Keeper logs, receipts, and alerts set up
- [ ] **Governance**: Multisig + timelock set up (if applicable)
- [ ] **Documentation**: Architecture, parameters, and risks documented
- [ ] **Community disclosure**: Risks and limitations communicated clearly

---

## Risk Levels Summary

| Threat | Level | Mitigation |
|---|---|---|
| Keeper compromise | HIGH | Secure infra, key rotation, monitoring |
| Smart contract bug | MEDIUM | Audit, testnet, gradual rollout |
| MEV / Front-run | MEDIUM-HIGH | Slippage caps, randomized execution, private RPC |
| RPC unavailability | MEDIUM | Fallback endpoints, retry logic, monitoring |
| Slippage spike | MEDIUM | Conservative caps, manual monitoring |
| Pool drain | MEDIUM | Monitor depth, pause if shallow |
| Role hijacking | MEDIUM-HIGH | Multisig, timelock, role separation |
| Token dilution | MEDIUM | Governance, renounce mint authority |

---

## Incident Response Plan

### Scenario 1: Keeper Offline for >1 Hour

```
1. Alert triggered by monitoring
2. Admin checks RPC status and keeper logs
3. If RPC down: await RPC recovery, restart keeper
4. If keeper crashed: restart process, check for errors
5. If suspicious activity: pause vault (guardian toggled pause)
6. Resume after root cause identified and fixed
```

### Scenario 2: Slippage Spike >5%

```
1. Alert: Swap reverted due to slippage
2. Keeper logs error; skips cycle
3. Admin monitors next 3 cycles
4. If persistent: reduce MAX_SPEND_SOL, increase COOLDOWN_SECONDS
5. Communicate status to community
```

### Scenario 3: Keeper Keypair Suspected Compromised

```
1. Pause vault immediately (via guardian)
2. Generate new keeper keypair
3. Re-initialize vault with new keeper
4. Audit all transactions from old keypair
5. If unauthorized swaps found: freeze funds (multisig decision)
6. Communicate incident to community
7. Rotate all credentials
```

### Scenario 4: Smart Contract Bug Found

```
1. Pause vault (guardian)
2. Assess severity (is fund loss possible?)
3. If critical: deploy patch to new contract
4. Migrate vault to new contract (manual transfer)
5. Audit new contract
6. Resume operations with monitoring
7. Post-mortem + lessons learned
```

---

## Testing & QA

### Unit Tests
- Initialize vault with valid/invalid parameters
- Deposit fees (authorization checks)
- Record swap receipt (state updates)
- Config changes (role checks, validation)
- Pause/unpause (state transitions)

### Integration Tests
- Keeper reads vault state correctly
- Keeper submits transactions and confirms them
- Jupiter quotes validated correctly
- On-chain events emitted as expected
- Receipts stored and parseable

### Fuzzing
- Invalid config parameters (overflow, underflow)
- Extreme balance amounts (max u64)
- Rapid cycles (back-to-back executions)
- Missing roles or accounts
- Concurrent operations

### Monitoring
- Keeper uptime (should be >99%)
- Swap success rate (% of attempted swaps that confirm)
- Slippage distribution (p50, p95, p99)
- Error logs (group by type, count per day)
- Receipt data quality (missing fields, corruption)

---

## Disclaimer

**This document identifies known risks. It does not guarantee safety.** Smart contracts and distributed systems are inherently complex and can fail in unexpected ways.

**MOLTBALL is experimental software. Use at your own risk. Do not invest money you cannot afford to lose.**
