---
Status: design stop
Date: 2026-07-26
Decision: STAGEB-GENERIC-LOOP-TRANSIENT-TYPE-D0
Classification: BoxCount / T2 interprocedural result-representation authority
Blocked row: OWN-GRAM-REJECT0 Hako transport half
Related:
  - docs/development/current/main/investigations/static-box-derive-compat-d0-design-question-2026-07-26.md
  - docs/development/current/main/workstreams/language-v1-convergence-current.md
---

# Stage-B generic-loop transient type authority

## Observed baseline

After `STATIC-BOX-DERIVE-COMPAT0-S0` removes the invalid generated instance
method from static Main, the unchanged Stage-B guard reaches its first later
failure:

```text
[plan/freeze:contract] generic_loop_v1 skeleton failed:
GenericLoop carrier representation failed:
MissingTransientType { init: ValueId(28) }
```

The failing value comes from the exact nested call shape in
`ParserBox.static_const_parse_add`:

```text
pos = ParserStringUtilsBox.skip_ws(
  text,
  me.static_const_eval_pos(ret),
)
```

## Established authority

```text
lowering-time transient result type
  = MirBuilder.function_state.type_ctx.value_types

GenericLoop skeleton
  = consumer/verifier only
  -> reads loop variable binding
  -> reads the init ValueId type from type_ctx
  -> rejects Missing or Unknown before blocks, claims, or body effects
```

The current exact-i64 callable catalog is intentionally proof-only for its
selected call sites. Actual ParserBox rows with a nested instance result remain
unselected, so neither the inner call nor the outer call has authority to
publish an Integer transient type at this point.

## Non-authority

```text
GenericLoop condition / step / loop role
final metadata or completed-module facts
callee name or module inventory
runtime observation
source annotation
retry through another route
```

None may infer or backfill `ValueId(28)`.

## Decision required

### A — derive a type inside GenericLoop (rejected)

This turns the consumer/verifier into an interprocedural result owner and
creates an unauditable second type source.

### B — borrow a later or heuristic source (rejected)

Using final metadata, callee naming, source annotations, runtime values, or a
route retry violates the lowering-time exact-carrier boundary.

### C — select a dedicated nested-result representation owner (recommended)

```text
CALLABLE-RESULT-NESTED-REP0
  source instance-result contract
  + exact source-site evidence
  + final remapped destination after successful call emission
  -> one Integer transient type publication
```

This is a new T2 owner. It must not be smuggled into the Stage-B guard or the
GenericLoop consumer. If the owner cannot be selected with exact source proof,
the correct result remains a typed pre-effect rejection.

## Failure law

```text
MissingTransientType
  -> before block allocation
  -> before carrier claims
  -> before loop body effects
  -> no fallback or retry
```

Existing focused regressions already state this boundary:

```text
actual_canonical_prefix_rejects_untyped_numeric_loop_carrier_before_claims
actual_parser_source_gate_is_all_unselected_without_activation_or_builder_state
actual_string_helpers_and_parser_wrapper_share_one_complete_result_catalog
```

## Required decision product

```text
source authority
  = exact nested instance-call result owner, or no admission

publish point
  = successful physical call emission only

carrier
  = one exact transient type for the remapped destination

forbidden
  = persistent site-to-ValueId map
  = GenericLoop inference
  = metadata/runtime/name/source-annotation recovery
```

## Non-claims

```text
GenericLoop representation repair
parser or Hako source edit
ownership grammar activation
VM/backend/PHI/finalization change
fallback or retry
OWN-GRAM Hako transport resumption
```
