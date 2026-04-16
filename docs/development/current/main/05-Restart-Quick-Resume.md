---
Status: Active
Date: 2026-04-16
Scope: 再起動直後に 2〜5 分で current lane に戻るための最短手順。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/10-Now.md
---

# Restart Quick Resume

## Quick Start

```bash
cd /home/tomoaki/git/hakorune-selfhost
git status -sb
cargo test --lib --no-run
cargo check --bin hakorune
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq
```

## Current

- lane: `phase-29bq selfhost mirbuilder failure-driven`
- guardrail: `phase-137x string corridor / exact-keeper guardrail`
- immediate next: `compiler expressivity first`
- immediate follow-on: `phase-29bq loop owner seam cleanup`

## Current Handoff

- blocker: `none`
- residue exact shape:
  - explicit facts-local `plan_residue` under `facts/`
  - intentional top-level owner surfaces remain under `recipes / lower / verify / ssa / cleanup / facts`
  - `plan/policies` is compat-only for already-moved cleanup policies
  - route-entry no longer needs a dedicated keep-plan bridge
- next exact handoff:
  - `plan/recipe_tree` now depends on top-level `recipes::{RecipeBody, refs}` owner surfaces
  - `plan/parts/join_scope.rs` split is landed
  - next shared-infra pointer is the `loop_scan_phi_vars_v0::nested_loop_recipe_handoff` cleanup / `loop_cond` remaining recipe-surface inventory (`continue-only` + `continue-with-return`)
  - keep top-level owner surfaces in `recipes / lower / verify / ssa / cleanup / facts`
  - keep `facts::plan_residue` explicit and thin while `plan/facts/*` ownership continues to move
  - keep `loop_cond` keep-plan residue internal to the family

## Read Next

1. `CURRENT_TASK.md`
2. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
3. `docs/development/current/main/10-Now.md`
4. `docs/development/current/main/15-Workstream-Map.md`
5. `docs/development/current/main/design/compiler-expressivity-first-policy.md`
6. `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
7. `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
8. `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`
9. `docs/development/current/main/phases/phase-137x/README.md`

## Current Proof Bundle

```bash
git status -sb
cargo test --lib --no-run
cargo check --bin hakorune
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq
```
