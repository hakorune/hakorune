---
Status: design_stop__no_connected_semantic_program_issuer
Task: MIR-CALL-PARSER-ARRAY-PUSH-B3-LOOPCOND-CARRIER-RELATION-D2
Parent: mir-call-parser-array-push-b3-branch-continuation-d1-2026-09-23
NextCard: none__frontier_pause_no_ready_candidate_after_scheduler_2026-09-23
Implementation permission: false; this D2 resolves to family-local NoSafeSlice. Do not edit code, add fixtures, allocate MIR, create a semantic receipt, or switch a caller.
---

# LoopCond Recipe carrier relation D2

## Six-line brief

```text
Decision: determine whether an already-connected canonical Recipe/JoinSig
issuer can co-seal exact LoopCond BindingRefs, carrier keys, and GeneralIf joins.
Source authority + canonical issuer: resolver/pre-effect rows are the source
authority; no canonical key issuer is connected to this LoopCond product.
Non-authority: AST rescans, identifier names, MIR phi shape, declaration order,
the GenericLoop-only carrier relation, and physical name maps as identity.
Fail-fast boundary: missing/duplicate/foreign/shadowed binding, carrier, or
join output must reject before physical block or PHI allocation.
Smallest next slice: close this B3 candidate as NoSafeSlice and return selection
to the family scheduler; do not open its I0 or invent a key issuer here.
Non-claims: no value/origin branch implementation yet, tail push, runtime
Text/Fault, serializer, VM/AOT parity, or shared MethodCall retirement.
```

## Confirmed current boundary

`CallableSemanticLoopHandoffPreEffectReceiptV1` carries exact BindingRefs,
source assignment receipts, and declaration scope. The local
`LoopCondRecipe<T>` contains only `RecipeBody` and `Vec<T>`; its
`LoopCondBreakContinueItem::GeneralIf(NoExitBlockRecipe)` retains structural
body/statement references, not semantic carrier or join-output keys.
`SourceLoopCondPhysicalInputV1` aggregates the pre-effect receipt, selected
planner Facts/Recipe, source forest, item/target relations, and source port,
but has no source-bound portable Core, carrier-key relation, or JoinSig
continuation. The current physical consumer collects carrier names from AST and
`LoopCondBreakContinuePhiMaterializer::prepare()` consumes `&[String]`, using
those names for entry values, header/step/after PHIs, and final values. The
source port is the plain `CallableLoopSourceExpressionPortV1`; the existing
GenericLoop `CallableLoopCarrierRelationV1` is issued from GenericLoop facts
and cannot be borrowed as proof for LoopCond. Portable `LoopBindingKeyV1`,
`LoopCarrierKeyV1`, `VerifiedLoopCoreProductV1`, and
`VerifiedLoopContinuationContractV1` exist in `loop_recipe_contract`, but the
caller-zero `CallableSingleLoopRecipeCoSealV1` path is a separate admitted
profile and is not connected to this LoopCond source product. A matching owner
or loop shape does not authorize pairing them.

The missing relation is needed before I0 can safely attach a GeneralIf join
ValueId to a source BindingRef. A name-keyed join followed by a name lookup in
the ledger would create a new identity authority. The physical binding may
remain a label, but the source binding must arrive through one canonical
semantic-program issuance that co-seals source context, source-bound Core,
Recipe-owned keys/relations, and JoinSig continuation before physicalization.
No such issuer is currently connected to this LoopCond route. Adding a detached
relation to `SourceLoopCondPhysicalInputV1` would make that aggregate issue
semantic meaning it does not own.

## D2 result: NoSafeSlice for this candidate

The read-only audit distinguishes three existing products; none can issue the
required tuple in this route:

| product | current authority | why it cannot issue the LoopCond tuple |
| --- | --- | --- |
| `LoopCondRecipe<T>` / `LoopCondBreakContinueFacts` | local planner composer | stores structural recipe items only; no Recipe carrier keys or JoinSig continuation |
| `CallableLoopCarrierRelationV1` | GenericLoop source admission | tied to `GenericLoopV1Facts` and its route admission; not a LoopCond product |
| `VerifiedLoopRecipeCoSealV1` / `VerifiedCallableSemanticProgramV1` | portable callable-loop issuer | owns portable Recipe/Core/JoinSig/continuation for its admitted profile, but this LoopCond route does not consume or co-issue it |

Fast-path entry is unmet: selecting a key or join here changes semantic
identity, and no connected producer owns that meaning for this LoopCond route.
Therefore D2 cannot honestly specify “wire the existing key” or make the
current name-keyed physical maps key-based by adapter. Doing so would either
invent Recipe authority in the physical input, borrow an unrelated issuer, or
pair products after issuance. All violate the semantic-program boundary. The
exact I0 remains unauthorized, and the production/old-edge delta for this B3
candidate is zero.

This is a family-local disposition. The 2026-09-23 scheduler pass found no
ready Promote/Stop/Delete candidate in the existing ordered queue: fixed EXE
acceptance is closed; M7-S has no remaining exclusive delete set; the finite
R7 owner list is closed/retained/deferred/`NoSafeSlice`; M10's semantic co-seal
is `NoSafeSlice`, its transfer/boundary/selected If/Exit prerequisites are
closed, and its S6C caller-zero branch has no selected production
consumer/delete set. Recursive String-result and parser composite-loop rows
also remain `NoSafeSlice`. The current pointer records an explicit frontier
pause; this does not close the overall MirBuilder goal.
Do not reopen a parked lane or append another B3 suffix. Reopen this candidate
only after an accepted existing semantic-program issuer can represent the
exact source→Core→Recipe/JoinSig→carrier/join relation as one issuance, with a
named physical consumer and old-edge disposition.

### Premise-reset circuit breaker check (2026-09-23)

Not triggered. The 2026-09-19 parser forest/exit co-seal concerns a different
`ParserProgramBox.parse/2` profile ([D0](mir-call-parser-loop-forest-exit-coseal-d0-2026-09-19.md));
the 2026-09-21 accepted resolver IfThen/IfElse ancestry extension and its I0
were intervening progress ([D0](mir-call-parser-loopbreak-composite-forest-boundary-d0-2026-09-21.md),
[I0](mir-call-parser-loopbreak-composite-forest-path-i0-2026-09-21.md)). This
B3 `StringHelpers.split_lines` carrier/join relation and M8 S6D's explicit
Break/Continue predicate/effect product ([S6D decision](../design/joinir-loop-selfhost-recipe-pipeline-ssot.md))
are distinct bounded LoopCond responsibilities, despite sharing the semantic-
program boundary. They are not three consecutive NoSafeSlice outcomes for one
responsibility. No connected LoopCond issuer or same-series consumer/delete
tuple was found, so retain the frontier pause; do not append another B3 suffix
or start implementation.

Finite state disposition:

| state | authority | pre-effect behavior | terminal | fallback |
| --- | --- | --- | --- | --- |
| ConnectedCanonicalProgram | none observed for this LoopCond route | not selectable | no terminal issued | none |
| NoSafeSlice | D2 source/type audit | do not allocate or lower B3 | retain this named design stop | no GenericLoop/name-map/AST recovery |
| RawCompatibilityLoopCond | existing raw route only | outside selected source-backed B3 claim | existing raw owner | unchanged compatibility behavior |

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

1. **Name the existing producer.** Resolved negatively: no producer connected
   to this LoopCond route owns the required portable key and continuation tuple.
   The source/pre-effect rows provide BindingRefs, but the local LoopCond
   Recipe does not issue keys and the separate portable issuer is not paired.

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

7. **D2 exit.** NoSafeSlice is established for this candidate. Keep I0
   suspended, production callers unchanged, and the delete set empty. Select
   another already-inventoried action under the family scheduler before
   implementation; do not infer that this family-local result blocks the repo.

## Preserved D1 decision and implementation boundary

When implementation is admitted, branch state resets only current ledger
values and active origins. Reads, assignments, calls, materialized locals,
removed source rows, named-array emission ports, and historical origins remain
monotonic because both branches are compiled once. The shared raw MethodCall
writer remains retained; its delete set is empty.

The post-Loop `arr.push(s.substring(last))` row, runtime Text/Fault ownership,
backend acceptance, and serializer remain outside this D2 census.
