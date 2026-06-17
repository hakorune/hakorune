# 296x-1051 FASTPATH-REACHABILITY-RETIRE-OR-ENABLE-DESIGN-001

Status: Landed
Date: 2026-06-17
Scope: FastPathReachability Rust vocabulary design

## Contract

```text
output_contract=hako-fastpath-reachability-retire-or-enable-design-v0
row_kind=design

selected_option=B_retire_rust_fastpath_reachability
fastpath_reachability_non_test_consumer_count=0
python_reachability_ledger_kept=1
report_invariants_kept=1

preemption_deny_reason_enabled=0
reachability_feedback_to_resolver=0
backend_reads_reachability_rows=0
implementation_started=0

next_task=FASTPATH-REACHABILITY-RUST-VOCAB-RETIRE-001
summary=ok
```

## Decision

Retire the Rust-side `FastPathReachability` passive vocabulary from
`object_storage_plan`.

Keep the external reachability ledger tooling:

```text
tools/hako_check/fastpath_reachability_ledger.py
tools/checks/k2_wide_phase296x_fastpath_reachability_ledger_v1_guard.sh
```

The Python ledger is the active owner for selected / preempted / unreachable
route reporting. The Rust `object_storage_plan` type has no non-test consumer
and currently duplicates that post-hoc report concept.

## Boundary

The retirement is code-vocabulary cleanup only.

```text
eligibility stays separate from reachability
preemption is not a Deny reason
backend does not read reachability rows
hako_check reachability reports remain valid
```

## Why Not Enable

There is no active Rust pass that consumes `FastPathReachability`. Wiring it
would create a second reachability owner beside the existing hako_check ledger.

Until resolver execution exists, Rust-side reachability vocabulary adds concept
load without execution value.

## Required Implementation Row

`FASTPATH-REACHABILITY-RUST-VOCAB-RETIRE-001` should:

```text
delete src/object_storage_plan/reachability.rs
remove object_storage_plan reachability re-export
remove reachability-only unit tests
update object_storage_plan README concept count
replace report fields with rust-vocab-retired/tooling-owner fields
keep hako_check reachability ledger and guards unchanged
```

## Stop Line

```text
do not delete hako_check fastpath_reachability_ledger.py
do not change route priority behavior
do not convert preemption into a Deny reason
do not let backend infer eligibility from reachability rows
do not change product/backend lowering behavior
```

## Verification

```bash
cargo test -q object_storage_plan --lib
python3 -m unittest tools.hako_check.tests.test_fastpath_reachability_ledger
python3 -m unittest tools.hako_check.tests.test_object_storage_plan_vocab_audit
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
