# Canonical Ownership Production Profile V1

Status: Closed inventory — behavior-neutral SSA-RC-P0 evidence

Date: 2026-07-14

Decision: exact BoxRef witness or trivial-only atomic cutover

Machine authority:
`tools/checks/fixtures/canonical_ownership_production_profile_v1.json`

Guard:
`tools/checks/lib/resolved_ownership_production_profile.py`

## Result

The current first canonical function family has no exact source-to-MIR
`BoxRef` representation producer.

```text
exact BoxRef source producers = 0
production CopyOwned callers = 0
production DestroyOwned callers = 0
ownership activation = 0
first SSA-I1 ownership profile = trivial-only
```

This is not inferred from the absence of a matching spelling. The guard fixes
the present authority boundary:

- typed parameters and returns are rejected by canonical preflight;
- calls and receivers are outside the first-family grammar;
- `StorageClass::BoxRef` remains a no-behavior-change inventory projection;
- the generic representation-fact vocabulary contains only
  `BoxedSumHandle`, not a generic BoxRef ABI witness;
- MIR JSON emits storage inventory, but direct JSON v0 ingress does not
  reconstruct or verify it;
- passive Ownership SSA opcodes do not exist before SSA-RC-A0.

## Closed matrix

| Profile | Rows | Meaning |
| --- | ---: | --- |
| `trivial_exact` | 3 | integer, boolean, and float literal representations are exact |
| `derived_trivial_only` | 7 | local, PHI, BlockExpr tail, binary, read, assignment, and return forward an already exact trivial profile |
| `typed_preflight_reject` | 4 | untyped parameter, Outbox/Void, borrowed text, and Void/Null cannot enter the first ownership cutover |
| `not_in_first_family` | 3 | receiver, call argument, and call result have no active first-family source route |

The machine artifact owns the exact 17 row identifiers and source anchors.
This note is explanatory and is not a second registry.

## Consequence

SSA-RC-A0 may now add passive vocabulary and its transport schema without
claiming production ownership. SSA-I1 must initially cut over only the closed
trivial grammar unless a separate `SSA-I1-O1` row first establishes all of:

```text
exact BoxRef source producer
sealed representation fact
caller/callee ownership ABI
direct JSON ingress verification
supported backend capability
```

`MirType::Box`, `StorageClass::BoxRef`, a box name, or backend runtime shape is
insufficient by itself.

## Non-claims

```text
BoxRef Ownership SSA production
borrowed text ownership
owned PHI forwarding
parameter/call ownership ABI
CopyOwned / DestroyOwned vocabulary
legacy ReleaseStrong retirement
grammar narrowing
runtime behavior change
```
