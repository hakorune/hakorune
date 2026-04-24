---
Status: Landed
Date: 2026-04-24
Scope: Task the Hotline/CoreMethodContract cleanup before implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - lang/src/runtime/meta/README.md
  - lang/src/runtime/collections/method_policy_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - src/mir/generic_method_route_plan.rs
---

# 291x-131 Hotline CoreMethodContract Task Plan

## Goal

Convert the design consultation into a clean implementation queue before code
changes.

This is a docs-only BoxShape card. It does not add a CoreBox row, route shape,
parser rule, environment variable, runtime helper, generated table, or hot
inline lowering.

## Decision

The next compiler-cleanliness lane is:

```text
CoreMethodContract SSOT
  -> generated manifest / enum-table
  -> MIR CoreMethodOp metadata
  -> .inc table consumer
  -> evidence-backed hot lowering later
```

`compiler_stageb.hako` thinning is valid, but it is a separate BoxShape series.
It must not be mixed with CoreMethodContract migration.

## Task Plan

### 1. Contract SSOT Seed

- Add the initial contract owner under `lang/src/runtime/meta/`.
- Seed only Array/String/Map rows listed in
  `docs/development/current/main/design/hotline-core-method-contract-ssot.md`.
- Import current vocabulary from
  `lang/src/runtime/collections/method_policy_box.hako`.
- Do not touch `.inc` lowering in the seed card.

### 2. Generated Metadata

- Add a small generation step or generated-table guard.
- Generate stable contract ids / op ids / alias rows / effect rows.
- Decide artifact placement in the card before code edits.
- Make manual generated-table edits detectable.

### 3. Drift and No-Growth Guard

- Keep the existing Set mirror guard.
- Add or extend a guard so new `.inc` method-name classification cannot appear
  without a CoreMethodContract row and deletion condition.
- Update `docs/tools/check-scripts-index.md` if a new script is added.

### 4. MIR CoreMethodOp Carrier

- Add one narrow carrier path first.
- Store decided op/proof/tier metadata before backend emission.
- Do not turn the carrier into a second semantic owner.
- Keep compatibility fallback for non-migrated rows.

### 5. `.inc` Consumer Migration

- Pick one method family only.
- Convert that family from local method-name classification to generated
  op/table consumption.
- Mark remaining mirror code with deletion conditions.

### 6. LoweringTier Metadata

- Add `hot_inline` / `warm_direct_abi` / `cold_fallback` vocabulary as data.
- Default initial rows away from `hot_inline` unless evidence already exists.
- Do not introduce inline lowering in the same card as the tier vocabulary.

### 7. HotlineGate Keeper Slice

- Choose exactly one CoreMethodOp after perf owner-first evidence.
- Apply the Hotline Law to the target loop.
- Inline only the fast path; keep slow path as a call.
- Record exact/meso/whole evidence and revert conditions before merge.

### 8. Stage-B Adapter Split

- Create a separate Stage-B split card.
- Extract tracing, legacy main/body compatibility, same-source defs scan, and
  JSON fragment injection into named boxes.
- Keep `compiler_stageb.hako` as entry adapter/orchestrator.
- Do not change CoreMethodContract or generic-method `.inc` policy in the same
  commit series.

### 9. Closeout

- Delete obsolete manual mirror branches only after generated consumers own the
  migrated rows.
- Update shim and meta/collection READMEs.
- Keep a final no-growth guard against policy mirror regrowth.

## Immediate Next Cards

Suggested order:

1. `291x-132 CoreMethodContract seed schema`
2. `291x-133 CoreMethodContract generated table and drift guard`
3. `291x-134 one-family CoreMethodOp MIR carrier`
4. `291x-135 one-family .inc table consumer`
5. Stage-B adapter split card, only after deciding not to mix with the above

## Guardrails

- BoxShape only until a separate card explicitly adds a new accepted route.
- No hot inline lowering before perf/asm evidence.
- No new `.inc` method-name classifier without a contract row and deletion
  condition.
- No benchmark-name or source-name branch.
- No Rust storage rewrite in the contract seed.
- No Stage-B adapter split in the same commit as CoreMethodContract migration.

## Proof

Docs-only proof:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Code cards must add their focused smoke or guard and also run:

```bash
cargo fmt -- --check
tools/checks/dev_gate.sh quick
git diff --check
```
