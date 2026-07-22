# HDR0-P0 Execution Task

Status: **Active — CUT0-P0 closed; CUT0-I0 preflight next**
Date: 2026-07-22
Scope: complete HeaderPort reader replacement and prepare one atomic all-route CUT0

Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/mirbuilder-headerport-i0-hdr0-p0-open-questions-2026-07-22.md`
- `docs/development/current/main/investigations/mirbuilder-headerport-i0-production-cutover-consultation-2026-07-21.md`
- `docs/development/current/main/investigations/cut0-i0-production-transaction-consultation-2026-07-22.md`
- `tools/checks/lib/headerport_header_reader_census.py`
- `tools/checks/lib/headerport_authority_erasure_guard.py`
- `tools/checks/lib/headerport_method_tail_compat_guard.py`
- `tools/checks/lib/cut0_s0_guard.py`

## Accepted decisions

| Question | Decision | Durable law |
|---|---|---|
| Q1 method-index freshness | Pure projection | An explicit header loan projects sorted candidates directly and never reads or updates the ambient legacy cache. |
| Q2 static tail resolver | Thread one short HeaderPort loan | Lower arguments first, then retain one explicit authority through resolve, emit, and annotation. |
| Q3 materializer presence | Explicit split | Invocation header presence and legacy compatibility presence are mutually exclusive modes. |
| Q4 lifecycle activation | Atomic all-route CUT0 | A sealed adapter may be built disconnected, but production activation occurs once at the outer module entry for every route. |

These decisions add no production capture, drain, finalizer, or external-commit
consumer by themselves.

## Central invariant

Once a call-lowering route selects an explicit HeaderPort authority, that
authority must remain present through selection, emission, and annotation.
The route must not erase it by entering a lookup-less legacy facade.

```text
lower arguments completely
-> collect nested declarations
-> borrow one short HeaderPort
-> resolve
-> emit with the same lookup authority
-> annotate with the same lookup authority
-> end loan
```

The explicit route must not reach:

```text
emit_legacy_call
lookup-less emit_unified_call
annotate_call_result_from_func_name
legacy method_candidates
current_module function-map readers
```

## Implementation queue

### HDR0-P0-AUTHORITY-ERASURE0 — closed

Close exactly the three known authority-loss sites:

1. explicit tail resolution selects and emits through the same lookup-aware
   route;
2. unique static recovery emits through the same lookup-aware route;
3. canonical materializer recovery annotates through the selected presence
   authority.

Use an exclusive presence type or two named public facades with one private
exclusive core. The current `Option<&lookup> + legacy_presence: bool` policy
surface must not remain able to express contradictory modes.

Acceptance:

- unique, ambiguous, and missing tail fixtures under the existing environment
  gate;
- explicit misses do not read the ambient module and do not change
  instruction or ValueId state;
- unique static recovery and materializer annotation preserve lookup
  authority to completion;
- a static guard rejects any explicit-authority path that reaches the five
  forbidden legacy reader/facade families above;
- `python3 tools/checks/lib/headerport_authority_erasure_guard.py` is green;
- production invocation capture/commit and CUT0 consumers remain zero.

This is a BoxShape slice. It must not add a new accepted call shape, resolver,
cache, environment variable, or production route.

### HDR0-P0-METHODTAIL-COMPAT0 — closed

Make the selected Q1 boundary executable:

- `Some(headers)` always uses deterministic pure projection and bypasses the
  legacy cache;
- `rebuild_method_tail_index_with_headers` remains disconnected or is removed;
- `prepare_module` clears the legacy cache and resets its source-length
  witness;
- legacy rebuild sorts source symbols and every candidate list.

Guard: `python3 tools/checks/lib/headerport_method_tail_compat_guard.py`.

Acceptance fixtures:

- same-size collector A to collector B exposes only B candidates;
- different insertion order produces the same projection order;
- fresh legacy snapshot and explicit headers agree for unique, ambiguous, and
  missing results.

### HDR0-P0-CALLER-CENSUS0 — closed

Re-run the full reader/caller census after the two code slices. Every
materializer mode and method-tail caller must have one named owner. Located
legacy observation remains diagnostic/disconnected unless new evidence
selects a separate task.

### HDR0-G0 — closed

Close only when:

```text
unclassified production function-map readers = 0
explicit HeaderPort -> legacy authority erasure = 0
second invocation header cache = 0
production capture/commit consumers = 0
```

## Atomic lifecycle queue

The production policy is fixed now, but CUT0 is not yet executable. After
HDR0-G0 and the accepted compatibility-policy closeout, use the following
queue.

### CUT0-S0 — disconnected linear owner

Build one sealed, production-disconnected ownership chain:

```text
ModuleLoweringInvocationCandidateV1
-> active lowering
-> MainPendingInvocationV1
-> MainCapturedInvocationV1
-> CompleteInvocationV1
-> PreparedInvocationDrainV1
-> DrainedModuleCandidateV1
-> DrainedModuleFinalizationInputV1
-> FinalizedModuleCandidateV1
-> external commit capability
```

Required structural closures:

- add a successful candidate handoff;
- make root completion a real typed transition;
- drain the exact same invocation state without reconstructing
  `shell + collector`;
- implement `finalize_drained_module_once` as Builder-free, collector-free,
  HeaderPort-free, and retry-free;
- do not pass a bare `MirModule` across the post-drain boundary.

All production invocation capture, drain, finalizer, and external-commit
consumers remain zero through CUT0-S0.

### CUT0-P0 — disconnected all-route proof

All nine route rows must execute the same sealed outer adapter. Prove success,
primary error, cleanup error, admission error, root error, drain error,
finalizer error, and panic. A passive row declaration alone is insufficient.

### CUT0-I0 — one production switch

Replace the outermost module entry once for raw, A+/trivial,
acyclic/recursive, Main/condition_fn, drain, finalization, and external commit.
Route-specific activation flags, partial production wiring, fallback, and
retry are forbidden.

Acceptance:

```text
outer invocation owner = 1
collector              = 1
drain consumer         = 1
finalizer consumer     = 1
external commit        = 1

direct pre-drain insertion       = 0
restore-then-publish             = 0
explicit header -> legacy retry  = 0
post-drain current_module read   = 0
failure retry                    = 0
```

#### CUT0-I0 preflight boundary

Before changing any production caller, freeze the owner census below as a
design boundary. The disconnected adapter and its guards are not evidence
that these owners can already be merged:

```text
raw module entry:
  MirBuilder::build_module -> live builder/current_module lifecycle

canonical resolved entries:
  CanonicalModuleLoweringSessionV1 -> isolated candidate commit

post-build publication:
  finish_built_module -> verifier/optimizer/final publication

disconnected CUT0 owner:
  ModuleLoweringInvocationCandidateV1 -> route-owned shell/collector
```

`CUT0-I0` must first select one common external-commit capability and one
production ingress owner that can consume both raw and canonical products.
Until that boundary is specified and guarded, the following remain forbidden:

- wiring only the raw `build_module` route;
- wiring only a canonical resolved route;
- adapting one route by rebuilding `shell + collector`;
- treating `CanonicalModuleLoweringSessionV1` as proof of the new raw
  capture/drain lifecycle;
- adding a route-specific activation flag, fallback, or retry.

The preflight task is read-only census plus a design decision. Its completion
condition is an explicit common owner/commit contract; otherwise the lane
stays at `CUT0-I0` design-stop with production capture and commit consumers at
zero.

The current census leaves four decisions open and therefore blocks production
wiring:

1. `MirCompiler::compile_legacy` still lowers through a live `MirBuilder`,
   while A+/trivial/acyclic/recursive entries use separate
   `CanonicalModuleLoweringSessionV1` candidates. These cannot be joined by a
   route-local adapter.
2. Function-draft publication and root `Main`/`condition_fn` insertion still
   write directly to `current_module`; both must move behind one collector
   admission and one external commit capability.
3. The route matrix marks canonical `condition_fn` as forbidden, while the
   shared finalizer currently synthesizes it unconditionally. The policy must
   be made explicit before a common transaction is selected.
4. The post-build verifier/optimizer/canonicalize sequence has no typed
   drained-candidate boundary, and persistent compiler settings cannot be
   copied by blindly cloning `CompilationContext`. State transfer must be an
   explicit ingress contract.

Until these four points have a single owner and guard, `CUT0-I0` is a design
consultation, not an implementation invitation.

#### CUT0-I0-CONSULT0 — unresolved common-owner contract

The production census adds six questions that must be answered on one SSOT
page before an API or route is changed:

1. **Ingress token:** what typed outer-ingress enum/token covers raw,
   A+/trivial, acyclic, recursive, and the nine route rows without a
   route-local activation flag?
2. **Receipt conversion:** how do `RawReceipt`, canonical single-function
   drafts, and callable batches become collector-owned facts without allowing
   a caller-supplied symbol inventory or a second declaration authority?
3. **Builder boundary:** how are persistent compiler settings transferred
   across the invocation boundary? `ModuleLoweringInvocationV1` borrows
   `&mut MirBuilder`; storing it in `MirBuilder` or `CompilationContext`, or
   merely wrapping `build_module`, is not a valid solution.
4. **Root policy:** is `condition_fn` required, forbidden, or explicitly
   route-dependent? The route matrix forbids it for canonical routes while
   the shared finalizer currently synthesizes it unconditionally.
5. **Postprocess/commit:** does optimizer/verifier/canonicalize belong before
   drain, in a typed post-drain finalizer, or behind a separate capability?
   The phrase `external commit = 1` must name one concrete publication
   boundary; a returned `MirCompileResult` is not automatically that proof.
6. **Poison/error state:** after primary, cleanup, admission, panic, or
   postprocess failure, what exact candidate/session state is consumed and
   what state is allowed to be retried? The answer must preserve the existing
   no-retry and primary-plus-cleanup laws.

Consultation close condition:

```text
one SSOT decision for all six questions
-> one route-neutral, disconnected API-only slice
-> fixtures/guard for the ownership and publication laws
-> only then reconsider production CUT0-I0
```

Until this closes, production capture, drain, finalizer, and external-commit
consumers remain zero. The current next action is consultation, not code.

### CUT0-G0 — retire old owners

Remove old closure terminals, restore-then-publish, direct module insertion,
direct callable publication, and current-module header fallback after their
consumer counts are zero.

## Existing consultation — closed

`CUT0-COMPAT-POLICY-CONSULT0` is closed with Candidate S-prime before
`CUT0-S0`. Duplicate Main source behavior and selected callable-Main failure
propagation are now explicit; Q4 remains the activation-topology decision.

## Non-claims

This card does not claim:

- that CUT0 gates are currently met;
- production HeaderPort capture/commit or lifecycle activation;
- the nine-route all-route proof required by CUT0-P0;
- retirement of all `current_module` readers;
- FACTSESSION0, finalization-repair removal, FastMem, LLVM, JoinIR retirement,
  or selfhost migration progress.

After CUT0-G0 the fixed queue resumes at FACTSESSION0. FastMem remains parked
until `MODULE-FINALIZE-VERIFY-CUT0` as recorded by the current state.

## HDR0-P0-AUTHORITY-ERASURE0 closeout

The first code-facing slice is closed on 2026-07-22. The explicit call route
now borrows headers only after argument descent, passes the same lookup through
unique static and deterministic tail recovery, keeps materializer presence in
an exclusive invocation/legacy enum, and carries the lookup into the
post-success annotation receipt. The legacy port still returns `None` and
retains its compatibility annotation path.

Evidence:

```text
python3 tools/checks/lib/headerport_authority_erasure_guard.py = green
python3 tools/checks/lib/headerport_candidate0_guard.py . = green
cargo check -q = green
cargo test -q explicit_header_authority_survives_unified_call_post_success --lib = green
cargo test -q explicit_projection_is_sorted_and_ignores_non_methods --lib = green
cargo test -q raw_invocation --lib = green (16 tests)
```

Production invocation capture/commit, lifecycle drain, external commit, and
CUT0 remain zero. The next code-facing row is
`HDR0-P0-METHODTAIL-COMPAT0`.

## HDR0-P0-METHODTAIL-COMPAT0 closeout

The selected Q1 compatibility slice is closed on 2026-07-22.
`prepare_module` now clears both the legacy method-tail cache and its
freshness witness; legacy rebuilds sort source symbols and every candidate
list; and the unused explicit-header cache writer is removed. Explicit
invocation routes continue to use pure projection and never update the
ambient cache.

Evidence:

```text
python3 tools/checks/lib/headerport_method_tail_compat_guard.py = green
python3 tools/checks/lib/headerport_candidate0_guard.py . = green
cargo check -q = green
cargo test -q method_tail_index --lib = green (3 tests)
```

The next code-facing row is `HDR0-P0-CALLER-CENSUS0`; production capture,
drain, finalizer, external commit, and CUT0 remain zero.

## HDR0-P0-CALLER-CENSUS0 closeout

The post-slice caller census is closed on 2026-07-22. Explicit projection
callers are limited to rewrite/header lookup, unified method observation, and
static recovery. Legacy `method_candidates` remains behind three named
compatibility consumers. Materializer presence has one legacy wrapper and one
exclusive authority core. No unclassified HeaderPort caller, new
`current_module` fallback, retry, or production capture/commit consumer was
found.

Evidence:

```text
python3 tools/checks/lib/headerport_header_reader_census.py = green (20 rows)
python3 tools/checks/lib/headerport_authority_erasure_guard.py = green
python3 tools/checks/lib/headerport_method_tail_compat_guard.py = green
python3 tools/checks/lib/headerport_candidate0_guard.py . = green
```

`HDR0-G0` is the final verification row for this lane and remains closed
below. The compatibility-policy consultation is recorded and closed below;
this card now owns the bounded CUT0-S0 implementation queue.

## CUT0-COMPAT-POLICY-CONSULT0-DESIGN-STOP (closed)

HDR0-G0 is closed. This section records the resolved design stop; the
Candidate S-prime decision and its implementation queue follow below.

Source authority:

```text
source-level Main/condition_fn compatibility behavior
VerifiedMainExpansionV1 and raw expansion receipt policy
existing language/reference semantics once explicitly decided
```

Non-authority:

```text
HeaderPort presence or method-tail freshness
current_module as an invocation fallback
route names, environment toggles, or a passing VM example
passive route matrix rows without semantic policy evidence
```

Fail-fast boundary:

```text
duplicate Main source boxes => no silent winner
Selected optional Main.main/N with lowering failure => no discarded error
any unresolved policy => CUT0-S0/I0 remains disconnected
partial route activation, fallback, and retry => forbidden
```

Decision axes for the consultation:

1. Choose whether duplicate Main source boxes are rejected, or whether one
   explicit source rule preserves a distinct compatibility identity.
2. Choose whether a selected optional `Main.main/N` lowering failure is
   propagated as a typed error or has an explicitly documented compatibility
   disposition.
3. Define the exact fixtures, error vocabulary, and CUT0 acceptance gate for
   both choices.

Resolved handoff:

```text
Candidate S-prime decision record
-> duplicate-Main and optional-callable failure fixtures
-> explicit acceptance/rejection policy
-> CUT0-S0 disconnected linear-owner implementation
```

## CUT0-COMPAT-POLICY-CONSULT0 closeout

Candidate S-prime is selected.

Duplicate Main policy:

- More than one top-level static `Main` declaration is rejected
  deterministically with `MainExpansionErrorV1::DuplicateMainBox`.
- No source-order winner, merge, legacy replacement, or synthetic owner
  identity is permitted.
- Rejection occurs before Builder effects, invocation candidate creation,
  receipt reservation, or collector mutation.
- General duplicate owner collisions involving non-static `Main` remain
  owned by the callable declaration catalog (`DuplicateBoxOwner`).

Callable Main compatibility policy:

- `NotSelected` performs no reservation and no lowering.
- `Selected` makes the `Main.main/N` compatibility draft mandatory.
- Session, cleanup, admission, or panic failure aborts the unpublished
  invocation and preserves the original typed failure.
- A selected failure is never downgraded to `NotSelected`, replaced with
  `MissingCallableMainCompatibility`, retried, or followed by inline-root
  lowering.
- `MissingCallableMainCompatibility` remains an adapter invariant for a
  missing receipt, not a recovery result for a failed selected child.

Compatibility selection is sealed once at the legacy module ingress. Builder
body lowering and child terminals do not read an ambient environment toggle.

## CUT0-S0 implementation task

Implement the policy as one disconnected, linear-owner slice before any
production route activation:

1. Add one `VerifiedRawRootExpansionV1` selector that classifies Script,
   exactly one static `Main`, and duplicate static `Main` before candidate
   open or Builder effects.
2. Snapshot `NYASH_BUILD_STATIC_MAIN_ENTRY` once at the legacy ingress into a
   typed `CallableMainCompatibilityPolicyV1`; remove Builder-body env reads.
3. Route `Selected` callable Main lowering through the existing capture,
   admission, seal, collect, and receipt ledger. Propagate the original typed
   child error and stop before inline root, root batch, drain, finalizer, or
   external commit.
4. Preserve primary plus cleanup failures and panic restoration under the
   existing candidate failure proof; do not add retry or fallback.
5. Add duplicate-Main and selected-callable failure fixtures, plus a static
   guard forbidding `let _ = lower_static_method_as_function(...)` and any
   selected-failure continuation into inline-root lowering.

Acceptance:

```text
duplicate Main: deterministic typed rejection, Builder/candidate/collector = 0
NotSelected: reservation = 0, lowering = 0
Selected success: exact compatibility receipt consumed once
Selected failure: original typed error retained, root/drain/finalizer/commit = 0
all routes: disconnected owner only, production capture/commit = 0
source/check files: < 800 lines
```

Still unclaimed: production lifecycle wiring, all-route CUT0 activation,
FACTSESSION0, finalization repair retirement, FastMem/LLVM execution, and
parser/selfhost migration. CUT0-S0 must close before those lanes advance.

## CUT0-S0-OWNER0 closeout

The disconnected linear-owner structural seam is closed on 2026-07-22.

```text
root capture -> root complete -> CompleteInvocationV1
candidate -> complete_success
CompleteInvocationV1 -> prepare_complete
PreparedInvocationDrainV1 -> drain_candidate
DrainedModuleFinalizationInputV1 -> finalize_drained_module_once
```

The handoff keeps the original shell+collector state and the root-completion
witness intact. The post-drain terminal returns a typed finalized candidate;
it does not expose Builder, collector, HeaderPort, a bare module, retry, or
publication authority. Production capture/commit, drain, finalizer, and
external-commit consumers remain zero.

Evidence:

```text
python3 tools/checks/lib/cut0_s0_guard.py = green
cargo test -q completed_candidate_drains_to_typed_candidate_without_rebuilding_state --lib = green
cargo test -q finalizer_ --lib = green (6 tests)
cargo test -q successful_candidate_handoff_preserves_the_same_complete_state --lib = green
cargo check -q = green
```

The next code-facing row is `CUT0-S0-COMPAT0`: seal the compatibility policy
once at ingress, reject duplicate static Main before effects, propagate
selected callable-Main typed failures, and add the required failure fixtures
and static guard. No production route activation is authorized yet.

## CUT0-S0-COMPAT0 closeout

The disconnected compatibility bridge is closed on 2026-07-22:

- `VerifiedRawRootExpansionV1` performs source-only Script/App selection and
  rejects duplicate top-level static `Main` before `prepare_module`.
- `CallableMainCompatibilityPolicyV1` snapshots the legacy environment toggle
  once in `prepare_module`; Builder body lowering has no ambient toggle read.
- Selected callable-Main lowering uses the typed function-session path and
  restores the enclosing static-box context before returning an error.
- The typed failure fixture and `cut0_s0_compat_guard.py` forbid the old
  discarded-error form and keep the source/check files below 800 lines.
- Required policy reserves a dedicated `CallableMainCompatibility` ledger
  request. Success consumes exactly one collector receipt with
  `ledger.complete`; Primary, Cleanup, Admission, and Panic consume
  `ledger.abort` and retain the typed source failure plus collector prefix.
- `NotSelected` performs no reservation and no lowering.

Evidence:

```text
cargo test -q module_compat_raw_ledger --lib = green (4 tests)
python3 tools/checks/lib/cut0_s0_compat_guard.py = green
python3 tools/checks/lib/headerport_route_inventory_guard.py . = green
```

The adapter and fixtures remain test-only/disconnected. Production
capture/commit, all-route CUT0 activation, and the nine-route proof remain
zero/unstarted. The next code-facing row is `CUT0-P0`.

## CUT0-P0 closeout

The disconnected all-route proof is closed on 2026-07-22.

One test-only `Cut0P0OuterAdapterV1` now executes every row from the existing
`InvocationRouteMatrixV1` through the same move-only candidate lifecycle. The
adapter projects the existing route-owned authority lanes without opening a
new production API or duplicating symbol inventories.

The exercised outcomes are:

```text
success
primary / cleanup / admission / root failure
drain preflight failure
post-drain finalizer failure
panic
```

The success path reaches `complete_success` -> `prepare_complete` ->
`drain_candidate` -> `finalize_drained_module_once` and a test-only external
commit probe exactly once. Every failure and panic keeps external commit at
zero and retry forbidden. The 9 x 8 execution matrix is asserted by fixtures;
the adapter and guard remain below 800 lines and production consumers remain
zero.

Evidence:

```text
cargo test -q module_invocation_cut0_p0 --lib = green (2 tests)
python3 tools/checks/lib/cut0_p0_guard.py = green
```

The next code-facing row is `CUT0-I0`. Production capture/commit and atomic
CUT0 activation remain forbidden until that row is explicitly selected.
