# 296x-1628 MIRBUILDER-COMPAT-ENTRY-LOOP-TRUE-GENERIC-DISJOINT-001

Status: closed
Date: 2026-06-22

## Purpose

Make the planner-required route candidate set disjoint for
`loop_true_break_continue` and `generic_loop_v1` before re-opening the
canonical MirBuilder compat entry redirect.

## Decision

When `loop_true_break_continue` facts are present, `generic_loop_v1` is not a
candidate.

This follows the existing `296x-1308` decision:

```text
The owner is loop_true_break_continue, not generic_loop_v1.
```

The change is a route-owner disjointness repair, not a semantic widening.

## Implementation

Updated the route registry candidate suppression so that:

```text
loop_true_break_continue candidate present
  -> suppress generic_loop_v1 candidate
```

The same suppression is used by both:

```text
strict/dev planner-required candidate collection
recipe-first routing iteration
```

## Evidence

```bash
cargo test -q generic_loop_v1_is_suppressed_when_loop_true_break_continue_owns_shape
cargo test -q loop_route
cargo check -q --lib
```

## Non-Goals

- No canonical compat entry yet.
- No Stage-A bridge redirect yet.
- No active smoke redirect yet.
- No weakening of `entry_ambiguous`.
- No physical deletion of `lang/src/compiler/mirbuilder/**`.

## Next

```text
next_blocker=MIRBUILDER-CANONICAL-COMPAT-ENTRY-REDIRECT-001
```

Resume the canonical compat entry implementation and caller redirect after this
route-owner overlap is no longer a blocker.
