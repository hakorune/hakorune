# Callable single-loop source map S1

Status: `planned after GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-RECIPE-MAP-D0; implementation not opened`

Parent: `GENERIC-CALLABLE-SINGLE-LOOP-SOURCE-RECIPE-MAP-D0`

## Change

Publish one caller-zero, AST-free source map for the selected
`StringHelpers.int_to_str/1` single-loop profile. The map consumes the
resolver-owned callable ledger and exposes exact source roles for the initial
carrier, condition read/bound/operator, step read/delta/operator, assignment
target/value, prefix boundary, and terminal tail. It does not publish a
portable Recipe, allocate `ValueId`/PHI/CFG, or call a Builder route.

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

## Acceptance

- Positive fixture matches the immutable D0 design fixture and seals every
  source row exactly once.
- Negative fixtures cover missing/duplicate/foreign owner or frame,
  wrong Scope/Region, unsupported operator/type/literal, binding mismatch,
  second/nested loop, missing initial carrier/prefix, and non-terminal tail.
- The map proves whole-callable declaration/reference/assignment/exit
  coverage is available to the outer canonical plan; it does not claim that
  the Loop-only physicalizer can finish the function.
- Focused tests are caller-zero and the reference/current/workstream docs are
  updated in the same commit as the implementation.

## Stop

Return to design if the resolver cannot issue exact operator/RHS/step/literal
rows or Scope/Region identity, if the prefix/tail boundary is not representable,
or if a Recipe/ValueId/CFG/PHI decision is needed. Those belong to
`RECIPE-S2` and `PHYS-S3`, not this row.
