# CUT0-I0 T-prime-r1 Execution Task

Status: **Design stop — CUT0-I0-ROOT0-D0 required before ROOT0 implementation**
Date: 2026-07-22
Decision: **Candidate T-prime-r1 selected**
Scope: build one invocation-branded module transaction, then perform one
atomic all-route production cutover

Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/cut0-i0-production-transaction-consultation-2026-07-22.md`
- `docs/development/current/main/investigations/mirbuilder-headerport-i0-hdr0-p0-execution-task-2026-07-22.md`
- `src/mir/builder/module_invocation_route_matrix.rs`
- `src/mir/builder/route_owned_invocation_inventory.rs`
- `src/mir/compiler/module_session.rs`

## Locked architecture

```text
public compile ingress
-> source/plan preflight
-> one private ModuleInvocationIdV1
-> one sealed five-family ModuleInvocationTokenV1

-> BuilderInvocationConfigV1
-> ModuleBuilderInvocationSessionV1
-> invocation-branded shell + collector

-> route-specific physical draft collection
-> source proof + collector co-seal
-> route-specific root witness
-> CompleteInvocationV1

-> source-derived drain preflight
-> DrainedModuleCandidateV1
-> fallible actual module finalization
-> FinalizedModuleCandidateV1
-> exact postprocess schedule
-> PostprocessedModuleCandidateV1

-> PreparedModuleExternalCommitV1
-> one MirCompiler-owned infallible commit
-> MirCompileResult
```

No caller-authored inventory, loose shell/collector reconstruction, bare
`MirModule` handoff, route flag, fallback, retry, or post-drain Builder read is
permitted.

## Current proof gap

The existing `Cut0P0OuterAdapterV1` is a topology proof only. All nine rows
select an authority-lane label and then run the same synthetic shell,
main-only collector, Main capture, drain, finalizer, and commit probe. Its
fixture also admits `main` as `LegacySymbol("main")`, not the route-correct
root identity.

Therefore current P0 proves only:

```text
route row -> common candidate/drain/finalizer/commit API
```

It does not prove the inseparable correspondence required by CUT0:

```text
real sealed source authority
<-> exact physical collector rows
<-> route-specific root policy
<-> exact postprocess schedule
<-> external publication
```

`CUT0-I0-P0-R1` must close this gap before production activation.

## Durable laws

### Identity

One non-Clone invocation ID is minted after source/plan preflight and before
Builder effects. The same ID brands:

```text
token
Builder session
shell
collector
raw ledger
InvocationDraftSourceProofV1
single/batch receipts
collected draft set
complete/drained/finalized/postprocessed candidates
prepared external commit
```

Foreign pairing fails before mutation. The raw ledger does not mint a second
owner identity.

### Family token

The token has exactly five private variants, each carrying its actual sealed
source authority:

```text
Raw
  -> verified raw source expansion + sealed compatibility policy
CanonicalAPlus
  -> CanonicalCurrentAPlusPlanV1 + verified owner header
BindingSsaTrivial
  -> CanonicalTrivialBindingSsaPlanV1 + verified owner header
BindingSsaAcyclic
  -> VerifiedAcyclicCallableModulePlanV1
BindingSsaRecursive
  -> VerifiedRecursiveCallableModulePlanV1
```

The nine route-matrix rows are static phase witnesses inside these families,
not caller-selectable runtime flags.

### Collection

The collector remains the sole physical `MirFunction`/completed-header owner.
Raw, canonical-single, and callable-batch proofs have separate private seal
terminals. Every terminal validates ID, key, symbol, arity, policy,
cardinality, missing rows, and surplus rows.

Callable batches follow:

```text
whole batch preflight
-> PreparedCallableBatchAdmissionV1
-> infallible collect_all
-> CollectedCallableBatchReceiptV1
```

Any batch admission failure leaves collector delta at zero. Recursive module
capability is a route-owned shell fact, never a function row.

### Builder session

`BuilderInvocationConfigV1` snapshots the persistent Builder inputs before
candidate creation:

```text
repl_mode
quiet_internal_logs
using_import_boxes
plugin_method_sigs
resolved source_file
BuilderCoreIdSeedV1
```

Raw uses `ContinueLive` CoreContext counters for CUT0 parity. Canonical
families use `Fresh`. A later all-Fresh generation policy requires a separate
task. Imports and source hints are never pre-written to the live Builder.

Before external commit, the candidate Builder must prove:

```text
current_module = None
current_function = None
current_block = None
current_slot_registry = None
compilation_context = None
recursion_depth = 0
all function-local stacks closed
```

### Root completion

Raw Main state is not a route-neutral lifecycle:

```text
RawMainPendingInvocationV1
-> RawMainCapturedInvocationV1
-> RawCompleteInvocationV1

Canonical single receipt
-> CanonicalSingleCompleteInvocationV1

Callable batch receipt
-> CallableBatchCompleteInvocationV1
```

They converge at a branded `CompleteInvocationV1` carrying one
`InvocationRootWitnessV1`.

The Raw witness keeps these products inseparable:

```text
Raw {
  root_body: CompletedRootBodyV1,
  condition: RequiredConditionFnReceiptV1,
  callable_main: CallableMainCompatibilityDispositionV1,
}
```

```text
Raw:
  main exactly 1
  condition_fn exactly 1

Canonical single:
  exact owner symbol only
  synthetic main = 0
  synthetic condition_fn = 0

Callable batch:
  exact catalog inventory
  synthetic main = 0
  synthetic condition_fn = 0
```

`ConditionFnPolicyV1::Optional`, caller-authored symbol inventories, and a
caller-provided `require_main` boolean have no production constructor.

### Postprocess and commit

The family token derives one exact schedule preserving current behavior:

| Family | Legacy RC | Verification barrier |
|---|---|---|
| Raw | Run | pre-transform result may be returned as `Err` inside `MirCompileResult` |
| CanonicalAPlus | Run | final verification required before commit |
| BindingSsaTrivial | Skip | final verification required before commit |
| BindingSsaAcyclic | Skip | final verification required before commit |
| BindingSsaRecursive | Skip | final verification required before commit |

The physical order remains rune refresh, optimizer, contract refresh,
pre-transform verifier, selected RC insertion, semantic refresh, callsite
canonicalization, conditional refresh, and canonical final verifier.

`finalize_drained_module_once` must become a real fallible finalizer or be
renamed to an honest seal terminal before production activation. Finalizer,
optimizer, contract, and canonical verification failures publish nothing.

External commit is a single-use product:

```text
PreparedModuleExternalCommitV1 {
  id,
  session,
  postprocessed_module,
}
```

Its constructor verifies matching IDs and commit-ready Builder state. Its
`commit(self, &mut live_builder)` terminal is infallible and owned only by one
private `MirCompiler::execute_preflighted_module_invocation` entry.

### Failure

Function-terminal restoration remains local evidence. Production policy is:

```text
first child primary/cleanup/admission error
-> failed child collect = 0
-> parent restore = 1
-> collector prefix unchanged
-> outer invocation abort
-> later sibling descent = 0
-> root/drain/finalizer/commit = 0
-> retry/fallback = 0
```

When cleanup fails after a primary failure, typed evidence retains both
causes as `DuringCleanup { primary, cleanup }` before the outer invocation
aborts.

Foreign invocation mismatch is a pre-mutation typed error. Same-invocation
identity/history corruption poisons the ledger permanently. Ordinary child
failure consumes typed abort evidence without turning every future object into
a global poison flag. Panic unwinding drops the candidate/session and leaves
the live Builder unchanged.

## Implementation rows

Each row is one BoxShape commit or a short Refactor Series Mode sequence. No
row may activate production before `CUT0-I0`.

### CUT0-I0-T0 — closed by decision lock

Close Candidate T-prime-r1 in the consultation and this execution card. No
production behavior changes. This docs-only row owns
`cut0_i0_t_prime_r1_guard.py`, which freezes the decision vocabulary, next
pointer, production-consumer zero, and the 800-line ceiling.

### CUT0-I0-ID0-S0 — closed

Add private, non-Clone invocation identity and the five-family sealed token.
Mint both only through one test-only/disconnected preflight factory. Add
fixtures proving foreign family/source construction is impossible through the
public module surface.

Acceptance:

```text
invocation ID producer = 1
five sealed family variants = 5
caller-selectable route flags = 0
production token consumers = 0
source/check files < 800 lines
```

Closeout evidence:

```text
RUSTFLAGS='-Awarnings' cargo test -q module_invocation_identity_p0 --lib = 4 passed
python3 tools/checks/lib/cut0_i0_id0_s0_guard.py = green
python3 tools/checks/lib/cut0_i0_t_prime_r1_guard.py = green
```

The production vocabulary has zero shell, collector, receipt, compiler, or
publication consumers. The next disconnected row is `CUT0-I0-ID0-P0`, now
closed below.

### CUT0-I0-ID0-P0 — closed: brand the complete owner chain

Thread the ID through session, shell, collector, ledger,
`InvocationDraftSourceProofV1`, every receipt, and all
complete/drained/finalized products. Replace loose production-capable
constructors with private branded constructors; keep any temporary loose
constructors test-only and guarded for retirement.

Fixtures:

```text
foreign shell + collector -> construction impossible or pre-mutation error
foreign raw receipt -> pre-mutation error
foreign canonical receipt -> pre-mutation error
foreign batch receipt -> pre-mutation error
same-invocation happy path -> exact ID preserved to final candidate
source proof + collector co-seal -> same invocation ID required
```

Closeout evidence:

```text
RUSTFLAGS='-Awarnings' cargo test -q module_invocation_brand_p0 --lib = 4 passed
python3 tools/checks/lib/cut0_i0_id0_p0_guard.py = green
python3 tools/checks/lib/cut0_i0_id0_s0_guard.py = green
python3 tools/checks/lib/cut0_i0_t_prime_r1_guard.py = green
```

The disconnected owner-chain vocabulary now carries one opaque brand through
the Builder session, source proof, receipt, collector co-seal, complete,
drained, finalized, and prepared external-commit products. Foreign collector
or receipt pairing fails before co-seal mutation, and receipt family policy is
checked against the sealed source family. No production consumer or ingress
was added. `CUT0-I0-COLLECT0-S0` is now closed below; the next disconnected
row is `CUT0-I0-COLLECT0-BATCH0`.

### CUT0-I0-COLLECT0-S0 — closed: raw and canonical-single co-seal

Add separate private `seal_raw` and `seal_canonical_single` terminals. They
consume source proof plus collector and emit one
`CollectedInvocationDraftSetV1`; no generic caller-built source enum and no
caller symbol list is accepted.

Acceptance covers bijection, key/symbol/arity/policy/cardinality, raw
replacement history, callable-Main disposition, surplus/missing rows, and
foreign invocation rejection before mutation.

```text
Raw: final ledger rows, collector rows, and physical receipts are bijective
Canonical-single: exact one row, exact owner key, no synthetic roots
```

Closeout evidence:

```text
RUSTFLAGS='-Awarnings' cargo test module_invocation_collect0_s0_p0 --lib = 3 passed
python3 tools/checks/lib/cut0_i0_collect0_s0_guard.py = green
python3 tools/checks/lib/cut0_i0_id0_p0_guard.py = green
```

`RawInvocationSourceProofV1` now owns a sealed raw expansion ledger and
`CanonicalSingleInvocationSourceProofV1` owns a verified owner header. The
private `seal_raw` and `seal_canonical_single` terminals compare those source
authorities with a branded physical collector and receipts before issuing
route-specific collected sets. Raw checks final ledger rows and replacement
history; canonical-single checks one exact owner row, symbol, arity, and
`CanonicalRejectDuplicate`. Foreign brand, family, cardinality, key, symbol,
arity, policy, and replacement failures remain pre-co-seal errors. No
production caller or route activation was added. The next disconnected row is
`CUT0-I0-COLLECT0-BATCH0`.

### CUT0-I0-COLLECT0-BATCH0 — closed: atomic callable collection

Teach the existing verified unpublished callable draft owner to prepare one
whole collector batch. All fallible validation precedes an infallible
`collect_all`; do not loop over single admissions with partial mutation. Seal
the recursive capability into the shell from the recursive family token.

Acceptance:

```text
late collision -> collector delta = 0
exact catalog keys/symbols/cardinality -> one batch receipt
replacement receipt count = 0
recursive capability preserved exactly once
new collector-batch production consumers = 0
existing publish_into(MirModule) remains unchanged until atomic CUT0-I0
```

Closeout evidence:

```text
RUSTFLAGS='-Awarnings' cargo test -q callable_batch_collection_p0 --lib = 6 passed
RUSTFLAGS='-Awarnings' cargo test -q callable_module_transaction --lib = 13 passed
python3 tools/checks/lib/cut0_i0_collect0_batch0_guard.py = green
RUSTFLAGS='-Awarnings' cargo check -q --lib = green
```

The existing `VerifiedUnpublishedCallableDraftSetV1` now retains its verified
catalog source while projecting exact `CanonicalCallable` rows into a new
collector-only batch terminal. All row checks, collector collisions, and
within-batch duplicate checks happen before `collect_all`; the commit terminal
is infallible and returns one whole-batch receipt with canonical policy and
inserted replacement disposition for every row. The callable source/collector
co-seal compares actual catalog headers, physical collector keys, arity, and
receipt rows under one invocation brand. The recursive fixture installs the
existing shell metadata capability exactly once and rejects a duplicate
installation; raw/acyclic routes carry no marker. No `publish_into` consumer,
production caller, or route activation changed. The next disconnected row is
`CUT0-I0-SESSION0`.

### CUT0-I0-SESSION0 — closed: route-neutral Builder transaction

Add explicit config snapshot/install and CoreContext seed policy around a
fresh candidate Builder. Prove live Builder invariance for every failure and
compiler-reuse parity for Raw `ContinueLive` and canonical `Fresh`.

Do not mutate live imports/source hints before session success. Add a
commit-ready state witness, but no external commit consumer yet.

Acceptance:

```text
all persistent config fields are snapshotted once
ContinueLive carries Value/Block/Binding/temp/debug cursors
Fresh starts all five cursors at zero
candidate drop/failure leaves live Builder unchanged
commit-ready witness rejects open module/function/slot state
prepared Builder commit is consuming and one-shot
new session production consumers = 0
```

Closeout evidence:

```text
RUSTFLAGS='-Awarnings' cargo test -q module_invocation_session_p0 --lib = 6 passed
RUSTFLAGS='-Awarnings' cargo check -q --lib = green
python3 tools/checks/lib/cut0_i0_session0_guard.py = green
python3 tools/checks/current_state_pointer_guard.py = green
```

`ModuleBuilderInvocationSessionV1` now owns a disconnected candidate Builder
with an explicit `BuilderInvocationConfigV1`. The config copies REPL/log,
imports, plugin signatures, resolved source metadata, and a typed
`ContinueLive`/`Fresh` CoreContext seed. `CoreContext::from_cursors` installs
all five counters without a second allocation authority. A consuming
commit-ready witness rejects open publication state before issuing the
one-shot prepared Builder commit. Fixtures prove explicit configuration,
five-counter parity, candidate-drop invariance, readiness rejection, and
single-use commit. Existing canonical session/open/commit callers are
unchanged and no production ingress or external commit consumer was added.
The next disconnected row is `CUT0-I0-ROOT0`.

### CUT0-I0-ROOT0-D0 — design stop: owner, evidence, and drain authority

ROOT0 implementation is paused. A read-only source/worker audit found three
unresolved authority boundaries:

```text
actual Builder session + shell + source/collector set -> one invocation brand
Raw root body + condition receipt + callable-main disposition -> one witness
route-specific complete state -> private source-derived drain plan
```

The current ID0 placeholder brand carries `()` payloads, the real Builder
session is unbranded, and the raw receipt ledger mints a second owner ordinal.
The raw root body is also consumed before collector receipt publication, while
the existing drain still accepts caller symbols/`require_main`/Optional and
unconditionally requires `main`. These must not be papered over by another
wrapper or boolean.

The design-stop brief is:

`docs/development/current/main/investigations/cut0-i0-root0-design-stop-2026-07-22.md`

The next executable slices are `ROOT0-BRAND0`, `ROOT0-RAW0`,
`ROOT0-CANON0`, `ROOT0-DRAIN0`, then `ROOT0-P0/G0`. Production capture,
drain, finalization, and CUT0 activation remain forbidden.

### CUT0-I0-ROOT0 — route-specific completion and drain policy

Separate raw Main transitions from canonical single and callable batch
completion. Derive the expected inventory and root policy only from the source
proof. Remove universal Main capture from the disconnected adapter and remove
production constructors for Optional condition or caller-owned inventories.

Raw completion must co-seal `CompletedRootBodyV1`, the required condition
receipt, and callable-Main disposition into the root witness. Missing or
foreign components fail before drain.

Required parity fixtures compare current canonical inventories before the
intentional removal of synthetic `main` and `condition_fn` is accepted. Any
semantic difference must be documented as the selected CUT0 behavior, not
hidden by fixture normalization.

### CUT0-I0-POST0 — actual finalization, postprocess, prepared commit

Make the post-drain finalization boundary honest and fallible. Move the exact
existing postprocess order behind `ModulePostprocessScheduleV1`. Preserve the
legacy non-fatal verifier result and canonical final-verifier commit barrier.
Add `PreparedModuleExternalCommitV1` and a test-only commit probe; production
consumer count remains zero.

### CUT0-I0-P0-R1 — real-authority all-route proof

Replace the current synthetic-main topology proof with one proof per actual
route source authority:

```text
sealed route source
<-> exact collector rows
<-> route root witness
<-> postprocess schedule
<-> prepared external commit
```

Run all nine rows across success, child primary, cleanup,
primary-plus-cleanup, admission, root, batch, drain, finalizer, postprocess,
final verifier, foreign-ID, and panic outcomes. Every failure has external
commit zero and no later sibling descent; primary-plus-cleanup retains both
typed causes.

### CUT0-I0 — atomic production switch

In one production patch, route every public compile ingress through one
private `execute_preflighted_module_invocation` owner. The same patch makes
these production consumers zero:

```text
raw direct live-builder build
canonical old session publication
callable publish_into(MirModule)
restore-then-publish
finalize_module direct function insertion
route-local activation flags/fallback/retry
```

Acceptance:

```text
common outer executor = 1
collector owner = 1
drain terminal = 1
module finalizer = 1
postprocessor = 1
external commit terminal = 1

failure/panic external commit = 0
all route inventories and verifier semantics preserve selected parity
```

### CUT0-G0 — retirement and final guards

Delete test-only loose constructors and all legacy publication owners after
consumer counts reach zero. Seal the production caller census and resume the
fixed queue at `FACTSESSION0`.

## Commit discipline

- One row per commit; Refactor Series Mode is allowed only within one named
  BoxShape purpose.
- Every modified/new source or check file stays below 800 lines.
- Add or update the row guard in the same commit as its vocabulary.
- No failed active fast gate is committed.
- Production consumer count remains zero through `CUT0-I0-P0-R1`.
- Commit and push after each closed row.

## Non-claims

This task does not claim CUT0 production activation, FACTSESSION0,
finalization-repair retirement, FastMem/LLVM execution, JoinIR retirement,
MirBuilder selfhost completion, Parser selfhost, or Stage-2 bootstrap. Those
remain downstream of the atomic CUT0 and its final guard.
