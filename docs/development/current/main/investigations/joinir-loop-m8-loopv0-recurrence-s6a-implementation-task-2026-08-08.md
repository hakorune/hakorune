# JOINIR Loop M8 variable-accum recurrence S6A — execution brief

Status: `closed — normal Main ingress, typed disposition, and identity/coherence negatives landed`
Date: 2026-08-08
Parent: `JOINIR-LOOP-M8-LOOPV0-RECURRENCE-S6A-D0`
Current row: `JOINIR-LOOP-M8-LOOPV0-RECURRENCE-S6A`

## Change

Add one resolver-backed, caller-zero `VariableAccumRecurrenceV1` observer and
provenance-only producer for the exact `acc = acc + i; i = i + 1` loop. Private
input/condition/update/step/coverage observations co-seal into one move-only
`VerifiedVariableAccumRecurrenceFactsV1`, then deterministically issue only the
existing Recipe, JoinSig, Core, input-set, and operation/effect products.

This is one bounded implementation cohort, not a new S6A suffix ladder. The
source observer may use private input/condition/update/step/coverage DTOs, but
the only neutral product is the one atomic Facts aggregate. It must retain the
resolver-issued non-Clone loop capability and exact frame/site identity; the
producer consumes Candidate once and never re-reads AST, resolves names, or
reclassifies the family.

## Contract

The Loop pipeline SSOT is the sole normative owner of the exact source
membership, C/D/U/R precedence, normalized 2-binding/11-operation/2-carrier
golden, and 2 input / 2 binding / 8 Core-effect / 11 item-source relation
counts. The D0 card is only the bounded execution brief. Recipe keys begin at
the producer; AST navigation stays in compiler projection and neutral Facts
remain AST-free. DirectAccum, Recipe/operation kinds, Core, JoinSig
elaboration, the family selector, and physical owners are not widened.

Producer success has one terminal move-only aggregate: the complete
source-bound Recipe/JoinSig/Core/input/operation-evidence result. No input set,
Core relation, or operation/effect product may escape independently before
that terminal is issued. Any missing, duplicate, foreign, or inconsistent row
returns a typed failure with published product count zero; there is no partial
publication or retry.

## Landed in this slice

The neutral Facts issuer, resolver-backed source projection, deterministic
existing-Recipe producer, and focused positive/non-`Less`/producer cardinality
tests are landed. Existing SimpleWhile/DirectAccum/G0 observations remain
unchanged. The producer seals the existing Recipe/JoinSig/Core/input/effect
owners with the exact `2/2/8/11` relation counts and no Builder effect.
Source/check files stay below 800 lines.

The exact normal `Main.main` resolver path and the typed C/D/U/R ingress
envelope are now landed. Focused tests cover Candidate, Declined, Unresolved
(incomplete coverage), and Rejected (foreign owner, duplicate binding role,
and source-site conflict) without creating a fifth disposition. The Facts
issuer now performs the bounded source-site coherence check through a separate
validation module, keeping the Facts owner below the 800-line limit. S6A is
closed; no production selector, physical route, or Builder caller was opened.

The same implementation commit updates `src/mir/compiler/README.md`,
`src/mir/loop_structural_facts/README.md`,
`src/mir/loop_route_policy/README.md`,
`src/mir/loop_recipe_contract/README.md`, and
`docs/reference/mir/loop-recipe-contract.md` with the landed producer ID and
2/2/8/11 receipt. A second reference update remains mandatory at M10b cutover.
Focused tests, current-pointer, Recipe in-place/no-fallback guards, formatting,
and `git diff --check` must pass.

The acceptance matrix distinguishes the four source outcomes using this
precedence: foreign/duplicate identity conflict = `Rejected`; missing or
opaque required evidence = `Unresolved`; fully observed non-family shape =
`Declined`; exact complete shape = `Candidate`. `NoSafeSlice` is only the
development status before the observer contract exists and is never emitted as
a fifth disposition.

The landed receipt must state the exact relation cardinalities:

```text
2 input-source relations
2 Recipe binding relations
8 Core binding-effect relations
11 item-source operation relations
```

It must also state the source-anchor rule: reads use variable-reference
expressions, constants use literal expressions, Compare/Add use whole binary
expressions, writes use assignment targets, carrier entries use Loop statement
plus carrier key, and inputs use declaration plus initializer expression.

## Stop

Return to design for missing resolver authority, incomplete/overlapping source
roles, a need to re-read AST in the producer, or any second selector/physical
route. Do not claim all-19 coverage, M9 parity, production activation,
retry/fallback retirement, outer print/return tail coverage, or legacy deletion.

## Implementation order

```text
1. audit and consume existing resolver/source capability (closed)
2. private source observations (closed)
3. atomic Facts + C/D/U/R disposition (closed for the bounded ingress)
4. deterministic existing-Recipe projection (closed)
5. complete input/Core/operation-source seals and golden (closed)
6. duplicate/identity/source-coherence negatives and line-count guards (closed)
7. same-commit README and reference closeout (closed for this slice; repeat at
   M10b)
```

No step may introduce a deeper S6A task suffix. A newly discovered prerequisite
reopens design rather than being guessed into the implementation.

After this bounded closeout, continue only through S6B-S6G and M9. The audited
semantic-program co-seal and JoinSig-authorized Layout work are separate
pre-cutover BoxShape rows owned by the two Loop SSOTs; S6A does not implement
or bypass them. Their implementation commits must update the exact module
README and `docs/reference/mir/loop-recipe-contract.md`, and M10b must update
the reference again when the production caller and legacy retirements change.

## Code homes

```text
src/mir/compiler/variable_accum_recurrence_projection.rs
  resolver-backed AST/source-view observer only
src/mir/loop_structural_facts/variable_accum_recurrence.rs
  atomic AST-free Facts and C/D/U/R outcome only
src/mir/loop_structural_facts/variable_accum_recurrence_validation.rs
  source-site coherence validation only; it does not create a second Facts owner
src/mir/loop_recipe_contract/variable_accum_recurrence_producer.rs
  terminal projection into existing Recipe/JoinSig/Core/input/effect owners
src/mir/loop_route_policy/
  no S6A code or selector change; README/reference receipt only
```

If implementation needs another code home, stop and revise the design card
before editing. The named homes and the pipeline SSOT together prevent a
second source, producer, or selector authority.
