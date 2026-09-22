---
Status: closed__NoSafeSlice__2026-09-22__RecursiveStringResultOwner
Task: MIR-CALL-PARSER-RECURSIVE-STRING-RESULT-AUTHORITY-D0
Date: 2026-09-22
Parent: mir-call-parser-loopbreak-composite-source-cutover-i3-2026-09-22.md
NextCard: none__frontier_pause__no_ready_semantic_candidate
Implementation permission: false; design-only owner selection
---

# Parser recursive String result authority D0

## Six-line brief

```text
Decision: decide whether the existing result/publication owner can carry the
  two parser self-recursive String helpers; otherwise retain their typed
  TargetOnly/RecursiveDependency terminal.
Source authority + canonical issuer: the existing source declaration/body
  proof, callable-result catalog, and VerifiedStaticCallResultPublicationOwnerV1.
Non-authority: names, AST rescans, MIR inference, fixture shrinking, VM,
  compatibility fallback, default String classes, or a second result solver.
Fail-fast boundary: before receiver/argument descent or Builder effects, every
  row is either an owner-backed selected result or a typed target-only terminal.
Smallest next slice: co-seal a finite termination, representation, effect,
  ABI, and physical-consumer decision for exactly the two helper rows.
Non-claims: no code, new semantic receipt, String ABI implementation, T4c
  publication, T5 caller switch, R0 deletion, or warning cleanup.
```

## Finite inventory

The boundary is the two source-backed static declarations reached by the real
merged parser before the selected `ParserProgramBox.parse/2 ->
ParserStringUtilsBox.starts_with/3` row:

| helper | source shape | current terminal |
| --- | --- | --- |
| `ParserStringUtilsBox.i2s/1` | negative branch calls `ParserStringUtilsBox.i2s/1` with `0 - n` | `Unavailable(RecursiveDependency)` -> `TargetOnly` |
| `StringHelpers.int_to_str/1` | negative branch calls `me.int_to_str/1` with `0 - v` | `Unavailable(RecursiveDependency)` -> `TargetOnly` |

The inventory is finite and source-backed.  It excludes `starts_with/3`,
unrelated parser targets, expression-If rows, VM/compatibility routes, and the
OwnedText map feature census.  No caller is promoted merely because its name
or declared return shape looks String-like.

## Existing-owner audit

The current monotone result solver is the canonical source-result authority.
Its stalled worklist seals direct or mutual cycles as
`Unavailable(RecursiveDependency)`.  `function_proof.rs` currently emits only
exact `i64` or exact nominal-Box outcomes, and `disposition.rs` exposes those
same two representations plus typed unavailable reasons.  The publication
owner consumes `TargetOnly` before receiver/argument descent and emits no
physical Call.

The existing owner therefore proves the rejection terminal, but it does not
prove bounded recursion termination or issue a String/OwnedText physical
representation, effect, ABI, and consumer relation.  The OwnedText T3 card is
already `ParkedSealed__NoSelectedOwnedTextCaller`; reopening it would require
a selected physical caller and output owner, which this D0 does not invent.

## Design alternatives and decision rule

1. **Extend the existing owner.** Accept only if one source transaction can
   prove the recursive measure, preserve exact caller/site/target identity,
   issue a String result relation, and hand it to one existing physical
   consumer without a second solver or ABI guess.
2. **Typed terminal.** Keep both rows as `TargetOnly/RecursiveDependency`,
   consumed once before effects, when the owner above cannot be shown.
3. **Unsupported shortcuts.** Filtering the rows, treating String as `i64`,
   shrinking the merged parser fixture, restoring compatibility fallback, or
   adding a VM route is rejected by this card.

The D0 closes only when alternative 1 has a named issuer, consumer, effect/ABI
relation, and caller-local delete set, or alternative 2 is recorded as the
explicit `NoSafeSlice__ResultFamilyOwnerAbsent` park with a reopen trigger.

## Acceptance and handoff

The design evidence is source proof, result-owner comparison, and a finite
state table; no Cargo run or new receipt is authorized.  The selected parser
LoopBreak card remains parked at T4c until this decision closes.  If an owner
is accepted, its implementation card must separately cover positive/negative,
duplicate/foreign, recursive termination, one-shot, residual, physical
consumer, production caller, and selected old-edge deletion evidence.  If no
owner exists, keep T4c/T5/R0 unopened and return the scheduler to another
already-inventoried family rather than widening this row.

```text
OwnerFound -> source-backed result product -> existing publication consumer
OwnerAbsent -> TargetOnly(RecursiveDependency) -> pre-effect terminal
Foreign/Duplicate/Residual -> named rejection -> no retry or fallback
```

No warning cohort, backend, VM, compatibility, or shared LegacyCallV0 work is
selected by this design card.

## D0 decision and frontier pause — 2026-09-22

The existing-owner comparison is complete.  The solver/catalog/publication
chain can issue and consume the typed `RecursiveDependency` terminal, but no
existing owner proves a bounded recursive measure and also supplies a
String/OwnedText representation, effect, ABI, and physical consumer.  The
OwnedText T3 card is explicitly parked without a selected physical caller.

The selected decision is therefore:

```text
NoSafeSlice__ResultFamilyOwnerAbsent
  -> keep TargetOnly(RecursiveDependency) before effects
  -> do not issue a String result or switch the parser caller
```

This D0 is closed as a family-local park.  Reopen only when an accepted owner
design names the source issuer, termination proof, representation/effect/ABI,
consumer, and selected old-edge delete set.  A future implementation card must
then carry positive/negative, foreign/duplicate, one-shot, residual, physical,
caller-switch, and retirement evidence together.

The scheduler has no other ready semantic Promote, Stop, or Delete row in this
authorized lane.  The Windows lifecycle capability card remains explicitly
`deferred__user_selected_later`, so it is not reopened as a substitute.  The
current pointer records this as a frontier pause; warning cleanup stays paused
at I147 and no code, fixture, fallback, VM, or backend change is authorized.
