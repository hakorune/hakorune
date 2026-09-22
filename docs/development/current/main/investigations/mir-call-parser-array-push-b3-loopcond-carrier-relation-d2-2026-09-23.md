---
Status: design_stop__loopcond_recipe_carrier_relation
Task: MIR-CALL-PARSER-ARRAY-PUSH-B3-LOOPCOND-CARRIER-RELATION-D2
Parent: mir-call-parser-array-push-b3-branch-continuation-d1-2026-09-23
NextCard: MIR-CALL-PARSER-ARRAY-PUSH-B3-IF-LOOPCOND-I0
Implementation permission: false; read-only owner/relation design only. Do not edit code, add fixtures, allocate MIR, create a semantic receipt, or switch a caller.
---

# LoopCond Recipe carrier relation D2

## Six-line brief

```text
Decision: determine how the existing LoopCond source product co-seals exact
source BindingRefs with Recipe-owned carrier keys and GeneralIf join outputs.
Source authority + canonical issuer: existing resolver loop/pre-effect
relations and existing LoopCond Facts/Recipe issuer; no new semantic receipt.
Non-authority: AST rescans, identifier names, MIR phi shape, declaration order,
the GenericLoop-only carrier relation, and physical name maps as identity.
Fail-fast boundary: missing/duplicate/foreign/shadowed binding, carrier, or
join output must reject before physical block or PHI allocation.
Smallest next slice: specify one existing-product relation and make the current
name-keyed PHI owner consume it without a second carrier table or solver.
Non-claims: no value/origin branch implementation yet, tail push, runtime
Text/Fault, serializer, VM/AOT parity, or shared MethodCall retirement.
```

## Confirmed current boundary

`CallableSemanticLoopHandoffPreEffectReceiptV1` already carries exact
BindingRefs, source assignment receipts, and declaration scope.
`SourceLoopCondPhysicalInputV1` retains that pre-effect product and the selected
LoopCond Facts/Recipe, but it does not carry a BindingRef-to-carrier relation.
The current physical consumer collects carrier names from AST and
`LoopCondBreakContinuePhiMaterializer::prepare()` consumes `&[String]`, using
those names for entry values, header/step/after PHIs, and final values. The
source port is the plain `CallableLoopSourceExpressionPortV1`; the existing
GenericLoop `CallableLoopCarrierRelationV1` is a different admitted product
and cannot be borrowed as proof for LoopCond.

The missing relation is needed before I0 can safely attach a GeneralIf join
ValueId to a source BindingRef. A name-keyed join followed by a name lookup in
the ledger would create a new identity authority. The physical binding may
remain a label, but the source binding must arrive through an existing
co-sealed relation.

## Finite LoopCond source inventory

For `StringHelpers.split_lines` the required outer state is `i` and `last`.
`arr`, `s`, and `n` are read-only for this selected loop; `ch` is iteration
local and must not escape. The exact source candidates are:

| role | source location | required treatment |
| --- | --- | --- |
| outer `last` declaration and rebind | function body ordinal 3; Loop body ordinal 1 / IfThen ordinal 1 | one stable outer BindingRef; branch join preserves implicit else input |
| outer `i` declaration, increment, and condition | function body ordinal 4; Loop body ordinal 2; Loop condition | one stable induction BindingRef across header/backedge/after |
| `ch` local | Loop body ordinal 0 | iteration local, excluded from carrier export |
| `arr` and `s` | Array construction/parameter | read-only, not branch rebind carriers |
| nested push | Loop body ordinal 1 / IfThen ordinal 0 | exact named-array row, existing ArrayPush consumer |
| post-Loop tail | function body ordinal 6 / IfThen ordinal 0 | separate non-claim |

These paths are a source witness, not numeric BindingRef evidence. D2 must
obtain identities from the same resolver ledger and exact source contexts.

## Ordered D2 tasks

1. **Name the existing producer.** Identify which existing LoopCond source
   product can own the relation. It must be issued from the already co-sealed
   pre-effect source rows plus the selected Facts/Recipe; do not introduce a
   detached `Verified*` or `Prepared*` receipt.

2. **Fix carrier key origin.** Use Recipe-owned carrier keys/slots. Map exact
   assignment/read sites for `i` and `last` to their existing BindingRefs and
   keys. Prove `ch` remains local and `arr`/`s`/`n` do not become carriers.

3. **Fix GeneralIf continuation.** Relate the selected GeneralIf Recipe item,
   then branch and implicit else entry to the same outer `last` key. Its join
   output must return to that exact BindingRef without looking it up by name.

4. **Fix loop continuation.** Relate header, step/backedge, break/after final
   values for `i` and `last` to those same keys. Specify Identity/Kill behavior
   for read-only values and iteration locals.

5. **Select the physical consumer.** Reuse the current LoopCond PHI materializer
   and `lower_if_join_state_core`; make their semantic inputs key-based while
   leaving the raw facade's existing name-based adapter intact. Do not create a
   second carrier table or another PHI/join solver.

6. **List rejection boundaries.** Missing, duplicate, foreign owner, shadowed
   binding, mismatched branch site, missing join output, and residual relation
   rows must fail before physical allocation using an existing owner terminal
   or a precisely named owner-local check in a later implementation slice.

7. **D2 exit.** Open the I0 only when one source→Facts/Recipe→carrier key→
   GeneralIf join→LoopCond header/backedge/after→physical owner relation is
   finite and checked, and the exact caller plus explicit retain/delete set are
   recorded. Otherwise keep `NoSafeSlice` local to this family and select an
   already-inventoried action under the family scheduler.

## Preserved D1 decision and implementation boundary

When implementation is admitted, branch state resets only current ledger
values and active origins. Reads, assignments, calls, materialized locals,
removed source rows, named-array emission ports, and historical origins remain
monotonic because both branches are compiled once. The shared raw MethodCall
writer remains retained; its delete set is empty.

The post-Loop `arr.push(s.substring(last))` row, runtime Text/Fault ownership,
backend acceptance, and serializer remain outside this D2 census.
