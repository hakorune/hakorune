# PRELOOP-STAGEB-CARRIER F5-F9 execution task

Status: active execution addendum

Parent:

```text
preloop-stageb-instance-function-session-reconciliation0-prime-r1
```

Current row:

```text
PRELOOP-OUTER-CARRIER-RECEIPT0-I0
```

## Closed prerequisites

```text
F1 exact declaration/body recipe retention
F2 generic payload-preserving pending function session
F3 exact located outer request through the existing static handler
F4 source-neutral static/global physical Call receipt sibling
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
