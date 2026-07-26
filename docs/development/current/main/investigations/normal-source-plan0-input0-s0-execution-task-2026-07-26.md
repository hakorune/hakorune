---
Status: active execution task
Date: 2026-07-26
Decision: NORMAL-SOURCE-PLAN0-prime-r1
Row: NORMAL-SOURCE-PLAN0-INPUT0-S0
Scope: one disconnected consuming NormalFile-to-source-plan request
ceremony_tier: T1 bounded boundary extension
sunset_id: NORMAL-SOURCE-PLAN0-PROOF-SUNSET-001
proof_inventory_before: closed source-family classifier and existing narrow Raw handoff
new_proofs: one front-door/source-plan consuming projection and one-read/one-parse receipt witness
retired_or_merged_proofs: none in INPUT0
net_proof_delta: zero; extend the existing source-plan family fixture/guard
sunset_budget: shared with NORMAL-SOURCE-PLAN0-S0
sunset_row: NORMAL-SOURCE-PLAN0-G0
retire_when: canonical-core route consumes the source-plan request and disconnected consumer count is zero
budget_repayment_evidence: NORMAL-FILE-CANONICAL-CORE0-G0
Related:
  - docs/development/current/main/investigations/normal-source-plan0-design-stop-2026-07-26.md
  - docs/development/current/main/investigations/normal-source-plan0-s0-execution-task-2026-07-26.md
  - docs/development/current/main/investigations/normal-file-vm0-frontdoor-forge-task-2026-07-26.md
---

# NORMAL-SOURCE-PLAN0-INPUT0-S0

## Outcome

Connect the already parsed normal-file owner to the closed source classifier
without exposing a bare AST:

```text
PreparedNormalFileSourceV1
  -> prepare_source_plan_request(self)
  -> PreparedNormalFileSourcePlanRequestV1
       source-plan input
       sealed entry profile
       one-read/one-parse receipt
  -> classify(self)
  -> ClassifiedNormalFileSourcePlanV1
```

The request and classified product are disconnected production-shaped owners.
Existing `normal-file-vm-reference` continues to use
`prepare_raw_vm_handoff(self)` unchanged.

## Boundary law

The MIR root facade exposes only the source-plan boundary products required by
the front door. The runner does not import `mir::compiler` internals.

```text
crate::mir facade:
  PreparedNormalSourcePlanInputV1
  SealedNormalSourcePlanV1
  RejectedNormalSourcePlanV1
  one classifier terminal

runner reference owner:
  PreparedNormalFileSourcePlanRequestV1
  ClassifiedNormalFileSourcePlanV1
  RejectedNormalFileSourcePlanningV1
```

`PreparedNormalFileSourcePlanRequestV1` retains the sealed profile but does not
match it. Profile admission remains the later
`NORMAL-SOURCE-PLAN0-ADMISSION0-S0` authority.

The path already owned by `PreparedNormalFileSourceV1` becomes the source-plan
display identity by move. The AST is moved exactly once. The read/parse receipt
is retained in the outer request; no path read, source read, parse, AST clone,
or source-text reconstruction occurs.

## Failure law

If classification rejects, the outer rejection retains:

```text
RejectedNormalSourcePlanV1
sealed entry profile
NormalFileSourceReceiptV1
```

It exposes only:

```text
stage()
error()
discard(self)
```

There is no owner extraction, alternate profile, Raw retry, second parse, or
Legacy entry.

## Narrow-route isolation

```text
existing prepare_raw_vm_handoff caller count = unchanged
normal-file-vm-reference behavior            = unchanged
new source-plan production caller            = 0
default route delta                          = 0
Raw compile/execution delta                  = 0
```

The temporary second consuming terminal exists only inside the bounded
front-door owner. It is removed or hidden behind the sole typed dispatcher
when `NORMAL-SOURCE-PLAN0-DISPATCH0-I0` connects the canonical-core route.

## Fixtures

Use a temporary file only to exercise the existing one-read/one-parse front:

```text
empty Script       -> ScalarRoot::Script
scalar Script      -> ScalarRoot::Script
Main.main/0        -> ScalarRoot::Main0
Main + helper      -> CallableModule
function only      -> retained MissingSourceEntry
Script + Main      -> retained MixedSourceFamilies
parse rejection    -> no source-plan request
using rejection    -> no source-plan request
```

Also prove:

```text
source read  = 1
parse        = 1
AST move     = 1
classifier   = 1
profile match= 0
Raw handoff  = 0 in source-plan fixtures
```

## File boundary

Prefer a child module rather than growing the 600+ line front-door file:

```text
src/runner/reference/normal_file_vm_frontdoor/
  source_plan_input.rs
  source_plan_input_tests.rs
```

The parent receives declarations and one consuming delegation only. Every
modified/new source or check file remains below 800 lines.

Extend the existing manifest-backed guard:

```text
tools/checks/run_row_guard.sh --only normal-source-plan0
```

Do not create another shell or row-specific guard.

## Acceptance

```bash
cargo check --lib
cargo test -q --lib mir::compiler::normal_source_plan
cargo test -q --lib runner::reference::normal_file_vm_frontdoor
tools/checks/run_row_guard.sh --only normal-source-plan0
python3 tools/checks/lib/normal_file_vm0_frontdoor_forge_guard.py
bash tools/checks/current_state_pointer_guard.sh
```

## Immediate continuation

```text
NORMAL-SOURCE-PLAN0-INPUT0-S0
-> NORMAL-SOURCE-PLAN0-G0
```

G0 fixes one classifier, one front-door input producer, zero production
consumer, zero reclassification, and the proof sunset handoff.

## Non-claims

```text
profile admission
Main/function lowering
callable catalog
entry thunk/publication
VM execution/process result
new CLI/default caller
existing narrow-route behavior change
```
