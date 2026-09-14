# amm_q3 — Solana Automated Market Maker

A constant product AMM (Automated Market Maker) built on Solana using the Anchor framework. Supports token swaps, liquidity deposits, and withdrawals using the `x × y = k` invariant.

---

## Overview

`amm_q3` allows users to:

- Initialize a liquidity pool for any two SPL token pairs
- Deposit liquidity and receive LP tokens in return
- Withdraw liquidity by burning LP tokens
- Swap between the two tokens with slippage protection

The swap math is powered by the `constant-product-curve` crate.

---

## Architecture

```
amm_q3/
├── programs/amm_q3/
│   ├── src/
│   │   ├── lib.rs              # Program entrypoint, instruction dispatch
│   │   ├── state.rs            # Config account definition
│   │   ├── error.rs            # Custom error codes
│   │   ├── constants.rs        # Shared constants
│   │   └── instructions/
│   │       ├── mod.rs
│   │       ├── initialize.rs   # Pool initialization
│   │       ├── deposit.rs      # Add liquidity
│   │       ├── withdraw.rs     # Remove liquidity
│   │       └── swap.rs         # Token swap
│   └── tests/
│       ├── tests.rs             # Test entrypoint, LiteSVM setup & test fns
│       └── ix_handlers/         # Instruction builders for tests
│           ├── mod.rs           # Re-exports all handlers
│           ├── init.rs          # create_initialize_ix()
│           ├── deposit.rs       # create_deposit_ix()
│           ├── withdraw.rs      # create_withdraw_ix()
│           └── swap.rs          # create_swap_ix()
├── tests/
│   └── amm_q3.ts                # Anchor/TS test stub (unused, LiteSVM preferred)
├── Anchor.toml
├── Cargo.toml                   # Workspace manifest
└── runbooks/                    # Notes and references
```

---

## Test Design

Tests use **LiteSVM** — an in-process Solana VM that runs entirely in Rust without a local validator. Each test follows this pattern:

```
setup() → creates LiteSVM instance, mints, PDAs, vaults
    ↓
create_*_ix() → builds the Anchor instruction with correct accounts
    ↓
send() → signs and submits via VersionedTransaction
    ↓
assert!(res.is_ok())
```

The `ix_handlers/` directory separates instruction construction from test logic, keeping `tests.rs` readable. Each handler mirrors the accounts struct from its corresponding instruction file in `src/instructions/`.

---

## Accounts

### Config (PDA)

The central pool state account. Seeds: `["config", seed]`

| Field | Type | Description |
|---|---|---|
| `seed` | `u64` | Unique pool identifier |
| `fee` | `u16` | Fee in basis points (e.g. 30 = 0.3%) |
| `authority` | `Option<Pubkey>` | Optional admin authority |
| `mint_x` | `Pubkey` | Token X mint address |
| `mint_y` | `Pubkey` | Token Y mint address |
| `config_bump` | `u8` | PDA bump for config |
| `lp_bump` | `u8` | PDA bump for LP mint |

### Vaults

Two ATAs owned by the config PDA:

- `vault_x` — holds token X reserves
- `vault_y` — holds token Y reserves

### LP Mint (PDA)

Seeds: `["lp", config]` — minted to liquidity providers on deposit, burned on withdrawal.

---

## Instructions

### `initialize(seed, fee, authority)`

Creates the pool config, LP mint, and vault token accounts.

### `deposit(amount, max_x, max_y)`

Deposits liquidity into both vaults proportionally and mints LP tokens to the user.

| Param | Description |
|---|---|
| `amount` | Amount of LP tokens to mint |
| `max_x` | Maximum token X to deposit (slippage) |
| `max_y` | Maximum token Y to deposit (slippage) |

### `withdraw(amount, min_x, min_y)`

Burns LP tokens and withdraws proportional token X and Y from vaults.

| Param | Description |
|---|---|
| `amount` | Amount of LP tokens to burn |
| `min_x` | Minimum token X to receive (slippage) |
| `min_y` | Minimum token Y to receive (slippage) |

### `swap(is_x, amount, min)`

Swaps one token for the other.

| Param | Description |
|---|---|
| `is_x` | `true` = swap X→Y, `false` = swap Y→X |
| `amount` | Amount of input token to swap |
| `min` | Minimum output tokens to receive (slippage guard) |

---

## Getting Started

### Prerequisites

- Rust
- Anchor CLI 1.0.x
- Solana CLI

### Build

```bash
anchor build
```

### Test

Tests use LiteSVM — an in-process Solana VM. No local validator needed.

```bash
cargo test -- --nocapture
```

To run a specific test:

```bash
cargo test test_swap -- --nocapture
```

### Test Coverage

| Test | Status |
|---|---|
| `test_initialize` | ✅ |
| `test_deposit` | ✅ |
| `test_withdraw` | ✅ |
| `test_swap` | ✅ |

---

## Key Dependencies

| Crate | Purpose |
|---|---|
| `anchor-lang` 1.0.x | Solana program framework |
| `anchor-spl` 1.0.x | SPL token CPI helpers |
| `constant-product-curve` | AMM swap math (x × y = k) |
| `litesvm` 0.12.0 | Fast in-process test validator |

---

## How the Swap Math Works

The pool maintains the invariant `x × y = k`. Given an input amount `Δx`:

```
Δy = y - k / (x + Δx)
```

A fee is deducted from the input before the calculation, reducing `Δx` effectively. If the output `Δy` is below the user-specified `min`, the transaction fails with `SlippageExceeded`.

---

## Project Structure Notes

- All program logic is in `programs/amm_q3/`
- Tests live in `programs/amm_q3/tests/`
- Instruction builders for tests are in `tests/ix_handlers/`
- The `idl-build` feature must be enabled for `anchor-spl` when using Anchor 1.0.x:

```toml
[features]
idl-build = ["anchor-lang/idl-build", "anchor-spl/idl-build"]
```
