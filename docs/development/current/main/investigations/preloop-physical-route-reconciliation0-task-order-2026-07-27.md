---
Status: accepted design and execution task order
Date: 2026-07-27
Decision: PRELOOP-PHYSICAL-ROUTE-RECONCILIATION0-prime-r1
Corrects:
  - the under-configured receiver fixture used after CALLABLE-RESULT-NESTED-PRELOOP-REP0-S0
Related:
  - preloop-located-argument-request-boundary0-d0-design-question-2026-07-27.md
  - nested-instance-result-emission-reconciliation-d1-design-question-2026-07-27.md
  - mirbuilder-cleanliness-feedback-task-order-2026-07-27.md
  - src/mir/builder/calls/lowering.rs
  - src/mir/builder/calls/parameter_setup.rs
  - src/mir/builder/calls/preloop_located_argument_ingress_p0_tests.rs
  - src/mir/callable_result_representation/tests/actual_parser_add_fixture.rs
First executable row: PRELOOP-PRODUCTION-PREFIX-FIXTURE0-S0
---

# Pre-loop Physical Route Reconciliation

## Decision

```text
Choice:
  A-prime — production-shaped instance-method prefix is the physical-route
            authority

Current manual Integer `me` fixture:
  rejected as route evidence

BoxCall-specific value receipt:
  not created

generic / BoxCall common receipt:
  not created

Router policy override:
  forbidden
```

The current `BoxCall` observation is not evidence that the selected ParserBox
instance call uses BoxCall in its production-shaped context. It is caused by a
test helper that binds `me` to an Integer literal and therefore publishes
neither `MirType::Box("ParserBox")` nor the ParserBox receiver origin.

The existing instance-method entry already owns the correct physical context:

```text
create_method_skeleton
-> declared signature
-> setup_method_params("ParserBox", ...)
-> me = parameter 0
-> type(me) = Box("ParserBox")
-> origin(me) = ParserBox
```

The bounded proof must reuse that entry. It must not reconstruct the receiver
with manual fact writes, a test-only type seed, a Router override, or a second
Builder setup API.

## Evidence

### Source authority

The actual fixture declares the caller under:

```text
box ParserBox
namespace = InstanceBoxMethod
caller = ParserBox.static_const_parse_add/2
selected site = Body(3).Value.Argument(1)
```

`SealedNestedInstanceResultContractV1`, the Raw located source view, the
selected call argument, and the pre-loop association are already co-sealed
from the same declaration-catalog allocation. That source identity law is
unchanged.

### Why the old fixture selected BoxCall

The former configured helper did this:

```text
enter_function_for_test(...)
me = Integer literal
bind_variable_for_test("me", me)
```

Consequently:

```text
receiver type/origin unavailable
-> resolver = UnknownBox
-> RouterPolicy = BoxCall
```

This is valid Router behavior for that synthetic input. It is not the physical
route of the declared ParserBox instance method.

### Production-shaped trace

The existing production-prefix harness was run with:

```bash
NYASH_RING0_LOG_LEVEL=DEBUG \
NYASH_ROUTER_TRACE=1 \
NYASH_CALL_RESOLVE_TRACE=1 \
RUSTFLAGS=-Awarnings \
NYASH_MIR_UNIFIED_CALL=1 \
cargo test -q --lib \
  actual_method_prefix_uses_canonical_parameters_and_keeps_one_live_scope_for_loop_handoff \
  -- --nocapture
```

The exact selected inner call reached:

```text
[call-resolve]
  Method
  box='ParserBox'
  method='static_const_eval_pos'
  recv=ValueId(0)
  recv_origin=ParserBox

[router]
  route=Unified
  reason=unified
  recv=ParserBox
  method=static_const_eval_pos
  arity=1
  certainty=Known
```

The test passed. Because the trace reached resolver and Router, an earlier
Known/Unique rewrite did not consume this bounded call.

This trace proves the expected route. The first executable row still replaces
the under-configured candidate fixture with a `prefix_len = 3` proof so that
the exact source association, configured receiver, and retained physical
receipt are observed in one fixture.

## Authority and non-authority

```text
source identity authority:
  same-allocation declaration catalog
  + exact caller
  + exact SourceExprSiteV1

receiver configuration authority:
  create_method_skeleton
  + setup_method_params

physical route authority:
  existing resolver
  + existing RouterPolicy
  + existing unified emitter terminal

physical destination authority:
  CompletedUnifiedValueCallEmissionV1

nested Integer source authority:
  SealedNestedInstanceResultContractV1

not authorities:
  manually bound `me`
  source spelling / callee name checks
  requested ValueId alone
  MIR instruction scans
  BoxCall compatibility success
  test-only type/origin insertion
```

The Builder-side callable catalog installed by the physical fixture supplies
the existing lowering context only. It is not substituted for the
same-allocation catalog that owns the source association.

## Corrected owner chain

```text
same-allocation source catalog
  -> exact pre-loop source association

actual ParserBox method declaration
  -> lower_instance_method_prefix_for_test(
       "ParserBox",
       declaration,
       prefix_len = 3,
     )
  -> existing method skeleton
  -> existing parameter publication
  -> exact Body(3) continuation

exact source association
  + production-shaped configured Builder
  -> candidate-only located ingress
  -> existing StaticReceiver outer route
  -> selected Argument(1)
  -> existing Me Standard(Unified) route
  -> existing generic physical Call terminal
  -> CompletedUnifiedValueCallEmissionV1

source association
  + successful generic physical receipt
  -> reached nested physical-call owner

outer terminal success
  -> EmittedNestedInstanceCallV1(final_destination)
  -> stop before type publication
```

The outer Global Call is not part of the nested-result physical receipt. Its
success is the transaction boundary that permits the candidate Port to publish
the completed nested receipt outside the in-flight state.

## WIP quarantine

The red WIP remains:

```text
stash@{0}: wip/preloop-rep0-generic-route-drift
```

Do not pop it before the production-prefix fixture is green.

The following semantic files are reusable after review:

```text
src/mir/builder/calls/method_call_terminal.rs
src/mir/builder/calls/preloop_located_argument_ingress.rs
src/mir/builder/calls/preloop_located_argument_port.rs
src/mir/builder/calls/preloop_nested_result_receipt.rs
```

The following fixture hunks must not be restored as-is:

```text
src/mir/builder/calls/preloop_located_argument_ingress_tests.rs
src/mir/builder/calls/preloop_located_argument_ingress_p0_tests.rs
```

Recover the four semantic-file patches selectively. Rebuild the positive and
failure fixtures on the production prefix harness. Do not restore the manual
Integer `me` setup.

The unrelated dirty file remains outside every commit:

```text
src/mir/builder/calls/member_route_descent_tests.rs
```

## Executable series

### 1. `PRELOOP-PRODUCTION-PREFIX-FIXTURE0-S0`

Replace the synthetic configured Builder in the successful physical-route
proof with:

```rust
lower_instance_method_prefix_for_test(
    "ParserBox",
    actual_parser_add_fixture::method_declaration_for_lowering(),
    3,
    |builder, suffix| {
        // execute the exact Body(3) candidate here
    },
)
```

The continuation first verifies:

```text
suffix[0] = exact Body(3) assignment
me        = current function parameter 0
type(me)  = Box("ParserBox")
origin(me)= ParserBox
ret binding exists
current function/block/scope are live
```

It then prepares the existing outer `StaticReceiver` route and executes the
candidate Port. No successful physical-route helper may write the receiver
type or origin. Synthetic Integer-receiver fixtures may remain only for
lower-level rejection boundaries and must state that they are not route
evidence.

Because the production prefix may already emit Calls, fixtures compare deltas:

```text
before = Call count at continuation entry

success:
  inner Method Call delta = 1
  outer Global Call delta = 1

failure before inner physical commit:
  Call delta = 0
```

Acceptance:

```text
manual successful-fixture `me` binding       = 0
manual receiver type/origin insertion        = 0
production instance receiver setup           = 1
selected inner Router route                  = Unified
selected inner generic receipt availability  = 1
BoxCall receipt                              = 0
Known/Unique rewrite receipt                 = 0
production caller                            = 0
```

Fail-fast branch:

```text
if the production-shaped candidate does not reach Generic:
  stop
  retain the observed exact route
  open PRELOOP-PHYSICAL-ROUTE-MISMATCH0-D0
  do not add a receipt or change Router policy automatically
```

### S0 closeout

`PRELOOP-PRODUCTION-PREFIX-FIXTURE0-S0` is closed.

The positive ingress proof now owns a fresh `MirModule` and uses the existing
production prefix harness at `prefix_len = 3`. It verifies the exact Body(3)
continuation, `me = parameter 0`, `MirType::Box("ParserBox")`, ParserBox
origin, live bindings/scope, and continuation-relative two-Call delta. The
selected inner physical Call is `ParserBox.static_const_eval_pos` with a
`Known` receiver whose Copy chain roots at that canonical `me`; the outer
`ParserStringUtilsBox.skip_ws/2` Call consumes the selected result.

The legacy hand-built Builder remains only for lower-level rejection tests and
is renamed `synthetic_rejection_builder`. It deliberately binds Integer `me`,
is documented as non-route evidence, and no longer supplies a successful
physical-route or fresh-success assertion. Those lower-level reject tests are
not a substitute for the production prefix proof.

The row made no receipt, Router, type-publication, production-caller, or
fallback change. The former REP0 WIP remains quarantine-only after the landed
manual reconstruction. Do not pop or apply it. Its later retirement requires
an explicit stash inventory/drop decision and is not part of TYPE-I0.

Verification:

```bash
RUSTFLAGS=-Awarnings cargo test -q --lib \
  configured_preloop_ingress_reaches_existing_inner_and_outer_call_terminals
RUSTFLAGS=-Awarnings cargo test -q --lib preloop_located_argument_ingress \
  -- --test-threads=1
bash tools/checks/current_state_pointer_guard.sh
```

### 2. `CALLABLE-RESULT-NESTED-PRELOOP-REP0-I0`

The disconnected S0 product landed in `0d142c1ba3` remains valid. Connect it to
the exact generic receipt without widening the emitter.

Recommended bounded products:

```text
ReachedPreloopNestedPhysicalCallV1
  owns:
    exact retained source association
    + CompletedUnifiedValueCallEmissionV1

EmittedNestedInstanceCallV1
  stores:
    final_destination only
```

The physical product is issued only by the existing generic value-Call receipt
terminal. It does not represent BoxCall, rewrite, legacy, or no-destination
success.

Temporal law:

```text
inner generic Call success:
  Armed -> ReachedPhysical(source + receipt)

outer terminal success:
  ReachedPhysical -> Emitted(final_destination)

outer terminal failure:
  retain source + physical receipt in typed rejection
  EmittedNestedInstanceCallV1 = 0

inner Call failure:
  generic receipt = 0
  nested emitted receipt = 0
```

Still zero:

```text
MirType publication
type_ctx write
loop-refresh activation
production Stage-B caller
fallback / retry
```

### 3. `CALLABLE-RESULT-NESTED-PRELOOP-REP0-P0`

Use only production-shaped prefix fixtures.

Positive matrix:

```text
exact selected site
me parameter/type/origin correspondence
inner route = Generic Unified
final receipt destination = exact inner Method Call.dst
outer Call consumes that value through ordinary normalization
outer success produces exactly one EmittedNestedInstanceCallV1
type_ctx Integer publication = 0
```

Failure matrix:

```text
generic Call instruction failure:
  physical receipt = 0
  emitted nested receipt = 0
  source retained

inner success -> outer terminal failure:
  inner physical Call delta = 1
  emitted nested receipt = 0
  source + physical receipt retained by rejection

BoxCall / rewrite / legacy / no-destination:
  generic receipt = 0
  no alternate route or fallback

failure -> fresh production-shaped fixture success:
  green
```

Do not assert a total function Call count. Assert the continuation-relative
delta and exact callee/destination correspondence.

### 4. `CALLABLE-RESULT-NESTED-PRELOOP-REP0-G0`

Extend an existing lane/manifest guard. Do not create a per-row shell wrapper.

```text
production receiver setup owner                    = 1
successful manual Integer-me fixture               = 0

generic physical Call writer                       = existing 1
generic value receipt producer                     = existing 1
pre-loop generic receipt consumer                  = 1

BoxCall receipt producer                           = 0
rewrite receipt producer                           = 0
second Call writer                                 = 0

EmittedNestedInstanceCallV1 producer                = 1
nested Integer type_ctx writer                     = 0
GenericLoop nested-result producer                 = 0

Builder source-site map                            = 0
persistent source-site -> ValueId map               = 0
fallback / retry / route reselection                = 0
production caller                                  = 0
```

At G0, migrate the reusable failure/reuse assertions into the REP0 matrix and
retire duplicated manual-ingress proof scaffolding where possible.

### 5. `CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0-D0`

Decision:

```text
Decision:
  CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0-prime-r1

Status:
  accepted

Choice:
  A — stored Unknown remains a replaceable non-fact
```

This reuses the existing global fact policy:

```text
type_ctx[dst] = None / Unknown
  -> TypeFactDecisionV1::Publish(Integer)

type_ctx[dst] = Integer
  -> TypeFactDecisionV1::Idempotent
  -> physical write 0

type_ctx[dst] = other exact type
  -> typed conflict

overwrite
  -> forbidden

receipt absent / Call failure / outer failure
  -> publication 0
```

`TypeFactDecisionV1` remains the sole decision authority and
`TypeContext::set_type` remains the sole write terminal. TYPE-I0 adds no
decision enum, TypeContext API, direct map insert, or GenericLoop publisher.

### REP0 I0/P0/G0 closeout

`CALLABLE-RESULT-NESTED-PRELOOP-REP0-I0`, `-P0`, and `-G0` are closed.

The shared standard Method terminal now has one receipt-required sibling. It
uses the existing generic unified Call writer and produces a physical value
receipt only after the selected inner `MirInstruction::Call` succeeds. The
shared terminal remains source-neutral: it does not own the pre-loop source
association, a `type_ctx` write, a source-site map, or a second Call writer.

The production-shaped ParserBox prefix matrix proves:

```text
inner generic Call success
  + exact source association
  -> retained reached-physical owner

outer Call success
  -> exactly one EmittedNestedInstanceCallV1(final_destination)

inner physical Call failure
  -> source retained, physical receipt = 0

inner success + outer terminal failure
  -> source + physical receipt retained in typed rejection

all failure cells
  -> fresh production-shaped fixture success
```

The existing callable-result guards were extended rather than adding a new
wrapper. They now inspect executable authority rather than rejecting prose
that describes a forbidden route. They keep these facts fixed:

```text
generic physical Call writer               = existing 1
generic value receipt writer               = existing 1
pre-loop receipt consumer                  = 1
BoxCall / rewrite receipt                  = 0
nested Integer type_ctx writer             = 0
GenericLoop nested-result producer         = 0
fallback / retry / route reselection       = 0
production caller                          = 0
```

The next row is therefore the already-reserved semantic stop
`CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0-D0`. It decides only the disposition
of an existing `Unknown` fact for this receipt-backed Integer publication.
It must reuse `TypeFactDecisionV1` and `TypeContext::set_type`; it must not
reopen receipt, Router, GenericLoop, or production-entry authority.

### 6. Type implementation series

The implementation is one T1 bounded extension. Keep the receipt-only module
unchanged and place the adapter and tests in separate files:

```text
src/mir/builder/calls/preloop_nested_result_type.rs
src/mir/builder/calls/preloop_nested_result_type_tests.rs
```

The current ingress P0 file is already large; do not append the TYPE matrix to
it. Every source/check file remains below 800 lines.

Exact owner chain:

```text
EmittedNestedInstanceCallV1
+ TypeContext::get_type(final_destination)
  -> TypeFactDecisionV1::prepare(existing, Integer)
  -> PreparedPreloopNestedIntegerPublicationV1
       owns:
         emitted receipt
         prepared fact decision
  -> one consuming commit
  -> TypeContext::set_type only for Publish(Integer)
```

On a concrete conflict,
`RejectedPreloopNestedIntegerPublicationV1` retains the emitted receipt and
the typed cause. It exposes inspection plus discard only. It has no retry,
resume, receipt recovery, Call emission, source lookup, or fallback authority.

Buildable task order:

```text
CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0-S0
  Builder-free prepared/rejected products
  pure None / Unknown / Integer / concrete-conflict decision matrix
  TypeContext write = 0

-> CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0-I0
  consume EmittedNestedInstanceCallV1 once
  commit only Publish through TypeContext::set_type
  Idempotent physical write = 0

-> CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0-P0
  production-prefix None / Unknown / Integer / conflict
  inner Call failure / outer failure / conflict -> fresh success

-> CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0-G0
  extend existing callable-result and type-fact guards
  one publisher, GenericLoop consumer-only, overwrite zero
```

### TYPE-I0-S0 closeout

`CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0-S0` is closed.

One Builder-free owner now consumes `EmittedNestedInstanceCallV1` and delegates
the complete fact decision to the existing `TypeFactDecisionV1`:

```text
None / stored Unknown
  -> PreparedTypeFactPublicationV1::Publish(Integer)

stored Integer
  -> PreparedTypeFactPublicationV1::Idempotent(Integer)

other concrete fact
  -> typed conflict retaining the emitted receipt
```

The product owns no `MirBuilder`, `TypeContext`, source lookup, Call emission,
GenericLoop capability, or fact-store write. Focused tests cover the four
decision cells in a separate sibling file; the source and test files remain
below 800 lines. `cargo test --lib preloop_nested_result_type`, `cargo check
--lib`, the current-state pointer guard, and diff check are green.

The next row is `CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0-I0`. It may add only
the consuming commit terminal: `Publish` calls the existing
`TypeContext::set_type`, while `Idempotent` performs no physical write.

### TYPE-I0-I0 closeout

`CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0-I0` is closed.

The sole terminal keeps the fact read, existing `TypeFactDecisionV1`
preparation, and consuming commit adjacent:

```text
emitted receipt
  + TypeContext::get_type(final_destination)
  -> prepared publication
  -> Publish only: TypeContext::set_type(final_destination, Integer)
  -> Idempotent: physical write 0
```

The prepared commit is private to the type owner. Callers cannot retain it
across another fact-store mutation, recover the consumed receipt, write the
map directly, or select another policy. Concrete conflicts leave the existing
fact unchanged and retain the receipt in the typed rejection.

Focused tests cover missing, stored Unknown, matching Integer, and concrete
conflict commit behavior. The source/test files are 111/93 lines;
`cargo test --lib preloop_nested_result_type`, `cargo check --lib`, the
current-state pointer guard, and diff check are green.

The next row is `CALLABLE-RESULT-NESTED-PRELOOP-TYPE-I0-P0`. It connects this
terminal only inside the existing production-prefix proof matrix and must
retain zero production callers.

Do not create a new guard script. Extend the existing callable-result guard
with the receipt consumer/decision/writer and P0 evidence, and add this writer
to the existing type-fact partition guard. The original ingress, port, and
receipt modules retain zero type writes.

### 7. Exact Stage-B frontier

```text
CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0
```

Rerun the exact Stage-B guard with the real pre-loop Integer fact. Select only
the first new failure:

```text
ownership syntax frontier:
  resume OWN-GRAM-REJECT0-HAKO0

loop-refresh frontier:
  open GENERIC-LOOP-NESTED-RESULT-ACTIVATION0-D0

other frontier:
  open one exact owner stop
```

Loop-refresh is a parked candidate, not a mandatory successor.

## Longer roadmap

If Stage-B selects ownership syntax, resume the already accepted sparse
ownership order; do not create a parallel Alias/View plan here.

```text
OWN-GRAM-REJECT0-HAKO0
-> OWN-GRAM-REJECT0-G0
-> Pack A: syntax safety and evidence
-> Pack B: passive grammar / Loan Flow
-> Pack C: first ScopedAlias (ALIAS-I0)
-> Pack D: callable ownership ABI
-> Pack E: first Anchored View (VIEW0 -> PROJ-S0 ... PROJ-I0)
-> OWNERSHIP-SPARSE-PRODUCT-READINESS-D0
```

Alias/View therefore remain explicitly scheduled after the pre-loop result
fact closes and the Stage-B guard proves ownership is the next frontier. They
must not be pulled into this physical-receipt series.

Product/default backend promotion, import-aware source families, Legacy
retirement, and final MirBuilder completion remain downstream of the existing
canonical-core and ownership readiness gates.

## Proof budget

```text
ceremony_tier = T1 bounded proof correction
sunset_id = PRELOOP-PRODUCTION-PREFIX-PROOF-SUNSET-001

proof_inventory_before =
  one under-configured manual receiver fixture
  + existing production-prefix harness

new_proofs =
  one production-prefix candidate correspondence

retired_or_merged_proofs =
  manual successful Integer-me configured fixture

net_proof_delta = 0
sunset_budget = 0

sunset_row =
  CALLABLE-RESULT-NESTED-PRELOOP-PROOF-RETIRE0-S0

retire_when =
  actual Stage-B owner consumes the exact source association, generic physical
  receipt, and Integer fact while preserving the production-shaped
  failure/reuse matrix
```

## Structural gate

```text
source declaration family                          = InstanceBoxMethod
same-allocation source association                 = 1

production method skeleton                         = 1
setup_method_params receiver owner                 = 1
manual receiver type/origin seed                   = 0

selected receiver type                             = Box(ParserBox)
selected receiver origin                           = ParserBox
selected Router route                              = Unified

generic receipt authority                          = existing 1
BoxCall receipt authority                          = 0
Router override                                    = 0

source association + receipt correspondence        = 1
outer-success nested receipt terminal              = 1

type publication before TYPE-I0                    = 0
GenericLoop type producer                          = 0
loop-refresh activation                            = 0

fallback / retry / route reselection               = 0
production caller                                  = 0
default route delta                                = 0

all modified/new source/check files                < 800 lines
```

## Required closeout

```text
Decision:
  PRELOOP-PHYSICAL-ROUTE-RECONCILIATION0-prime-r1

Status:
  accepted

Choice:
  A-prime

Physical-route authority:
  existing production-shaped instance-method prefix

Observed manual BoxCall:
  under-configured fixture evidence only

Preserved:
  UNIFIED-CALL-PHYSICAL-RECEIPT0
  BoxCall receipt = 0
  Router policy unchanged

First executable row:
  PRELOOP-PRODUCTION-PREFIX-FIXTURE0-S0

Near task order:
  production-prefix fixture
  -> REP0-I0
  -> REP0-P0/G0
  -> TYPE-I0-D0
  -> TYPE-I0 implementation
  -> exact Stage-B guard
```

## Non-claims

```text
BoxCall value-receipt activation
Known/Unique rewrite receipt
Router policy change
production Stage-B caller
Integer publication before TYPE-I0
loop-refresh activation
GenericLoop publisher migration
ownership grammar activation
Alias/View activation
parser / grammar / VM / backend change
default backend cutover
Legacy retirement
```
