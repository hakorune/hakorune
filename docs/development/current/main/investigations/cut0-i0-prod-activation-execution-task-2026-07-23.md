# CUT0-I0 Production Activation Execution Task

Status: **Design stop — RAW-SOURCE0-CONSULT0; SOURCE-FIRST-prime-r1 is locked; production ingress remains disconnected**
Date: 2026-07-23
Decision: **Candidate ACT-prime-r1 accepted**

Related decision:

- `docs/development/current/main/investigations/cut0-i0-prod-activation-consultation-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-root0-drain0-execution-task-2026-07-23.md`

## Scope and stop line

This card turns the accepted ACT-prime-r1 decision into small executable rows.
It does not permit partial production wiring. The root-retention design stop
is closed with OR-prime, but implementation is split into mutation-free
preflight, one-shot commit, and token handoff rows. Until the atomic CUT0 row,
production consumers of the new drained products, finalizer, postprocessor,
and external commit remain zero.

```text
FINAL0 -> POST0 -> COMMIT0 -> P0-R1 -> atomic CUT0/G0
```

Every row keeps source authority, physical evidence, Builder session, and the
same non-Clone invocation token together. No old Main-only adapter, bare
`MirModule`, bare `MirCompileResult`, retry, fallback, or `current_module`
re-observation is allowed.

## FINAL0 — route-specific finalization (closed)

Add compiler-private finalization products that consume the closed DRAIN0
products directly:

```rust
CanonicalDrainedInvocationV1::Single
  -> CanonicalSingleFinalizationInputV1

CanonicalDrainedInvocationV1::Callable
  -> CanonicalCallableFinalizationInputV1

RawDrainedInvocationV1
  -> RawFinalizationInputV1
```

`PreparedBuilderModuleSessionV1` is a consuming readiness product. It checks
the candidate Builder's current module/function/block, slot registry,
compilation context, recursion depth, and function-state closure before
finalization. It does not expose `builder_mut()` afterwards.

`CanonicalModuleFinalizerV1` may apply only retained module-level facts,
source-file metadata, capability evidence, and exact inventory checks. It must
not call `MirBuilder::finalize_module`, generate `main`/`condition_fn`,
re-resolve source/catalog data, or read `current_module` to recreate expected
inventory.

Raw finalization remains route-specific. Raw and canonical products do not
share the old `DrainedModuleCandidateV1` adapter.

Acceptance:

```text
A+ / trivial / acyclic / recursive finalization fixtures = green
Raw finalization fixture = disconnected only
source-file and declaration facts retained
recursive capability retained
canonical synthetic main/condition_fn = impossible/rejected
inventory drift = typed failure before mutation
Builder readiness failure = rejected owner, live Builder unchanged
finalizer production consumer = 0
all touched files < 800 lines
```

FINAL0 closeout (2026-07-23):

```text
PreparedBuilderModuleSessionV1 is a consuming readiness product with a
rejected owner for all seven Builder closure checks. The real drained
Single/Callable products now enter compiler-private
CanonicalFinalizationInputV1 variants through prepare_finalization(self).
CanonicalModuleFinalizerV1 performs route/family/brand validation without
legacy Main-only finalization, synthetic root creation, or source/current-module
re-observation. The disconnected Single and Callable fixtures, session
readiness matrix, FINAL0 census guard, cargo check, focused tests, PHYSICAL0
guard, and pointer guard are green. No production finalizer consumer exists.
```

FINAL0 is closed. POST0 and COMMIT0 are also closed as disconnected proofs;
P0-R1 is the only remaining executable row before atomic CUT0/G0.

## POST0 — one postprocess owner (closed; production disconnected)

Add compiler-private `ModulePostprocessOwnerV1` and derive its schedule only
from `ModuleInvocationFamilyV1`:

```text
refresh rune plans
-> optimizer
-> contract refresh/validation
-> pre-transform verifier
-> family-selected RC policy
-> semantic metadata refresh
-> callsite canonicalization
-> changed-only semantic refresh
-> canonical final verifier
```

The schedule is fixed:

```text
Raw                 RC Run,  pre-transform verifier Err reportable
CanonicalAPlus      RC Run,  final verifier required
BindingSsaTrivial   RC Skip, final verifier required
BindingSsaAcyclic   RC Skip, final verifier required
BindingSsaRecursive  RC Skip, final verifier required
```

The postprocessor consumes `FinalizedModuleInvocationV1` and produces a
non-Clone `PostprocessedModuleInvocationV1`. It never accepts caller-selected
RC/verifier/fallback policy and never re-reads source or `current_module`.

Acceptance:

```text
stage-order fixture = exact
legacy verifier Err remains reportable
canonical final verifier Err => commit 0
optimizer/contract/RC failures => commit 0
inventory evidence preserved
postprocessor production consumer = 0
```

### POST0-CANONICAL-S0 closeout (2026-07-23)

The disconnected family-owned schedule and compiler-private
`ModulePostprocessOwnerV1` are implemented for the canonical drained products.
The exact existing stage order is preserved: rune refresh, optimizer,
contract refresh, pre-transform verification, family RC policy, semantic
refresh, callsite canonicalization, changed-only refresh, and canonical final
verification. The schedule matrix and canonical success fixture are green;
the postprocess guard proves policy is family-derived and production
consumers remain zero.

The historical remaining POST0 boundary was **POST0-RAW-S0**. The Raw
finalization and postprocess slices below are now closed as disconnected
proofs; COMMIT0 is no longer blocked by a missing Raw evidence path.

## POST0-RAW-S0 — Raw finalization input (closed)

Add the Raw-side retained physical owner and route-specific finalization input.
It must preserve legacy reportable pre-transform verifier errors without
introducing the old Main-only `DrainedModuleCandidateV1` adapter. Keep the
new owner disconnected until its focused failure/success matrix and guard are
green.

POST0-RAW-S0 closeout (2026-07-23):

```text
RawCompleteInvocationV1 now has a consuming physical-owner bridge that keeps
the same Raw token, Builder session, branded empty shell, collector, sealed
ledger, and retained root witness together. Mutation-free preparation rejects
foreign identity, published shells, and non-exact Main/condition inventory;
success emits RawFinalizationInputV1 with the drained module and legacy
verifier/ledger evidence. Two focused fixtures, the Raw guard, cargo check,
and pointer guard are green. There is no production consumer.
```

The next boundary was **POST0-RAW-FINALIZER0**; that slice is now closed and
its result is consumed by the Raw postprocess proof below.

## POST0-RAW-FINALIZER0 — Raw compiler finalization (closed)

The compiler-side `RawModuleFinalizerV1` now consumes the retained Raw
physical input by value, checks the Raw family plus token/session/ledger/root
brand agreement, and consumes Builder readiness into a
`RawFinalizationInputV1`. The success product is sealed as
`RawFinalizedModuleInvocationV1`; readiness failures retain the unpublished
Raw owner and preserve the typed error. This boundary never calls the legacy
`MirBuilder::finalize_module`, never constructs `MirCompileResult`, and has no
production consumer.

Focused success and readiness-failure fixtures are green. The dedicated
`POST0-RAW-FINALIZER0` census guard and pointer guard are green.

The next boundary was **POST0-RAW-POSTPROCESS0**: its family schedule and
reportable verifier evidence are now closed as a disconnected proof.

## POST0-RAW-POSTPROCESS0 — Raw family postprocess (closed)

Extend the one compiler-private `ModulePostprocessOwnerV1` to consume the
sealed Raw finalization product. Raw derives the existing family schedule
(`RC Run`, `ReportPreTransformOnly`) and retains the pre-transform verifier
result as `ModuleVerificationEvidenceV1::Raw`, including an `Err` without
turning it into a fatal final-verifier failure. Canonical schedule and evidence
semantics must remain unchanged.

The row remains disconnected: no public ingress, external commit, retry, or
legacy finalization caller is allowed.

POST0-RAW-POSTPROCESS0 closeout (2026-07-23):

```text
ModulePostprocessOwnerV1 now consumes both canonical and Raw finalized
products through one owner. Raw derives its schedule from the sealed Raw
family, runs the existing RC-enabled stage order, and retains the exact
pre-transform verifier Result as ModuleVerificationEvidenceV1::Raw; a
verifier Err remains reportable rather than becoming a canonical final-barrier
failure. The Raw success/evidence fixture, canonical POST0 fixtures, four
POST0 guards, cargo check, and pointer guard are green. Production consumers
remain zero.
```

COMMIT0 is now closed as the disconnected paired Builder/module external-commit
proof. The next executable row is **P0-R1**; public ingress and atomic CUT0
remain disconnected.

## COMMIT0 — paired external commit (closed; production disconnected)

The compiler-private, non-Clone product is now implemented:

```rust
PreparedModuleExternalCommitV1 {
    token,
    builder: PreparedBuilderExternalCommitV1,
    postprocessed: PostprocessedModuleInvocationV1,
    verification: ModuleVerificationEvidenceV1,
}
```

Construction proves token/session/module brand and family equality, capability
agreement, postprocess completion, canonical final verification where needed,
inventory preservation, and Builder commit readiness.

Only `MirCompiler::commit_prepared_module(self, prepared)` may consume it.
That terminal is one-shot and infallible; it is the only production
`MirCompileResult` construction site after cutover.

Acceptance:

```text
foreign/missing evidence => preparation failure
unfinished Builder => preparation failure
failure/panic => live Builder unchanged, commit 0
success => external commit exactly 1
commit product consumer = 0
```

COMMIT0 closeout (2026-07-23):

```text
PreparedModuleExternalCommitV1 consumes the postprocessed invocation, retains
the PreparedBuilderExternalCommitV1 readiness owner, the postprocessed module,
and route-appropriate verification evidence, then commits exactly once into a
live Builder and constructs MirCompileResult at that terminal only. Token
brand/family must match Builder readiness and Raw/canonical evidence variants;
canonical evidence includes the final-verifier seal while Raw retains the
reportable pre-transform Result. The disconnected success fixture, COMMIT0
guard, cargo check, focused tests, and pointer guard are green. No public
ingress or production commit consumer is wired.
```

The next executable row is **P0-R1**: replace the synthetic all-route harness
with real source authority and exercise the complete disconnected chain.

## P0-R1 — real-authority all-route proof

Replace the disconnected all-route harness with the actual chain:

```text
sealed source
-> one outer executor
-> token/session/shell/collector
-> route lowering/collection/completion
-> route finalization
-> common postprocess
-> paired external commit
```

Cover Raw, A+, trivial, acyclic, and recursive routes, plus child, root,
batch, drain, finalizer, optimizer, verifier, capability, readiness, and
panic failures. Every failure proves publication and retry remain zero.

Production consumers stay zero while P0-R1 is disconnected.

P0-R1 success slice (2026-07-23):

```text
The canonical A+, trivial, acyclic, and recursive fixtures now consume the
real compiler-owned bridge through drain, route finalization, family-owned
postprocess, and the paired external-commit terminal. A Raw fixture consumes
the retained Raw root/physical owner through the same finalization,
postprocess, and paired commit boundary. The Raw fixture uses the existing
test-only Raw root issuer because no production Raw source-bound ingress exists
yet; this is evidence only and does not authorize production wiring.

The five success routes are green, and a real-authority Builder-readiness
failure now rejects before the external commit terminal. The P0-R1 guard
proves the complete disconnected chain with production consumers still at
zero. A published-shell drain failure and a foreign callable capability are
also rejected before commit. The remaining child/root/batch, typed
postprocess failures, panic, and other commit-zero outcome rows remain open
under P0-R1.

The first POST failure slice is now fixed: a real canonical trivial route is
mutated after finalization with a jump to a non-existent MIR block, and the
canonical final-verifier barrier rejects before any external commit. This
fixture is test-owned only; optimizer, contract/RC, pre-transform verifier,
child/root-batch, and panic rows remain open.

The existing Raw root-batch admission fixture is now counted in P0-R1: a
late duplicate `SyntheticConditionFn` admission is rejected by the whole-batch
preflight before collector/ledger root publication. This proves the typed
publication-zero boundary only; rejected-owner retention and child/panic
coverage remain separate rows.

The Raw side of POST is also fixed: an invalid CFG edge remains a successful
Raw postprocess result with `pre_transform` verification error evidence,
preserving the legacy reportable-verifier contract. Canonical final-verifier
failure remains fatal as recorded above.
```

The failure consultation is closed with Candidate F-prime-r1. The next row is
**P0-R1-CLOSEOUT0** and is guard/docs/evidence-only. It must freeze the
bounded publication-zero evidence and non-claims before the separate
`OWNER-RETENTION0` and `POST-FAILURE0` rows. The outer executor/CUT0
activation boundary remains disconnected.

### P0-R1 executable failure subrows

Implement and prove these in order. Each subrow remains disconnected and must
leave the live Builder unchanged on failure:

```text
P0-R1-CHILD
  real child primary, cleanup, admission, and primary+cleanup failures
  -> parent restore exactly once
  -> failed draft is not collected
  -> later sibling descent, root lowering, drain, retry, and commit = 0

P0-R1-ROOT-BATCH
  root completion failure and callable-batch late collision
  -> collector/ledger delta = 0 for the rejected batch
  -> no completion, drain, finalizer, or commit product

P0-R1-POST
  optimizer, contract/RC, pre-transform verifier, and canonical final
  verifier failure evidence
  -> Raw verifier Err remains reportable
  -> canonical final-verifier failure is fatal and unpublished

P0-R1-PANIC-COMMIT
  panic unwind and external-commit readiness failure
  -> candidate/session drops without live-Builder mutation
  -> external commit = 0, fallback = 0, retry = 0

P0-R1-G0
  measured caller census, focused fixture registration, line-limit check,
  pointer guard, cargo check, and row guard
  -> all production consumers remain zero
```

Do not add a verifier fixture that merely clears a function's block map: the
current verifier accepts that shape. A final-verifier failure fixture must use
a genuinely invalid MIR edge (for example, a jump to a non-existent block)
and must be kept separate from the already-green success slice.

## P0-R1-CLOSEOUT0 — bounded evidence freeze (closed)

This row is guard/docs/evidence-only. It does not add a fault field, malformed
plan route, outer `catch_unwind`, ambient failure switch, retry, fallback, or
production consumer.

Acceptance:

```text
five success routes rerun = green
readiness / published-shell / capability failures = green
canonical final-verifier failure = green
Raw reportable verifier error = green
Raw root-batch admission failure = green
production consumer census = 0
source-bound plan contract remains unchanged
non-claims are recorded in the consultation closeout
```

After this row, the remaining work is explicitly split:

```text
OWNER-RETENTION0
  rejected-owner products for root/finalizer/postprocess failures

POST-FAILURE0
  deterministic optimizer/contract/RC failure evidence

atomic CUT0/G0
  only after the above rows and the required caller census are green
```

P0-R1-CLOSEOUT0 closeout (2026-07-23):

```text
The five success routes and six bounded failure categories are covered by
focused tests and the P0-R1 guard. The guard reports production_consumers=0,
all touched files below 800 lines, and the source-bound plan contract remains
unchanged. This row claims publication-zero only for the observed fixtures;
it does not claim universal rejected-owner retention, real-authority child or
panic injection, or typed optimizer/contract/RC failure coverage.
OWNER-RETENTION0 and POST-FAILURE0 remain mandatory before activation.
```

## OWNER-RETENTION0 — rejected-owner products (OR-prime selected)

This is the next code-facing row. It must add no production consumer and must
not change the source-bound plan contract. Split the work into three narrow
terminals:

```text
OWNER-RETENTION0-ROOT
  complete_raw_root and root-batch rejection retain the unpublished
  collector/ledger/batch owner instead of returning only a bare error

OWNER-RETENTION0-FINAL
  canonical/raw finalizer validation and Builder-readiness failures retain
  the exact drained owner until explicit discard

OWNER-RETENTION0-POST
  optimizer/contract/RC/final-verifier failures retain the postprocess owner;
  no retry, replacement manifest, or recovery-to-complete terminal
```

### OWNER-RETENTION0-ROOT-CONSULT0 — closed (OR-prime selected)

The Raw root boundary is not a test-only omission. `complete_raw_root` still
accepts seven loose owned arguments, `prepare_root_batch(self, batch)` can
consume the prepared batch before a later ledger check, and a late ledger
failure is mapped to a bare error. Therefore the next implementation must not
extend only `RejectedRootCollectorBatchV1`; that would still drop part of the
unpublished owner.

Candidate OR-prime is selected:

```text
RawRootCompletionInputV1
  owns token / branded collector / ledger / prepared root batch /
       reservations / callable-main disposition

-> prepare(self)
     read-only family, brand, reservation, collector, and ledger checks

success -> PreparedRawRootCompletionV1
failure -> RejectedRawRootCompletionV1 { owner, typed error }

-> commit(self)
     one infallible collector + ledger + root-witness publication
```

The consultation closed Q1–Q4:

```text
Q1  one private root input owner, token/family sealed once
Q2  collector borrowed validation before consuming batch
Q3  ledger borrowed validation before one commit terminal
Q4  PREFLIGHT -> COMMIT -> TOKEN-HANDOFF refactor series
```

The first executable row is
`ROOT-RETENTION0-PREFLIGHT`. It must retain the full input owner on every
preflight rejection, keep collector/ledger/root-body state unchanged, and
leave production consumers at zero. `ROOT-RETENTION0-COMMIT` and the later
token handoff are separate rows. Root-body completion and reservation
allocation retention are explicitly not claimed by this consultation.

No canonical source changes, production Raw ingress, physical drain,
finalizer/postprocess wiring, atomic CUT0, retry, fallback, or recovery
terminal may be added by this row.

Acceptance for each subrow:

```text
typed rejected owner contains the unpublished chain
shell/collector/live Builder mutation before rejection = 0
later sibling, drain, finalizer, postprocess, and external commit = 0
retry/fallback/re-entry terminal = 0
focused failure fixture + measured guard = green
production consumer = 0
all touched source/check files < 800 lines
```

Do not widen a generic `Rejected<Owner, Error>` API across unrelated layers;
each terminal must retain only the route-specific owner it already controls.

ROOT-RETENTION0-PREFLIGHT closeout (2026-07-23):

```text
RawRootCompletionInputV1 now owns the token, branded collector, mutable
ledger, prepared root batch, both reservations, and callable-main
disposition. Borrowed collector and ledger validation runs before any
consuming terminal. Four focused fixtures cover success, foreign collector,
foreign reservation, and duplicate collector admission; every rejection
retains the full owner and leaves collector/ledger state unchanged.

The dedicated ROOT-RETENTION0-PREFLIGHT guard, focused test, cargo check, and
diff check are green. Production root completion, physical binding, finalizer,
postprocess, and external commit consumers remain zero. ROOT-RETENTION0-COMMIT
is the next executable row; TOKEN-HANDOFF remains separate.
```

ROOT-RETENTION0-COMMIT closeout (2026-07-23):

```text
PreparedRawRootCompletionV1::commit(self) is now the only consuming mutation
terminal. It moves the collector-issued Main/condition receipts, root-body
witness, and sealed ledger into RawCompleteInvocationV1; all remaining error
branches are invariant failures after the borrowed preflight, not semantic
rejection paths. The focused commit fixture proves one complete root pair and
the guard verifies the single commit terminal and zero production consumers.

TOKEN-HANDOFF remains separate: RawCompleteInvocationV1 still exposes the
existing brand-only physical bridge until the next row removes loose token
re-entry. Production Raw ingress, finalizer, postprocess, and external commit
remain disconnected.
```

ROOT-RETENTION0-TOKEN-HANDOFF closeout (2026-07-23):

```text
RawCompleteInvocationV1 now owns the original non-Clone
ModuleInvocationTokenV1 by value; brand is derived from that token and no
longer acts as a replacement authority. `into_parts` and committed-root
construction move the token through the complete product. Physical binding
consumes the complete product with `bind_physical(self, session, shell)` and
has no loose token parameter, so a second token cannot be re-entered at the
handoff boundary.

The focused raw completion, physical finalization, preflight guard, RAW0
guard, cargo check, and diff check are green. Production Raw ingress,
finalizer, postprocess, and external commit remain disconnected. The next
row is OWNER-RETENTION0-POST; no canonical or atomic CUT0 wiring is opened.
```

OWNER-RETENTION0-FINAL progress (2026-07-23):

```text
CanonicalModuleFinalizerV1 now returns RejectedCanonicalFinalizerV1 with the
complete route-specific finalization input and typed validation error. A
foreign physical brand fixture proves the retained input and unchanged live
Builder. ROOT and POST retention remain open.
```

### OWNER-RETENTION0-POST-CONSULT0 — closed (Candidate PR-prime)

The postprocess boundary is now locked:

```text
Q1  fatal stages return RejectedModulePostprocessV1 with the exact input
Q2  in-place unpublished mutation is retained and discard-only
Q3  existing optimizer/contract/canonical-final-verifier failures only;
    Raw pre-transform verifier Err remains reportable; RC failure is deferred
Q4  paired commit preparation must consume route evidence into a sealed
    PostprocessEvidenceSealV1
```

The next executable row is `OWNER-RETENTION0-POST-P0`. It adds rejected-owner
retention and focused natural-failure fixtures only. Evidence sealing is a
separate follow-up row; production postprocess, external commit, public
ingress, retry, and atomic CUT0 remain disconnected.

### OWNER-RETENTION0-POST-P0 closeout (2026-07-23)

`ModulePostprocessOwnerV1::run` and `run_raw` now return
`RejectedModulePostprocessV1` for every existing fatal stage. The rejected
product retains the exact current `ModulePostprocessInputV1`, family-derived
schedule, failure stage, and typed/opaque stage error. Its only terminals are
error/stage inspection and discard; retry, resume, replacement manifest, and
fallback are absent.

The postprocess module is allowed to contain the successful in-place prefix at
failure time, but it is still unpublished and the live Builder remains
unchanged. Raw pre-transform verifier errors remain reportable evidence, while
canonical final-verifier errors are rejected. The focused rejection fixture,
P0-R1 compatibility fixture, POST0 guard, cargo check, and diff check are
green. RC-failure injection and commit evidence sealing remain separate
non-claims. The next row is `OWNER-RETENTION0-POST-EVIDENCE0`.

### OWNER-RETENTION0-POST-EVIDENCE0 closeout (2026-07-23)

`PostprocessedModuleInvocationV1::into_external_commit_parts` no longer
drops route evidence. It produces a route-specific evidence input, and
`PreparedModuleExternalCommitV1::prepare` consumes that input exactly once
into `PostprocessEvidenceSealV1` after token/family/brand checks.

The seal retains canonical continuation plus receipt/inventory, callable
capability plus receipt/inventory, or Raw ledger/root evidence until the
one-shot commit consumes the prepared product. No source/catalog/current
module re-observation or bare evidence drop was added. The COMMIT0 fixture,
COMMIT0/POST0 guards, cargo check, focused tests, and diff check are green.
Production postprocess and external commit consumers remain zero. The next
row is `POST-FAILURE0`; RC fault injection and universal optimizer/contract
failure coverage remain explicitly deferred.

## POST-FAILURE0-CONSULT0 — design stop (2026-07-23)

The next row cannot be implemented safely by adding a generic fault field or
ambient failure switch. Existing canonical final-verifier failure and Raw
reportable verifier evidence are already covered; optimizer diagnostics depend
on existing diagnostic policy, contract refresh needs a naturally invalid
module-level fact, and RC insertion remains infallible.

The dedicated consultation card
`cut0-i0-prod-activation-post-failure-consultation-2026-07-23.md` is closed
with NF-prime-r1. The natural-failure row below is the only permitted next
implementation; production postprocess, outer executor, retry, fallback, RC
fallibility refactoring, and atomic CUT0 remain disconnected until it closes.

## POST-FAILURE0-NATURAL-P0 — natural failure matrix

Candidate NF-prime-r1 is selected. This row uses no new fault authority:

```text
optimizer:
  real canonical trivial route
  + one existing unlowered type-op Call
  + test-scoped NYASH_OPT_DIAG_FAIL policy
  -> RejectedModulePostprocessV1::Optimizer

contract:
  real canonical trivial route
  + existing StaticDataPlan without StaticTableContractSpec
  -> RejectedModulePostprocessV1::ContractRefresh
```

The optimizer policy scope saves/restores exact process environment values and
serializes the fixture. RC insertion, semantic refresh, callsite
canonicalization, and real-route panic remain explicit non-claims because the
current operations are infallible or have no sanctioned injection terminal.
No production postprocess, external-commit, outer-executor, retry, fallback,
or public-ingress consumer is permitted.

Acceptance:

```text
optimizer natural rejection = green
Static Table orphan-plan rejection = green
existing canonical final-verifier fixture = green
Raw reportable verifier fixture = green
RejectedModulePostprocessV1 is discard-only
external commit = 0
live Builder mutation = 0
all touched source/check files < 800 lines
```

The dedicated guard is
`tools/checks/lib/cut0_i0_prod_activation_post_failure0_guard.py`. After this
row closes, the next boundary is atomic CUT0/G0; production consumers remain
zero until that single activation patch.

### POST-FAILURE0-NATURAL-P0 closeout (2026-07-23)

NF-prime-r1 is closed as a disconnected natural-failure proof. The real
canonical trivial route rejects one existing optimizer diagnostic shape under
the serialized existing policy scope, and the same route rejects an orphan
Static Table plan during contract refresh. Both failures retain the current
unpublished input in `RejectedModulePostprocessV1` and expose discard only.

The dedicated natural-failure guard, focused two-fixture test, existing POST0
and P0-R1 fixtures, cargo check, diff check, and pointer guard are green. RC,
refresh, and real-route panic failure semantics remain explicit non-claims.
The next row is the one-shot atomic `CUT0-I0-ATOMIC-CUT0/G0` activation.

## Atomic CUT0/G0

In one activation patch, route all public canonical and legacy ingress wrappers
through `MirCompiler::execute_preflighted_module_invocation`. The same patch
must make these non-test production caller counts zero:

```text
CanonicalModuleLoweringSessionV1 open/commit
MirBuilder::build_module
build_resolved_*_function_module
build_*_callable_module_candidate
callable publish_into(&mut MirModule)
MirBuilder::finalize_module direct publication
old DrainedModuleCandidateV1/finalize_drained_module_once
```

The census must also cover host-provider direct Builder bridges and live
config pre-writes (`imports`, source-file hint, REPL/log settings). Staged
disconnected proofs are allowed; staged production wiring is not.

## CUT0-I0-ATOMIC-CUTOVER-CONSULT0 — design stop (2026-07-23)

The worker census found that the accepted ACT-prime shape is not yet a wiring
slice. No `execute_preflighted_module_invocation` exists; canonical public
methods still open `CanonicalModuleLoweringSessionV1` and use the old module
builders. Legacy lowering still mutates live Builder configuration before
calling `MirBuilder::build_module`. `runtime/mirbuilder_emit.rs` has a real
production AST-JSON direct Builder path.

Most importantly, the Raw chain has no production source-bound issuer. The
existing `VerifiedRawRootExpansionV1::from_program` and Raw root/ledger owner
are reached through the legacy Builder path or test-only factories. Therefore
an all-five-route atomic CUT0 claim would be false without a new Raw source
binding row.

The dedicated consultation card
`cut0-i0-atomic-cutover-consultation-2026-07-23.md` asks whether to keep all
production consumers disconnected, how Raw obtains compiler-owned identity,
whether runtime AST-JSON is in CUT0 scope, and where configuration is sealed.
Until Q1–Q5 close, no outer executor or public ingress is wired.

## SOURCE-FIRST-prime-r1 closeout and next row

The Atomic CUT0 consultation is closed with Candidate SOURCE-FIRST-prime-r1.
The current boundary remains disconnected: Q1=3, Q2=1, Q3 split between AST
JSON parity and the existing Program(JSON v0) compatibility lane, Q4=1, and
Q5=1. The only future production executor is an all-five-family atomic
cutover; canonical partial activation is not allowed.

The next design row is `RAW-SOURCE0-CONSULT0`. It owns only the Builder-side
Raw source authority:

```text
LegacyModuleLoweringInputV1
-> RawIngressRequestV1
-> source-only preflight
-> SourceBoundRawPackageV1
-> existing Raw session/collector/ledger/root chain
```

This row must not add a public executor, change runtime JSON behavior, or
retire `MirBuilder::build_module`. Program(JSON v0) remains a later,
independent `PROGRAM-V0-SOURCE0` design row.

## Required evidence per row

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
python3 tools/checks/<row-specific-guard>.py
RUSTFLAGS='-Awarnings' cargo test -q <row-specific-tests> --lib
```

All new or touched source/check files must remain below 800 lines. Update
`CURRENT_STATE.toml` and this card only when a row closes; keep the pointer on
the current row and do not claim CUT0 activation before atomic CUT0/G0.
