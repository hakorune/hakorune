# CUT0-I0 Production Activation Execution Task

Status: **Active — POST0 is the only executable row; FINAL0 closed**
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

Next executable row: **POST0**. COMMIT0, P0-R1, and atomic CUT0/G0 remain
disconnected and forbidden.

## POST0 — one postprocess owner (next)

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

The remaining POST0 boundary is **POST0-RAW-S0**: Raw needs a route-specific
finalization input that retains its Builder session/module owner and carries
reportable pre-transform verifier evidence. The existing
`RawCompleteInvocationV1` currently retains collector/ledger/root evidence but
not that physical module/session pair, so COMMIT0 remains blocked by design.

## POST0-RAW-S0 — Raw finalization input (next)

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

The next boundary is **POST0-RAW-FINALIZER0**: consume the retained Raw
finalization input into a route-specific finalized product and carry the
reportable pre-transform verifier result into POST0's evidence type.

## POST0-RAW-FINALIZER0 — Raw compiler finalization (next)

Add the compiler-side Raw finalization wrapper and its disconnected fixture.
Do not connect Raw or canonical postprocess to public ingress yet.

## COMMIT0 — paired external commit

Add compiler-private, non-Clone:

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
