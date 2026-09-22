---
Status: fast__2026-09-22__StaticResultTargetOnlyTerminal
Task: MIR-CALL-PARSER-STATIC-RESULT-TARGET-ONLY-TERMINAL-I0
Date: 2026-09-22
Parent: mir-call-parser-static-result-authority-d1-2026-09-22.md
Implementation permission: true; one existing result/publication owner extension and one pre-effect stop
NextCard: mir-call-parser-loopbreak-composite-source-cutover-i3-2026-09-22.md
---

# Parser static-result target-only terminal I0

## Six-line brief

```text
Decision: retain every cataloged static call site as Selected or a reasoned
  TargetOnly disposition; TargetOnly stops before argument descent and effects.
Source authority + canonical issuer: the existing declaration/body result
  solver plus branded source-target catalog, extended at the existing
  VerifiedSameModuleCallableResultCatalogV1 proof boundary.
Non-authority: target-only physical lowering, method/name/ordinal lookup,
  MIR ValueId/type inference, AST rescan, VM, compatibility fallback, retry,
  or a second publication owner.
Fail-fast boundary: exact caller/site/target/package brand, one of the six
  unavailable reasons, one-shot take, no residual rows, and no physical Call
  for TargetOnly.
Smallest next slice: carry the reason through the existing publication owner
  and ingress, then make the selected member/me callers return the named
  target-only terminal before argument descent; keep Selected unchanged.
Non-claims: no Bool/String result ABI, no successful whole-package parser
  acceptance, no I3 caller switch, no R0 deletion, and no warning sweep.
```

## Source and consumer boundary

The source relation is the already-branded pair of
`VerifiedSameModuleCallableDeclarationCatalogV1` and
`VerifiedSourceStaticCallTargetCatalogV1`. The result catalog's existing
function proof already owns the six observed unavailable classes:

```text
UnknownExpression
StaticCallTargetAuthorityUnavailable
StaticCallResultUnavailable
KnownNonI64Return
RecursiveDependency
UnsupportedStatementKind
```

The implementation retains those reasons beside the exact `(caller, site,
target)` key. `VerifiedStaticCallResultPublicationOwnerV1` remains the sole
one-shot consumer. `Selected` rows continue through the existing
`VerifiedStaticCallResultPublicationHandoffV1` and physical bridge. `TargetOnly`
rows become a typed terminal and are never passed to
`lower_target_only_static_result_publication_v1`.

This is a Stop boundary for rows whose source result is unavailable. It does
not pretend that those rows are successful results, and it does not filter
 them from package coverage. A parser package containing such a row still
stops at the named terminal until a later result-family card supplies a real
representation and consumer.

## Implementation tasks

1. Extend the existing result/publication owner data path so a target-only row
   carries its exact unavailable reason. Preserve declaration/target-catalog
   brands and reject a missing or unsupported disposition at issue time.
2. Extend the publication ingress vocabulary and display error with the
   reasoned typed terminal. Keep compatibility `Unavailable` separate from a
   cataloged `TargetOnly` row.
3. Update the qualified static member route and the `me` static-current-owner
   route to return the named terminal before creating argument descent. The
   raw LoopBreak source port must retain its existing residual/target guards.
4. Keep the existing selected publication path byte-for-byte in meaning:
   exact handoff, argument cardinality, physical receipt, and post-success
   publication remain unchanged.
5. Add focused positive/negative guards for selected, each reason class,
   foreign brand, wrong site/target, duplicate take, second take, residual
   finish, and proof that TargetOnly emits no Call or argument effects.
6. Update the callable-result and Builder module READMEs with the terminal
   contract in the same implementation slice.

## Acceptance

Positive:

- Existing exact-I64 and exact-nominal selected-owner tests remain green.
- A cataloged non-I64 target returns one reasoned TargetOnly terminal and
  `finish_empty` succeeds only after that row is consumed.
- The merged parser fixture reaches the same named terminal before argument
  descent; no target-only physical call is emitted.

Negative and guards:

- foreign declaration/target brands, missing dispositions, mismatched site or
  target, duplicate rows, second consumption, and residual rows fail closed;
- every six-class reason remains distinguishable;
- compatibility/unlocated source remains `Unavailable`, never TargetOnly;
- selected rows cannot be consumed twice and TargetOnly cannot fall through to
  the selected physical bridge.

Required checks are the focused callable-result/publication and ingress tests,
then the selected merged-parser lifecycle test. Cargo remains one process with
quick profile and at most four build jobs.

## Exit and next queue

This I0 closes only when the typed terminal is observed on the production
member/me ingress and all focused guards pass. It does not claim parser
publication or caller cutover. After closeout, return to I3 task 4; then I3
task 5 switches the selected parser caller, and R0 task 6 proves caller-zero
before deleting the old edge. The I147 warning cohort remains paused and
`dead_code` stays owner debt until that semantic sequence is complete.

## Non-claims

No new result ABI, Bool/String coercion, target-only filtering, fallback,
VM/compatibility change, production caller switch, legacy retirement, or
acceptance beyond the typed terminal boundary is claimed here.
