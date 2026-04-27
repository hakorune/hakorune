---
Status: SSOT
Date: 2026-04-27
Scope: MIR root facade export contract.
Related:
  - src/mir/mod.rs
  - tools/checks/mir_root_facade_guard.sh
  - tools/checks/mir_root_facade_allowlist.txt
  - tools/checks/mir_root_import_hygiene_guard.sh
  - docs/development/current/main/phases/phase-291x/291x-523-semantic-metadata-root-export-inventory-card.md
  - docs/development/current/main/phases/phase-291x/291x-537-mir-root-facade-contract-card.md
---

# MIR Root Facade Contract

## Purpose

`src/mir/mod.rs` is a small public facade for core MIR infrastructure and
pipeline orchestration. It is not the owner of semantic metadata vocabulary.

The root can make common compiler plumbing easy to import, but it must not
hide where route/fact/proof/policy decisions live.

## Allowed Root Exports

Keep these categories available at the MIR root:

- Core MIR data model:
  - `MirModule`, `MirFunction`, `MirInstruction`, `BasicBlock`
  - `ValueId`, `LocalId`, core type/value/op enums
  - `Effect`, `EffectMask`, call/callee definitions
- Compiler facade entry points:
  - builder/compiler/optimizer/printer/query/verifier surfaces
  - small cross-cutting query helpers used as MIR infrastructure
- Refresh orchestration entry points:
  - `refresh_function_*`
  - `refresh_module_*`
  - whole-pipeline semantic refresh helpers

Refresh functions may remain at the root because callers are asking the MIR
pipeline to recompute metadata. They are orchestration entry points, not
semantic vocabulary ownership.

## Forbidden Root Exports

Do not re-export these from the MIR root:

- Domain semantic metadata structs/enums.
- Route/seed/window record types.
- Proof, policy, state, candidate, plan, selection, layout, contract, or
  provenance vocabulary.
- Types used only by JSON emitters, tests, fixtures, shims, or one backend
  helper family.

Consumers that construct or inspect those values must import the owner module
directly, for example:

```rust
use crate::mir::string_corridor::StringCorridorFact;
use crate::mir::string_corridor_placement::StringCorridorCandidate;
use crate::mir::sum_placement_selection::SumPlacementSelection;
```

## New Export Gate

Before adding a new `pub use` in `src/mir/mod.rs`, answer all of these:

- Is this a core MIR model type or compiler facade?
- If it is metadata, is it only a refresh entry point?
- Would importing the owner module be clearer?
- Is the export temporary? If yes, is there a phase card with a removal
  condition?
- Does the export make `.inc`, JSON, tests, or a backend helper depend on the
  MIR root instead of the owner?

If any answer points to semantic ownership, do not add the root export.

## Import Hygiene

Avoid broad root imports in new code:

```rust
use crate::mir::*;
```

Tests may use `use super::*` inside a small owner module when it improves local
readability, but not as a path back to pruned MIR-root vocabulary. If a test
constructs semantic metadata, import that metadata from its owner module.

## Review Commands

Use these during cleanup cards:

```bash
bash tools/checks/mir_root_facade_guard.sh
bash tools/checks/mir_root_import_hygiene_guard.sh
rg -n "use crate::mir::\\*;" src
rg -n "crate::mir::(StringCorridor|SumPlacement|ThinEntry|PlacementEffect|StorageClass|ValueConsumer)" src/mir src/runner -g'*.rs'
rg -n "pub use .*\\{[^}]*(Fact|Plan|Route|Candidate|Selection|Layout|Contract|Policy|Proof|State|Kind|Surface|Demand|Carrier|Reason|Provenance)" src/mir/mod.rs
```

Expected shape:

- root wildcard imports stay absent
- owner modules expose their own vocabulary
- root keeps refresh entry points and core MIR surfaces
- `tools/checks/mir_root_facade_guard.sh` reports the allowlisted export count
- `tools/checks/mir_root_import_hygiene_guard.sh` reports `ok`, including no
  root-path loop-canonicalizer detection bridge

## Current State

Phase 291x pruned the major semantic metadata root exports through the
root-export cleanup cards. The remaining MIR root surface should be treated as
a facade, not a semantic metadata catalog.
