---
Status: Active
Decision: provisional
Date: 2026-04-02
Scope: product ownership と engineering(stage0/bootstrap) residue が同居している source/smoke を split し、`vm-rust` keep のまま live surface を slimmer にする。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-31x/README.md
  - docs/development/current/main/phases/phase-32x/32x-90-product-engineering-split-ssot.md
  - docs/development/current/main/phases/phase-32x/32x-91-task-board.md
---

# Phase 32x: Product / Engineering Split

## Goal

- `vm-rust` を delete/archive 方向ではなく `engineering(stage0/bootstrap + tooling keep)` に固定したまま、product ownership と engineering residue が同居している source/smoke を split する。
- first target は `src/runner/build.rs` と `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`。
- top-level engineering keeps は caller drain を固定して canonical home へ rehome し、その後に deeper shell residue へ進む。

## Fixed Reading

- `phase-31x engineering lane isolation` は landed precursor として読む。
- この phase は docs-only ではなく source/smoke split plan を first-class に持つ。
- `vm-rust` は current phase でも keep。thin にする対象は `vm.rs` そのものではなく mixed ownership surfaces。
- `vm-hako` は reference/conformance keep、`wasm` は experimental keep のまま扱う。
- raw backend default / token / dispatch flip はこの phase の主題にしない。

## Non-Goals

- `src/cli/args.rs` の default rewrite
- `src/runner/dispatch.rs` の central selector rewrite
- `src/runner/modes/vm.rs` の archive/delete
- `vm-hako` / `wasm` の promotion or archive

## Exact Next

1. `32x-90-product-engineering-split-ssot.md`
2. `32x-91-task-board.md`
3. `src/runner/build.rs`
4. `tools/smokes/v2/profiles/integration/core/phase2100/run_all.sh`

## Canonical Child Docs

- split rules / no-touch-first / disposition:
  - `32x-90-product-engineering-split-ssot.md`
- concrete queue / evidence commands:
  - `32x-91-task-board.md`

## Acceptance Summary

- `build.rs` mixed ownership is inventoried and split target is fixed
- `phase2100/run_all.sh` mixed aggregator is inventoried and split into role sub-runners behind the stable public path
- current front is now the shared-helper follow-up gate after the `core_executor` direct-MIR seam was fixed
- bootstrap/plugin top-level keeps get explicit caller-drain plans
- direct `--backend vm` shell residues are reduced only behind dedicated split tasks
- raw backend default remains deferred until mixed-owner surfaces are thinned
