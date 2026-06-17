# 296x-1052 FASTPATH-REACHABILITY-RUST-VOCAB-RETIRE-001

Status: Landed
Date: 2026-06-17
Scope: retire Rust-side FastPathReachability passive vocabulary

## Contract

```text
output_contract=hako-fastpath-reachability-rust-vocab-retire-v0
row_kind=implementation

source_evidence=296x-1051
fastpath_reachability_rust_vocabulary_retired=1
fastpath_reachability_non_test_consumer_count=0
fastpath_reachability_tooling_owner=hako_check
python_reachability_ledger_kept=1

preemption_deny_reason_enabled=0
reachability_feedback_to_resolver=0
backend_reads_reachability_rows=0
backend_behavior_changed=0
product_default_changed=0

summary=ok
```

## Changes

Removed the Rust `FastPathReachability` data carrier from
`object_storage_plan`.

Reachability remains a post-hoc report concern owned by hako_check tooling:

```text
tools/hako_check/fastpath_reachability_ledger.py
```

The `object_storage_plan` Rust concept groups now exclude reachability. The
remaining Rust-side fastpath path is:

```text
FastPathDecision -> Allow(LocalFastPathFact) | Deny(reason)
```

## Non-Goals

```text
do not delete hako_check reachability ledger tooling
do not change route priority behavior
do not make preemption a Deny reason
do not change backend lowering behavior
```

## Verification

```bash
cargo test -q object_storage_plan --lib
python3 -m unittest tools.hako_check.tests.test_fastpath_reachability_ledger
python3 -m unittest tools.hako_check.tests.test_object_storage_plan_vocab_audit
python3 tools/hako_check/object_storage_plan_vocab_audit.py --repo-root .
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
