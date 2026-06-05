#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
FIXTURE_DIR="$ROOT/tools/hako_check/tests/fastmem_capability_inventory"
OUT="$(mktemp "${TMPDIR:-/tmp}/hako_alloc_owner_schema.XXXXXX")"
trap 'rm -f "$OUT"' EXIT

bash "$ROOT/tools/hako_check.sh" fastmem-capability-inventory \
  --report "$FIXTURE_DIR/report.kv" \
  >"$OUT"

grep -q '^alloc_owner_id_capability=1$' "$OUT"
grep -q '^alloc_owner_id_kind=allocator_arena_owner$' "$OUT"
grep -q '^alloc_owner_id_source=benchmark_c_pthread_tls$' "$OUT"
grep -q '^alloc_owner_id_width_bits=64$' "$OUT"
grep -q '^alloc_owner_id_generation_enabled=1$' "$OUT"
grep -q '^alloc_owner_id_zero_is_unowned=1$' "$OUT"
grep -q '^alloc_owner_id_escape_count=0$' "$OUT"
grep -q '^worker_id_capability=1$' "$OUT"
grep -q '^worker_id_kind=allocator_arena_owner$' "$OUT"
grep -q '^worker_id_source=benchmark_c_pthread_tls$' "$OUT"
grep -q '^worker_id_equals_os_thread_id_claim=0$' "$OUT"
grep -q '^worker_id_equals_runtime_worker_id_claim=0$' "$OUT"
grep -q '^worker_id_equals_hako_task_id_claim=0$' "$OUT"
grep -q '^worker_id_escape_count=0$' "$OUT"

grep -q '^allocator_tls_arena_enabled=1$' "$OUT"
grep -q '^allocator_tls_arena_mode=benchmark_c_tls$' "$OUT"
grep -q '^allocator_tls_arena_init_count=2$' "$OUT"
grep -q '^allocator_tls_arena_live_count=2$' "$OUT"
grep -q '^allocator_tls_arena_peak_count=2$' "$OUT"
grep -q '^allocator_tls_arena_init_fail_count=0$' "$OUT"
grep -q '^allocator_tls_arena_fallback_count=0$' "$OUT"

grep -q '^page_owner_check_enabled=1$' "$OUT"
grep -q '^page_owner_check_route=page_meta_owner_worker_id$' "$OUT"
grep -q '^page_owner_check_count=8163984$' "$OUT"
grep -q '^page_owner_same_count=8163984$' "$OUT"
grep -q '^page_owner_remote_count=0$' "$OUT"
grep -q '^page_owner_unowned_count=0$' "$OUT"
grep -q '^page_owner_stale_generation_count=0$' "$OUT"
grep -q '^page_owner_invalid_count=0$' "$OUT"
grep -q '^page_owner_count_mismatch=0$' "$OUT"

grep -q '^benchmark_thread_origin=c_pthread$' "$OUT"
grep -q '^hako_source_thread_support_claim=0$' "$OUT"
grep -q '^provider_activation=0$' "$OUT"
grep -q '^hook_installed=0$' "$OUT"
grep -q '^global_allocator_product_claim=0$' "$OUT"
grep -q '^winner_claim=0$' "$OUT"
grep -q '^summary=ok$' "$OUT"

echo "[TEST/OK] fastmem_alloc_owner_schema"
