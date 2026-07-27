---
Status: accepted design; function preparation row closed
Date: 2026-07-27
Decision: PRELOOP-STAGEB-CARRIER-HANDOFF0-prime-r1
Closes:
  - PRELOOP-STAGEB-CARRIER-HANDOFF0-D0
Closes observation row:
  - CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0
Observed frontier:
  - ProductionCarrierHandoffMissing
First executable row:
  - PRELOOP-STAGEB-SOURCE-PRODUCER-SELECTION0-D0
Related:
  - preloop-physical-route-reconciliation0-task-order-2026-07-27.md
  - stageb-generic-loop-transient-type-d0-design-question-2026-07-26.md
  - src/mir/builder/calls/preloop_nested_result_receipt.rs
  - src/mir/builder/calls/preloop_nested_result_type.rs
  - tools/checks/generic_loop_progression_role_v0_guard.sh
---

# Pre-loop Stage-B Carrier Handoff Design Question

## Accepted closeout

```text
Q1 source activation:
  A-prime
  separate owned one-row pre-loop carrier activation plan

Q2 function-scoped handoff:
  existing legacy function transaction
  + existing port-aware draft/body preparation seams
  + exact selected instance-method body-handoff sibling

  RawInvocationChildPortV1 / collector publication cutover:
    rejected by post-answer code audit

Q3 outer carrier receipt:
  CompletedPreloopOuterCarrierCallV1
  owns the actual successful outer generic Call receipt

Q4 outer result authority:
  existing static exact-i64 result proof
  + exact structural CallArgument(1)
  + sealed inner exact-i64 contract
  -> bounded outer Integer contract

Q5 publication:
  existing TypeFactDecisionV1
  + existing TypeContext::set_type
  -> success-only publication to the assignment-published outer carrier
```

The current inner receipt and TYPE-I0 remain internally valid for the selected
inner `me.static_const_eval_pos(ret)` call. They are not the outer carrier
authority and must never be reused as one.

The accepted chain is:

```text
one exact declaration-catalog allocation
  -> borrowed outer static-result proof
  -> structural CallArgument(1)
  -> sealed inner Integer contract
  -> PreparedPreloopStageBCarrierRowsV1
  -> VerifiedPreloopStageBCarrierActivationPlanV1
  -> one module install of the existing callable catalog
  -> one exact selected instance-method capture
  -> Body(3) prefix / selected assignment / suffix handoff
  -> successful inner physical Call receipt
  -> successful outer physical Call receipt
  -> assignment destination correspondence
  -> TypeFactDecisionV1
  -> TypeContext::set_type(assigned_outer_carrier, Integer)
  -> existing GenericLoop consumer
```

### Selected authorities

```text
source/catalog identity:
  VerifiedSameModuleCallableDeclarationCatalogV1

static outer result evidence:
  VerifiedSameModuleCallableResultCatalogV1
  projected as one bounded exact-i64 requirement

inner instance result:
  SealedNestedInstanceResultContractV1

source navigation:
  existing SourcePathV1 / child-role / Raw callable-source view

function lifecycle:
  existing CanonicalFunctionLoweringSessionV1
  existing skeleton/signature/parameter/finalizer owners
  bounded port-aware body descent sibling

function publication:
  existing current-module draft publication after session success

physical Call:
  existing UnifiedCallEmitterBox writer
  existing CompletedUnifiedValueCallEmissionV1

type fact decision:
  existing TypeFactDecisionV1

type fact commit:
  existing TypeContext::set_type

GenericLoop:
  consumer only
```

### Explicit non-authorities

```text
callee or Box spelling
the inner destination as the outer destination
runtime values
GenericLoop use
Builder-wide source registries
persistent SourceExprSite -> ValueId maps
AST re-walk or catalog reseal
direct value_types insertion
fallback, retry, or route reselection
```

## Exact accepted products

### Source result contract

The first bounded contract accepts only:

```text
outer target result:
  ExactI64

required argument set:
  exactly [structurally issued CallArgument(1)]

selected argument:
  same source-view child as the sealed inner contract

inner result:
  unconditional ExactI64
```

The source-result module may issue a small borrowed requirement view, but the
new activation owner must not retain the complete borrowed result catalog or
create a second general result solver.

```rust
VerifiedStaticExactI64RequirementV1<'result>
SealedPreloopOuterCarrierResultContractV1
```

The exact existing evidence path is:

```text
targets.target(caller, outer_site)
  -> source_target.target()
  -> results.disposition(target_key)
  -> required_i64_arguments() == [1]
```

`results.call_result(caller, outer_site)` is intentionally absent for this
nested-instance argument and must not be synthesized. The current exact
target/result construction is test-only; RESULT-CONTRACT0-S0 must add one
bounded real-source producer before the owned activation can seal.

Any required set other than exact `[1]`, a foreign catalog allocation, or a
non-Integer inner contract is a typed pre-Builder rejection.

### Owned activation

```rust
PreparedPreloopStageBCarrierRowsV1
OwnedPreloopStageBCarrierRowV1
VerifiedPreloopStageBCarrierActivationPlanV1
```

The borrowed proof phase copies only canonical keys, structural source sites,
the selected argument ordinal, the bounded result disposition, and the body
handoff. It then drops every borrow before sealing beside the same boxed
declaration catalog allocation.

The only module terminal consumes the plan:

```text
boxed declaration catalog
  -> existing callable-catalog install

owned one-row activation
  -> module-scoped single-use ledger
  -> exact selected function
```

No activation row is stored in `MirBuilder`.

### Function ingress and body schedule

Post-answer code audit found that directly selecting
`RawInvocationChildPortV1::capture_instance_box_method_pending_v1` would also
select an invocation-local `ModuleDraftCollectorV1`, collector-backed header
lookup, pending-session admission, and collector publication. The exact
Stage-B legacy caller owns none of those. Direct RawInvocation selection would
therefore be a publication/header-authority cutover rather than a bounded
function handoff.

```text
RawInvocationChildPortV1 direct Stage-B consumer = 0
ModuleDraftCollectorV1 Stage-B owner              = 0
collector-backed header lookup                    = 0
```

The accepted Q2 intent is retained through one selected sibling under the
current legacy function transaction:

```text
module_lifecycle root-local owned selector
  -> exact canonical function key
  -> lower_method_as_function_with_preloop_stageb_handoff_v1
  -> existing CanonicalFunctionLoweringSessionV1
  -> shared instance-method preparation
  -> exact Body(3) body handoff
  -> existing finalizer
  -> existing current-module draft publication
```

Shared preparation reuses:

```text
create_method_skeleton
declared signature
setup_method_params
StepTree guard
existing header lookup authority
existing finalizer
```

Ordinary instance methods continue through their existing route. Selection is
by the catalog-issued canonical function key exactly once. A selected failure
must not retry through ordinary or legacy lowering.

`build_instance_method_draft_v1` and
`build_instance_method_draft_with_port_v1` may share a private preparation
core, but the selected sibling must not adopt the RawInvocation collector or
pending-session terminal. This is a behavior-neutral BoxShape extraction
before the selected body handoff is connected.

The source-issued body handoff fixes:

```text
prefix:
  Body(0..3), ordinary existing Port

selected:
  exact Body(3) assignment/RHS
  candidate pre-loop Port

suffix:
  Body(4..), ordinary existing Port
  GenericLoop sees the committed outer fact
```

The integer `3` is not caller authority. The existing source-path/body-child
machinery issues the prefix, selected statement, and suffix relation.

### Outer receipt and assignment

```rust
CompletedPreloopOuterCarrierCallV1
CompletedVariableAssignmentV1
CompletedPreloopCarrierAssignmentV1
```

`CompletedPreloopOuterCarrierCallV1` retains:

```text
exact source evidence
successful inner physical receipt
bounded outer Integer contract
successful outer physical Call receipt
```

Its public destination is the outer physical receipt's final destination.
The inner destination is test-only correspondence evidence.

The existing assignment completion may project the RHS through
`LocalContractWrite` before publishing it to `variable_map`. Therefore the
outer Call receipt alone is not the assignment/GenericLoop carrier receipt.

A source-neutral private sibling of the existing assignment driver retains:

```rust
CompletedVariableAssignmentV1 {
    rhs_destination,
    assigned_destination,
}
```

The ordinary facade continues returning only `assigned_destination`.
The bounded pre-loop path consumes the completion receipt and requires:

```text
rhs_destination == outer final destination
assigned_destination == outer final destination
```

The second equality is a bounded property of the current untyped `pos` row,
not a general assignment law. A `LocalContractWrite` projection or any other
destination drift rejects this row before type publication; it must not be
silently followed or generalized.

`CompletedPreloopCarrierAssignmentV1` then co-seals the outer receipt with the
assignment completion. GenericLoop correspondence is proved against the
assigned destination actually stored in `variable_map`, never by assuming the
RHS destination survived assignment.

### Outer type publication

No second type-policy D0 is required when the first correspondence row proves
the exact `[Argument(1)] -> Integer` result contract.

```text
None / Unknown:
  Publish(Integer)

Integer:
  Idempotent

other concrete fact:
  Conflict

failed inner/outer Call or absent receipt:
  publication zero
```

The commit writes only the assignment-published outer carrier through the
existing `TypeContext::set_type`. For this bounded row correspondence must
prove that it is the same `ValueId` as the outer physical destination. The
current inner TYPE-I0 has no production role in the outer carrier chain.

## Executable series

### 1. `PRELOOP-STAGEB-CARRIER-CORRESPONDENCE0-P0`

Closed read-only/code-facing proof (2026-07-27):

```text
inner destination != outer destination
outer destination == assignment result
outer destination == GenericLoop init carrier
outer static result == ExactI64(required=[Argument(1)])
current activation production consumer == 0
current inner TYPE-I0 production consumer == 0
```

If the exact required set cannot be proved, stop at a new result-contract D0.
Do not widen the solver or infer from names.

### 2. `PRELOOP-OUTER-CARRIER-RESULT-CONTRACT0-S0`

Closed Builder-free source proof (2026-07-27):

```text
VerifiedStaticExactI64RequirementV1
SealedPreloopOuterCarrierResultContractV1
typed foreign-catalog / ordinal / result mismatch
general call-result substitution rejected
production consumer == 0
```

### 3. `PRELOOP-STAGEB-SOURCE-ACTIVATION0-S0`

Closed owned, non-Clone, one-row activation (2026-07-27):

```text
PreparedPreloopStageBCarrierRowsV1
OwnedPreloopStageBCarrierRowV1
VerifiedPreloopStageBCarrierActivationPlanV1
exact root assignment prefix / selected / suffix schedule
same boxed declaration-catalog allocation
```

Production consumer, Builder, MIR, type publication, and catalog reseal remain
zero through this row.

### 4. `PRELOOP-STAGEB-FUNCTION-PREPARATION0-S0`

Closed behavior-neutral BoxShape extraction (2026-07-27):

```text
shared instance-method skeleton / signature / runes / uses / params
shared StepTree observation guard
ordinary and port-aware body descent remain separate thin consumers
legacy current-module and port-aware header finalizers remain separate
empty-body and scalar-Return field parity green
activation selection and production consumer remain zero
```

### 5. `PRELOOP-STAGEB-SOURCE-PRODUCER-SELECTION0-D0`

Before production ingress, select the real whole-source producer:

```text
target inventory = whole resolver-observable static-call inventory
candidate zero = ordinary route unchanged
candidate one = exact owned activation
candidate many or selected drift = typed pre-Builder rejection
unsupported inventory availability = explicit disposition, never silent fallback
alias authority = compiler-supplied typed using/import snapshot
```

The D0 must also prove that the owned activation row removes every catalog
borrow before the exact catalog is installed into the mutable Builder.

### 6. `PRELOOP-STAGEB-FUNCTION-INGRESS0-I0`

Connect exactly one selected instance method:

```text
catalog install once
function selection once
legacy function transaction once
selected lower_method_as_function_with_preloop_stageb_handoff_v1
prefix / selected / suffix body handoff
production consumer exactly one
ordinary route delta zero
RawInvocation/collector cutover zero
```

### 7. `UNIFIED-CALL-OUTER-CARRIER-RECEIPT0-S0`

Add one source-neutral receipt-returning sibling for the outer static/global
value terminal. It must call the existing generic physical writer and expose
only `CompletedUnifiedValueCallEmissionV1`.

### 8. `PRELOOP-OUTER-CARRIER-RECEIPT0`

```text
I0:
  pair inner + outer physical receipts with the bounded source contract

P0:
  inner failure / outer failure / alternate route / assignment mismatch

G0:
  one outer receipt producer, no inner-as-outer conversion
```

### 9. `PRELOOP-OUTER-CARRIER-ASSIGNMENT0-S0`

Add one source-neutral assignment-completion sibling and one bounded pre-loop
co-seal:

```text
rhs destination
assigned destination
variable_map publication
outer physical destination
```

The current row accepts exact identity only. Typed-local projection remains an
explicit rejection and no general assignment-result typing is introduced.

### 10. `PRELOOP-OUTER-CARRIER-TYPE-I0`

```text
S0:
  prepare-only outer publication owner

I0:
  success-only outer destination commit

P0:
  None / Unknown / Integer / conflict / failed-Call matrix

G0:
  one production outer publisher; direct map insert zero
```

### 11. `CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0`

Only after the preceding rows:

```bash
bash tools/checks/generic_loop_progression_role_v0_guard.sh
```

Do not change the guard's expected frontier before the implementation reaches
it. The new actual output selects the next blocker.

## Buildable commit plan

```text
Commit A:
  closed CORRESPONDENCE0-P0

Commit B:
  closed RESULT-CONTRACT0-S0

Commit C:
  owned SOURCE-ACTIVATION0-S0
  + disconnected same-allocation/ownership matrix

Commit D:
  shared instance-method preparation extraction
  + legacy/port-aware parity

Design stop:
  SOURCE-PRODUCER-SELECTION0-D0
  + whole static-call inventory / zero-one-many selection

Commit E:
  FUNCTION-INGRESS0-I0
  + exact owned body handoff

Commit F:
  source-neutral outer receipt terminal
  + outer carrier receipt
  + source-neutral assignment completion

Commit G:
  bounded assignment correspondence
  + outer TYPE-I0
  + focused parity/reuse
  + existing lane guard consolidation

Commit H:
  actual Stage-B rerun
  + current pointer closeout
```

Every commit must build. BoxCount and BoxShape must not be mixed. If one
commit exceeds a reviewable semantic boundary, split it without changing this
owner order.

## File placement and line budget

Prefer small sibling modules:

```text
src/mir/preloop_stageb_carrier/
  README.md
  mod.rs
  outer_result.rs
  activation.rs
  rejection.rs
  tests.rs

src/mir/builder/calls/
  preloop_stageb_function_handoff.rs
  preloop_outer_carrier_receipt.rs
  preloop_outer_carrier_type.rs
  preloop_outer_carrier_tests.rs

src/mir/builder/stmts/
  variable_assignment_completion.rs
```

Do not grow these near-cap files with new implementations:

```text
src/mir/builder/calls/lowering.rs
src/mir/builder/calls/unified_emitter.rs
src/mir/builder/recursive_child_lowering.rs
tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_v0.py
tools/checks/lib/mirbuilder_type_fact_partition_guard.py
```

They may receive only thin declarations/delegations when required. Every
modified/new source or check file remains below 800 lines.

## Proof budget and sunset

```text
ceremony_tier:
  T2 new source activation and outer publication boundary

sunset_id:
  PRELOOP-INNER-TYPE-PROOF-SUNSET-001

proof_inventory_before:
  one test-only inner physical receipt chain
  one test-only inner TYPE-I0 chain
  one stale Stage-B guard expectation

new_proofs:
  one bounded outer result-contract proof
  one owned activation proof
  one outer physical/assignment receipt proof
  one production outer publication proof

retired_or_merged_proofs:
  none before the outer production chain is green

net_proof_delta:
  temporarily positive for the T2 boundary

sunset_budget:
  repay after outer Stage-B publication is green

sunset_row:
  PRELOOP-INNER-TYPE-PROOF-RETIRE0-S0

retire_when:
  outer production publisher == 1
  inner TYPE-I0 production consumer == 0
  actual Stage-B guard no longer needs the proof-only inner publisher

budget_repayment_evidence:
  exact inner proof consumer census
  actual Stage-B guard
  focused outer publication parity/reuse matrix
```

No new per-row shell guard is authorized. Extend the existing call/source
focused tests and the existing Stage-B guard only when its real expected
frontier changes.

## Verification

At each buildable milestone:

```bash
cargo check --lib
cargo test -q --lib source_instance_result_contract
cargo test -q --lib callable_result_representation
cargo test -q --lib preloop_nested_result
cargo test -q --lib recursive_child_lowering_rawport
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

At the final Stage-B row:

```bash
bash tools/checks/generic_loop_progression_role_v0_guard.sh
```

Before every commit:

```text
protected dirty file:
  src/mir/builder/calls/member_route_descent_tests.rs
  must not be staged or edited

parked stash:
  stash@{0}: wip/preloop-rep0-generic-route-drift
  must not be popped wholesale

all modified/new source/check files:
  less than 800 lines
```

```text
docs_only_closeout = forbidden
code_or_artifact_delta_required = 1
```

## Historical consultation basis

The exact Stage-B guard was rerun after TYPE-I0-G0 before the accepted
closeout:

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

This observation prohibited implementation until the D0 selected the source
owner, outer-carrier physical receipt, and function-scoped consuming handoff.
The accepted closeout above now authorizes only the listed executable series.

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

## Historical alternatives

The consultation compared:

```text
A:
  separate owned one-row activation

B:
  widen the existing generic static activation plan

C:
  reconstruct source evidence during lowering

D:
  Builder registry or GenericLoop inference
```

A was accepted. B would mix static-result, nested-instance, and outer-carrier
authority. C and D violate same-allocation source identity and consumer-only
GenericLoop laws.

Post-answer code audit also rejected direct selection of
`RawInvocationChildPortV1`: the exact Stage-B caller does not own its
`ModuleLoweringPortV1`/collector/header/publication world. The accepted bounded
implementation reuses only its port-aware preparation/descent pattern inside
the current legacy function transaction.

## Non-claims

```text
current TYPE-I0 proves the outer carrier
loop-refresh activation
GenericLoop publisher migration
general instance-call result inference
general located lowering activation
RawInvocation / ModuleDraftCollector cutover
ownership syntax / Alias / View activation
parser / Hako / VM / backend change
default route cutover
fallback / retry
```
