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

echo "[TEST/OK] fastmem_alloc_owner_shadow_counter"
