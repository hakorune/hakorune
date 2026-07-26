---
Status: accepted design / execution handoff
Date: 2026-07-26
Decision: STAGEB-GENERIC-LOOP-TRANSIENT-TYPE-D0
Classification: BoxCount / T2 interprocedural result-representation authority
Selected owner: CALLABLE-RESULT-NESTED-REP0
First executable row: CALLABLE-RESULT-NESTED-REP0-S0
Blocked row: OWN-GRAM-REJECT0 Hako transport half (parked until this exact
result-representation boundary reaches its closeout)
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

## Decision

### A — derive a type inside GenericLoop (rejected)

This turns the consumer/verifier into an interprocedural result owner and
creates an unauditable second type source.

### B — borrow a later or heuristic source (rejected)

Using final metadata, callee naming, source annotations, runtime values, or a
route retry violates the lowering-time exact-carrier boundary.

### C — dedicated nested-result representation owner (accepted)

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

The selected owner is deliberately narrower than an interprocedural type
inference framework:

```text
one exact current-owner nested instance-result source site
  + its sealed result contract
  + its one successful physical Call destination after remap
  -> one exact Integer type publication
```

It does not make all instance calls typed, create a global callee result
table, or turn a source annotation into a lowering-time type fact.

## Owner chain

```text
exact source MethodCall site
  -> SealedNestedInstanceResultSiteV1
  -> SealedNestedInstanceResultContractV1
  -> PreparedNestedInstanceResultEmissionV1
  -> successful physical Call emission with final remapped destination
  -> PublishedNestedInstanceResultV1
  -> consume_once(type_ctx)
  -> type_ctx.value_types[final_destination] = Integer
  -> existing GenericLoop verifier
```

The site product is source-only.  The emission product is local to one
lowering transaction.  `PublishedNestedInstanceResultV1` is non-Clone and has
one consuming terminal; it cannot survive as a site-to-`ValueId` registry.

The first selected source shape is the observed receiver call in the Parser
fixture:

```hako
me.static_const_eval_pos(ret)
```

used as the nested argument of `ParserStringUtilsBox.skip_ws`.  The selection
must be made from the existing source-site route and exact callee-result
contract, never from the outer `skip_ws` call, loop role, names, metadata, or
runtime values.

## Authority and failure law

```text
source contract authority
  = SealedNestedInstanceResultContractV1

final destination authority
  = successful physical Call emission receipt

type write authority
  = PublishedNestedInstanceResultV1::consume_once

GenericLoop
  = pure consumer/verifier; unchanged
```

Before physical call success, a selected nested result has no type write.
After a call failure, no type write occurs.  The existing function transaction
remains the rollback boundary, so a failure leaves no persistent type fact and
no partial module publication.

Forbidden:

```text
GenericLoop type inference or backfill
callee-name / outer-call / condition / step inference
source annotation, final metadata, or runtime recovery
persistent source-site -> ValueId map
post-failure type publication
retry through Raw, legacy, or another route
Hako source workaround
```

## Executable series

### CALLABLE-RESULT-NESTED-REP0-S0 — source contract and exact selection

Builder-free, MIR-free source-side owner only.

```text
exact current-owner MethodCall route
  -> sealed site evidence
  -> exact Integer-result contract or typed unselected rejection
```

S0 does not allocate a `ValueId`, write `type_ctx`, emit a call, or broaden the
existing proof-only callable catalog.  It retains the source owner on failure
and exposes only `stage()`, `cause()`, and `discard(self)`.

### CALLABLE-RESULT-NESTED-REP0-P0 — emission-local receipt

Prepare a non-Clone receipt which combines the S0 contract with the exact
lowering site.  The sole later input is the final destination returned by the
existing successful physical call emitter.  This row has no `type_ctx` write
and no persistent map.

### CALLABLE-RESULT-NESTED-REP0-I0 — success-only publication

Connect the receipt to the existing unified/member call success boundary.
Only a successfully emitted physical call may produce
`PublishedNestedInstanceResultV1`, and only its consuming terminal may insert
`MirType::Integer` for that final remapped destination.  Existing call result
owners, GenericLoop, VM/backend policy, and function finalization remain
unchanged.

### CALLABLE-RESULT-NESTED-REP0-P0b — focused Stage-B parity

Run the real `ParserBox.static_const_parse_add` path through the existing
Stage-B guard.  Prove the exact nested destination is typed before the
GenericLoop skeleton asks for it, while missing/unselected and failed-call
fixtures still reject before loop claims/body effects.  Reuse the existing
generic-loop and callable-result proof utilities; do not add a row-specific
shell guard.

### CALLABLE-RESULT-NESTED-REP0-G0 — closeout and reuse

Require:

```text
source contract producer                    = 1
successful-call type publisher              = 1
type write before physical success           = 0
persistent source-site -> ValueId mapping    = 0
GenericLoop type producer                    = 0
metadata/runtime/name/annotation recovery    = 0
fallback/retry                               = 0
failure -> later fresh compiler success      = green
```

After G0, rerun the exact Stage-B guard.  Only if that guard reaches the
parked ownership-syntax boundary may `OWN-GRAM-REJECT0-HAKO0-S0` resume; this
series does not silently claim that its whole unrelated Hako half is closed.

## Proof and ceremony budget

```text
ceremony_tier             = T2
proof_inventory_before    = existing generic-loop carrier and callable-result
                            source-proof guards
new_proofs                = focused source-contract / success-only receipt
                            fixtures only
retired_or_merged_proofs  = none in this series
net_proof_delta           = positive, bounded T2 safety evidence
sunset_id                 = CALLABLE-RESULT-NESTED-REP0-PROOF-SUNSET-001
sunset_row                = CALLABLE-RESULT-NESTED-REP0-RETIRE0-S0
retire_when               = a generalized canonical callable-result contract
                            owns this exact source/effect pairing and the
                            nested-specific proof consumer count is zero
budget_repayment_evidence = generalized owner parity plus zero direct
                            nested-receipt consumers
```

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
  = SealedNestedInstanceResultContractV1, or no admission

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
