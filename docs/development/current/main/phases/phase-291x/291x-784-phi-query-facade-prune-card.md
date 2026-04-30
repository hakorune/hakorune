# 291x-784 PHI Query Facade Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/phi_query.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

Worker inventory split the remaining MIR structural vocabulary into active
runtime families and narrow cleanup seams:

- `cond_profile` is active loop-shape vocabulary
- `hints` is active runtime metadata plumbing
- `phi_query` is active runtime/query plumbing, but one facade remained unused

The unused seam was:

- `infer_phi_base_relation`

Repository evidence showed no live `src/` or `tests/` callers. The real active
surface remains:

- `collect_phi_carry_relations`
- `infer_phi_base_query_with_anchors`
- `collect_passthrough_phi_parents`
- `infer_phi_base_query` (kept for in-file tests / narrow future callers)

## Decision

Delete only the zero-use `infer_phi_base_relation` facade. Keep the rest of
`phi_query` intact as active MIR runtime vocabulary.

## Landed

- Removed the unused `infer_phi_base_relation` wrapper from `src/mir/phi_query.rs`.
- Left `infer_phi_base_query` and the anchored query path intact.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

The `cond_profile / hints / phi_query` family is no longer an undifferentiated
inventory bucket:

- `cond_profile` stays active vocabulary
- `hints` stays active vocabulary
- `phi_query` keeps only active query surfaces after this narrow prune

Remaining MIR structural vocabulary inventory now concentrates on LocalSSA and
extractor/detector seam candidates.

## Proof

- `rg -n "infer_phi_base_relation|infer_phi_base_query" src tests -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo fmt --check`
- `cargo test --lib --no-run`
- `git diff --check`
