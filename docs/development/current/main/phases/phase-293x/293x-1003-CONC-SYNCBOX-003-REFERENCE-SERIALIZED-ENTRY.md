# 293x-1003 CONC-SYNCBOX-003 Reference Serialized Entry

Status: landed-code
Date: 2026-06-13

## Decision

Implement `sync box` v0 as reference semantics only:

```text
scope=reference_only
serialized_truth_owner=runtime_object_instance_side
method_dispatch_role=enter_exit_only_later
reentrant_sync_call_policy=v0_fail_fast
```

Program JSON, MIR, and LLVM lowering remain unsupported and must continue to
fail-fast instead of treating `sync box` as an ordinary `box`.

## Implementation

Code owner:

```text
src/runtime/sync_box.rs
```

The row adds:

```text
SyncState
SyncState::enter(object_id, method_name)
SyncEntryGuard
SyncBoxError::ReentrantEntry
SyncBoxError::Poisoned
sync_box_reference_report_fields()
```

`SyncState` is the reference shape for future per-instance serialized state.
The current implementation is not connected to normal compiler lowering.

## Report Fields

```text
sync_box_reference_runtime_enabled=1
sync_box_mir_lowering_enabled=0
sync_box_program_json_enabled=0
sync_box_llvm_enabled=0
sync_box_fairness_guarantee=0
sync_box_reentrancy_guarantee=0
sync_box_lock_order_verifier_enabled=0
sync_box_worker_pool_route_enabled=0
```

## Stop Lines

- No Program JSON support.
- No MIRBuilder support.
- No LLVM support.
- No ordinary-box fallback.
- No fairness guarantee.
- No reentrancy guarantee.
- No lock-order verifier.
- No worker-pool route activation.
- No source-level `lock<T>` promotion.

## Next Rows

```text
CONC-CHANNEL-002:
  implement await-visible close semantics for future Channel<T> queue runtime.

CONC-CHANNEL-003:
  implement await-visible send/recv route shape or fail-fast bridge.
```

`CONC-SYNCBOX-004` may later carry MIR metadata only after reference semantics
and unsupported-backend guards are stable.

## Evidence

```bash
cargo test -q --lib runtime::sync_box
cargo test -q --lib parser_sync_box_surface
cargo test -q --lib source_to_program_json_v0_rejects_sync_box_until_runtime_rows
rg -n "sync_box_reference_report_fields|sync_box_mir_lowering_enabled|sync_box_program_json_enabled" \
  src/runtime docs/reference/concurrency/semantics.md docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
