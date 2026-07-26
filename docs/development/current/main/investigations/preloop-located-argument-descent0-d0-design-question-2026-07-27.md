# Pre-loop located argument descent D0

```text
Decision:
  PRELOOP-LOCATED-ARGUMENT-DESCENT0-prime-r1

Closes:
  PRELOOP-LOCATED-ARGUMENT-DESCENT0-D0

Status:
  accepted design

Choice:
  A-prime-Port

First executable row:
  PRELOOP-LOCATED-ARGUMENT-RELATION0-S0

Next physical row:
  UNIFIED-CALL-PHYSICAL-RECEIPT0-S0
```

## Verdict

The original A-prime direction is correct: the selected pre-loop
`Argument(1)` needs a candidate-only owner. Option B, an optional override in
`AssociatedMethodCallArgumentsV1`, is rejected.

The repository audit found a still cleaner A-prime implementation. Do not add
a new completion trait. The required source-neutral capability already exists:

```text
MethodCallDescentPortV1
+ MethodCallValueTerminalPortV1
+ MeCallHeaderObservationPortV1
  =
MethodCallLoweringPortV1
```

`build_member_method_call_v1()` and its handlers are already generic over
that port. The exact extension is:

```text
ordinary route:
  existing Port
  -> existing AssociatedMethodCallArgumentsV1

selected proof route:
  existing Port
  -> PreloopLocatedArgumentPortV1<Port>
  -> existing AssociatedMethodCallArgumentsV1
```

```text
candidate-only Port                                  = accepted
optional override in AssociatedMethodCallArgumentsV1 = rejected
new MethodCallCompletionV1 trait                     = rejected
second ordered-argument driver                       = rejected
```

## Audit corrections required before connection

### 1. The proposed parent/child type was self-referential

The consultation sketch made a child borrow a parent that the outer product
also owned. That cannot be a safely movable Rust struct.

The source-view factory instead consumes the parent input and produces:

```rust
struct VerifiedRawLocatedCallArgumentV1<'view, 'catalog> {
    parent: RawLocatedMethodCallInputV1<'view, 'catalog>,
    index: u32,
    child: RawLocatedExprInputV1<'view, 'catalog>,
}
```

Both inputs borrow only the external catalog-backed view. The factory uses
only:

```text
ExprChildRoleV1::CallArgument(index)
SourcePathV1
existing source projector
```

No second navigation engine, AST walk, pointer search, name search, or ordinal
reconstruction is allowed.

The final relation is:

```rust
struct PreparedPreloopLocatedArgumentV1<'site, 'view, 'catalog> {
    selected: VerifiedRawLocatedCallArgumentV1<'view, 'catalog>,
    association:
        PreparedPreloopNestedResultAssociationV1<'site, 'view, 'catalog>,
}
```

Its private constructor co-seals:

```text
same source view and declaration-catalog allocation
same declaration and caller
selected child site == association input site
selected child AST  == association input AST
structural child role == CallArgument(index)
```

The current result is `index = 1`, but a caller-provided integer is not
authority. The source-view-issued `CallArgument(1)` relation is authority.

### 2. Keep the current ordered-argument algorithm

The sole ordered policy is already:

```text
drive_call_arguments_v1
+ CallArgumentDescentPortV1
```

It owns:

```text
validate every input
-> same-call moved-value preflight
-> record-value preflight per index
-> ordered lowering
-> undefined-result observation
```

The candidate port must enter that algorithm. Do not create
`drive_ordered_method_call_arguments_v1()` or copy any preflight.

To consume one selected input, this behavior-neutral port signature may become
mutable:

```rust
fn argument_expression_input(
    &mut self,
    input: &Self::ArgumentsInput,
    index: usize,
) -> Result<Self::ExpressionInput, String>;
```

Existing implementations receive mechanical updates only.

### 3. Remove the false source dependency from the terminal

Every implementation of `MethodCallValueTerminalPortV1` ignores its
`MethodCallInput`:

```text
Raw blanket       = _input
LocatedLegacy     = _input
route-test port   = _input
```

Passing an outer input to an inner terminal would still encode false
authority. Make the terminal genuinely source-neutral:

```text
MethodCallValueTerminalPortV1:
  no MethodCallInput parameter
  no MethodCallDescentPortV1 supertrait

MethodCallLoweringPortV1:
  MethodCallDescentPortV1
  + MethodCallValueTerminalPortV1
  + MeCallHeaderObservationPortV1
```

Source syntax remains the descent port's authority. Value emission remains the
terminal port's authority. This row is behavior-neutral.

### 4. `MeCall` is not proof of `MeStandardUnified`

`plan_member_call_route()` currently seals only:

```text
ReceiverNormalized(MeCall)
```

The Me policy may then choose:

```text
inline record helper
inline setter
lowered Global
standard route
static fallback
```

The standard route can still choose weak-load, record/setter inline, or a
unified Method request. Some alternate routes can lower or emit before the
current terminal.

Split the existing policy, without changing it, into:

```text
prepare exact subroute
-> execute exactly that prepared subroute
```

Conceptual vocabulary:

```text
PreparedMeCallExecutionV1:
  InlineRecord
  InlineSetter
  LoweredGlobal
  Standard(PreparedStandardMethodCallExecutionV1)
  StaticFallback
  NotApplicable

PreparedStandardMethodCallExecutionV1:
  WeakLoad
  InlineRecord
  InlineSetter
  UnifiedMethod
```

Preparation may borrow existing declaration, record, setter, header, and
current-`me` facts. It performs:

```text
argument lowering = 0
MIR emission       = 0
Call request       = 0
type publication   = 0
```

The ordinary route executes every existing disposition. The candidate accepts
only:

```text
Standard(UnifiedMethod)
```

No candidate-only callee-name or Box-name policy is allowed.

### 5. Candidate failure needs an isolated transaction

Argument zero may emit candidate MIR before argument one fails. The local
driver therefore cannot claim all Builder effects are zero.

The honest law is:

```text
candidate Builder effects may exist
live MirCompiler Builder effects = 0
module publication               = 0
```

Use one isolated, unpublished candidate owner over the existing canonical
module session:

```text
OpenPreloopLocatedArgumentCandidateV1
  - isolated CanonicalModuleLoweringSessionV1
  - PreparedPreloopLocatedArgumentV1
```

A thin wrapper may install the already-required production-shaped function
skeleton and lowering environment. It must not reseal equal-looking source
data as relation authority. This row exposes no commit or publication
terminal.

Rejection retains the complete candidate and source relation and exposes only:

```text
stage()
cause()
bounded_report()
discard(self)
```

No `into_owner`, resume, retry, alternate route, rollback, or fallback is
allowed.

## Candidate Port contract

The candidate port wraps the existing port and owns only one selected
association and route state:

```text
PreloopLocatedArgumentPortV1<Port>
  - ordinary Port
  - selected structural index
  - one-shot association state
  - inner-route state
```

It implements the existing traits:

```text
RecursiveChildLoweringPortV1
CallArgumentDescentPortV1
MethodCallDescentPortV1
MethodCallValueTerminalPortV1
MeCallHeaderObservationPortV1
```

It receives the blanket `MethodCallLoweringPortV1`; no new argument or
completion trait is introduced.

Expression input is conceptually:

```rust
enum PreloopLocatedExpressionInputV1<Legacy, Selected> {
    Legacy(Legacy),
    Selected(Selected),
}
```

```text
unselected index:
  existing port input -> existing lowering

selected structural index:
  take association once
  -> exact located inner MethodCall
  -> existing member planner
  -> RawLegacy conversion = 0
```

One-shot state must preserve the association for the later receipt row:

```text
Armed(association)
-> InFlight
-> ReachedStandardTerminal(association)

or

Armed(association)
-> InFlight
-> Poisoned(association)
```

A payloadless `Consumed` success state is forbidden.

## Exact owner chain

The actual source roles are different:

```hako
ParserStringUtilsBox.skip_ws(
    text,
    me.static_const_eval_pos(ret),
)
```

```text
outer call:
  StaticReceiver
  owns ordered argument descent

inner selected call:
  ReceiverNormalized(MeCall)
  must prepare Standard(UnifiedMethod)
```

The accepted chain is:

```text
PreparedPreloopLocatedArgumentV1
  ↓
isolated candidate
  ↓
PreloopLocatedArgumentPortV1(existing Port)
  ↓
existing build_member_method_call_v1(outer located skip_ws)
  ↓
StaticReceiver
  ↓
existing drive_call_arguments_v1
  ├─ Argument(0) -> ordinary descent
  └─ Argument(1) -> selected located input
                         ↓
       existing build_member_method_call_v1(inner located call)
                         ↓
       ReceiverNormalized(MeCall)
                         ↓
       prepared route = Standard(UnifiedMethod)
                         ↓
       existing standard terminal
                         ↓
       emit_unified_call_with_lookup(CallTarget::Method)
```

This series proves an exact standard-unified Method **request**, not a
successful generic physical Call.

## Boundary before physical receipt

The unified emitter may still select:

```text
special rewrite
BoxCall
legacy compatibility emission
actual generic MirInstruction::Call
```

Therefore this series closes with:

```text
MeStandardUnified request                    = 1
CompletedUnifiedValueCallEmissionV1 producer = 0
EmittedNestedInstanceCallV1 producer         = 0
Integer publication                          = 0
```

Only `UNIFIED-CALL-PHYSICAL-RECEIPT0-S0` may issue:

```text
finalized mir_call.dst
-> successful emit_instruction(MirInstruction::Call)
-> existing post-success commit
-> CompletedUnifiedValueCallEmissionV1
```

Special rewrite, BoxCall, legacy emission, no-destination calls, and failed
instruction emission produce no receipt.

## Buildable implementation series

This is one BoxShape series. It changes no accepted source shape or result
fact. Every implementation commit must build.

### 1. `PRELOOP-LOCATED-ARGUMENT-RELATION0-S0`

Add the self-reference-free parent/argument product and the final source-only
co-seal. Builder references are zero.

Fixtures:

```text
Body(3).Value + Argument(1) succeeds
out-of-range index rejects
Argument(0) mismatches the selected association
foreign equal-looking view rejects
loop-refresh Argument(1) mismatches the pre-loop association
```

#### Closeout (2026-07-27)

Closed. `VerifiedRawLocatedCallArgumentV1` consumes its parent input and owns
the projected child, avoiding a self-referential owner. The final
`PreparedPreloopLocatedArgumentV1` co-seals that relation with the existing
pre-loop association by exact source-view identity, child site, and child AST
pointer. Its focused matrix covers the five fixtures above. No Builder,
candidate Port, physical Call receipt, `ValueId`, or type publication was
added.

Next executable row:

```text
METHOD-CALL-TERMINAL-NEUTRAL0-S0
```

### 2. `METHOD-CALL-TERMINAL-NEUTRAL0-S0`

Remove the unused source input from the value terminal and update the Raw,
LocatedLegacy, and route-test implementations mechanically.

```text
terminal behavior delta = 0
MIR delta               = 0
error-tag delta         = 0
```

#### Closeout (2026-07-27)

Closed. `MethodCallValueTerminalPortV1` is now source-neutral and receives
only already-materialized terminal operands. `MethodCallLoweringPortV1`
explicitly carries the separate descent capability needed by route handlers.
The Raw blanket implementation, LocatedLegacy session, and route-test port
were updated mechanically; targeted terminal, route-descent, and descent tests
remain green.

Next executable row:

```text
CALL-ARGUMENT-ONE-SHOT-SEAM0-S0
```

### 3. `CALL-ARGUMENT-ONE-SHOT-SEAM0-S0`

Permit one-shot mutable expression-input projection while keeping
`drive_call_arguments_v1` as the sole ordered algorithm.

```text
new driver       = 0
preflight copies = 0
default parity   = green
```

Repair the existing ARG0 guard before using it as acceptance evidence.

#### Closeout (2026-07-27)

Closed. `CallArgumentDescentPortV1::argument_expression_input` is now mutable,
but it remains callable only from the existing `drive_call_arguments_v1`
driver after whole-list and per-argument preflight. The Raw-compatible,
LocatedLegacy, and focused test ports preserve their existing projection
behavior. The existing ARG0 guard now recognizes the generic Raw-compatible
implementation and the current two production raw-facade references.

Next executable row:

```text
STANDARD-METHOD-EXECUTION-PREP0-S0
```

### 4a. `STANDARD-METHOD-EXECUTION-PREP0-S0`

Extract behavior-neutral standard prepare/execute products. Preserve weak
load, upgrade rejection, record/setter inline, and unified fallback.

#### Closeout (2026-07-27)

Closed. `PreparedStandardMethodExecutionV1` is now the only decision product
for the Standard method route: weak load, deprecated-upgrade rejection,
record-helper inline, allowlisted setter inline, or unified fallback. Its
prepare step borrows only existing Builder facts and callable declarations;
argument descent and MIR effects begin exclusively in the shared execute step.
Both the Legacy and associated descent callers consume that same product.
Focused prepare/execute coverage fixes the no-MIR-before-execute boundary.

Next executable row:

```text
ME-CALL-EXECUTION-PREP0-S0
```

### 4b. `ME-CALL-EXECUTION-PREP0-S0`

Extract behavior-neutral Me prepare/execute products and use the prepared
standard owner for its Standard branch.

```text
ordinary decisions unchanged
candidate connection = 0
argument/Call effects during prepare = 0
```

#### Closeout (2026-07-27)

Closed. `PreparedMeCallExecutionV1` seals the existing precedence as inline
record helper, inline setter, lowered global, Standard, static fallback, or
not-applicable. Its Standard variant retains the existing
`PreparedStandardMethodExecutionV1`, so `Standard(Unified)` is observable
before lowering while ordinary execution still consumes every disposition.
The focused bound-`me` fixture proves preparation emits no MIR and execution
alone reaches the existing Standard terminal.

Next executable row:

```text
PRELOOP-LOCATED-ARGUMENT-PORT0-S0
```

### 5. `PRELOOP-LOCATED-ARGUMENT-PORT0-S0`

Add small sibling modules:

```text
calls/preloop_located_argument_port.rs
calls/preloop_located_argument_rejection.rs
```

Add the candidate input, one-shot state, route state, and port. Do not add a
field or constructor to `AssociatedMethodCallArgumentsV1`.

### 6. `PRELOOP-LOCATED-ARGUMENT-DESCENT0-I0`

Connect one proof-only candidate:

```text
source relation
-> isolated transaction
-> candidate Port
-> exact selected argument
-> Standard(UnifiedMethod)
-> unified Method request
```

```text
production caller = 0
test/proof caller  = 1 bounded family
receipt            = 0
type publication   = 0
```

### 7. `PRELOOP-LOCATED-ARGUMENT-DESCENT0-P0`

Focused matrix:

```text
selected Argument(1) reaches exact unified Method request
Argument(0) remains ordinary
association is taken exactly once
success retains the association
duplicate selected descent rejects
foreign view and parked loop reject
alternate outer/inner/member/Me/standard routes reject
lowering failure poisons only the candidate
candidate failure -> fresh candidate success
ordinary Raw route parity
```

Put these in a new small test file. Do not extend
`member_route_descent_tests.rs`; it is already near 800 lines.

### 8. `PRELOOP-LOCATED-ARGUMENT-DESCENT0-G0`

Close the structural gate and point current work to:

```text
UNIFIED-CALL-PHYSICAL-RECEIPT0-S0
```

Do not add a new per-row shell wrapper. Repair and consolidate:

```text
tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_arg0.py
tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_route0.py
tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_v0.py
```

These guards are baseline-red against the current blanket/moved module layout
and are not acceptance evidence until repaired. Do not extend
`callable_result_i64_catalog_s0.py`; it is already near 800 lines.

## Verification

At the relevant commits:

```bash
cargo check --lib
cargo test -q --lib call_argument_descent
cargo test -q --lib method_call_descent
cargo test -q --lib member_route_descent
cargo test -q --lib method_call_terminal
cargo test -q --lib source_instance_result_contract
cargo test -q --lib raw_callable_source_view
```

After guard repair:

```bash
python3 tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_arg0.py
python3 tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_route0.py
python3 tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_v0.py
python3 tools/checks/lib/callable_result_i64_catalog_s0.py
bash tools/checks/current_state_pointer_guard.sh
```

Use the active card's heavier gate only at P0/G0.

## Structural gate

```text
source relation producer                              = 1
source navigation authority                          = existing 1
second navigation engine                             = 0
self-referential owner                               = 0

MethodCallLoweringPortV1 authority                   = existing 1
new MethodCallCompletionV1 trait                     = 0
Associated owner optional override                   = 0
candidate-only Port                                  = 1

ordered argument algorithm                           = existing 1
second ordered argument driver                       = 0
candidate preflight copy                             = 0
terminal source-input dependency                     = 0

Me and standard prepare/execute owners                = 1 each
ordinary route decision delta                        = 0
callee/Box-name candidate policy                     = 0

selected RawLocated -> RawLegacy conversion           = 0
AST re-walk / path reconstruction                     = 0
Builder source-site registry                         = 0
persistent source-site -> ValueId map                 = 0

MeStandardUnified request producer                   = 1
physical Call receipt producer                       = 0
nested result receipt producer                       = 0
MirType / type_ctx write                             = 0

LocatedLegacy production activation                  = 0
loop-refresh adapter / publisher delta               = 0

candidate effects before rejection                   = isolated only
live compiler Builder mutation on rejection          = 0
partial module publication                           = 0
fallback / retry / route reselection                 = 0

production caller                                    = 0
default route delta                                  = 0
all modified/new source/check files                  < 800 lines
```

## Proof inventory and sunset

```text
ceremony_tier:
  T2 new source-to-lowering boundary

proof_inventory_before:
  exact source-only association = 1
  descent consumer              = 0

new_proofs:
  located parent/argument relation
  one bounded candidate-Port fixture family

net_proof_delta:
  +1 until the physical/nested receipt series closes

sunset_id:
  PRELOOP-LOCATED-ARGUMENT-PROOF-SUNSET-001

sunset_owner:
  CALLABLE-RESULT-NESTED-PRELOOP-REP0-G0

sunset_row:
  CALLABLE-RESULT-NESTED-PRELOOP-PROOF-RETIRE0-S0

retire_when:
  real Stage-B consumer = 1
  + physical receipt correspondence green
  + nested receipt consumer = 1
  + proof-only candidate caller = 0
  + fallback = 0
```

## Required closeout

```text
Decision:
  PRELOOP-LOCATED-ARGUMENT-DESCENT0-prime-r1

Status:
  accepted

Choice:
  A-prime-Port

First executable row:
  PRELOOP-LOCATED-ARGUMENT-RELATION0-S0

Series:
  RELATION0
  -> TERMINAL-NEUTRAL0
  -> ONE-SHOT-SEAM0
  -> STANDARD-EXECUTION-PREP0
  -> ME-CALL-EXECUTION-PREP0
  -> PORT0
  -> DESCENT0-I0
  -> DESCENT0-P0
  -> DESCENT0-G0

Next:
  UNIFIED-CALL-PHYSICAL-RECEIPT0-S0
```

## Non-claims

```text
actual generic physical Call success
physical or nested-result receipt
Integer publication

loop-refresh production activation
GenericLoop publisher migration
LocatedLegacyLoweringSession activation

general located descent for all calls
production/default caller activation

parser / grammar / VM / backend change
ownership grammar activation
```
