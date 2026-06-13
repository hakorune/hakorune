# 293x-1002 CONC-CAP-INVENTORY-001 Send / Share / ThreadRoot Gaps

Status: landed-code
Date: 2026-06-13

## Decision

Before routing `.hako` values across runtime workers, Hakorune needs stable
diagnostics for send/share/thread-root gaps.

This row adds inventory vocabulary only. It does not enforce capabilities and
does not move values across threads.

## Inventory Fields

```text
hako_send_candidate_count=0
hako_share_candidate_count=0
hako_thread_root_candidate_count=0
rejected_non_send_count=0
rejected_non_share_count=0
thread_root_required_count=0
cross_worker_value_move_enabled=0
```

Code owner:

```text
src/runtime/thread_capability.rs
```

## Reading

Current Rust-level `Send + Sync` substrate is not the same as language-level
HakoSend / HakoShare authorization.

```text
rust_box_trait_send_sync_bound=substrate
hako_send_share_enforced=0
thread_registry_gc_roots_enabled=0
cross_worker_value_move_enabled=0
```

## Stop Lines

- No behavior change.
- No source-level parallel syntax.
- No capability enforcement.
- No cross-worker `.hako` value movement.
- No default worker-pool activation.
- No ThreadRegistry GC root ownership.

## Next Row

```text
CONC-SYNCBOX-003:
  add VM/reference serialized method-entry behavior for canonical sync box.
```

This should stay independent from raw `lock<T>` promotion and from source-level
worker syntax.

## Evidence

```bash
cargo test -q --lib runtime::thread_capability
rg -n "thread_capability_inventory_report_fields|cross_worker_value_move_enabled" \
  src/runtime docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
