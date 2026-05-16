# 293x-481 MIR-EXTERN-SPEC-001 Extern-Call Route Spec Table

Status: landed
Date: 2026-05-16

## Decision

`MIR-EXTERN-SPEC-001` is the BoxShape cleanup selected by `MIMAP-041B`.

It makes `src/mir/extern_call_route_plan.rs` table-driven by introducing a
single `ExternCallRouteSpec` owner for per-route constants.

## Scope

- Add an `ExternCallRouteSpec` table for accepted extern routes.
- Make `ExternCallRouteKind` accessors delegate to the spec table instead of
  repeating per-kind `match self` blocks.
- Move arity, value-argument index, and void-result acceptance to the same spec.
- Keep the public route JSON shape unchanged.
- Expose a read-only spec slice for future consumers such as vm-hako subset
  validation.

## Stop Lines

- Do not add, remove, or rename extern routes.
- Do not change route ids, symbols, proof strings, return shapes, value demands,
  effect tags, arity, or result acceptance.
- Do not rewrite `src/runner/reference/vm_hako/subset_check/mod.rs` in this row;
  that should be a separate row after the route spec table lands.
- Do not change backend lowering or C shim behavior.
- Do not change allocator behavior, provider activation, hooks, host allocator
  replacement, or `#[global_allocator]`.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `EXTSPEC.1` | Document row and selected owner. | Current points to this card. | no code before docs |
| `EXTSPEC.2` | Add route spec table and accessors. | Existing extern route tests pass unchanged. | no route semantics change |
| `EXTSPEC.3` | Route classification and value-arg selection read specs. | No per-kind arity/value-arg match ladder remains. | no subset validator rewrite |
| `EXTSPEC.4` | Verify guards. | Required evidence is green. | no backend/provider activation |

## Required Evidence

```text
cargo test -q extern_call_route_plan
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
```

## Closeout

This row closes when extern-call route constants have one table owner and
current moves to the next row selection card.

## Landed Implementation

```text
owners:
  src/mir/extern_call_route_plan.rs
  src/mir/extern_call_route_plan/tests.rs
```

`ExternCallRouteSpec` now owns the route id, core op, symbol, aliases, arity,
value-argument index, proof, return shape, value demand, effect tags, and
void-result acceptance for each accepted extern route. Route kind accessors,
classification, and route materialization read the spec table.

`extern_call_route_specs()` exposes the read-only table for later consumers,
but this row does not rewrite vm-hako subset validation.

Evidence:

```text
cargo test -q extern_call_route_plan
tools/checks/dev_gate.sh quick
bash tools/checks/current_state_pointer_guard.sh
```

Closeout:

```text
current blocker moves to MIR-EXTERN-SPEC-002 post-extern-spec row selection.
```
