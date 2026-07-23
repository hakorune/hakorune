# CUT0-I0 RAW-SOURCE0 LOWER ROOT0 OWNER0 consultation

Status: **Design stop — `RAW-SOURCE0-LOWER0-ROOT0-OWNER0-CONSULT0`**
Date: 2026-07-23
Scope: decide the one-time handoff from the landed source-derived Root0 plan
to a route-owned Script/App physical owner. No implementation or production
consumer is authorized until this consultation is locked.

Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/cut0-i0-raw-source0-lower-root-plan0-execution-task-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-raw-source0-lower-root-consultation-2026-07-23.md`
- `src/mir/compiler/raw_root_plan0.rs`
- `src/mir/compiler/raw_source_binding.rs`
- `src/mir/builder/raw_draft_invocation.rs`
- `src/mir/builder/module_lowering_invocation_state.rs`
- canonical reference: `src/mir/compiler/source_bound_package.rs`

## Evidence that forces this stop

`RawRootPlanV1::from_package` currently consumes the source-bound Raw package,
keeps the token and derived plan facts, and drops the owned AST/source
projection, sealed `RawSourceContinuationV1`, `BuilderInvocationConfigV1`, and
module name. The App plan also copies the callable-Main disposition into
`bool` fields. Opening a physical Root0 owner from the current plan alone
would therefore require one of the forbidden actions:

```text
AST/body re-resolution
ambient config or script-argument re-read
loose (plan, package) re-pairing
token re-issuance or post-hoc rebranding
selection from a copied bool instead of the sealed disposition
```

The existing `RawDraftInvocationV1::open` is not a safe bridge: it creates a
Main-only `ModuleLoweringInvocationStateV1`, starts `MainPending`, opens the
legacy physical state, and later re-finds one static child from the AST. Root0
must not call `capture_main`, `complete_root`, `lower_root`, `finalize_module`,
or the loose legacy physical-finalization seam.

## Questions to lock

### Q1 — source handoff owner

Which one-time product carries the exact source authority into OWNER0?

```text
1. SourceBoundRawRootPackageV1 (recommended)
   owns token + RawRootPlanV1 + OwnedRawSourceV1 + sealed continuation
   + Builder config + module name; one consuming open_physical(self)

2. RawRootPlanV1 itself owns all of the above
   and exposes one private consuming split to the physical owner

3. (plan, SourceBoundRawPackageV1) loose pairing
   rejected: it permits foreign/same-family re-pairing and duplicate identity
```

The selected form must be non-`Clone`, retain body/parameter payloads needed
by later lowering, and expose no token getter that permits reconstruction.

### Q2 — source plan versus retained source

Should OWNER0 retain the existing `OwnedRawSourceV1` until child/body lowering,
or should PLAN0 move every lowering body/parameter payload into the plan?

The default recommendation is to retain the exact owned source object in the
one-shot package. It already co-owns AST and its bound projection, avoids a
second source authority, and can be consumed later without re-resolution.
Moving body payloads into a new parallel schedule is allowed only if the
source locator, body, parameters, origin, and continuation remain one
non-`Clone` authority. A copied locator-only plan is insufficient.

### Q3 — callable-Main selection

The sealed `RawSourceContinuationV1::callable_main()` is the only selection
authority. Should the current `callable_main_selected: bool` and header-row
`selected: bool` be removed, or retained only as non-authoritative diagnostic
projections?

The recommended decision is to remove them from authority-bearing products.
`NotSelected` must perform no reservation/descent/receipt; `Selected` must be
retained by value and later drive the exact child. No bool copy may select or
repair the route.

### Q4 — physical carrier and state boundary

Should OWNER0 introduce a new carrier and route owner, separate from the
Main-only state?

Recommended shape:

```rust
struct RawRootPhysicalStateV1 {
    shell: InvocationBranded<ModuleLoweringShellV1>,
    collector: InvocationBranded<ModuleDraftCollectorV1>,
    ledger: InvocationBranded<RawExpansionReceiptLedgerV1>,
    tracker: RootBodyCompletionTrackerV1,
}

enum RawRootInvocationV1 {
    Script(RawScriptRootInvocationV1),
    App(RawAppRootInvocationV1),
}
```

The owner opens exactly one `ModuleBuilderInvocationSessionV1` with the
sealed Raw config (`ContinueLive` for Core IDs), one empty shell, one branded
collector/ledger/tracker carrier, and the retained source continuation. It
does not lower children or the root body yet.

### Q5 — unsupported source facts

Which facts remain explicit non-claims for OWNER0 and later rows?

```text
runtime script args / entry safepoint snapshot
unknown top-level declaration classification
instance methods / constructors / top-level callable catalog completeness
closure source sites
process-global type/method-slot reservation
static-table value/locator completeness
```

The owner must not silently turn these into `RuntimeStatement`, re-read an
ambient environment, or reserve a process-global slot. Each deferred fact
must either have an explicit unsupported disposition and a named follow-up
row, or be sealed in the source plan before physical effects.

## OWNER0 acceptance after lock

```text
one consuming package/plan -> one RawRootInvocationV1 constructor
token/session/shell/collector/ledger/tracker brand correspondence = exact
source continuation/config/module name retained to later lowering
physical shell and collector start empty
live Builder snapshot unchanged before external commit
Script/App route is explicit; source Main arity never changes physical main/0
Selected/NotSelected is sealed disposition only; no child descent in OWNER0
new owner references MainPending/capture_main/complete_root = 0
AST re-resolution/current_module/MirModule.functions/ambient reads = 0
global slot mutation/reservation = 0
retry/fallback/catch_unwind = 0
production root/physical/finalizer/postprocess/commit consumers = 0
all new or modified source/check files < 800 lines
```

Required fixtures after the decision:

```text
raw_root_owner0_script_opens_route_owned_physical_state
raw_root_owner0_app_omitted_preserves_not_selected
raw_root_owner0_app_required_preserves_selected_without_descent
raw_root_owner0_source_arity_never_changes_physical_main0
raw_root_owner0_shared_brand_and_empty_physical_state
raw_root_owner0_live_builder_unchanged_before_commit
```

## Non-claims while stopped

No child traversal, callable-Main lowering, root-body lowering, Main/condition
batch, drain, finalization, postprocess, external commit, public ingress, JSON
behavior, or CUT0 activation is changed by this consultation.

The next executable row can be selected only after Q1–Q5 are decision-locked
and the exact source/plan handoff is represented in the SSOT.
