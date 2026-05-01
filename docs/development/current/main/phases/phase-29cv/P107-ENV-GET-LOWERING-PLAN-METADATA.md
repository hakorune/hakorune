---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: add MIR-owned metadata for plan-backed `env.get/1` without changing ny-llvmc acceptance.
Related:
  - docs/development/current/main/phases/phase-29cv/P106-PHASE29CG-MIR-FIRST-REPLACEMENT-BLOCKER.md
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - src/mir/extern_call_route_plan.rs
  - src/runner/mir_json_emit/root.rs
  - crates/nyash_kernel/src/exports/env.rs
---

# P107 EnvGet LoweringPlan Metadata

## Goal

Start the `env.get/1` root fix on the MIR side.

P106 showed that the full Stage1 env MIR reaches pure-first and stops at:

```text
mir_call Extern env.get/1
reason=mir_call_no_route
```

The fix must not be a raw `.inc` matcher. The first clean slice is to make
`env.get/1` visible as a MIR-owned `LoweringPlan` entry.

## Decision

- Add `src/mir/extern_call_route_plan.rs` as the owner for narrow extern call
  route metadata.
- Add `FunctionMetadata.extern_call_routes`.
- Derive `metadata.lowering_plan` entries from `extern_call_routes`.
- Represent `env.get/1` as:

```text
source=extern_call_routes
source_route_id=extern.env.get
core_op=EnvGet
tier=ColdRuntime
emit_kind=runtime_call
symbol=nyash.env.get
proof=extern_registry
return_shape=string_handle_or_null
```

## Non-goals

- no ny-llvmc acceptance change
- no C `.inc` raw `env.get/1` classifier
- no Stage1 artifact redesign
- no dominance repair

## Next

The next card may teach ny-llvmc to consume this plan entry. That consumer must
validate `source=extern_call_routes`, `core_op=EnvGet`, `symbol=nyash.env.get`,
and `proof=extern_registry` before emitting a call.

## Acceptance

```bash
cargo test -q extern_call_route
cargo test -q build_mir_json_root_emits_extern_call_routes_and_lowering_plan
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
