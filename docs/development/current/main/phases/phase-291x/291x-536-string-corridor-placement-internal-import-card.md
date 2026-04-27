---
Status: Landed
Date: 2026-04-27
Scope: String-corridor placement internal import hygiene
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/string_corridor_placement/mod.rs
  - src/mir/string_corridor_placement/types.rs
  - src/mir/string_corridor_placement/candidate_infer.rs
  - src/mir/string_corridor_placement/plan_infer.rs
  - src/mir/string_corridor_placement/relation_carry.rs
  - src/mir/string_corridor_placement/tests.rs
---

# 291x-536: String-Corridor Placement Internal Import Hygiene

## Goal

Remove broad `use super::*` imports from the string-corridor placement
implementation submodules.

This keeps the owner module thin and readable: submodules name the exact MIR
core types, string-corridor fact vocabulary, recognizers, and candidate
vocabulary they consume instead of inheriting everything through the parent
module.

## Inventory

Cleaned implementation imports:

- `src/mir/string_corridor_placement/mod.rs`
- `src/mir/string_corridor_placement/types.rs`
- `src/mir/string_corridor_placement/candidate_infer.rs`
- `src/mir/string_corridor_placement/plan_infer.rs`
- `src/mir/string_corridor_placement/relation_carry.rs`
- `src/mir/string_corridor_placement/tests.rs`

## Cleaner Boundary

```text
string_corridor
  owns fact/publish/provenance vocabulary

string_corridor_recognizer
  owns pure shape recognizers

string_corridor_placement::types
  owns candidate/plan/proof/publication vocabulary

implementation submodules
  import only the dependencies they consume
```

## Boundaries

- BoxShape-only.
- Do not change candidate inference, relation carry, or plan inference.
- Do not change metadata field names or refresh entry points.
- Do not change `.inc` behavior, helper symbols, or lowering.

## Acceptance

- `string_corridor_placement` implementation submodules no longer use broad
  `use super::*`.
- `cargo test --no-run -q` passes.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Made owner-internal dependencies explicit.
- Preserved string-corridor placement metadata behavior.

## Verification

```bash
cargo test --no-run -q
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
