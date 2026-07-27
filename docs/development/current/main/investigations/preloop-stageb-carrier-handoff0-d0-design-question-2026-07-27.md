---
Status: design consultation required
Date: 2026-07-27
Decision: PRELOOP-STAGEB-CARRIER-HANDOFF0-D0
Closes observation row:
  - CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0
Observed frontier:
  - ProductionCarrierHandoffMissing
Related:
  - preloop-physical-route-reconciliation0-task-order-2026-07-27.md
  - stageb-generic-loop-transient-type-d0-design-question-2026-07-26.md
  - src/mir/builder/calls/preloop_nested_result_receipt.rs
  - src/mir/builder/calls/preloop_nested_result_type.rs
  - tools/checks/generic_loop_progression_role_v0_guard.sh
---

# Pre-loop Stage-B Carrier Handoff Design Question

## Consultation stop

The exact Stage-B guard was rerun after TYPE-I0-G0:

```bash
bash tools/checks/generic_loop_progression_role_v0_guard.sh
```

The guard remained green because it still expects and observes the old
frontier:

```text
ParserBox.static_const_parse_add/2
-> GenericLoop carrier representation failed:
   MissingTransientType { init: ValueId(28) }
```

This is not evidence that ownership syntax or loop-refresh is next. The first
exact frontier is:

```text
ProductionCarrierHandoffMissing
```

No implementation row is authorized until this D0 selects the source owner,
the outer-carrier physical receipt, and the function-scoped consuming handoff.

## Factual correction

TYPE-I0 proved one internally consistent receipt-backed fact publication:

```text
exact selected inner source contract
+ successful physical inner Method Call
+ successful containing outer Call
-> EmittedNestedInstanceCallV1
-> Integer fact publication for receipt.final_destination
```

However, that destination is the selected inner call:

```hako
me.static_const_eval_pos(ret)
```

The GenericLoop carrier is the value assigned to `pos`, which is the result of
the containing outer call:

```hako
pos = ParserStringUtilsBox.skip_ws(
    text,
    me.static_const_eval_pos(ret),
)
```

Therefore:

```text
inner Method Call destination
  !=
outer skip_ws Call destination
  ==
assignment / GenericLoop init destination
```

Publishing Integer for the current inner receipt cannot establish the outer
carrier fact. Treating the two destinations as interchangeable is forbidden.

## Current caller census

The following surfaces have test-only consumers:

```text
PreparedPreloopLocatedArgumentV1 construction
PreloopLocatedArgumentPortV1::new
PreloopLocatedArgumentPortV1::into_emitted_nested_result
publish_preloop_nested_integer_result_v1
```

The exact Stage-B guard still reaches the legacy production route:

```text
compile_legacy_request / build_module
-> lower_root
-> lower_method_as_function
-> build_instance_method_draft_v1
-> setup_method_params
-> lower_method_body
-> cf_block / drive_legacy_block_v1
-> RawLegacyChildLoweringPortV1
-> GenericLoop suffix planning
```

It does not issue the same-allocation pre-loop source association, install the
candidate Port, retain an outer carrier receipt, or invoke TYPE-I0.

The repository also contains a port-aware Raw invocation sibling through
`RawInvocationChildPortV1` and
`build_instance_method_draft_with_port_v1`, but that is not the caller used by
this exact Stage-B guard. Activating or selecting that sibling is therefore a
decision, not an observed fact.

The current production-prefix tests also use two catalog roles: the source
association borrows an external exact catalog, while the configured Builder
installs a fresh equal-looking catalog for lowering/header lookup. Equality of
content is not same-allocation authority. Production design must not preserve
this test-only split or create a self-referential Builder borrow.

## Preserved authorities

```text
source declaration/catalog:
  VerifiedSameModuleCallableDeclarationCatalogV1

inner instance result:
  SealedNestedInstanceResultContractV1

physical Call emission:
  existing UnifiedCallEmitterBox writer

type fact decision:
  existing TypeFactDecisionV1

type fact write:
  existing TypeContext::set_type

GenericLoop:
  consumer only
```

This D0 must not create a second Call writer, type policy, source navigator, or
GenericLoop publisher.

## Q1 — source activation owner

Which owner should convert the borrowed exact source evidence into a
production-usable, owned, one-row activation plan before Builder effects?

### A — separate owned pre-loop carrier activation plan

Seal one owned row from the same declaration-catalog allocation:

```text
caller identity
outer StaticReceiver call site
structural CallArgument(1)
inner same-owner MethodCall site
exact inner Integer contract
exact outer carrier result contract
```

The plan owns the catalog until a consuming split/co-install terminal. The
exact row then stays function-scoped and stack-scoped. It must also establish
how one allocation supplies source association and lowering/header facts
without borrowing a Builder-owned catalog across mutable lowering.

### B — widen `VerifiedCallableResultActivationPlanV1`

Add the outer-plus-nested relation to the existing static callable-result
activation plan.

This is acceptable only if the existing owner can represent the inner
instance contract and the outer carrier relation without conflating static
result authority, nested instance authority, or loop-refresh.

### C — reconstruct inside method lowering

Rescan the method AST, names, or source paths when lowering
`ParserBox.static_const_parse_add/2`.

This conflicts with the existing same-allocation source authority and is not
recommended.

### D — Builder registry or GenericLoop inference

Store source-site metadata in `MirBuilder`, or let GenericLoop infer Integer
from source/callee/runtime facts.

This conflicts with the persistent-map and consumer-only laws and is not
recommended.

## Q2 — function-scoped handoff

Where should the selected activation enter the actual instance-method route?

Candidate seams:

```text
legacy production:
  build_instance_method_draft_v1

existing port-aware sibling:
  build_instance_method_draft_with_port_v1

both require:
  -> skeleton/signature
  -> setup_method_params
  -> install exact function-scoped activation
  -> body lowering through one scoped Port
```

Required properties:

```text
ordinary methods remain unchanged
selected activation is consumed exactly once
no optional override on the ordinary Raw port
no Builder-wide source-site registry
no second AST scan or catalog reseal
failure cannot retry through ordinary lowering
```

The exact source/effect correspondence is rooted at the located Body(3)
assignment/RHS boundary, not at the emitter and not at GenericLoop. The chosen
function-scoped owner must preserve that assignment relation while delegating
physical calls to the existing terminals.

The decision must say whether to:

```text
select the existing port-aware Raw invocation owner for this exact function
add a bounded port-aware sibling under the current legacy function session
or place the activation in a separate outer function/body transaction
```

It must not silently reinterpret the exact guard as already using
`RawInvocationChildPortV1`.

## Q3 — outer carrier receipt

What product proves that the actual outer `skip_ws` Call succeeded and exposes
its final physical destination without reusing the inner receipt?

Candidate shape:

```rust
CompletedPreloopOuterCarrierCallV1 {
    nested_result_evidence,
    outer_result_contract,
    outer_physical_destination,
}
```

Required temporal law:

```text
inner Call failure
  -> outer receipt = 0

outer Call failure
  -> outer receipt = 0

successful outer physical Call
  -> exactly one outer carrier receipt

receipt absent
  -> outer Integer publication = 0
```

The existing unified Call writer remains the sole physical writer. The outer
receipt may only be committed by the existing outer success branch.

## Q4 — outer result authority

Which sealed evidence proves that the outer `skip_ws` result is Integer?

Candidates include:

```text
existing static callable-result activation evidence
an exact projection from the owned pre-loop activation plan
a new bounded outer-carrier contract
```

The answer must not infer the result from:

```text
callee or Box spelling
the inner Integer argument
runtime value
GenericLoop use
method-name policy
```

If the outer result needs a new fact policy, it must reuse
`TypeFactDecisionV1` and `TypeContext::set_type` and open a separate
`PRELOOP-OUTER-CARRIER-TYPE-I0-D0`.

## Q5 — failure and publication law

Required minimum:

```text
source/admission failure:
  Builder effects = 0

function ingress / route mismatch:
  candidate function publication = 0
  alternate route retry = 0

inner or outer Call failure:
  outer carrier receipt = 0
  outer type write = 0

type conflict:
  existing concrete fact retained
  candidate function/module publication = 0

success:
  outer destination receives one exact fact
  GenericLoop only reads that fact
```

The retained rejection owner must keep the owned activation evidence until
inspection and discard. It must not expose retry, resume, source recovery, or
fallback.

## Recommended task boundary after decision

The exact task names may be adjusted by the accepted design, but the semantic
order must remain:

```text
PRELOOP-STAGEB-CARRIER-CORRESPONDENCE0-P0
  prove inner destination != outer carrier destination
  freeze production consumer count = 0
  mark the current Stage-B guard expectation as pre-activation evidence

-> PRELOOP-STAGEB-SOURCE-ACTIVATION0-S0
   Builder-free owned source plan
   production caller = 0

-> PRELOOP-STAGEB-FUNCTION-INGRESS0-I0
   one exact instance-method consumer

-> PRELOOP-OUTER-CARRIER-RECEIPT0-S0
   successful outer Call destination receipt

-> PRELOOP-OUTER-CARRIER-TYPE-I0-D0
   only if outer fact disposition is not already sealed

-> PRELOOP-OUTER-CARRIER-TYPE-I0
   one success-only fact publication

-> CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0
   rerun the real Stage-B guard
```

## Structural gate

```text
same declaration-catalog allocation              = 1
owned pre-loop activation producer               = 1
exact instance-method activation consumer        = 1

inner destination authority                      = existing 1
outer carrier destination authority              = exact 1
inner destination treated as outer               = 0

physical Call writer                             = existing 1
second Call writer                               = 0

TypeFactDecisionV1 authority                     = existing 1
TypeContext::set_type authority                  = existing 1
direct type map insert                           = 0

GenericLoop type producer                        = 0
Builder source-site registry                     = 0
persistent SourceExprSite -> ValueId map          = 0
callee/Box-name selection                        = 0
AST rewalk / catalog reseal                      = 0

loop-refresh activation                          = 0
ownership grammar activation                     = 0
fallback / retry / route reselection             = 0

all modified/new source/check files              < 800 lines
```

## Non-claims

```text
production activation selected before this D0
current TYPE-I0 proves the outer carrier
loop-refresh activation
GenericLoop publisher migration
general instance-call result inference
general located lowering activation
ownership syntax / Alias / View activation
parser / Hako / VM / backend change
default route cutover
fallback / retry
```
