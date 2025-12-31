# Axiom Core

This repository contains the **reference implementation of the Axiom Layer-1 blockchain**.

Axiom is a deterministic, high-performance, object-based blockchain designed
to support scalable on-chain applications with strong economic and execution
invariants.

---

## Repository Scope

This repository implements **protocol-level logic only**, including:

- Proof-of-Stake consensus
- Deterministic execution runtime
- Object-based state model
- Transaction processing
- Validator staking & slashing
- Protocol economics & governance
- Upgrade mechanisms

---

## Non-Goals

This repository intentionally does **not** contain:

- Wallets
- User interfaces
- Explorers
- Indexers
- SDKs
- First-party applications

Those live in separate repositories under the Axiom Labs organization.

---

## Architecture Philosophy

Axiom Core is built on the following principles:

- **Determinism over convenience**
- **Explicit state over global state**
- **Parallelism by construction**
- **Protocol-enforced economics**
- **Minimal trusted components**

The protocol favors compile-time safety, explicit invariants,
and auditable logic over dynamic or implicit behavior.

---

## Repository Structure

```text
core/
├── types/        # Core protocol primitives (Address, Hash, Slot, etc.)
├── state/        # Object-based state model (upcoming)
├── runtime/      # Execution engine (upcoming)
├── consensus/    # Proof-of-Stake & finality (upcoming)
├── economics/    # Emissions, rewards, staking (upcoming)
└── governance/   # On-chain governance & upgrades (upcoming)

