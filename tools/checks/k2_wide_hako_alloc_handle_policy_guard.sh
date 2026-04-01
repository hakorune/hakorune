#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

HOST_HANDLES_FILE="src/runtime/host_handles.rs"
HOST_HANDLES_POLICY_FILE="src/runtime/host_handles_policy.rs"
ENV_FLAGS_FILE="src/config/env/helper_boundary_flags.rs"

echo "[k2-wide-hako-alloc-handle-policy] running handle reuse policy acceptance pack"
echo "[k2-wide-hako-alloc-handle-policy] --- parser/policy/registry acceptance ---"
cargo test -q host_handle_alloc_policy_invalid_value_panics -- --nocapture
cargo test -q host_handles_policy_lifo_reuses_last_dropped_handle -- --nocapture
cargo test -q host_handles_policy_none_disables_reuse -- --nocapture
cargo test -q host_handles_policy_fresh_issue_is_monotonic -- --nocapture
cargo test -q host_handles_registry_lifo_reuses_dropped_handle -- --nocapture
cargo test -q host_handles_registry_none_issues_fresh_handle_after_drop -- --nocapture
cargo test -q host_reverse_call_map_slots -- --nocapture

echo "[k2-wide-hako-alloc-handle-policy] --- kernel/cache acceptance ---"
cargo test -q -p nyash_kernel cache_invalidates_on_drop_epoch_when_handle_is_reused --lib -- --nocapture
cargo test -q -p nyash_kernel invalid_handle_short_circuits_all_routes --lib -- --nocapture
cargo test -q -p nyash_kernel string_indexof_hh_cached_pair_route_roundtrip --lib -- --nocapture

echo "[k2-wide-hako-alloc-handle-policy] --- policy/body route lock ---"
rg -F -q 'host_handles_policy::take_reusable_handle(policy_mode, &mut table.free)' "$HOST_HANDLES_FILE"
rg -F -q 'host_handles_policy::issue_fresh_handle(policy_mode, &mut table.next)' "$HOST_HANDLES_FILE"
rg -F -q 'host_handles_policy::recycle_handle(self.alloc_policy_mode(), &mut table.free, h);' "$HOST_HANDLES_FILE"
rg -F -q 'host_handles_registry_lifo_reuses_dropped_handle' "$HOST_HANDLES_FILE"
rg -F -q 'host_handles_registry_none_issues_fresh_handle_after_drop' "$HOST_HANDLES_FILE"
rg -F -q 'HostHandleAllocPolicyMode::Lifo' "$HOST_HANDLES_POLICY_FILE"
rg -F -q 'HostHandleAllocPolicyMode::None' "$HOST_HANDLES_POLICY_FILE"
rg -F -q 'fn host_handle_alloc_policy_invalid_value_panics()' "$ENV_FLAGS_FILE"

echo "[k2-wide-hako-alloc-handle-policy] ok"
