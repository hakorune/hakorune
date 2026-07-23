# CUT0-I0 Production Activation Execution Task

Status: **Active — P0-R1 is the only executable row; FINAL0, POST0, and COMMIT0 closed as disconnected proofs**
Date: 2026-07-23
Decision: **Candidate ACT-prime-r1 accepted**

Related decision:

- `docs/development/current/main/investigations/cut0-i0-prod-activation-consultation-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-root0-drain0-execution-task-2026-07-23.md`

## Scope and stop line

This card turns the accepted ACT-prime-r1 decision into small executable rows.
It does not permit partial production wiring. Until the atomic CUT0 row,
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
```

Next P0-R1 slice: add the real-authority failure matrix and keep the outer
executor/CUT0 activation boundary disconnected.

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
