---
Status: Closed
Date: 2026-07-14
Decision: SSA-RC-V0 — path-sensitive, edge-indexed Ownership SSA
Parent: mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
Next: SSA-RC-A1b Rust Owned forwarding and ABI
---

# SSA-RC-V0 Ownership Verifier Evidence

## Outcome

`src/mir/ownership_ssa/` now owns one disconnected verifier and one sealed
function-branded result. It classifies MIR values as `None`, `Borrowed`, or
`Owned` without introducing another reaching-value map.

Each reachable block has one exact live-Owned entry set. `DestroyOwned` and
Owned `Return` consume from that set. Owned Phi inputs are processed on the
selected predecessor edge as a parallel transfer: all selected sources are
removed before any destinations are published. Alternative predecessor inputs
are therefore mutually exclusive dispositions, not globally counted uses.

## Closed failures

```text
duplicate consume or use after consume
one branch missing an ownership disposition
same source forwarded twice on one edge
non-CFG, duplicate, or missing Phi predecessor
Borrowed Phi or Borrowed Return
ordinary Copy of Owned
CopyOwned from trivial None
canonical edge arguments
unreachable ownership blocks
managed call ownership without a sealed ABI witness
```

## Verification

```text
cargo test -q ownership_ssa::tests -- --nocapture
  17/17 green

bash tools/checks/resolved_region_flow_authority_guard.sh
  ownership profile 17/17 green
  production verifier callers 0
  authority guard green

cargo build --release --bin hakorune
  green (existing warnings only)

bash tools/checks/dev_gate.sh quick
  PASS 66/66
```

Largest new source/check files are `tests.rs` at 426 lines and `verify.rs` at
354 lines. Every new or modified source/check file remains below 800 lines.

## Non-claims

```text
Rust interpreter Owned Phi/Return forwarding = 0
canonical production ownership caller = 0
BoxRef source producer = 0
LLVM/Wasm/Hako interpreter ownership support = 0
legacy ReleaseStrong retirement = not started by this row
```
