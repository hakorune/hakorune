# 296x-1627 MIRBUILDER-COMPAT-ENTRY-PLANNER-AMBIGUITY-INVENTORY-001

Status: closed
Date: 2026-06-22

## Purpose

Classify the first failure found while probing a canonical
`lang/src/mir/builder/compat/program_json_v0_entry.hako` entry before wiring
Stage-A or smoke callers to it.

## Observation

A local, reverted probe added a canonical compat entry that read
`HAKO_PROGRAM_JSON` / `HAKO_PROGRAM_JSON_FILE` and called:

```text
MirBuilderBox.emit_from_program_json_v0(...)
```

Under `HAKO_JOINIR_PLANNER_REQUIRED=1`, the probe failed during MIR
compilation before the compat conversion logic ran:

```text
[plan/freeze:contract] entry_ambiguous: candidates=loop_true_break_continue,generic_loop_v1
```

This is not evidence that the canonical compat redirect is semantically wrong.
It is evidence that the planner-required entry candidate set is not disjoint for
the imported entry shape.

## Cause

`entry_ambiguous` is emitted by:

```text
src/mir/builder/control_flow/joinir/route_entry/router.rs
```

The candidate set comes from:

```text
src/mir/builder/control_flow/joinir/route_entry/registry/mod.rs
```

The relevant predicates are in:

```text
src/mir/builder/control_flow/joinir/route_entry/registry/predicates.rs
```

Current facts:

- `loop_true_break_continue` is a specific owner for `loop(true)` bodies with
  break/continue/exit trees.
- `generic_loop_v1` is a broader loop-variable route.
- The triggering shape is likely the wrapper's own `loop(true)` input scan or
  CLI/env/file branch, not the `MirBuilderBox.emit_from_program_json_v0` call
  itself.
- `generic_loop_v1` can derive a loop variable from an increment assignment
  even under a boolean-literal loop condition.
- Existing suppressions block `generic_loop_v1` for `loop_char_map`,
  `loop_simple_while`, `nested_loop_minimal`, and `loop_cond_break_continue`.
- No current suppression blocks `generic_loop_v1` when
  `loop_true_break_continue` is also present.

This conflicts with the existing closeout decision in
`296x-1308-RUST-SUBSET-APP-FRONT-LOOP-TRUE-BREAK-CONTINUE-SMOKE-CLOSEOUT-001`,
which states:

```text
The owner is loop_true_break_continue, not generic_loop_v1.
```

## Decision

Phase 1628 should fix the route-owner disjointness first.

The minimal repair is to make `loop_true_break_continue` exclude
`generic_loop_v1` in strict/dev planner-required candidate collection and in
the actual recipe-first routing path.

Do not paper over this by disabling planner-required or by routing the new
compat entry through the legacy compiler tree.

## Stop Lines

- Do not add the canonical compat entry yet.
- Do not repoint Stage-A bridge yet.
- Do not repoint active JoinIR smokes yet.
- Do not delete `lang/src/compiler/mirbuilder/**`.
- Do not weaken `entry_ambiguous`; it is correctly catching a non-disjoint
  owner set.

## Next

```text
next_blocker=MIRBUILDER-COMPAT-ENTRY-LOOP-TRUE-GENERIC-DISJOINT-001
```

Expected implementation scope:

```text
src/mir/builder/control_flow/joinir/route_entry/registry/mod.rs
src/mir/builder/control_flow/joinir/route_entry/registry/predicates.rs
focused unit coverage for candidate disjointness
```

Acceptance:

```text
loop_true_break_continue + generic_loop_v1 candidate overlap => one owner
owner is loop_true_break_continue
entry_ambiguous remains active for true unknown overlaps
cargo test -q loop_route
cargo check -q --lib
```
