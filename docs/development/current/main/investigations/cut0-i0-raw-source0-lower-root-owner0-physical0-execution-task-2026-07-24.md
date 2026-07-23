# RAW-SOURCE0 LOWER ROOT0 OWNER0 — PHYSICAL0 execution task

Status: **Active — Candidate RAW-OWNER-prime-r1 selected**  
Date: 2026-07-24

## Boundary

`OWNER0-PHYSICAL0` consumes only the sealed
`EligibleSourceBoundRawRootPackageV1`. It opens one route-owned physical
invocation for Script or plain static-Main App, but does not lower children or
the root body yet.

```text
EligibleSourceBoundRawRootPackageV1
  -> RawRootInvocationV1::{Script, App}
       owns exact token/source/continuation/config/module/plan
       owns one BuilderInvocationSession
       owns one empty shell + branded collector + raw ledger + root tracker
```

Production ingress, root capture, child descent, callable-Main descent,
root-body lowering, Main/condition batch, drain, finalization, postprocess,
external commit, retry, fallback, and JSON behavior remain zero.

## Locked ownership laws

1. The eligible wrapper is the only physical-open input. Raw package or plan
   alone cannot open a session.
2. The same non-Clone token brands session, shell, collector, ledger, and
   tracker. No ordinal copy, token conversion, or post-hoc branding exists.
3. The retained `RawSourceContinuationV1` remains the sole callable-Main
   selection authority. `Selected`/`NotSelected` is preserved by value.
4. Core-ID policy is the sealed Raw `ContinueLive` configuration. Live Builder
   state is not mutated before external commit.
5. The shell starts empty, and no Main-only `MainPending/MainCaptured` state,
   `current_module` lookup, ambient runtime read, or process-global slot call
   is allowed in this row.
6. Every open failure returns a discard-only rejected owner retaining the
   eligible package; no retry, resume, fallback, or re-pairing terminal exists.

## Products

```rust
RawRootPhysicalStateV1
RawScriptRootInvocationV1
RawAppRootInvocationV1
RejectedRawRootPhysicalOpenV1
```

The route-specific owner may share the existing neutral physical/session
vocabulary, but it must not create a second identity system or widen the
legacy Main-only state. `RawRootPhysicalStateV1` is private to the Builder
boundary and is not a bare shell/collector tuple exposed to callers.

## Required fixtures

```text
raw_owner0_script_opens_empty_physical_state
raw_owner0_app_omitted_preserves_not_selected
raw_owner0_app_required_preserves_selected_without_descent
raw_owner0_source_arity_never_changes_physical_main0
raw_owner0_shared_brand_across_session_shell_collector_ledger_tracker
raw_owner0_live_builder_unchanged_before_commit
raw_owner0_foreign_or_ineligible_package_cannot_open
raw_owner0_open_failure_retains_discard_only_owner
```

Every failure proves:

```text
session/shell/collector/ledger/tracker partial construction = 0 or fully retained
live Builder mutation = 0
global slot mutation = 0
child descent/root lowering = 0
retry/fallback/re-pairing = 0
production consumer = 0
```

## File and guard budget

Keep each touched source/check file below 800 lines. Prefer new Builder child
modules for physical owner state and focused fixtures; do not grow the old
Main-only completion file. Add or extend one reusable Raw OWNER0 guard that
checks exact producer counts and forbidden legacy/production call sites.

## Evidence

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
RUSTFLAGS='-Awarnings' cargo check -q --lib
RUSTFLAGS='-Awarnings' cargo test -q raw_owner0 --lib -- --test-threads=1
python3 tools/checks/lib/cut0_i0_root0_raw_source0_lower_root_owner0_physical0_guard.py
```

The next row after PHYSICAL0 is child/root lowering only after this owner
boundary is green. This task does not authorize production cutover.
