---
Status: Landed
Date: 2026-04-04
Scope: choose the next source lane after the rust-vm retirement corridor ended in residual explicit keep.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-63x/63x-90-rust-vm-final-retirement-decision-ssot.md
  - docs/development/current/main/phases/phase-63x/README.md
---

# 64x-90 Next Source Lane Selection SSOT

## Intent

- pick the next source lane after the `60x -> 63x` corridor
- keep the rust-vm decision stable:
  - mainline retirement: achieved
  - full source retirement: deferred
  - residual explicit keep: frozen
- keep `vm-hako` out of the retirement corridor as reference/conformance

## Starting Read

- current direct/core mainline is stable
- rust-vm remains residual explicit keep only
- the next lane should not reopen broad rust-vm ownership

## Candidate Directions

- `stage1/selfhost mainline hardening`
  - evidence:
    - `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
    - `docs/development/current/main/design/frontend-owner-proof-index.md`
    - `lang/src/runner/stage1_cli_env.hako`
    - `tools/selfhost/lib/stage1_contract.sh`
    - `tools/selfhost/build_stage1.sh`
- `llvm compare residue cleanup`
  - evidence:
    - current `cargo check --bin hakorune` dead_code warnings point at
      `src/host_providers/llvm_codegen/ll_emit_compare_driver.rs`,
      `ll_emit_compare_source.rs`, and `route.rs`
- `residual keep reevaluation`
  - evidence:
    - `phase-63x` froze residual explicit keep and did not open a new delete-ready wave

## Decision Rule

- prefer the lane with the highest leverage on current source clarity
- do not reopen rust-vm broad-owner work without new caller-zero or replacement evidence
- keep `vm-hako` as live reference/conformance, not archive/delete scope

## Ranking

1. `phase-65x stage1/selfhost mainline hardening`
   - highest leverage on current mainline source ownership
2. `llvm compare residue cleanup`
   - real hygiene target, but narrower than the stage1/selfhost owner cluster
3. `rust-vm residual keep reevaluation`
   - deferred; no new caller-zero evidence

## Decision

- selected successor lane:
  - `phase-65x stage1/selfhost mainline hardening`
- why:
  - `.hako` / Stage1 authority remains the most important non-vm source owner cluster
  - rust-vm corridor already ended in residual explicit keep, so another keep-focused pass is lower leverage than moving the direct mainline cluster forward

## Handoff

- `phase-64x` is selection-only and now landed
- next lane:
  - `phase-65x stage1/selfhost mainline hardening`

## Big Tasks

1. `64xA1` successor lane inventory lock
2. `64xA2` candidate lane ranking
3. `64xB1` successor lane decision
4. `64xD1` proof / closeout
