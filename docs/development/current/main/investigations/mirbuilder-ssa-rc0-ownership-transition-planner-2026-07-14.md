# SSA-RC0 Ownership Transition Planner Evidence

Status: Closed — disconnected pure planner

Date: 2026-07-14

Decision: seal local-binding ownership transitions before the atomic Binding
SSA production cutover.

## Structure

The planner is isolated under:

```text
src/mir/builder/resolved_lowering/ownership/
  README.md
  value.rs
  assignment.rs
  scope_exit.rs
  error.rs
  tests.rs
```

Its closed input vocabulary is:

```text
LocalBindingClass = Receiver | Parameter | Local | Outbox
LoweredValueOwnership = Trivial | Owned | BorrowedStrong
```

Upvar, capture cell, field, index, and general place storage cannot be
represented by this planner. Their preflight remains outside SSA-RC0.

## Sealed laws

```text
exact BindingRef provenance is the only self-assignment authority
raw ValueId equality across different bindings is not self-assignment
borrowed strong replacement copies next before old-token destruction
owned temporary replacement transfers next before old-token destruction
trivial replacement reuses the existing ValueId
declaration uses the same closed next-value vocabulary
scope-local BlockExpr tail transfers and leaves the destroy set
outer borrowed BlockExpr tail copies exactly once
owned temporary tail forwards without a copy
closing roots destroy in reverse source declaration order
terminal borrowed Return copies before root destruction
terminal owned Return transfers without a copy
fallthrough/trivial Return destroys every current owned root
result ownership matches the sealed function ABI profile
unpublished draft discard exposes no runtime-action collection
foreign owners, duplicate roots/tokens, and ambiguous tail provenance reject
```

The replacement plan exposes `next` separately from
`previous_after_commit`; it does not expose a sortable action list.

## Purity and authority boundary

```text
MirBuilder imports:          0
MirInstruction imports:      0
BasicBlockId imports:        0
BindingRef -> ValueId maps:  0
ValueId allocation calls:    0
MIR emission calls:          0
production planner callers:  0
accepted grammar delta:      0
production activation:       0
```

## Verification

```text
focused planner fixtures:       18/18 green
resolved-lowering group:        74/74 green
private purity/caller guard:    green
authority guard:                green
release build:                  green
dev_gate quick:                 66/66 green
largest source/check file:      500 lines
```

## Non-claims

SSA-RC0 does not claim Binding SSA production use, ownership opcode emission,
legacy lifecycle retirement, general place ownership, capture ownership,
full call-convention ownership, or whole-language ownership completion.

## Next row

`SSA-I1` is the atomic current-owner cutover. It must connect Binding SSA,
canonical CFG sealing, this transition planner, Ownership SSA verification,
coverage, and unpublished-function publication as one closed production
transaction. Partial canonical/legacy value authority is forbidden.
