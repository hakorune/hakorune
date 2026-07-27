---
Status: accepted decision; execution-order reconciliation
Date: 2026-07-27
Decision: PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION-RECONCILIATION0-prime-r1
Supersedes:
  - the R4-and-later execution order in preloop-stageb-owned-located-authority0-prime-r1-task-map-2026-07-27.md
Preserves:
  - PRELOOP-STAGEB-SOURCE-PRODUCER-SELECTION0-prime-r1
  - PRELOOP-STAGEB-OWNED-LOCATED-AUTHORITY0-prime-r1
---

# PRELOOP-STAGEB Instance Function Session Reconciliation

## Decision

```text
Decision:
  PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION-RECONCILIATION0-prime-r1

Status:
  accepted

Classification:
  BoxShape execution-order correction
  semantic decision delta = 0

First executable row:
  PRELOOP-STAGEB-INSTANCE-DECLARATION-RECIPE0-S0
```

The A1 whole-source producer and A-double-prime owned-located authority remain
accepted. R1-R3 are closed and are not rebuilt.

Read-only R4 audits found three implementation facts that make the previous
R4-and-later order non-executable:

```text
1. the verified callable declaration row omits uses and DeclarationAttrs

2. the selected outer RawLocated MethodCall has no direct execution seam;
   the current proof fixtures clone it into RawLegacyMethodCallInputV1

3. the actual ParserBox suffix contains GenericLoop and consumes the outer
   skip_ws result type before the old downstream outer-type rows could run
```

The third fact is a temporal cycle:

```text
old order:
  finish selected function
  -> activate production ingress
  -> create outer receipt
  -> publish outer Integer

actual dependency:
  selected outer Call succeeds
  -> assign exact outer destination
  -> publish exact outer Integer
  -> suffix GenericLoop may lower
  -> selected function may finish
```

Therefore outer physical receipt, assignment correspondence, and exact type
publication move inside the selected statement transaction, before suffix
descent. This does not change the accepted source result or type policy.

## Preserved authority chain

```text
one shared exact callable catalog allocation
  ↓
PreparedPreloopStageBFunctionIngressV1
  - exact caller
  - exact declaration
  - exact prefix / selected / suffix schedule
  - exact outer MethodCall
  - structural CallArgument(1)
  - exact nested Integer result authority
  ↓
selected function transaction
  ├─ prefix: existing ordinary Raw descent
  ├─ selected statement:
  │    exact located outer StaticReceiver Call
  │      ↓
  │    exact located inner Me Standard(Unified) Call
  │      ↓
  │    existing physical Call writer
  │      ↓
  │    outer physical success receipt
  │      ↓
  │    existing assignment-from-value authority
  │      ↓
  │    exact outer Integer publication
  └─ suffix: existing ordinary Raw descent
       ↓
existing finalizer
       ↓
unpublished function draft + owned Stage-B completion evidence
```

## Correction 1 — complete declaration execution recipe

The existing verified declaration row is already the exact source declaration
owner, but currently retains only:

```text
key / params / param_decls / return type / body
```

The existing instance-function preparation additionally requires:

```text
uses / DeclarationAttrs
```

R4 must not synthesize empty values. The same verified row retains the missing
metadata and exposes borrowed accessors.

```rust
pub(crate) struct VerifiedSameModuleCallableDeclarationV1 {
    // existing fields
    uses: Box<[String]>,
    attrs: DeclarationAttrs,
}
```

The Stage-B function recipe projects, from the same row:

```text
canonical key
owner / method / arity
params / ParamDecl
declared return
body
uses
attrs
exact selected variable assignment target
```

This is declaration transport, not a second metadata policy.

The selected target is issued during the existing source-view descent. F1
requires:

```text
SelectedStatementMustBeVariableAssignment
```

and projects the exact variable target into a named owned seal:

```rust
OwnedPreloopCarrierAssignmentTargetV1
```

The seal holds the exact variable binding spelling but is constructible only
from the source-view-issued selected statement. Field, index, dereference, and
other targets reject in F1. A caller-provided string, body ordinal, or late AST
observation in F5/F6 is not assignment authority.

## Correction 2 — located outer completion

The selected inner MethodCall already has a candidate-only located ingress and
must never convert to RawLegacy. The R4 audit also found that the outer located
MethodCall currently has no exact execution seam.

The clean correction is one thin source-neutral completion capability over the
existing member/static route owners:

```text
RawLocatedMethodCallInputV1
  -> existing MethodCallSyntaxViewV1
  -> existing member route preparation
  -> require StaticReceiver
  -> existing static handler
  -> existing ordered argument driver
```

The static handler may be generalized over the already-existing descent and
terminal traits. It must not gain source selection policy.

Concretely, the current static handler's
`AssociatedMethodCallArgumentsV1` parameter must be made mechanically generic
over:

```text
MethodCallArgumentDescentV1
+ the existing value-terminal completion capability
```

The ordinary `AssociatedMethodCallArgumentsV1` remains one adapter and the
located outer completion becomes the second consumer. An optional override in
the ordinary owner is forbidden.

```text
member route planner                              = existing 1
static handler algorithm                          = existing 1
ordered argument driver                           = existing 1
selected outer RawLocated -> RawLegacy conversion = 0
selected inner RawLocated -> RawLegacy conversion = 0
```

If the implementation inventory proves that the existing static handler cannot
accept a source-neutral completion capability without duplicating route or
argument policy, stop at:

```text
PRELOOP-LOCATED-OUTER-COMPLETION0-D0
```

Do not fall back to an AST clone or a second dispatcher.

## Correction 3 — payload-bearing function session

The existing function pending session fixes its operation to:

```text
Result<MirFunction, String>
```

The selected Stage-B transaction must retain an unpublished draft together
with an owned completion payload, and typed failures must survive exact parent
restoration. Add one generic payload sibling over the same private session
kernel.

Conceptual products:

```rust
LegacyFunctionPayloadPendingSessionV1<'builder, P>

LegacyFunctionPayloadSessionErrorV1<E, P> {
    Primary(E),
    CleanupAfterSuccess {
        payload: P,
        detail: Box<str>,
    },
    DuringCleanup {
        primary: E,
        detail: Box<str>,
    },
}
```

The existing `Result<MirFunction, String>` terminal becomes the `P = ()`
adapter. Restoration logic is not copied.

Forbidden:

```text
external mutable Option payload slot
payloadless Consumed / Poisoned
draft accessor before completion
retry / resume / rearm
```

## Correction 4 — outer receipt and assignment before suffix

The source-neutral global/static value terminal gets one receipt-required
sibling using the existing generic physical Call terminal:

```text
finalized outer Call destination
-> MirInstruction::Call emit success
-> existing post-success commit
-> CompletedUnifiedValueCallEmissionV1
```

It issues no receipt for:

```text
rewrite / BoxCall / legacy route / no destination / Call failure
```

The Stage-B owner then seals:

```rust
CompletedPreloopOuterCarrierCallV1 {
    source,
    inner,
    result,
    outer,
}

CompletedPreloopCarrierAssignmentV1 {
    outer,
    assigned_value,
}
```

Required correspondence:

```text
outer.final_destination
  == assigned_value
  == value installed for the exact selected variable target
```

The current Port state must be corrected explicitly:

```text
InnerReached(
  source + ReachedPreloopNestedPhysicalCallV1
)
  + CompletedUnifiedValueCallEmissionV1(outer)
-> OuterReached(CompletedPreloopOuterCarrierCallV1)
```

It must not first collapse the inner authority into the current
destination-only `EmittedNestedInstanceCallV1` and then treat that destination
as the carrier.

```text
EmittedNestedInstanceCallV1 as outer-carrier producer = 0
```

Before the R3 HRTB callback ends, every borrowed selected rejection must be
consumed into an owned Stage-B rejection. Private consuming projections may
recover the opaque owned rebind authority from:

```text
RejectedPreloopLocatedArgumentIngressV1
ReachedPreloopNestedPhysicalCallV1
```

They are not public owner escape or retry terminals.

Assignment uses the existing `MirBuilder::build_assignment_from_value`
authority after its exact variable target has been source-sealed. No second
assignment implementation is added.

## Correction 5 — success-only type publication

The already accepted policy remains unchanged:

```text
None / Unknown -> Publish(Integer)
Integer        -> Idempotent
other concrete-> Conflict
```

Only:

```text
TypeFactDecisionV1
TypeContext::set_type
```

may commit the fact.

Exact temporal law:

```text
inner Call failure
  -> outer Call / assignment / type / suffix = 0

outer Call failure
  -> assignment / type / suffix = 0

assignment mismatch
  -> type / suffix = 0

type conflict
  -> suffix / function publication = 0

type commit success
  -> suffix ordinary descent may start
```

The inner destination is never used as the outer carrier destination.
GenericLoop remains a fact consumer and gains no publisher.

## One body driver

Use one `drive_legacy_block_v1` invocation with a bounded schedule port:

```text
index < selected:
  existing ordinary statement descent

index == selected:
  exact located assignment transaction

index > selected:
  existing ordinary statement descent
```

Suffix routing is fenced by the sealed schedule:

```text
prefix route input   = stops before selected
selected route input = none
suffix route input   = existing suffix behavior
```

This prevents the ordinary suffix router from consuming across the selected
statement boundary.

## Reconciled buildable series

### F0 — closeout

```text
PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION-RECONCILIATION0-D1-CLOSEOUT
```

This card closes D1. No code behavior changes.

### F1 — complete declaration recipe

```text
PRELOOP-STAGEB-INSTANCE-DECLARATION-RECIPE0-S0
```

Status: closed

Implement:

```text
catalog retains exact uses and DeclarationAttrs
Stage-B recipe retains exact selected assignment target
existing declaration order/key/lookup behavior unchanged
```

Focused proof:

```text
non-empty uses retained exactly
non-default attrs retained exactly
catalog reorder parity unchanged
default ordinary lowering parity unchanged
```

Landed evidence:

```text
verified catalog row retains exact uses / DeclarationAttrs
metadata remains paired with canonical rows across declaration reorder
selected statement is issued by the existing catalog-backed source view
variable assignment target is privately sealed as
  OwnedPreloopCarrierAssignmentTargetV1
field assignment target rejects at OwnedRow / BodyHandoff
function-ingress recipe retains target / uses / attrs
Builder / Call / type / production behavior delta = 0
```

### F2 — generic function payload session

```text
PRELOOP-STAGEB-FUNCTION-PAYLOAD-SESSION0-S0
```

Implement the generic payload/error sibling and make the existing legacy
terminal a thin unit-payload adapter.

Focused proof:

```text
payload success retained until completion
typed primary retained
cleanup-after-success retains payload
cleanup-during-failure retains primary
parent restored exactly once
failure -> fresh session success
```

Landed evidence:

```text
one private PendingFunctionPayloadSessionCloseV1 restoration owner
LegacyFunctionPayloadPendingSessionV1<P> keeps draft + non-Clone payload
typed Primary / CleanupAfterSuccess / DuringCleanup ownership is preserved
existing resolved and legacy pending terminals use the P=() adapter
P: 'static / Clone requirements = 0
external mutable payload slot / draft accessor / retry / resume = 0
focused payload, unit-terminal, and legacy invocation regressions = green
```

### F3 — exact located outer completion

```text
PRELOOP-LOCATED-OUTER-COMPLETION0-S0
-> PRELOOP-LOCATED-OUTER-COMPLETION0-P0/G0
```

Implement only the source-neutral completion seam. Production caller remains
zero.

Focused proof:

```text
outer route = exact StaticReceiver
Argument(0) ordinary
Argument(1) existing selected located ingress
alternate route typed reject
RawLegacy construction zero
new outer physical receipt / type publication zero
existing selected-inner physical receipt remains unchanged
```

Landed evidence:

```text
exact outer RawLocatedMethodCallInputV1 consumer = 1
StaticMethodCallCompletionV1 authority = 1
member route planner / static handler / ordered argument driver = existing 1
Argument(0) ordinary + structural Argument(1) selected = green
outer RawLocated -> RawLegacy conversion = 0
success retains exact inner source/physical owner
outer product carries requested ValueId only
new outer physical receipt / type publication / production caller = 0
alternate outer route rejects before argument effects
```

### F4 — outer physical receipt

```text
UNIFIED-CALL-OUTER-CARRIER-RECEIPT0-S0
-> UNIFIED-CALL-OUTER-CARRIER-RECEIPT0-P0
```

Add one source-neutral receipt-required global/static terminal.

Focused proof:

```text
actual generic Call success -> exact final destination receipt
alternate route / Call failure -> receipt zero
source import / result inference / type write zero
```

Landed evidence:

```text
PreparedGlobalValueCallRequestV1 producer = 1
ordinary static/global terminal consumer = 1
receipt-required static/global terminal consumer = 1
generic physical Call writer / post-success authority = existing 1
success receipt destination == emitted Call destination
unified-disabled / emission failure receipt = 0
legacy retry / source policy / result inference / type write = 0
production caller = 0
```

### F5 — outer carrier transaction

```text
PRELOOP-OUTER-CARRIER-RECEIPT0-I0
-> PRELOOP-OUTER-CARRIER-ASSIGNMENT0-S0
-> PRELOOP-OUTER-CARRIER-TYPE-I0-S0
-> PRELOOP-OUTER-CARRIER-TYPE-I0-I0/P0/G0
```

Close, in order:

```text
outer Call receipt
exact assignment correspondence
existing Integer fact decision
success-only type commit
```

This series is disconnected from a production function until F6.

### F6 — selected instance-function session

```text
PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION0-I0
-> PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION0-P0/G0
```

Compose the already-closed F1-F5 products:

```text
existing instance-function preparation
existing StepTree guard
one bounded body schedule driver
prefix ordinary
selected located carrier transaction
outer Integer commit
suffix ordinary
existing finalizer
generic payload pending session
```

Success retains:

```text
unpublished MirFunction
exact nested Call evidence
exact outer carrier evidence
exact assignment evidence
exact type publication evidence
```

Suffix, finalizer, or session-cleanup failure retains the recipe plus the
strongest completed carrier/type evidence. It must not collapse that owner to
`String`, and it must not expose a retryable Port or source witness.

Status: closed

Landed evidence:

```text
one catalog-backed source projection
  -> existing instance preparation
  -> existing StepTree guard
  -> sole F6 body schedule
  -> shared finalizer preparation
  -> header-aware finalization
  -> generic payload session

Phase-A-indexed actual Parser:
  unpublished function draft                          = 1
  physical parameters                                = 3
  inner physical Call precedes outer physical Call    = 1
  inner destination != outer destination              = 1
  assignment destination == outer destination         = 1
  unannotated function return                         = Unknown
  exact outer carrier fact                            = Integer
  parent restoration                                  = exact
  module publication                                  = 0

without Phase A:
  typed stage                                         = BodySchedule
  completed carrier evidence retained                 = 1
  draft publication                                   = 0
  parent restoration                                  = exact
  same Builder + fresh ingress + existing indexer
    -> successful unpublished function                = 1

retry / resume / rearm                                = 0
activation-ledger consumer                            = 0
compile_request consumer                              = 0
```

### F7 — function activation ledger

```text
PRELOOP-STAGEB-INSTANCE-METHOD-CAPTURE-SEAM0-S0
-> PRELOOP-STAGEB-FUNCTION-ACTIVATION-LEDGER0-I0
-> PRELOOP-STAGEB-FUNCTION-ACTIVATION-LEDGER0-P0
-> PRELOOP-STAGEB-FUNCTION-ACTIVATION-LEDGER0-G0
```

Only now add states with real producers:

```text
Armed
-> InFlight
-> Completed(exact function receipt)
 | Rejected(retained owner + typed cause)
```

Current implementation fact:

```text
legacy_module_activation/ledger.rs:
  Armed + SelectedCallerNotObserved only

preinstalled-root lowering:
  immutable installed context only
  stack ledger is not threaded to the instance-method inventory

F6 completed-function producer:
  real and green
```

Buildable cells:

```text
F7-1 INSTANCE-METHOD-CAPTURE-SEAM0-S0
  thread one mutable stack-scoped selection capability through the existing
  post-install root kernel and instance-method inventory
  ordinary roots and ordinary methods remain mechanically unchanged

F7-2 FUNCTION-ACTIVATION-LEDGER0-I0
  exact canonical caller key:
    mismatch -> Ordinary without consumption
    exact first observation -> Armed -> InFlight
    F6 success -> Completed(exact unpublished function receipt)
    F6 rejection -> Rejected(exact retained typed owner)

F7-3 FUNCTION-ACTIVATION-LEDGER0-COMMIT0-I0
  admit the completed F6 draft through the existing module-lowering
  collector exactly once
  no draft is visible before ledger completion

F7-4 FUNCTION-ACTIVATION-LEDGER0-P0/G0
  exact-once / missing / duplicate / F6 failure retention
  ordinary method parity / fresh-ledger reuse / structural guard
```

Required fixtures:

```text
exact selected caller once        -> Completed(F6 receipt)
non-selected instance method      -> existing ordinary lowering
selected caller absent            -> SelectedCallerNotObserved
selected caller observed twice    -> SelectedCallerConsumedTwice
selected F6 failure               -> Rejected(retained F6 owner)
failed ledger -> fresh ledger     -> success
```

Forbidden:

```text
Builder source-site field
name or symbol scan
payloadless Consumed / Poisoned
reset / retry / resume / rearm
selected failure -> ordinary lowering
catalog reseal
module publication before ledger completion
compile_request consumer
```

F7 hard stop:

```text
the exact canonical key cannot be observed at the existing instance-method
inventory without a Builder field, source map, name scan, or second method
lowering orchestration
```

### F8 — compile-request production ingress

```text
PRELOOP-STAGEB-UNAVAILABLE-DISPOSITION0-S0/P0
-> PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION-CORRESPONDENCE0-P0
-> PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-D0
-> PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-S0/P0/G0
-> PRELOOP-STAGEB-LEGACY-WHOLE-SOURCE-REQUEST0-I0
-> PRELOOP-STAGEB-LEGACY-ORDINARY0-P0
-> PRELOOP-STAGEB-COMPILE-REQUEST-INGRESS0-I0
-> PRELOOP-STAGEB-COMPILE-REQUEST-INGRESS0-P0/G0
```

This remains the first production selector behavior change:

```text
MirCompiler::compile_request Legacy arm = sole consumer
Ordinary = existing route
Selected = exact Stage-B transaction
selected failure -> no Ordinary retry
```

Do not rebuild these already-closed prerequisites:

```text
typed compiler-supplied import snapshot
owned Legacy whole-source request
whole-source target and Stage-B candidate inventory
explicit Zero / One / Many products
explicit Ordinary / Selected disposition
same-allocation selected activation
atomic catalog + alias install
preinstalled-root shell
outer physical Call receipt
assignment correspondence
outer Integer publication
```

Buildable cells:

```text
F8-1 UNAVAILABLE-DISPOSITION0-S0/P0
  retain the deterministic first bounded proof-unavailable stage in the
  existing one-pass inventory
  unrelated source -> NoExactCandidate
  nested-looking but incomplete bounded proof
    -> ExactCandidateProofUnavailable(stage)
  import alias and identity invariants remain typed errors

F8-2 SELECTED-CANDIDATE-SESSION-CORRESPONDENCE0-P0
  inventory the exact candidate configuration and commit matrix
  prove the current canonical and branded Raw sessions are not exact owners

F8-3 mandatory SELECTED-CANDIDATE-SESSION0-D0
  select one source-neutral isolated candidate owner before code connection

F8-4 SELECTED-CANDIDATE-SESSION0-S0/P0/G0
  candidate creation
  -> existing prepare_module
  -> activation preflight
  -> same-allocation catalog + typed alias install
  -> preinstalled root + completed F7 ledger
  -> existing finalize_module / compiler postprocess
  -> success-only live Builder replacement

F8-5 LEGACY-WHOLE-SOURCE-REQUEST0-I0
  MirLoweringRequestV1::Legacy carries the owned request
  compile_with_source supplies None
  compile_with_source_and_imports supplies Explicit
  remove live Builder alias set/clear before selection

F8-6 LEGACY-ORDINARY0-P0
  None / Explicit(empty) / Explicit(map) parity
  ProgramV0 / REPL / direct Builder callers unchanged

F8-7 COMPILE-REQUEST-INGRESS0-I0/P0/G0
  MirCompiler::compile_request Legacy arm is the sole selector consumer
  0 -> Ordinary
  1 -> Selected
  many -> typed pre-Builder rejection
  selected failure never retries Ordinary
```

The mandatory D0 must preserve:

```text
CoreContext continuation
repl_mode
quiet_internal_logs
plugin_method_sigs
diagnostic source hint
typed aliases only through the selected activation transaction
all failure -> isolated candidate drop
all success -> one live Builder replacement
```

Known non-solutions:

```text
CanonicalModuleLoweringSessionV1
  preserves quiet_internal_logs only

ModuleBuilderInvocationSessionV1
  preserves broader configuration
  but requires a Raw/Canonical family token and brand

install catalog/aliases before prepare_module
  invalid because prepare_module clears candidate context
```

The D0 must choose a Builder-owned narrow prepare/root/finalize terminal or an
equivalent source-neutral kernel. It must not duplicate the build algorithm or
borrow Raw/canonical lifecycle authority.

### F9 — real Stage-B proof and retirement

```text
CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0
-> PRELOOP-INNER-TYPE-PROOF-CENSUS0-P0
```

Run the real progression guard only after F8. Do not change its expectation
before the new producer is active and green.

Exact order:

```text
1. run generic_loop_progression_role_v0_guard.sh unchanged
2. retain the first actual result
3. reconcile the guard expectation only from that observation
4. run PRELOOP-INNER-TYPE-PROOF-CENSUS0-P0
5. select the next frontier from the observed blocker
```

Do not preselect ownership grammar, loop-refresh, Alias/View, or another
representation row before the unchanged guard names the next frontier.

The census decides whether the old inner publisher remains a proof-only
consumer or is retired. `PRELOOP-INNER-TYPE-PROOF-RETIRE0-S0` is executable
only when:

```text
production consumers = 0
proof-only required consumers = 0
fallback / retry = 0
```

Otherwise it is parked with its exact proof-only owner documented; retirement
is not mandatory merely because the outer carrier path is green.

After the guard, select the next actual frontier from evidence:

```text
ownership grammar
parked loop-refresh activation
another missing representation
```

Alias/View language semantics are not selected by this series.

## Failure retention

The selected session owns typed stage/cause and the strongest durable owner
available at the failure time:

```text
before selected Call:
  recipe + owned nested rebind authority

after inner success:
  recipe + inner physical evidence

after outer success:
  recipe + outer physical evidence

after assignment:
  recipe + assignment correspondence

after type commit:
  recipe + completed carrier evidence
```

Public rejection surface:

```text
stage()
cause()
bounded_report()
discard(self)
```

Forbidden:

```text
into_owner()
retry()
resume()
rearm()
try ordinary
route reselection
```

## Structural gate

```text
callable declaration metadata owner                 = existing catalog row 1
fabricated empty uses / attrs                       = 0

selected outer located completion producer          = 1
selected outer RawLocated -> RawLegacy conversion   = 0
selected inner RawLocated -> RawLegacy conversion   = 0
second member/static route algorithm                = 0

function session restoration authority              = existing 1
generic payload sibling                             = 1
external mutable payload slot                       = 0

physical Call writer                                = existing 1
outer receipt producer                              = 1
inner destination treated as outer                  = 0
EmittedNestedInstanceCallV1 outer producer           = 0

assignment-from-value authority                     = existing 1
second assignment implementation                    = 0

TypeFactDecisionV1 authority                        = existing 1
TypeContext::set_type authority                     = existing 1
direct value_types insert                           = 0
concrete overwrite                                  = 0

suffix descent before outer type success            = 0
GenericLoop type producer                           = 0

whole-source production selector                    = exact 1 after F8
direct Builder / JSON / Raw selector caller         = 0
fallback / retry                                    = 0

all modified/new source/check files                 < 800 lines
```

## Non-claims

```text
general located MethodCall lowering
whole port-aware Raw cutover
general instance-method result inference
general Builder transaction

loop-refresh activation
GenericLoop publisher migration

Alias / View language semantics
ownership grammar activation
parser / VM / LLVM / backend changes
default compiler/backend cutover
fallback / retry
```
