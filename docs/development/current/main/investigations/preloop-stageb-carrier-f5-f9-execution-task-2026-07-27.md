# PRELOOP-STAGEB-CARRIER F5-F9 execution task

Status: bounded source-observation repair active; F8/F9 parked after candidate=1

Parent:

```text
preloop-stageb-instance-function-session-reconciliation0-prime-r1
```

Current row:

```text
SAME-MODULE-CALLABLE-RECEIVER-POLICY0-S0
```

## Closed prerequisites

```text
F1 exact declaration/body recipe retention
F2 generic payload-preserving pending function session
F3 exact located outer request through the existing static handler
F4 source-neutral static/global physical Call receipt sibling
F5-A exact outer carrier receipt

source-producer prerequisites already closed:
  whole-source inventory
  typed Legacy request and alias snapshot
  explicit Ordinary | Selected decision
  same-allocation module activation preparation/install
```

F4 commit:

```text
d9b18cb58d
```

## Current bounded terminal

The current A-prime repair ends at:

```text
SAME-MODULE-CALLABLE-RECEIVER-POLICY0-S0
-> STATIC-CURRENT-OWNER-METHOD-OBSERVATION0-S0/P0
-> PRELOOP-STAGEB-STATIC-CURRENT-OWNER-TARGET0-P0
-> PRELOOP-STAGEB-SOURCE-INVENTORY0-P0b/G0
```

Required terminal evidence is one exact actual Parser candidate at `Body(3)`.
After G0, all candidate-session, Legacy request/compile ingress, F9, and
retirement-census rows in this card are parked. The current pointer returns to
`OWN-GRAM-REJECT0`.

They may be reselected only if the unchanged ownership gate proves one of
them is a direct prerequisite. Discovery inside this card does not extend the
bounded terminal.

The remaining chain is:

```text
owned function body recipe
  + exact located inner/outer source owner
  + successful inner physical Call
  + successful outer physical Call
  + exact assignment correspondence
  + existing Integer fact decision
  -> unpublished selected function
  -> single-use activation ledger
  -> sole compile_request Legacy consumer
  -> real Stage-B proof
```

Do not reconstruct authority from names, MIR scans, runtime values, or a
destination-only inner receipt.

## F5-A — outer carrier receipt

Rows:

```text
PRELOOP-OUTER-CARRIER-RECEIPT0-I0
-> PRELOOP-OUTER-CARRIER-RECEIPT0-P0
```

Required owner:

```text
CompletedPreloopOuterCarrierCallV1
  - complete owned function body recipe
  - exact inner ReachedPreloopNestedPhysicalCallV1
  - CompletedUnifiedValueCallEmissionV1 for the outer Call
```

Co-seal all of:

```text
recipe caller       == exact located caller
recipe outer site   == exact located outer site
recipe selected idx == exact structural argument index
recipe inner site   == exact located inner site
recipe result       == exact owned Integer marker
```

The outer destination authority is only:

```text
CompletedUnifiedValueCallEmissionV1::final_destination()
```

Forbidden:

```text
requested ValueId as final authority
inner destination as outer destination
EmittedNestedInstanceCallV1 as outer producer
second Call writer
source/result inference in the terminal
```

Port transition:

```text
ReachedPhysical(inner)
  + successful F4 receipt terminal
-> OuterReached(inner + outer receipt)

failure
-> retained typed rejection
```

Because `StaticMethodCallCompletionV1` still returns `Result<ValueId, String>`,
the candidate Port retains the typed outer failure first and projects one
bounded control error only for the existing handler. Do not widen the general
trait.

Focused matrix:

```text
inner success + outer success -> one outer receipt
outer Call failure            -> outer receipt zero
unified disabled              -> legacy retry zero
recipe/source drift           -> typed reject
inner destination != outer destination
```

Landed evidence:

```text
inner receipt preflight before outer emission = 1
outer physical receipt producer = existing F4 sibling 1
duplicate outer emission = 0
outer failure retains exact inner physical owner
wrong completion terminal = typed retained rejection
recipe/source/physical co-seal producer = 1
outer destination from physical receipt only
assignment / type / production caller = 0
```

## F5-B — assignment correspondence

Rows:

```text
PRELOOP-OUTER-CARRIER-ASSIGNMENT0-S0
-> PRELOOP-OUTER-CARRIER-ASSIGNMENT0-P0
```

Reuse:

```text
existing MirBuilder::build_assignment_from_value authority
```

Add one source-neutral immediate completion product at that authority:

```text
CompletedVariableAssignmentV1
  - exact target
  - RHS ValueId
  - returned assigned ValueId
```

Required product:

```text
CompletedPreloopCarrierAssignmentV1
  - CompletedPreloopOuterCarrierCallV1
  - CompletedVariableAssignmentV1
  - exact source-sealed assignment target
```

Required equality:

```text
outer final destination
  == assignment RHS
  == assignment returned carrier
```

Do not prove this by reading `variable_map` after the assignment. If the
existing assignment authority creates a different destination, reject with a
typed correspondence error and stop this row; do not repair or infer.

Focused matrix:

```text
exact destination equality -> success
RHS mismatch               -> typed reject
returned carrier mismatch  -> typed reject
assignment failure         -> type publication zero
failure -> fresh fixture success
```

Buildable cells:

```text
F5-B1 source-neutral CompletedVariableAssignmentV1 sibling
F5-B2 exact pre-loop carrier/assignment co-seal
F5-B3 actual Parser fixture, negative matrix, and pointer closeout
```

Hard stop:

```text
actual selected assignment returns a carrier different from
the outer physical destination
```

If observed, retain the typed mismatch and open a new assignment-carrier D0.
Do not compensate with `variable_map`, a copy, or a second assignment writer.

Landed evidence:

```text
source-neutral assignment completion producer = 1
existing build_assignment_from_value calls     = exactly 1
second assignment writer                       = 0
post-assignment variable_map inference         = 0

source-sealed target == assignment target
outer destination == assignment RHS == returned carrier

assignment failure retains complete outer carrier
correspondence drift retains both complete owners
failure -> fresh fixture success
type publication / suffix / production caller = 0
```

## F5-C — outer Integer publication

Rows:

```text
PRELOOP-OUTER-CARRIER-TYPE-I0-S0
-> PRELOOP-OUTER-CARRIER-TYPE-I0-I0
-> PRELOOP-OUTER-CARRIER-TYPE-I0-P0
-> PRELOOP-OUTER-CARRIER-TYPE-I0-G0
```

Required chain:

```text
CompletedPreloopCarrierAssignmentV1
-> existing TypeFactDecisionV1
-> private adjacent commit
-> CompletedPreloopOuterCarrierIntegerPublicationV1
```

Decision matrix:

```text
None / Unknown -> Publish(Integer)
Integer        -> Idempotent, physical write zero
other concrete-> typed conflict, existing fact unchanged
```

Only `Publish` may call `TypeContext::set_type`. Direct
`value_types.insert` is forbidden.

Both success and rejection retain the complete assignment carrier. The
publisher must not import the inner receipt or inner destination. GenericLoop
remains consumer-only.

Focused matrix:

```text
None
Unknown
Integer
Bool conflict
inner destination unchanged
conflict -> fresh fixture success
Call/assignment failure -> publisher product zero
```

Buildable cells:

```text
F5-C1 prepared/complete publication typestates
F5-C2 assignment transaction connection
F5-C3 focused fact matrix and structural guard
```

Landed evidence:

```text
outer publication prepared/completed/rejected owner = 1 each
outer publication terminal                           = 1
TypeFactDecisionV1 policy call                       = 1
TypeContext::set_type writer                         = 1 Publish branch only

None / Unknown -> Publish(Integer)
Integer        -> Idempotent with physical write zero
Bool conflict  -> existing fact and complete carrier retained
conflict -> fresh configured fixture success

inner destination sentinel remains unchanged
inner receipt/destination import in publisher = 0
direct value_types insert                      = 0
GenericLoop producer                           = 0
suffix / function session / production caller = 0
```

## F6 — selected instance-function session

Rows:

```text
PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION0-I0
-> PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION0-P0
-> PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION0-G0
```

One bounded body schedule:

```text
prefix ordinary
-> selected located carrier transaction
-> outer Integer publication
-> suffix ordinary
-> existing finalizer
-> F2 generic payload pending session
```

Suffix cannot begin before F5-C success. Every failure retains the recipe and
the strongest completed F5 evidence. No ordinary retry is allowed.

Buildable cells:

```text
F6-1 PRELOOP-STAGEB-COMPLETION-EVIDENCE0-S0
  consume the borrowed F3-F5 chain inside the HRTB callback
  -> lifetime-free CompletedPreloopStageBCarrierV1

F6-2 PRELOOP-STAGEB-BODY-SCHEDULE0-I0
  one bounded LegacyBlockDescentPortV1
  prefix ordinary -> selected F3-F5 transaction -> suffix ordinary

F6-3 PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION0-I0
  existing instance preparation / StepTree / finalizer
  + capture_legacy_function_payload_pending_session_v1

F6-4 PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION0-P0/G0
  actual Phase-A-indexed Parser full-function parity
  + typed failure retention, parent restoration, reuse, structural gate
```

Landed F6-1 evidence:

```text
CompletedPreloopStageBCarrierV1 producer          = 1
borrowed F3-F5 -> owned completion projection     = 1
retained-only nested authority                    = 1
exact inner Call receipt                          = 1
exact outer Call receipt                          = 1
exact assignment receipt                          = 1
exact publication disposition                     = 1

borrowed ingress rejection -> owned rejection     = 1
borrowed outer rejection -> owned rejection       = 1
borrowed carrier rejection -> owned rejection     = 1
borrowed assignment rejection -> owned rejection  = 1
borrowed type rejection -> owned rejection        = 1

actual Parser HRTB owned-success escape            = green
production consumer                               = 0
retry / rebind / source-MIR re-observation         = 0
```

Landed F6-2 evidence:

```text
bounded Stage-B body schedule producer             = 1
Legacy block driver                                = existing 1
Legacy statement driver                            = existing 1
second body/statement driver                       = 0

prefix suffix-routing stops before selected row    = 1
selected F3-F5 transaction consumer                = 1
suffix routing before publication success          = 0

actual Parser:
  inner Call != outer Call
  assignment carrier == outer Call
  outer Integer fact visible before suffix
  Body(4) remains the exact typed suffix frontier

suffix rejection retains complete published carrier = 1
retry / fallback / rebind                            = 0
function finalizer / activation ledger / caller      = 0
```

The F6 body-schedule hard stop is cleared. The sole next cell is
`PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION0-I0`; it must place the bounded
schedule inside the existing instance preparation, StepTree, finalizer, and
generic payload-preserving pending session. The local F6-2 fixture keeps its
observed Body(4) frontier as a typed boundary proof because it deliberately
does not run module-root declaration indexing. F6-3/F6-4 must use a separate
actual Parser fixture after the existing Phase A declaration indexer has
populated the module context. It must not fabricate a `ParserStringUtilsBox`
binding or add a second suffix/GenericLoop algorithm.

### F6 ownership closure

F3-F5 products borrow the located source chain and cannot escape
`PreparedPreloopStageBFunctionIngressV1::with_prepared_located_argument`.
Before that callback ends, one private consuming projection must produce:

```text
CompletedPreloopStageBCarrierV1
  - owned recipe / nested rebind witness
  - inner physical destination evidence
  - outer physical destination evidence
  - assignment correspondence
  - Integer publication disposition
```

Use the existing
`PreparedPreloopLocatedArgumentV1::into_owned_rebind_witness()` through private
delegating terminals. Public `into_owner`, AST/MIR re-observation, destination-
only collapse, borrowed-owner escape, and an external mutable payload slot are
forbidden.

Recommended implementation layout:

```text
src/mir/builder/calls/
  preloop_stageb_instance_function_session/
    session.rs
    session_rejection.rs
    session_tests.rs
    test_support.rs                    # cfg(test), only if needed

tools/checks/lib/
  callable_result_i0_site0_r0_expr0_m0_v0_stageb_session.py
```

The module owns one bounded schedule port, one Stage-B request, one private
pending newtype over the existing generic payload session, one owned completion
payload, and typed primary/session rejection families. It does not own a
second body driver, function finalizer, or parent-restoration algorithm.

Required session products:

```text
PreparedPreloopStageBInstanceFunctionV1
CompletedPreloopStageBInstanceFunctionPayloadV1
PendingPreloopStageBInstanceFunctionSessionV1
CompletedPreloopStageBInstanceFunctionV1

PreloopStageBInstanceFunctionPrimaryRejectionV1
RejectedPreloopStageBInstanceFunctionSessionV1
```

The completed payload owns the entire
`CompletedPreloopStageBBodyScheduleV1`; destination summaries are
insufficient. Map all three existing
`LegacyFunctionPayloadSessionErrorV1` variants structurally while the typed
payload or primary rejection is still owned:

```text
Primary(primary)
CleanupAfterSuccess { payload, detail }
DuringCleanup { primary, detail }
```

No failure may be collapsed to `String` before that projection.

Exact operation order:

```text
prepare existing instance skeleton/signature/uses/attrs/params
-> existing StepTree guard
-> one drive_legacy_block_v1
     prefix ordinary
     selected exact outer Call -> assignment -> Integer publication
     convert borrowed F5 success/rejection to owned Stage-B evidence
     suffix ordinary only after publication success
-> existing header-aware finalizer
-> return (unpublished MirFunction, owned Stage-B evidence)
-> existing generic payload pending session
```

Do not call `build_instance_method_draft_with_port_v1`; it would run the
ordinary whole body instead of the bounded schedule. Reuse:

```text
prepare_instance_method_draft_body_v1
run_function_body_step_tree_guard_v1
drive_preloop_stageb_body_schedule_v1
finalize_function_draft_with_headers
capture_legacy_function_payload_pending_session_v1
```

in that exact order. Extract one small shared prepared-completion helper for
the current-function return disposition if needed; do not copy its calculation
into a third owner.

The schedule state is monotonic:

```text
Armed
-> Prefix
-> SelectedInFlight
-> SelectedCompleted(owned F5 evidence)
-> Suffix
-> Completed
 | Rejected(strongest retained owner)
```

Failure retention:

| Failure | Retained authority | Later effects |
| --- | --- | --- |
| prefix | recipe + owned rebind | selected/F5/suffix = 0 |
| inner Call | exact selected source | outer/assignment/type/suffix = 0 |
| outer Call | successful inner physical evidence | assignment/type/suffix = 0 |
| assignment | complete outer carrier | type/suffix = 0 |
| type conflict | assignment + unchanged prior fact | suffix/finalizer = 0 |
| suffix | owned completed F5 evidence | publication = 0 |
| finalizer | owned completed F5 evidence | publication = 0 |
| cleanup after success | generic session payload | publication = 0 |
| cleanup during failure | typed primary Stage-B rejection | publication = 0 |

F6-4 P0 must split its evidence:

```text
local F6-2 actual Parser schedule:
  Body(4) suffix failure remains pinned
  full published carrier remains retained

Phase-A-indexed actual Parser session:
  user-defined-box facts come from the existing declaration indexer
  inner Call precedes outer Call
  outer destination == assignment carrier
  Integer is visible before the suffix frontier
  Body(4) clears without a new GenericLoop or suffix algorithm
  Body(5) / whole-body completion reaches the existing finalizer
  signature / params / uses / attrs remain exact
  completed draft retains the entire body-schedule payload
  draft remains unpublished
  parent function/block/variables/type facts/scope/recursion restore exactly
  child draft/module publication = 0

compact exact Stage-B source:
  exercises injected finalizer and cleanup failure branches
  pending owner keeps the parent captured until one consuming completion
  typed payload/primary rejection is retained without draft escape

same parent/module candidate Builder:
  typed failure -> exact restoration -> fresh one-shot success
```

Hard stop:

```text
the Phase-A-indexed actual Parser session still fails at Body(4)
```

If observed, retain the first typed rejection and open a new D0. A compact
fixture is not a substitute for actual Parser success. F6 must not seed a
test-only `ParserStringUtilsBox` value or weaken the pinned F6-2 failure.

Landed F6-3 I0 evidence:

```text
catalog-backed instance source projection             = 1
second AST walk / catalog reseal                       = 0

instance preparation                                   = existing 1
StepTree guard                                         = existing 1
F6-2 body schedule                                     = existing 1
shared finalizer preparation                           = 1
header-aware finalizer                                 = existing 1
generic payload-preserving function session            = existing 1

Phase-A-indexed actual Parser:
  Body(4) frontier clears
  complete unpublished draft                           = 1
  complete body-schedule/carrier payload                = 1
  parent captured until consuming completion            = 1
  parent restoration                                    = 1
  module publication                                    = 0

unannotated function signature                         = Unknown
outer carrier fact                                     = Integer
the two authorities are not conflated

activation-ledger consumer                             = 0
compile_request consumer                               = 0
retry / fallback / rearm                               = 0
```

The sole next row is
`PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION0-P0`. It closes the typed
preparation/StepTree/body/finalizer/cleanup failure matrix, exact metadata and
call-order parity, parent restoration, one-shot completion, and
failure-to-fresh-session reuse before F7 may consume the completed function.

F6 hard stop:

```text
the F5 owner cannot yield the existing owned rebind witness inside the HRTB
callback without a public owner escape or source/MIR re-observation
```

Only that contradiction opens
`PRELOOP-STAGEB-INSTANCE-FUNCTION-COMPLETION-EVIDENCE0-D0`.

## F7 — activation ledger

Rows:

```text
PRELOOP-STAGEB-FUNCTION-ACTIVATION-LEDGER0-P0
-> PRELOOP-STAGEB-FUNCTION-ACTIVATION-LEDGER0-G0
```

States:

```text
Armed
-> InFlight
-> Completed(exact function receipt)
 | Rejected(retained typed owner)
```

Required cardinality:

```text
selected caller consumption = exactly 1
missing caller               = typed reject
duplicate caller             = typed reject
```

Buildable cells:

```text
F7-1 INSTANCE-METHOD-CAPTURE-SEAM0-S0
  one behavior-neutral exact-function capture seam

F7-2 FUNCTION-ACTIVATION-LEDGER0-I0
  exact canonical caller key:
    Armed -> InFlight -> Completed(F6 receipt) | Rejected(F6 rejection)

F7-3 FUNCTION-ACTIVATION-LEDGER0-P0/G0
  exact-once / missing / duplicate / failure retention / reuse
```

The current ledger has only `Armed` and `SelectedCallerNotObserved`, and the
preinstalled root does not yet pass a capture consumer. F8 must remain
disconnected until F7 supplies one real completed-function payload.

```text
selected caller comparison = exact canonical key
selected consumer           = exactly 1
ordinary method delta       = 0
Builder source-site field   = 0
selected -> ordinary retry  = 0
```

## F8 — compile-request production ingress

Rows:

```text
PRELOOP-STAGEB-SOURCE-INVENTORY0-P0
-> PRELOOP-STAGEB-SOURCE-SELECTION0-S0
-> PRELOOP-STAGEB-MODULE-ACTIVATION0-S0
-> PRELOOP-STAGEB-COMPILE-REQUEST-INGRESS0-I0
-> PRELOOP-STAGEB-COMPILE-REQUEST-INGRESS0-P0
-> PRELOOP-STAGEB-COMPILE-REQUEST-INGRESS0-G0
```

The inventory, request, selection, and disconnected module-activation rows in
this list are already closed. They remain prerequisites and are not rebuilt.
The first new F8 behavior is the exact Legacy-arm consumer after F7.

Closed products that must be reused:

```text
CompilerSuppliedStaticImportSnapshotV1
LegacyWholeSourceCompileRequestV1
whole-source target inventory
Stage-B candidate inventory and 0 / 1 / many selection
PreparedSelectedPreloopStageBWholeSourceV1
PreparedPreloopStageBModuleActivationV1
atomic same-allocation catalog + alias install
preinstalled-root shell
```

The following rows remain designed but parked after the bounded source
inventory terminal:

```text
PRELOOP-STAGEB-UNAVAILABLE-DISPOSITION0-S0/P0
  retain the first bounded proof-unavailable stage in the existing inventory
  Zero -> NoExactCandidate | ExactCandidateProofUnavailable(stage)
  no second traversal or policy owner

PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION-CORRESPONDENCE0-P0
  inventory the exact Legacy config/commit matrix
  prove existing canonical and branded Raw sessions are not exact owners

PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-D0
  select one source-neutral isolated candidate kernel
  keep Raw/canonical/Legacy-selected authority in thin family wrappers
```

The correspondence audit already found that neither existing wrapper is an
exact owner:

```text
CanonicalModuleLoweringSessionV1:
  preserves quiet_internal_logs only

ModuleBuilderInvocationSessionV1:
  preserves the needed config
  but requires a Raw/Canonical family brand and token
```

The D0 must preserve:

```text
CoreContext continuation
repl_mode
quiet_internal_logs
plugin_method_sigs
source hint
typed alias installation only through the selected activation transaction
success-only live Builder replacement
all failure -> candidate drop
retry / fallback = 0
```

Do not connect Selected directly to the live compiler Builder or borrow the
Raw family brand for a Legacy source.

Sole production consumer:

```text
MirCompiler::compile_request
  / MirLoweringRequestV1::Legacy arm
```

Selection:

```text
0 exact rows -> explicit Ordinary
1 exact row  -> Selected
many         -> typed reject
```

The request owns the final AST plus compiler-supplied typed import snapshot.
Selection occurs before Builder mutation. Selected installs the same catalog
allocation once; Ordinary preserves the existing route. Selected failure never
retries as Ordinary.

Buildable cells:

```text
F8-1 LEGACY-WHOLE-SOURCE-REQUEST-PLUMBING0-S0
  route compile_with_source/imports into the existing owned typed request
  remove Builder alias mutation before selection

F8-2 LEGACY-ORDINARY-PARITY0-P0
  preserve Ordinary None / Explicit(empty) / Explicit(map)
  ProgramV0 / REPL / direct Builder callers unchanged

F8-3 connect Selected to the completed F7 ledger/session

F8-4 selected/ordinary/error/same-compiler-reuse P0

F8-5 sole-caller, no-direct-Builder, no-fallback G0
     + register the reused structural guard in docs/tools/check-scripts-index.md
```

Current code-facing delta:

```text
MirLoweringRequestV1::Legacy
  currently carries LegacyModuleLoweringInputV1 only

compile_legacy / compile_with_source_and_imports
  currently mutate Builder aliases before selection

MirCompiler::compile_request Legacy arm
  currently has zero PreloopStageBWholeSourceProducerV1 consumers
```

`ExactCandidateProofUnavailable` is not a new policy to invent during F8.
Reconcile its name with the existing bounded ordinary disposition and retain
the law:

```text
incomplete bounded proof -> explicit Ordinary
partial Selected attempt -> 0
```

Exact F8 frontier:

```text
src/mir/compiler/mod.rs
src/mir/compiler/lowering_input.rs
src/mir/compiler/legacy_whole_source_request.rs
src/mir/compiler/legacy_source_selection.rs
src/mir/compiler/legacy_module_activation.rs
src/mir/compiler/legacy_module_activation/{install,ledger}.rs
```

## F9 — real Stage-B proof and retirement census

Rows:

```text
CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0
-> PRELOOP-INNER-TYPE-PROOF-CENSUS0-P0
```

Run the real progression guard only after F8 is green. Do not change expected
output first.

Retire the old inner publisher only when:

```text
production consumers = 0
proof-only required consumers = 0
fallback / retry = 0
```

Then select the next frontier from the real guard:

```text
ownership grammar
parked loop-refresh activation
another missing representation
```

Alias/View language semantics are not selected by this series.

## Reconciled commit order

```text
1. F6 owned HRTB completion closure
2. F6 one bounded body schedule
3. F6 payload function session + actual Parser P0/G0
4. F7 behavior-neutral capture seam
5. F7 exact single-use ledger P0/G0
6. F8 typed Legacy request plumbing + Ordinary parity
7. F8 selected candidate session D0 closeout and isolated session
8. F8 compile_request sole consumer P0/G0
9. F9 unchanged-first real Stage-B guard + retirement census
```

No earlier source inventory, selector, catalog, alias snapshot, module install,
outer receipt, assignment, or type publisher is recreated.

## Verification ladder

```text
F6:
  cargo check -q --lib
  cargo test -q --lib preloop_stageb
  cargo test -q --lib preloop_outer_carrier
  cargo test -q --lib function_session

F7:
  cargo test -q --lib legacy_module_activation
  cargo test -q --lib preloop_stageb

F8:
  cargo test -q --lib source_call_target
  cargo test -q --lib preloop_stageb
  cargo test -q --lib module_lowering_invocation_reentrant
  cargo check -q --lib

G0 milestones:
  python3 tools/checks/lib/callable_result_i0_site0_r0_expr0_m0_v0.py
  bash tools/checks/current_state_pointer_guard.sh
  tools/checks/dev_gate.sh quick

F9 only:
  bash tools/checks/generic_loop_progression_role_v0_guard.sh
```

## Parked cleanup after F9

The following cleanliness items are real but must not be mixed into the
carrier activation commits:

```text
PRELOOP-INNER-TYPE-PROOF-RETIRE0-S0
PRELOOP-STAGEB-LEGACY-ALIAS-MUTATION-RETIRE0-S0
PRELOOP-STAGEB-SOURCE-PRODUCER-RETIRE0
```

Alias/View language semantics, loop-refresh activation, feature-flag cleanup,
legacy comment/history cleanup, and general call-result inference remain
separate workstreams selected only after the real F9 frontier is known.

## Structural gate

```text
physical Call writer                         = existing 1
outer final-destination authority            = 1
inner destination treated as outer           = 0
assignment-from-value authority              = existing 1
second assignment implementation             = 0
TypeFactDecisionV1 authority                 = existing 1
TypeContext::set_type authority              = existing 1
direct value_types insert                    = 0
GenericLoop publisher                        = 0
whole-source production selector             = exact 1 after F8
direct Builder / JSON / Raw selector caller  = 0
fallback / retry / route reselection         = 0
all modified/new source/check files          < 800 lines
```

## Design stop

Open a new D0 only if evidence proves one of:

```text
owned recipe cannot be paired with the F3 source owner
existing assignment authority changes the carrier destination
TypeFactDecisionV1 cannot express the required conflict law
compile_request Legacy is not the sole bounded producer
```

Unsupported language/backend features remain typed rejects and do not open a
new design consultation.
