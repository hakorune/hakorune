# CUT0-I0 RAW-SOURCE0 LOWER ROOT0 PLAN0 execution task

Status: **Closed — `RAW-SOURCE0-LOWER0-ROOT0-PLAN0` (implementation landed in `0268d46c9b`)**
Date: 2026-07-23  
Scope: source-derived Raw Script/App root plan only. No Builder/session,
shell, collector, ledger, lowering, reservation, receipt, root body,
finalizer, postprocess, public ingress, JSON behavior, external commit, or
CUT0 activation is allowed here.

Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/cut0-i0-raw-source0-lower-root-consultation-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-raw-source0-lower-execution-task-2026-07-23.md`
- `src/mir/compiler/raw_source_binding.rs`
- `src/mir/builder/raw_draft_invocation.rs`
- `src/mir/builder/module_lifecycle.rs`

## Decision

`RAW-ROOT-prime-r1` is selected. Root lowering will use a dedicated
Script/App protocol and will not widen the legacy Main-only state. PLAN0 is a
source authority row: it must produce a complete, deterministic, owned plan
before any Builder effect exists.

## Objective

Implement one non-Clone, source-derived root plan product:

```text
SourceBoundRawPackageV1
-> RawRootPlanBindingV1
-> RawRootKindV1::{Script, App}
-> complete ordered RawRootEnvironmentPlanV1
```

The product is disconnected evidence. It is not a root owner and cannot open
or mutate a Builder session.

## Required products

### Physical root identity

Seal exactly one physical identity vocabulary:

```text
main        = symbol "main", arity 0
condition_fn = symbol "condition_fn", arity 1
```

The source `Main.main/N` locator may retain N for source parameter-local
semantics and callable compatibility identity, but it must not affect physical
`main/0`.

### Root kind

```rust
enum RawRootKindV1 {
    Script(RawScriptRootPlanV1),
    App(RawAppRootPlanV1),
}
```

Script owns a deterministic top-level runtime-statement schedule and has no
callable-Main selection. App owns the source Main locator/arity, static-child
locators, declaration schedule, and a callable-Main locator only as a locator;
the sealed continuation disposition remains the selection authority.

### Environment plan

`RawRootEnvironmentPlanV1` must own or immutably seal:

```text
complete ordered source work schedule
declaration/index plan
callable catalog projection
static-table specs/plans or explicit unsupported disposition
closure source sites or explicit unsupported disposition
root access requirements
runtime-input snapshot (script args / entry safepoint)
```

Every source declaration is classified exactly once or rejected with a typed
unsupported-capability error. The plan must not contain a final collector row
list, receipt prediction, `MirModule` map, Builder session, or current-module
snapshot.

### Source ownership

Consume the already bound source package without re-running
`VerifiedRawRootExpansionV1` and without cloning a second projection authority.
The plan may retain stable owned locators and source-derived facts, but it must
not re-resolve source declarations after binding.

## Forbidden paths

```text
ModuleLoweringInvocationStateV1::capture_main / complete_root
MirBuilder::lower_root / finalize_module
current_module or MirModule.functions inventory reads
ambient compatibility/environment reads
process-global type/method-slot mutation
Builder/session/shell/collector/ledger construction
ledger reservation or collector admission
source AST rescan or second expansion
retry/fallback/catch_unwind
production consumer or public executor wiring
```

Process-global method-slot use is a later `ROOT0-SLOT0` decision. PLAN0 must
record the requirement or reject the unsupported shape; it must not reserve a
global slot as a side effect.

## Fixtures

Minimum focused fixtures:

```text
Script with deterministic top-level statements -> Script plan
App with Main.main/N -> App plan with physical main/0
App with callable-Main locator + NotSelected -> no selected child work
App with callable-Main locator + Selected -> disposition retained, not re-read
declaration reorder -> deterministic schedule/projection parity
unknown/unclassified top-level declaration -> typed rejection
missing static/closure fact -> typed unsupported rejection, no Builder effect
runtime input snapshot is value-owned and ambient reread = 0
```

## Acceptance

```text
RawRootKind variants = Script + App
physical root identity producer = 1
source Main arity -> physical main arity propagation = 0
callable locator presence != Selected disposition
every source work item classified exactly once or rejected
source/catalog/static/closure projection producer = 1 each
source AST re-resolution after binding = 0
current_module reads = 0
process-global method-slot mutation = 0
Builder/session/shell/collector/ledger mutations = 0
production root consumer/executor = 0
all modified/new source/check files < 800 lines
```

## Follow-up rows

PLAN0 does not implement the following. They require separate cards and
guards:

```text
ROOT0-OWNER0       route-owned Script/App physical owner
ROOT0-DECLACCESS0  atomic Builder-index + shell-metadata installation
ROOT0-SLOT0        invocation-local slot authority or typed unsupported gate
ROOT0-CHILDREN0    source-ordered child traversal
ROOT0-CALLMAIN0    Selected/NotSelected callable Main sequencing
ROOT0-BODY0       main/0 body and CompletedRootBody witness
ROOT0-PAIR0       paired Main/condition reserve/preflight/commit
ROOT0-COMPLETE0   CompletedRawSourceInvocation handoff
ROOT0-P0/G0        success/failure matrix and production-zero census
```

## Evidence commands

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_plan0_guard.py
```

No public ingress, JSON bridge, Raw production consumer, or physical
finalization is authorized by this row.

## Closeout

The source-derived Script/App plan is landed and verified. It seals physical
`main/0` and `condition_fn/1`, an ordered top-level work schedule, declaration
and static-data projections, callable-header projections, explicit closure and
runtime-input dispositions, and the compiler-issued invocation token. The
plan is non-`Clone`; focused Script/App/selection fixtures pass, the source
binding and Root0 guards pass, `cargo check --lib` passes, and no Builder,
session, shell, collector, ledger, reservation, lowering, or production
consumer was added.

The next owner handoff is intentionally not claimed here. The current plan
consumer retains only derived plan facts and the token; the source AST/body,
sealed `RawSourceContinuationV1`, Builder config, and module name must be
retained or split exactly once before `ROOT0-OWNER0`. No loose plan/package
re-pairing, AST re-resolution, ambient re-read, or copied callable-Main
selection may be introduced. Those choices are the next design consultation.
