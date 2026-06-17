# 296x-1046 CALLSITE-CANONICALIZE-ENTRY-OWNER-DESIGN-001

Status: Landed
Date: 2026-06-17
Scope: callsite canonicalize schedule owner design

## Contract

```text
output_contract=hako-callsite-canonicalize-entry-owner-design-v0
row_kind=design

selected_option=B_thin_schedule_facade
production_entry_count=4
entry_removal_enabled=0
schedule_reorder_enabled=0
semantic_refresh_timing_changed=0
program_json_and_mir_json_policy_merged=0

transform_owner=src/mir/passes/callsite_canonicalize
schedule_owner=src/mir/passes/callsite_canonicalize/schedule
centralized_schedule_owner_target=1

implementation_started=0
next_task=CALLSITE-CANONICALIZE-SCHEDULE-FACADE-001
summary=ok
```

## Decision

Adopt option B from `RESIDUE-RETIRE-DESIGN-QUEUE-001`:

```text
Keep the four production entries, but make them delegate through one thin
schedule facade.
```

This reduces ownerless duplication without proving timing equivalence across
different pipelines.

## Why Not A

Option A keeps the current four direct calls and only documents timing seams.
That is safe, but it leaves the residue unchanged:

```text
centralized_schedule_owner=0
```

The transform owner is already single, but the scheduling contract is still
spread across four callers.

## Why Not C

Option C removes or moves entries after proving timing equivalence. That is too
large for this row because the four entries belong to different surfaces:

```text
src/mir/compiler/mod.rs:
  AST compile pipeline after RC insertion and semantic refresh

src/mir/optimizer/core.rs:
  optimizer late-call/inline pipeline before rune-plan inline refresh

src/runner/json_v0_bridge/core.rs:
  Program(JSON v0) bridge after lowering

src/runner/mir_json_v0.rs:
  MIR JSON loader before VM preflight consumers
```

Merging or deleting entries could accidentally change semantic refresh timing
or JSON acceptance behavior.

## Facade Shape

The implementation row should add a small schedule module under the existing
transform owner:

```text
src/mir/passes/callsite_canonicalize/
  schedule.rs
```

Required API:

```rust
pub enum CallsiteCanonicalizeScheduleSite {
    MirCompilerPostRc,
    MirOptimizerLateCallAndInline,
    ProgramJsonV0Bridge,
    MirJsonV0Loader,
}

pub fn canonicalize_for_site(
    module: &mut MirModule,
    site: CallsiteCanonicalizeScheduleSite,
) -> usize
```

The first implementation row may ignore `site` internally and call the existing
`canonicalize_callsites(module)`. The value of the facade is the named schedule
entry and future single point for timing guards.

## Required Call Sites

The implementation row should replace direct production calls with facade calls:

```text
src/mir/compiler/mod.rs
  canonicalize_for_site(..., MirCompilerPostRc)

src/mir/optimizer/core.rs
  canonicalize_for_site(..., MirOptimizerLateCallAndInline)

src/runner/json_v0_bridge/core.rs
  canonicalize_for_site(..., ProgramJsonV0Bridge)

src/runner/mir_json_v0.rs
  canonicalize_for_site(..., MirJsonV0Loader)
```

The implementation row must not remove any of those call sites.

## Post-Implementation Report Target

The inventory report should change from:

```text
centralized_schedule_owner=0
```

to:

```text
centralized_schedule_owner=1
production_entry_count=4
entry_removal_enabled=0
schedule_reorder_enabled=0
```

## Stop Line

```text
do not remove callsite canonicalize entries
do not change schedule order
do not change semantic refresh timing
do not merge Program(JSON v0) bridge and MIR JSON loader behavior
do not change callsite canonicalization transform semantics
do not use this row to retire legacy MIR instructions
```

## Verification

Design row only:

```bash
python3 tools/hako_check/callsite_canonicalize_entry_inventory.py --repo-root .
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Implementation row should add focused Rust tests for facade site mapping and
run the existing callsite canonicalize tests.
