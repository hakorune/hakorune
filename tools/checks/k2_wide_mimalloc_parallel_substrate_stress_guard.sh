#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-mimalloc-parallel-substrate-stress"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

CARD="docs/development/current/main/phases/phase-293x/293x-399-MIMAP-PAR-STRESS-001-NATIVE-MULTI-WORKER-STRESS.md"
BOUNDARY="docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md"
SUBSTRATE="docs/reference/runtime/substrate-capabilities.md"
INDEX="docs/tools/check-scripts-index.md"
TEST_FILE="crates/nyash_kernel/src/tests/mimalloc_parallel_substrate.rs"
TESTS_MOD="crates/nyash_kernel/src/tests.rs"
SELF_SCRIPT="tools/checks/k2_wide_mimalloc_parallel_substrate_stress_guard.sh"

echo "[$TAG] running MIMAP-PAR-STRESS-001 native substrate stress guard"

guard_require_files \
  "$TAG" \
  "$CARD" \
  "$BOUNDARY" \
  "$SUBSTRATE" \
  "$INDEX" \
  "$TEST_FILE" \
  "$TESTS_MOD" \
  "$SELF_SCRIPT"
guard_require_exec_files "$TAG" "$SELF_SCRIPT"

guard_expect_in_file "$TAG" 'MIMAP-PAR-STRESS-001' "$CARD" "card must identify the native stress row"
guard_expect_in_file "$TAG" 'native multi-worker substrate stress' "$CARD" "card must describe the stress row"
guard_expect_in_file "$TAG" 'MIMAP-PAR-STRESS-001' "$BOUNDARY" "boundary SSOT must track the row"
guard_expect_in_file "$TAG" 'native multi-worker stress' "$SUBSTRATE" "substrate reference must summarize the stress contract"
guard_expect_in_file "$TAG" "$SELF_SCRIPT" "$INDEX" "check script index must list this guard"
guard_expect_in_file "$TAG" 'mod mimalloc_parallel_substrate;' "$TESTS_MOD" "nyash_kernel test module must register the fixture"

guard_expect_in_file "$TAG" 'mimalloc_parallel_substrate_stress_exercises_native_worker_tls_atomic_and_remote_free' "$TEST_FILE" "stress fixture name missing"
guard_expect_in_file "$TAG" 'std::thread::spawn' "$TEST_FILE" "stress must use native OS threads"
guard_expect_in_file "$TAG" 'hako_worker_current_id_i64' "$TEST_FILE" "stress must exercise worker identity substrate"
guard_expect_in_file "$TAG" 'hako_tls_cache_slot_set_i64' "$TEST_FILE" "stress must write TLS cache slot"
guard_expect_in_file "$TAG" 'hako_tls_cache_slot_get_i64' "$TEST_FILE" "stress must read TLS cache slot"
guard_expect_in_file "$TAG" 'hako_atomic_slot_fetch_add_i64' "$TEST_FILE" "stress must exercise fixed-slot atomic fetch_add"
guard_expect_in_file "$TAG" 'hako_atomic_ptr_cas_ordered' "$TEST_FILE" "stress must exercise pointer CAS remote-free push/pop"
guard_expect_in_file "$TAG" 'hako_mem_alloc' "$TEST_FILE" "stress must allocate through hako_mem"
guard_expect_in_file "$TAG" 'hako_mem_free' "$TEST_FILE" "stress must free through hako_mem"

if rg -n 'worker_local|Channel|task_scope|nowait|await|lock<|#\[global_allocator\]|install_hook|provider[A-Za-z0-9_]*[[:space:]]*\(|hook[A-Za-z0-9_]*[[:space:]]*\(' \
  "$TEST_FILE" >/tmp/"$TAG".forbidden 2>&1; then
  echo "[$TAG] ERROR: native stress fixture leaked user-facing concurrency or provider activation surface" >&2
  cat /tmp/"$TAG".forbidden >&2
  rm -f /tmp/"$TAG".forbidden
  exit 1
fi
rm -f /tmp/"$TAG".forbidden

if rg -n 'mimalloc_parallel_substrate_stress|MIMAP-PAR-STRESS-001' lang/c-abi/shims/*.inc >/tmp/"$TAG".inc 2>&1; then
  echo "[$TAG] ERROR: native stress row leaked into backend .inc matcher surface" >&2
  cat /tmp/"$TAG".inc >&2
  rm -f /tmp/"$TAG".inc
  exit 1
fi
rm -f /tmp/"$TAG".inc

cargo test -q -p nyash_kernel mimalloc_parallel_substrate_stress

echo "[$TAG] ok"
