#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE_DIR="$ROOT/tools/hako_check/tests/fastmem_capability_inventory"
OUT="$(mktemp "${TMPDIR:-/tmp}/hako_alloc_owner_shadow.XXXXXX")"
trap 'rm -f "$OUT"' EXIT

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --report "$FIXTURE_DIR/owner_shadow_report.kv" \
  >"$OUT"

grep -q '^replacement_front_owner_shadow_counters=1$' "$OUT"
grep -q '^alloc_owner_id_capability=1$' "$OUT"
grep -q '^alloc_owner_id_kind=allocator_arena_owner$' "$OUT"
grep -q '^alloc_owner_id_source=benchmark_c_pthread_tls$' "$OUT"
grep -q '^alloc_owner_id_width_bits=64$' "$OUT"
grep -q '^alloc_owner_id_generation_enabled=0$' "$OUT"
grep -q '^worker_id_capability=1$' "$OUT"
grep -q '^worker_id_kind=allocator_arena_owner$' "$OUT"
grep -q '^worker_id_source=benchmark_c_pthread_tls$' "$OUT"

grep -q '^allocator_tls_arena_enabled=1$' "$OUT"
grep -q '^allocator_tls_arena_mode=benchmark_c_tls$' "$OUT"
grep -q '^allocator_tls_arena_init_count=2$' "$OUT"
grep -q '^allocator_tls_arena_live_count=2$' "$OUT"
grep -q '^allocator_tls_arena_peak_count=2$' "$OUT"

grep -q '^page_owner_check_enabled=1$' "$OUT"
grep -q '^page_owner_check_route=page_meta_owner_worker_id$' "$OUT"
grep -q '^page_owner_check_count=1000$' "$OUT"
grep -q '^page_owner_same_count=990$' "$OUT"
grep -q '^page_owner_remote_count=10$' "$OUT"
grep -q '^page_owner_unowned_count=0$' "$OUT"
grep -q '^page_owner_stale_generation_count=0$' "$OUT"
grep -q '^page_owner_invalid_count=0$' "$OUT"
grep -q '^page_owner_count_mismatch=0$' "$OUT"
grep -q '^same_owner_free_local_candidate_count=990$' "$OUT"
grep -q '^same_owner_free_local_push_count=0$' "$OUT"
grep -q '^remote_owner_free_remote_candidate_count=10$' "$OUT"
grep -q '^remote_owner_free_remote_push_count=0$' "$OUT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --report "$FIXTURE_DIR/owner_shadow_report.kv" \
  --format kv \
  >"$OUT"

grep -q '^failure_count=0$' "$OUT"
grep -q '^summary=ok$' "$OUT"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --report "$FIXTURE_DIR/same_owner_route_report.kv" \
  >"$OUT"

grep -q '^replacement_front_owner_shadow_counters=1$' "$OUT"
grep -q '^same_owner_free_local_route_enabled=1$' "$OUT"
grep -q '^replacement_front_same_owner_local_free_route=page_meta_owner_local_free$' "$OUT"
grep -q '^page_owner_check_count=1000$' "$OUT"
grep -q '^page_owner_same_count=1000$' "$OUT"
grep -q '^page_owner_remote_count=0$' "$OUT"
grep -q '^same_owner_free_local_candidate_count=1000$' "$OUT"
grep -q '^same_owner_free_local_push_count=1000$' "$OUT"
grep -q '^same_owner_free_local_fallback_count=0$' "$OUT"
grep -q '^remote_owner_free_remote_push_count=0$' "$OUT"
grep -q '^atomic_remote_head_plan=1$' "$OUT"
grep -q '^atomic_remote_head_route=page_remote_head_cas$' "$OUT"
grep -q '^atomic_remote_head_pilot_enabled=0$' "$OUT"
grep -q '^atomic_remote_head_enabled=0$' "$OUT"
grep -q '^remote_free_memory_order=acq_rel$' "$OUT"
grep -q '^summary=ok$' "$OUT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --report "$FIXTURE_DIR/same_owner_route_report.kv" \
  --format kv \
  >"$OUT"

grep -q '^failure_count=0$' "$OUT"
grep -q '^summary=ok$' "$OUT"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --report "$FIXTURE_DIR/atomic_remote_head_pilot_report.kv" \
  >"$OUT"

grep -q '^replacement_front_owner_shadow_counters=1$' "$OUT"
grep -q '^page_owner_check_count=1000$' "$OUT"
grep -q '^page_owner_same_count=990$' "$OUT"
grep -q '^page_owner_remote_count=10$' "$OUT"
grep -q '^remote_owner_free_remote_candidate_count=10$' "$OUT"
grep -q '^remote_owner_free_remote_push_count=10$' "$OUT"
grep -q '^remote_owner_free_fallback_lock_count=0$' "$OUT"
grep -q '^atomic_remote_head_plan=1$' "$OUT"
grep -q '^atomic_remote_head_route=page_remote_head_cas$' "$OUT"
grep -q '^atomic_remote_head_pilot_enabled=1$' "$OUT"
grep -q '^atomic_remote_head_enabled=1$' "$OUT"
grep -q '^remote_free_push_count=10$' "$OUT"
grep -q '^remote_free_drain_count=10$' "$OUT"
grep -q '^remote_free_cas_retry_count=0$' "$OUT"
grep -q '^remote_free_memory_order=acq_rel$' "$OUT"
grep -q '^replacement_front_cross_thread_free_smoke_ok=1$' "$OUT"
grep -q '^replacement_front_cross_thread_free_arena_registry_overflow_count=0$' "$OUT"
grep -q '^hako_source_thread_support_claim=0$' "$OUT"
grep -q '^summary=ok$' "$OUT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --report "$FIXTURE_DIR/atomic_remote_head_pilot_report.kv" \
  --format kv \
  >"$OUT"

grep -q '^failure_count=0$' "$OUT"
grep -q '^summary=ok$' "$OUT"

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --report "$FIXTURE_DIR/owner_lifecycle_shadow_report.kv" \
  >"$OUT"

grep -q '^replacement_front_owner_shadow_counters=1$' "$OUT"
grep -q '^alloc_owner_id_capability=1$' "$OUT"
grep -q '^alloc_owner_id_generation_enabled=1$' "$OUT"
grep -q '^allocator_owner_lifecycle_state_machine=1$' "$OUT"
grep -q '^allocator_owner_generation_enabled=1$' "$OUT"
grep -q '^allocator_owner_id_kind=arena_owner$' "$OUT"
grep -q '^allocator_owner_id_repr=packed_u64_slot_generation$' "$OUT"
grep -q '^allocator_owner_active_count=2$' "$OUT"
grep -q '^allocator_owner_exiting_flush_count=1$' "$OUT"
grep -q '^allocator_owner_abandoned_count=1$' "$OUT"
grep -q '^allocator_thread_exit_observed_count=1$' "$OUT"
grep -q '^allocator_thread_exit_flush_supported=1$' "$OUT"
grep -q '^allocator_thread_exit_flush_count=1$' "$OUT"
grep -q '^allocator_thread_exit_flush_page_count=2$' "$OUT"
grep -q '^allocator_abandoned_page_count=2$' "$OUT"
grep -q '^allocator_abandoned_live_page_count=1$' "$OUT"
grep -q '^allocator_abandoned_empty_page_count=1$' "$OUT"
grep -q '^allocator_abandoned_remote_candidate_count=1$' "$OUT"
grep -q '^allocator_abandoned_reclaim_success_count=0$' "$OUT"
grep -q '^allocator_abandoned_reclaim_blocked_count=1$' "$OUT"
grep -q '^remote_candidate_unhandled_reclaim_block_count=1$' "$OUT"
grep -q '^page_reclaimed_with_remote_candidates=0$' "$OUT"
grep -q '^remote_free_drain_supported=1$' "$OUT"
grep -q '^summary=ok$' "$OUT"

bash "$ROOT/tools/hako_check.sh" fastmem-check \
  --report "$FIXTURE_DIR/owner_lifecycle_shadow_report.kv" \
  --format kv \
  >"$OUT"

grep -q '^failure_count=0$' "$OUT"
grep -q '^summary=ok$' "$OUT"

echo "[TEST/OK] fastmem_alloc_owner_shadow_counter"
