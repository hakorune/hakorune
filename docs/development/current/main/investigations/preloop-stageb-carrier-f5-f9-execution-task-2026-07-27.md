# PRELOOP-STAGEB-CARRIER F5-F9 execution task

Status: active execution addendum

Parent:

```text
preloop-stageb-instance-function-session-reconciliation0-prime-r1
```

Current row:

```text
PRELOOP-STAGEB-INSTANCE-FUNCTION-SESSION0-I0
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

## Execution law

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
F6-1 selected statement transaction
F6-2 prefix/selected/suffix schedule connection
F6-3 payload function-session completion and failure retention
F6-4 actual Parser parity and G0
```

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
F7-1 produce InFlight from Armed only
F7-2 retain Completed | Rejected payloads
F7-3 missing/double/reuse matrix and G0
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

Before that consumer is added, close two bounded correspondence rows:

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
F8-1 route compile_with_source/imports into the owned typed request
F8-2 preserve Ordinary None / Explicit(empty) / Explicit(map) import parity
F8-3 connect Selected to the completed F7 ledger/session
F8-4 selected/ordinary/error/reuse P0
F8-5 sole-caller, no-direct-Builder, no-fallback G0
     + register the reused structural guard in docs/tools/check-scripts-index.md
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
