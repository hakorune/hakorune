---
Status: superseded historical task map; execution authority none
Date: 2026-07-28
Decision: PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-prime-r1
Choice: A-double-prime
Closes:
  - PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-D0
First executable row:
  - none
Return row:
  - none
Superseded by:
  - docs/development/current/main/design/mirbuilder-inplace-replacement-policy-ssot.md
  - docs/development/current/main/investigations/mirbuilder-inplace-replacement0-task-map-2026-07-28.md
Related:
  - docs/development/current/main/investigations/preloop-stageb-selected-candidate-session0-d0-design-question-2026-07-28.md
  - docs/development/current/main/investigations/preloop-stageb-carrier-f5-f9-execution-task-2026-07-27.md
  - docs/development/current/main/investigations/hakorune-sparse-ownership-surface-task-2026-07-15.md
---

# PRELOOP-STAGEB selected candidate session task map

## Supersession

The design evidence remains historical, but this execution sequence must not
run.

It built a special source route while keeping production consumers at zero.
The current policy instead reconnects reusable neutral parts inside the
ordinary production MirBuilder and deletes the selected old path immediately.
The Stage-B-specific source selection, activation, and type-publication route
is parked for classification as `FixtureOnly` or `Delete`.

## Decision

```text
BuilderCandidateSessionCoreV1
  = candidate Builder
  + BuilderInvocationConfigV1
  + private success-only replacement primitive

ModuleBuilderInvocationSessionV1
  = Raw/Canonical brand + family
  + brand-free core
  + existing strict external-commit quiescence

LegacySelectedBuilderCandidateSessionV1
  = selected Legacy whole-source owner
  + brand-free core
  + Legacy-specific lifecycle/readiness
  + compiler-finish-before-commit
```

The core does not own universal readiness. Raw/Canonical and Legacy have
different valid post-lowering states, so each family wrapper retains its exact
replacement readiness law.

Legacy Selected starts from `ContinueLive`, preserves the existing Legacy
configuration, starts with no aliases, and permits the already-closed atomic
selected activation transaction to be the sole typed-alias installer.

No Legacy brand, invocation token, publication identity, Builder rollback,
Ordinary retry, or route reselection is introduced.

## Worker audit correction

The accepted design needs one additional mechanical handoff before the Legacy
finalizer.

```text
collect_preloop_stageb_instance_function_v1
  -> ModuleLoweringInvocationStateV1::collector

existing finalize_module
  -> MirBuilder::current_module
```

The selected function draft is collected correctly, but there is currently no
terminal that moves that exact collected draft into the candidate
`current_module`. Without this handoff, a successful selected session could
return a module missing `ParserBox.static_const_parse_add/2`.

This is not a new design stop. The collector already owns:

```text
FunctionDraftKeyV1::LegacySymbol
exact symbol and arity
DraftPublicationPolicyV1::LegacyReplaceWholePair
CollectedDraftAdmissionReceiptV1
```

The new terminal must consume those exact authorities and reuse the existing
Legacy draft-publication semantics. It must not create a second lowering,
inventory, symbol policy, or publication algorithm.

## Exact owner chain

```text
PreparedSelectedPreloopStageBWholeSourceV1
        +
BuilderInvocationConfigV1::snapshot_for_legacy_selected
  - ContinueLive
  - repl / quiet / plugin / source hint preserved
  - aliases empty
        ↓
LegacySelectedBuilderCandidateSessionV1
        ↓
existing prepare_module
        ↓
existing selected activation preflight and atomic install
        ↓
existing lower_root_with_preinstalled_catalog_v1
        ↓
CompletedPreloopStageBPreinstalledRootV1
        ↓
PreparedPreloopStageBSelectedDraftPublicationV1
        ↓ infallible exact Legacy publication
CompletedPreloopStageBSelectedDraftPublicationV1
  - selected draft present in candidate module exactly once
  - collector empty
  - selected source/result evidence retained
        ↓
existing finalize_module
        ↓
CompletedLegacySelectedModuleFinalizationV1
  - MirModule
  - candidate core
  - completed selected evidence
  - route-specific Legacy readiness
        ↓
existing MirCompiler::finish_built_module(..., Legacy)
        ↓
PreparedLegacySelectedCompileCommitV1
  - MirCompileResult
  - prepared candidate replacement
  - completed selected activation evidence
        ↓ infallible
replace live Builder exactly once
        ↓
return MirCompileResult
```

## Readiness boundary

Raw/Canonical keep the existing strict external-commit check:

```text
current module/function/block = closed
all FunctionOwned state       = empty
slot/context/recursion        = closed
```

Legacy Selected does not use or weaken that check. Its readiness is issued
only after the existing Legacy finalizer succeeds.

Required Legacy correspondence:

```text
current_module             = None
current_function           = None
current_slot_registry      = None
box compilation context    = None
recursion depth            = 0

selected activation        = exactly once
selected caller            = exact selected source key
selected function in module= exactly once
collector                  = empty

CoreContext lineage        = ContinueLive
repl / quiet / plugins     = opening config
source hint                = selected request
typed aliases              = selected atomic install
```

Legacy readiness does not require:

```text
current_block = None
all type/variable/function facts empty
```

Those are not invariants of the existing Legacy finalizer. The implementation
must not add implicit cleanup to imitate Raw/Canonical quiescence.

The P0 row compares normalized post-build state from ordinary Legacy and
selected isolated Legacy. Any unexplained difference is a hard stop:

```text
LEGACY-POST-FINALIZE-STATE-D0
```

Do not repair such a difference inside the selected session.

## Failure law

```text
candidate open failure:
  live Builder mutation = 0

prepare / activation / lowering failure:
  isolated candidate effects may exist
  complete strongest selected owner retained
  live Builder mutation = 0

selected draft publication mismatch:
  exact collector/source owner retained
  finalize = 0
  live Builder mutation = 0

finalize / readiness failure:
  selected evidence retained
  compiler finish = 0
  live Builder mutation = 0

compiler finish failure:
  candidate never replaces live Builder
  Ordinary retry = 0

success:
  compiler finish returns Ok
  PreparedLegacySelectedCompileCommitV1 = 1
  live Builder replacement = exactly 1
```

Legacy keeps its existing verifier behavior. `finish_built_module` returning
`Ok(MirCompileResult)` is the commit condition; an error stored in the
result's legacy verification field is not promoted into a new canonical
verification barrier.

Rejected products expose only:

```text
stage()
cause()
bounded_report()
discard(self)
```

They do not expose retry, resume, owner recovery, Ordinary fallback, brand
minting, or family reselection.

## Buildable implementation series

### Commit 1 — `BUILDER-CANDIDATE-SESSION-CORE0-S0`

Behavior-neutral extraction:

```text
BuilderCandidateSessionCoreV1
  owns candidate/config/replacement mechanism

ModuleBuilderInvocationSessionV1
  retains brand/family/readiness/publication laws
```

Also repair the stale `CURRENT_STATE` prose assertion in
`cut0_i0_session0_guard.py`; preserve its structural config/cursor/readiness
checks. Do not create a new public guard.

Acceptance:

```text
existing module_invocation_session_p0 = green
Raw/Canonical readiness behavior      = unchanged
Raw/Canonical commit receipts          = unchanged
Legacy production consumer             = 0
```

### Commit 2 — `PRELOOP-STAGEB-SELECTED-DRAFT-PUBLICATION0-S0`

Add a narrow collector-to-candidate-module terminal in a sibling module.

Prepare checks:

```text
collector draft count = 1
exact LegacySymbol key
exact selected symbol / receiver-adjusted arity
exact collected receipt
LegacyReplaceWholePair policy
candidate current_module exists
no shell/collector inventory drift
```

Commit is an infallible move using existing Legacy publication semantics.

Acceptance:

```text
selected symbol in candidate module = exactly 1
collector final count               = 0
ordinary method publication         = unchanged
duplicate/symbol/arity/receipt drift = typed reject
```

### Commit 3 — `PRELOOP-STAGEB-LEGACY-MODULE-COMPLETION0-S0`

Add narrow Builder-owned production facades over the existing
`prepare_module()` and `finalize_module()` implementations. Do not duplicate
their algorithms and do not grow `module_lifecycle.rs`.

Add:

```text
CompletedLegacySelectedModuleFinalizationV1
VerifiedLegacySelectedPostFinalizeV1
```

The successful existing finalizer is the Legacy route-specific readiness
authority. No FunctionOwned cleanup or generic external-quiescence check is
added.

### Commit 4 — `PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-S0`

Add:

```text
BuilderInvocationConfigV1::snapshot_for_legacy_selected
LegacySelectedBuilderCandidateSessionV1
PreparedLegacySelectedFinishInputV1
```

Drive only the existing lifecycle:

```text
prepare
-> atomic selected activation
-> preinstalled root
-> selected draft publication
-> finalize
-> Legacy readiness
```

Acceptance:

```text
ContinueLive five-cursor seed       = exact
repl/quiet/plugin/source preserved  = exact
candidate initial alias count       = 0
selected atomic alias installer     = sole 1
catalog reseal                      = 0
```

### Commit 5 — `PRELOOP-STAGEB-SELECTED-COMPILE-COMMIT0-I0`

Add a thin compiler sibling module and keep `compiler/mod.rs` as a facade.

```text
PreparedLegacySelectedFinishInputV1
  -> existing finish_built_module(..., Legacy)
  -> PreparedLegacySelectedCompileCommitV1
  -> one infallible live replacement
```

Acceptance:

```text
live replacement before finish success = 0
finish failure live mutation            = 0
loose result/session tuple               = 0
Legacy brand/publication owner           = 0
```

### Commit 6 — `PRELOOP-STAGEB-SELECTED-CANDIDATE-SESSION0-P0/G0`

Focused matrix:

```text
ordinary-vs-selected normalized post-finalize state
ContinueLive cursor preservation
config and typed-alias preservation
selected symbol exact one in returned module
prepare / activation / root / publication / finalize failures
compiler optimizer / contract failures
success-only one replacement
failure -> fresh same-compiler success
success -> later same-compiler success
zero-candidate Ordinary parity
no retry / fallback / reselection
```

Evolve the existing Stage-B child guard. Do not add a new public row guard.

### Commit 7 — `PRELOOP-STAGEB-LEGACY-WHOLE-SOURCE-REQUEST0-I0`

Complete owned request plumbing:

```text
LegacyWholeSourceCompileRequestV1
  = Legacy input + typed imports + diagnostic source hint

Ordinary
  -> existing Legacy route

Selected
  -> selected candidate session
```

Remove pre-selection live-Builder alias mutation. This repays:

```text
PRELOOP-STAGEB-LEGACY-ALIAS-MUTATION-SUNSET-001
```

### Commit 8 — `PRELOOP-STAGEB-COMPILE-REQUEST-INGRESS0-I0/P0/G0`

Connect the sole production selector:

```text
MirCompiler::compile_request
  / MirLoweringRequestV1::Legacy arm
```

Laws:

```text
0 complete candidates -> explicit Ordinary
1 complete candidate  -> Selected
many                   -> typed pre-Builder reject
Selected failure       -> no Ordinary retry
direct build_module callers delta = 0
```

At this commit, update the existing Stage-B child guard from production
selector count `0` to exact `1` in the Legacy arm.

### Commit 9 — progression and immediate return

Run the unchanged real progression proof:

```text
CALLABLE-RESULT-NESTED-PRELOOP-STAGEB0-P0
```

If green, do not insert cleanliness, loop-refresh, or a new design inventory.
Restore the parked ownership WIP and return directly to:

```text
OWN-GRAM-REJECT0-HAKO0-S0
```

The ownership WIP stash remains parked until this proof is green.

## File placement and size budget

Do not grow these near-cap files:

```text
src/mir/builder/module_lifecycle.rs = 795 lines
src/mir/builder/unified_emitter.rs  = 789 lines
src/mir/compiler/mod.rs             = 766 lines
```

Use small sibling modules for:

```text
candidate session core
selected draft publication
Legacy selected module completion/readiness
Legacy selected compiler finish/commit
Legacy selected P0 fixtures
```

Keep the existing public module facades thin. All modified/new source and
check files remain below 800 lines.

## Structural gate

```text
BuilderCandidateSessionCoreV1 producer               = 1
candidate Builder construction authority             = 1
physical live Builder replacement primitive          = 1

core owns brand/family/source policy                  = 0
core owns universal readiness                         = 0

Raw/Canonical strict quiescence consumer              = existing families only
Legacy strict quiescence consumer                     = 0
Legacy route-specific readiness producer              = 1
implicit Legacy finalizer cleanup                     = 0

selected draft collector producer                     = existing 1
selected draft candidate-module publication terminal = 1
second draft lowering/publication policy              = 0

Legacy Selected config seed                           = ContinueLive
Legacy Selected initial aliases                       = 0
selected atomic alias installer                       = existing sole 1

prepare/lower/finalize/finish authorities             = existing 1 each
second lifecycle/finalizer/postprocess                 = 0

PreparedLegacySelectedCompileCommitV1 producer        = 1
live replacement before compiler finish               = 0

Legacy brand/token/family/publication receipt          = 0
Builder clone/rollback                                 = 0
selected failure -> Ordinary retry                     = 0
fallback/reselection                                   = 0

production compile_request consumer before ingress    = 0
production compile_request consumer after ingress     = exact 1

ownership grammar activation during Stage-B           = 0
all modified/new source/check files                    < 800 lines
```

## Proof and sunset budget

```text
ceremony_tier:
  T2 for the candidate-session/replacement boundary
  T0/T1 for exact selected draft publication and request plumbing

new public guards:
  0

proof inventory:
  evolve callable_result_i0_site0_r0_expr0_m0_v0_stageb_session.py
  retain module_invocation_session_p0
  repair stale prose-only CUT0 assertion

sunset:
  PRELOOP-STAGEB-LEGACY-SOURCE-PRODUCER-SUNSET-001 remains parked
  until the selected Legacy consumer migrates to canonical/normal source-plan
  authority or the exact repair is no longer required
```

## Non-claims

```text
general Legacy transaction cutover
direct MirBuilder::build_module activation
Raw / Canonical lifecycle merge
new invocation or publication identity
universal Builder quiescence
Legacy finalizer cleanup redesign

loop-refresh activation
GenericLoop-side type publication
ownership grammar implementation during Stage-B
default compiler/backend route change
fallback / retry
```
