# Callable single-loop source map S1

Status: `ready; caller-zero MAP-S1 implementation; production route remains closed`

Parent: `GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-RECIPE-MAP-D0`

Dependency: `RESOLVER-SYNTAX-FACTS-D0`

## Change

Publish one caller-zero, AST-free source map for the selected
`StringHelpers.int_to_str/1` single-loop profile. The map consumes the
resolver-owned callable ledger and exposes exact source roles for the initial
carrier, condition read/bound/operator, step read/delta/operator, assignment
target/value, prefix boundary, and terminal tail. It does not publish a
portable Recipe, allocate `ValueId`/PHI/CFG, or call a Builder route.

## MAP-S1 API audit (2026-08-07)

The existing resolver ledger safely provides owner/origin/source-kind, typed
source inventory, variable reads, assignment targets, direct-call identity,
exit identity, Loop frame, and Loop region topology. It does not provide the
operator, RHS literal, initializer value, prefix-call-to-binding relation, or
terminal-tail shape required by the D0 table. Filling those rows from AST,
`variable_map`, path suffixes, or a second resolver is forbidden.

This row is therefore closed as `NoSafeSlice` until a separate syntax-facts
design chooses the authority and seal for those facts. A partial map containing
only condition/body reads would not prove the selected profile and must not be
promoted.

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

## Acceptance

- Positive fixture matches the immutable D0 design fixture and seals every
  source row exactly once.
- Negative fixtures cover missing/duplicate/foreign owner or frame,
  wrong Scope/Region, unsupported operator/type/literal, binding mismatch,
  second/nested loop, missing initial carrier/prefix, and non-terminal tail.
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
