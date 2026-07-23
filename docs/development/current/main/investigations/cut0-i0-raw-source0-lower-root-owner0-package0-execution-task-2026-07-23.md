# CUT0-I0 RAW-SOURCE0 LOWER ROOT0 OWNER0-PACKAGE0 execution task

Status: **Closed — `RAW-SOURCE0-LOWER0-ROOT0-OWNER0-PACKAGE0`**
Date: 2026-07-23
Scope: repair the one-time source-plan handoff only. No Builder session,
shell, collector, ledger, tracker, lowering, publication, or production
consumer is authorized.

Related:

- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/investigations/cut0-i0-raw-source0-lower-root-owner-consultation-2026-07-23.md`
- `docs/development/current/main/investigations/cut0-i0-raw-source0-lower-root-plan0-execution-task-2026-07-23.md`
- `src/mir/compiler/raw_source_binding.rs`
- `src/mir/compiler/raw_root_plan0.rs`
- canonical reference: `src/mir/compiler/source_bound_package.rs`

## Decision

`Candidate RAW-OWNER-prime-r1` is selected. The first load-bearing slice is a
compiler-owned non-Clone root package. Physical owner construction waits for a
later eligibility row.

## Objective

Implement exactly one consuming ownership transition:

```text
SourceBoundRawPackageV1
-> borrowed source-only planning
-> SourceBoundRawRootPackageV1
```

The result must retain the exact invocation identity and every source/config
owner needed by later Root0 rows:

```rust
struct SourceBoundRawRootPackageV1 {
    token: ModuleInvocationTokenV1,
    source: OwnedRawSourceV1,
    continuation: RawSourceContinuationV1,
    config: BuilderInvocationConfigV1,
    module_name: Box<str>,
    plan: RawRootPlanV1,
}
```

The package is non-Clone and exposes no public token getter, generic
`into_parts`, replacement field API, or loose constructor.

## Required construction law

The only constructor is conceptually:

```rust
impl SourceBoundRawPackageV1 {
    fn into_root_package(
        self,
    ) -> Result<
        SourceBoundRawRootPackageV1,
        RejectedRawRootPlanningV1,
    >;
}
```

Its order is fixed:

```text
borrow the complete bound package
-> derive/validate RawRootPlanV1
-> failure: retain the original bound package + typed error
-> success: destructure the bound package exactly once
-> infallibly co-seal all six owners
```

Planning failure must not drop or partially move the token, source,
continuation, config, or module name. The rejected owner exposes only error
inspection and discard; retry, resume, source replacement, and package
recovery terminals are forbidden.

## RawRootPlan authority correction

`RawRootPlanV1` becomes tokenless source facts. Remove:

```text
RawRootPlanV1.token
RawRootPlanV1.origin
RawRootPlanV1::token()
RawRootPlanV1::brand()
```

The retained source/continuation owns origin. The root package token owns
identity. The plan must not create a second identity or origin authority.

Callable-Main identity and selection are also separated:

```text
RawAppRootPlanV1.callable_main
  = source identity locator

RawSourceContinuationV1::callable_main()
  = sole Selected/NotSelected authority
```

Delete:

```text
RawAppRootPlanV1.callable_main_selected
RawCallableHeaderRowV1.selected
```

No replacement boolean or route flag is permitted.

## File structure

Do not grow the existing 621-line `raw_root_plan0.rs` into the next size
boundary. Prefer:

```text
src/mir/compiler/raw_root_package.rs
  SourceBoundRawRootPackageV1
  RejectedRawRootPlanningV1
  sole consuming package terminal

src/mir/compiler/raw_root_plan0.rs
  tokenless source facts and borrowed plan builder only

src/mir/compiler/raw_root_package_p0.rs
  focused fixtures
```

Every new or modified source/check file must remain below 800 lines.

## Required fixtures

```text
raw_root_package_script_retains_exact_bound_owners
raw_root_package_app_omitted_retains_not_selected_only_in_continuation
raw_root_package_app_required_retains_selected_only_in_continuation
raw_root_package_source_main_arity_keeps_physical_main0
raw_root_package_planning_failure_retains_original_bound_owner
raw_root_package_has_no_loose_token_or_parts_constructor
```

Positive fixtures must use the compiler-owned Raw binding path. They must not
use a test token factory, token copy, post-hoc brand wrapper, or a second source
projection.

## Acceptance

```text
SourceBoundRawRootPackageV1 definition = 1
SourceBoundRawPackage -> root package consuming constructor = 1
root package Clone/Arc = 0

root package owns exactly:
  token
  OwnedRawSourceV1
  RawSourceContinuationV1
  BuilderInvocationConfigV1
  module name
  RawRootPlanV1

planning failure retains exact original bound package
token re-issuance/post-hoc rebrand = 0
loose (plan, package) pairing = 0
public token getter/public split/replacement API = 0

RawRootPlan token/origin authority = 0
callable selection bool fields = 0
selection authority = retained typed continuation only

session/shell/collector/ledger/tracker construction = 0
MainPending/capture_main/complete_root references = 0
AST re-resolution/current_module/MirModule.functions/ambient reads = 0
global slot mutation/reservation = 0

Raw S0 behavior delta = 0
production package/owner consumer = 0
retry/fallback/catch_unwind = 0
all modified/new source/check files < 800 lines
```

## Non-claims

PACKAGE0 does not decide whether a deferred source capability is eligible for
physical effects. It retains the existing plan dispositions by value only.

The following remain later rows:

```text
OWNER0-ELIGIBILITY0
  runtime input snapshot and complete source-capability seal

OWNER0-PHYSICAL0
  Script/App session + empty shell/collector + ledger + tracker

OWNER0-P0/G0
  success/failure and old-seam census

ROOT0-DECLACCESS0
  declaration/index and shell metadata installation
```

No child traversal, callable-Main lowering, root-body lowering,
Main/condition batch, drain, finalizer, postprocess, external commit, public
ingress, JSON behavior, or CUT0 activation is authorized.

## Evidence commands

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' \
  cargo test -q raw_source_binding_p0 --lib -- --test-threads=1
RUSTFLAGS='-Awarnings' \
  cargo test -q raw_root_plan0 --lib -- --test-threads=1
RUSTFLAGS='-Awarnings' \
  cargo test -q raw_root_package --lib -- --test-threads=1
python3 \
  tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_plan0_guard.py
python3 \
  tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_owner0_package0_guard.py
```

This row requires a real code/artifact delta. A docs-only closeout is
forbidden.

## Closeout

PACKAGE0 landed in `7fbab72456`.

`SourceBoundRawRootPackageV1` now borrows the complete bound package to build
the tokenless `RawRootPlanV1`, and only after successful planning consumes the
original package into one non-Clone owner. `OwnedRawSourceV1`, the typed
`RawSourceContinuationV1`, Builder config, module name, token, and plan remain
co-sealed. Planning rejection retains the original package in a
discard-only `RejectedRawRootPlanningV1`.

`RawRootPlanV1` no longer owns token/origin identity. Callable-Main selection
booleans were removed; the retained typed continuation is the only selection
authority. No session, shell, collector, ledger, tracker, lowering, or
production consumer was added.

Evidence:

```text
cargo check -q --lib                         = green
cargo test -q raw_root_plan0 --lib           = 3 passed
cargo test -q raw_root_package --lib         = 3 passed
PACKAGE0 guard                               = green
current-state pointer guard                  = green
```

The next boundary is the design stop
`RAW-SOURCE0-LOWER0-ROOT0-OWNER0-ELIGIBILITY0-CONSULT0`.
