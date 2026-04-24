---
Status: SSOT
Date: 2026-04-24
Scope: Zero-cost hot-line policy and CoreMethodContract/CoreMethodOp migration boundary.
Related:
  - AGENTS.md
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-131-hotline-core-method-contract-task-plan.md
  - lang/src/runtime/meta/README.md
  - lang/src/runtime/collections/method_policy_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - src/mir/generic_method_route_plan.rs
---

# Hotline CoreMethodContract SSOT

## Purpose

This document turns the current generic-method cleanup direction into a small,
sequenced compiler-cleanliness plan.

The goal is not to make every route fast immediately. The goal is to make the
compiler shape clean enough that later hot-path wins are natural:

```text
method surface
  -> compiler contract
  -> decided semantic op
  -> proof / lowering tier
  -> backend emit
  -> Rust only for cold / slow / storage substrate
```

## Decision

The design boundary is fixed as:

```text
.hako contract owner
  -> generated manifest / generated enum-table
  -> MIR CoreMethodOp metadata
  -> .inc table consumer / emitter
  -> Rust cold executor and storage substrate
```

This replaces the long-term target of manual `.hako` plus `.inc` policy mirrors.
Existing guarded mirrors may stay during migration, but new method-name
classification must not grow in `.inc` unless the same card includes a deletion
plan and a drift guard.

## Vocabulary

### CoreMethodContract

A compiler contract row for one user-facing method surface.

Minimum row shape:

| field | meaning |
| --- | --- |
| `box` | receiver family, for example `ArrayBox` |
| `canonical` | canonical method name |
| `aliases` | compatibility spellings |
| `arity` | accepted argument count shape |
| `effect` | `pure_read`, `mutates_slot`, `mutates_shape`, or later vocabulary |
| `core_op` | semantic operation id after contract resolution |
| `hot_lowering` | intended hot lowering tier, if any |
| `cold_lowering` | fallback helper / runtime route |
| `runtime_owner` | Rust or `.hako` storage owner for slow/cold behavior |
| `status` | `seed`, `active`, `generated`, `deleted_mirror` |
| `guards` | smoke / drift guard names that pin the row |

### CoreMethodOp

A compiler-internal operation id resolved from a method surface.

Working examples:

- `ArrayLen`
- `ArrayGet`
- `ArraySet`
- `ArrayPush`
- `MapGet`
- `MapSet`
- `MapHas`
- `MapLen`
- `StringLen`
- `StringSubstring`
- `StringIndexOf`

The exact Rust enum name can differ. The required property is that backend code
receives a decided op/proof, not raw method names that it reclassifies.

### LoweringTier

Lowering tier is separate from method semantics:

| tier | meaning |
| --- | --- |
| `hot_inline` | emit inline IR for the fast path; slow guard may call out |
| `warm_direct_abi` | direct helper call is acceptable outside the target hot loop |
| `cold_fallback` | runtime method / compatibility path |
| `dynamic_fallback` | legacy or unknown method route; not a keeper hot path |

### HotlineGate

HotlineGate is a keeper gate for optimization cards. It is not a reason to
rewrite code before owner evidence exists.

## Ownership Rules

- `.hako` owns contract vocabulary and method semantics.
- `lang/src/runtime/meta/` is the preferred home for compiler semantic tables.
- `lang/src/runtime/collections/method_policy_box.hako` is the current
  collection policy precursor and migration input.
- MIR owns the decided op/proof metadata carried to the backend.
- `.inc` consumes generated enums/tables and emits backend calls or inline IR.
- `.inc` must not become a second semantic planner, method-name classifier,
  source scanner, legality checker, or policy owner.
- Rust owns storage mechanics, handle registry, allocator mechanics, cold
  fallback, slow path, debug/probe, and canonical runtime behavior.
- Rust helpers are allowed in cold/warm paths. Tiny hot-path helper calls are
  not keepers unless generated assembly proves the call boundary is erased.

`src/mir/generic_method_route_plan.rs` is an existing narrow route-plan seam for
generic `has`. Treat it as an interim proof that MIR-carried route metadata is
useful, not as the final CoreMethodContract owner.

## Hotline Law

A transformation is not a keeper if the target hot loop still contains:

1. generic method dispatch
2. method-name classification or string compare
3. cross-ABI runtime helper call for a tiny scalar/storage operation
4. lock acquire/release
5. allocation fast-path function call
6. runtime legality/provenance check
7. source-name, benchmark-name, or test-name specific branch

Allowed exceptions:

- operation is inherently external or I/O
- call is only on a proven cold guard-failure path
- operation is large enough that call overhead is not material
- generated assembly proves the call boundary is inlined or erased

Evidence order stays owner-first:

1. split `exact / meso / whole`
2. identify owner/state transition
3. choose one seam
4. then apply HotlineGate to keeper judgment

Do not delete helpers or widen contracts by speculation.

## Target Pipeline

```text
MethodCall(ArrayBox, "length", [])
  -> CoreMethodContract resolution
  -> CoreMethodOp(ArrayLen)
  -> proof / effect / residence metadata
  -> LoweringTier selection
  -> .inc consumes op/tier and emits code
  -> Rust only for cold fallback or storage slow path
```

The same shape applies to `MapBox.get/has/set/size`, `StringBox.length`,
`StringBox.substring`, and later collection/runtime rows.

## Initial Contract Seed Scope

The first seed is intentionally small:

| surface | aliases | effect | op |
| --- | --- | --- | --- |
| `ArrayBox.length` | `len`, `size` | `pure_read` | `ArrayLen` |
| `ArrayBox.get` | none | `pure_read` | `ArrayGet` |
| `ArrayBox.set` | none | `mutates_slot` | `ArraySet` |
| `ArrayBox.push` | none | `mutates_shape` | `ArrayPush` |
| `MapBox.get` | none | `pure_read` | `MapGet` |
| `MapBox.set` | none | `mutates_slot` | `MapSet` |
| `MapBox.has` | none | `pure_read` | `MapHas` |
| `MapBox.size` | `len`, `length` | `pure_read` | `MapLen` |
| `StringBox.length` | `len`, `size` | `pure_read` | `StringLen` |
| `StringBox.substring` | `substr` | `pure_read` | `StringSubstring` |
| `StringBox.indexOf` | `find` | `pure_read` | `StringIndexOf` |

`RuntimeDataBox` stays protocol/facade-only until a separate contract card
decides which rows are real semantic surfaces and which are compatibility
routes.

## Task Ledger

### HCM-0 Documentation Boundary

- Status: landed by `291x-131`.
- Add this SSOT.
- Link the perf owner-first SSOT to HotlineGate.
- Add a phase task card that splits implementation work.
- Update Current pointers because the active blocker changes.

### HCM-1 Contract Owner Seed

- Add the first contract owner file or table under `lang/src/runtime/meta/`.
- Import existing vocabulary from
  `lang/src/runtime/collections/method_policy_box.hako` instead of inventing a
  second policy source.
- Seed only the Initial Contract Seed Scope rows.
- No generated code, no backend behavior change, no hot inline lowering.

Done when:

- contract rows are readable from one owner location
- row schema matches this SSOT
- no `.inc` row changes are required

### HCM-2 Generated Manifest / Enum Table

- Add the smallest generator or checked generation step.
- Generate a stable enum/table consumed by `.inc` or a temporary guard.
- Decide artifact placement before writing code.
- Generated output must include contract id, canonical name, aliases, effect,
  op, and lowering tier placeholders.

Done when:

- generated artifacts match the `.hako` contract owner
- `git diff --check` and the focused generation guard pass
- manual edit of the generated table is detectable

### HCM-3 Drift Guard / No-Growth Guard

- Extend or add a focused guard that rejects untracked `.inc` method-name
  classifier growth.
- Keep the existing `Set` mirror guard intact.
- Allow existing guarded mirror code during migration.
- Fail if a new `strcmp` method surface is added without a matching contract
  row and deletion plan.

Done when:

- the guard is wired into the appropriate quick gate
- `docs/tools/check-scripts-index.md` is updated if a new script is added

### HCM-4 MIR CoreMethodOp Carrier

- Introduce a narrow metadata carrier for one family first.
- Prefer extending the existing route-plan seam only if it does not become a
  second semantic owner.
- The carrier stores decided op/proof/tier, not raw method names for backend
  reclassification.

Done when:

- MIR has a visible op/proof row before backend emission
- backend still has a compatibility path for rows not migrated
- no new language feature or CoreBox surface is added

### HCM-5 `.inc` Table Consumer Slice

- Convert one `.inc` family to consume generated op/table metadata.
- Start with one small family, preferably `ArrayLen` or existing generic
  `has`, after HCM-2/HCM-3 are in place.
- Keep fallback behavior unchanged for non-migrated rows.

Done when:

- target `.inc` path no longer classifies that method surface by name
- focused smoke and current pointer guard pass
- old mirror code is marked with a concrete deletion condition

### HCM-6 LoweringTier Metadata

- Add tier metadata as data, not as an optimization rewrite.
- Default first rows to `warm_direct_abi` or `cold_fallback` unless perf/asm
  evidence justifies `hot_inline`.
- Do not inline a helper in the same card that introduces tier vocabulary.

Done when:

- tier exists in contract/metadata
- no hot path behavior changes
- later optimization cards can point at a tier field

### HCM-7 First Evidence-Backed Hot Lowering

- Pick exactly one CoreMethodOp.
- Run perf owner-first evidence before code edits.
- Apply HotlineGate to the target loop.
- Inline fast path only when the owner and state transition are proven.
- Slow path remains a call boundary.

Done when:

- target hot loop has no generic dispatch, method-name compare, tiny helper
  call, lock, allocation fast-path call, or runtime legality branch
- exact/meso/whole evidence is recorded in the phase/investigation docs
- keeper/revert condition is explicit before merge

### HCM-8 Stage-B Thin Adapter Split

This is a separate BoxShape series. Do not mix it with CoreMethodContract
migration.

Tasks:

- create a Stage-B split card before code edits
- extract tracing to a Stage-B trace box
- extract legacy main/body compatibility detection
- extract same-source defs scan and JSON fragment injection
- keep parser invocation authority explicit
- reduce `compiler_stageb.hako` to entry adapter plus orchestration

Done when:

- each extracted box has one responsibility
- Stage-B smoke stays green after each commit
- no CoreMethodOp or generic-method policy rows are changed in the same commit

### HCM-9 Closeout / Deletion

- Delete obsolete manual mirror branches only after the generated consumer owns
  the same rows.
- Update shim README and collection/meta READMEs.
- Keep a final drift guard that prevents mirror regrowth.

Done when:

- no active `.inc` branch owns semantic method classification for migrated rows
- generated artifacts and MIR metadata are the only backend inputs for those
  rows
- docs point to one current owner

## Non-Goals

- no broad CoreOp rewrite in one card
- no benchmark-specific branch
- no immediate deletion of all `.inc` files
- no Rust storage rewrite
- no new language surface
- no hot inline lowering without perf/asm evidence

## Required Proof Pattern

Docs-only cards:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Code cards add the smallest relevant focused smoke plus:

```bash
cargo fmt -- --check
tools/checks/dev_gate.sh quick
git diff --check
```
