# Callable single-loop source map S1

Status: `closed; caller-zero MAP-S1 source-map product; RECIPE-COSEAL-D0 is next design stop`

Parent: `GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-RECIPE-MAP-D0`

Dependency: `RESOLVER-SYNTAX-FACTS-D0`

## Change

Publish one caller-zero, AST-free source map for the selected
`StringHelpers.int_to_str/1` single-loop profile. The map consumes the
resolver-owned callable ledger and exposes exact source roles for the initial
carrier, condition read/bound/operator, step read/delta/operator, assignment
target/value, prefix boundary, and terminal tail. It does not publish a
portable Recipe, allocate `ValueId`/PHI/CFG, or call a Builder route.

## MAP-S1 API audit (2026-08-07; superseded by SyntaxFacts-S1)

The ledger provides owner/origin/source-kind, typed source inventory, variable
reads, assignment targets, exit identity, Loop frame, and Scope/Region
topology. The resolver does not own operator, literal, initializer, or
call-boundary policy. `VerifiedSourceSyntaxFactsV1` now supplies those neutral
shapes; the MAP product joins them without copying AST or minting a second
resolver authority. `variable_map`, path suffixes, names, and fallback remain
forbidden.

## Contract

- Every row retains typed source site, source role, owner/origin/source-kind,
  resolver-issued Loop source/frame, and resolver-issued Scope/Region pair.
- One source expression may map to several roles; coverage key is
  `(source_site, role, target_kind)`, not source-site uniqueness alone.
- Non-synthetic rows carry exact source anchors. Carrier/JoinSig glue is
  explicit derived data and cannot masquerade as a source row.
- Prefix `value` and terminal `return value` are part of whole-callable
  coverage but remain outside the Loop Recipe. The map never re-lowers them.
- Resolver identity is the only source authority. AST/name/path-suffix lookup,
  ordinal inference, lowering-state `variable_map`, and a second resolver are
  forbidden.
- Missing, duplicate, foreign, nested, opaque, unsupported, or incomplete rows
  decline before any Builder effect. No retry, fallback, reselect, or legacy
  route repair is allowed.

After the dependency is accepted, the map input will be the intersection of
resolver facts and a sanctioned `VerifiedSourceSyntaxFactsV1` product joined
by owner-branded typed sites. The map remains AST-free and compiler-side; the
syntax observer is not a Recipe or physical owner.

The dependency sequence is intentionally finite: the caller-zero
`SyntaxFacts-S1` product is now sealed after `RESOLVER-SYNTAX-FACTS-D0`, so
this `MAP-S1` row is the next implementation boundary. No row-specific D0
suffixes are authorized.

## Implementation receipt (2026-08-07)

`callable_single_loop_source_map.rs` now publishes the sealed
`VerifiedCallableSingleLoopSourceMapV1` product under `cfg(test)`. It co-seals
the nine fixed syntax roles plus the separate prefix boundary with the
resolver ledger:

```text
InitialCarrier
ConditionRead / ConditionBound / ConditionOperator
StepRead / StepDelta / StepOperator / StepWrite
TailReturnRead
PrefixBoundary
```

Every row retains a typed source site and role. Binding reads and the step
write join exact resolver `BindingRefV1`/assignment evidence; the tail joins
one explicit terminal return; loop source/frame/Scope/Region are reissued from
the ledger and compared in full with the syntax context rather than
reconstructed from raw paths. The initial carrier, condition read, step
read/write, and loop-side assignment share one exact `BindingRef`; the tail is
checked as an exact lexical read and terminal return because this fixture
returns the separate prefix `value` binding. The prefix is an outer-callable
boundary with neutral call shape and its local declaration binding. An
applicable resolver direct-call receipt is joined exactly and retained as an
optional target; this selected MethodCall has no canonical callable target and
remains an outer-plan boundary. No target or Recipe meaning is invented.

MAP-side profile policy admits only initial literal `0`, condition/step literal
`1`, condition operator `Less`, and step operator `Add`; typed/other literals
and other operators reject before any Builder effect.

Four focused tests are green: positive nine-row-plus-prefix sealing,
source-unit lifetime independence, foreign-owner rejection, and a typed
out-of-profile condition-bound rejection. SyntaxFacts
continues to own nested/shape/cardinality rejects; MAP rejects missing or
duplicate resolver evidence before any Builder effect. No Recipe, JoinSig,
ValueId, CFG, PHI, physicalizer, production caller, retry, fallback, or
legacy-retirement route was added.

## Acceptance

- Positive fixture matches the immutable D0 design fixture and seals every
  source row exactly once.
- The focused implementation tests cover positive sealing, source lifetime,
  foreign-owner rejection, and one profile-policy negative. SyntaxFacts supplies the second/nested-loop,
  shape, and tail/cardinality rejects; MAP's typed reject enum covers missing,
  duplicate, binding, assignment, and terminal-return evidence. Expanding
  each negative fixture remains part of the later Recipe co-seal gate.
- The map joins the syntax obligations to the ledger and proves that
  whole-callable declaration/reference/assignment/exit coverage is available
  to the outer canonical plan; the syntax observer alone does not own that
  proof, and the map does not claim that a Loop-only physicalizer can finish
  the function.
- Focused tests are caller-zero and the reference/current/workstream docs are
  updated in the same commit as the implementation.

## Stop

Return to design if the syntax-facts owner is not sealed, if any operator/RHS/
step/literal/prefix/tail row remains opaque, or if a Recipe/ValueId/CFG/PHI
decision is needed. When this row is green, stop at one
`RECIPE-COSEAL-D0` design decision for the common Recipe/JoinSig/effect/
After/Tail and Scope/Region/frame co-seal. Physicalization, production
selection, retry/fallback retirement, and legacy deletion remain closed until
that design stop is accepted.

## Closeout

- Result: caller-zero `VerifiedCallableSingleLoopSourceMapV1` is sealed with
  nine source rows plus one prefix boundary.
- Authority: SyntaxFacts owns neutral source shape; the resolver ledger owns
  owner, BindingRef, assignment, exit, Loop source/frame, and Scope/Region.
- Boundary: the product is AST-free and downstream-only; it does not issue
  Recipe/JoinSig/ValueId/CFG/PHI or enter Builder/MIR.
- Verification: focused MAP tests and `cargo check --lib` are green; the
  source map remains under the 800-line source limit.
- Next: open one shallow `RECIPE-COSEAL-D0` design stop. Do not deepen the
  ladder or open physical/production/legacy work from this closeout.
