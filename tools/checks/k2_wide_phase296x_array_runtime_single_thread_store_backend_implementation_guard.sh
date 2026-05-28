#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
CARD="$ROOT/docs/development/current/main/phases/phase-296x/296x-205-ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-IMPLEMENTATION.md"
PREV="$ROOT/docs/development/current/main/phases/phase-296x/296x-204-ARRAY-RUNTIME-SINGLE-THREAD-STORE-BACKEND-SSOT.md"
BACKEND="$ROOT/crates/nyash_kernel/src/plugin/array_slot_backend.rs"
STORE="$ROOT/crates/nyash_kernel/src/plugin/array_slot_store.rs"
LOAD="$ROOT/crates/nyash_kernel/src/plugin/array_slot_load.rs"
ENV_DOC="$ROOT/docs/reference/environment-variables.md"
TMP_DIR="$(mktemp -d /tmp/hakorune_row205_array_backend.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row205-array-store-backend] missing line in ${file#$ROOT/}: $expected" >&2
    exit 1
  fi
}

require_text() {
  local file="$1"
  local expected="$2"
  if ! grep -Fq "$expected" "$file"; then
    echo "[row205-array-store-backend] missing text in ${file#$ROOT/}: $expected" >&2
    exit 1
  fi
}

require_report_line() {
  local file="$1"
  local expected="$2"
  if ! grep -q "^${expected}$" "$file"; then
    echo "[row205-array-store-backend] missing report line in ${file#$TMP_DIR/}: $expected" >&2
    cat "$file" >&2
    exit 1
  fi
}

require_line "$CARD" "Status: Current"
require_line "$PREV" "Status: Landed"
require_line "$CARD" "output_contract=array-runtime-single-thread-store-backend-v0"
require_line "$CARD" "default_backend=SafeRwLockArrayBox"
require_line "$CARD" "selected_backend=SingleThreadExactArrayStore"
require_line "$CARD" "selection_env=HAKO_ARRAY_SLOT_STORE"
require_line "$CARD" "allowed_values=safe_rwlock|single_thread_exact"
require_line "$CARD" "invalid_backend_fail_fast=1"
require_line "$CARD" "arraybox_public_storage_changed=0"
require_line "$CARD" "exported_abi_unchanged=1"
require_line "$CARD" "default_visible_arraybox_semantics_unchanged=1"
require_line "$CARD" "single_thread_exact_store_i64_path=implemented"
require_line "$CARD" "single_thread_exact_load_i64_path=implemented"
require_line "$CARD" "safe_rwlock_smoke=ok"
require_line "$CARD" "single_thread_exact_smoke=ok"
require_line "$CARD" "invalid_backend_fail_fast=ok"
require_line "$CARD" "winner_claim=0"
require_line "$CARD" "summary=ok"

require_text "$BACKEND" "const ARRAY_SLOT_STORE_ENV: &str = \"HAKO_ARRAY_SLOT_STORE\";"
require_text "$BACKEND" "ArraySlotBackend::SafeRwLock"
require_text "$BACKEND" "ArraySlotBackend::SingleThreadExact"
require_text "$BACKEND" "[freeze:contract][array-slot-store/backend]"
require_text "$BACKEND" "SINGLE_THREAD_I64_SLOTS"
require_text "$STORE" "array_slot_backend::store_i64"
require_text "$LOAD" "array_slot_backend::load_encoded_i64"
require_text "$ENV_DOC" "HAKO_ARRAY_SLOT_STORE={safe_rwlock\\|single_thread_exact}"

(cd "$ROOT" && cargo build --release -p nyash_kernel >/dev/null)

run_smoke() {
  local backend="$1"
  local out="$2"
  HAKO_ARRAY_SLOT_STORE="$backend" HAKO_TYPED_OBJECT_STORE=single_thread_exact \
    bash "$ROOT/tools/allocator/hako_exe_memory_runner.sh" \
      --app "$ROOT/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako" \
      --workload representative-object-lifecycle-small-block-v0 \
      --runtime-config empty \
      --operation-repeat 1 \
      --out "$out" >/dev/null

  require_report_line "$out" "output_contract=hako-exe-memory-evidence-v0"
  require_report_line "$out" "allocation_count=524288"
  require_report_line "$out" "free_count=524288"
  require_report_line "$out" "select_page_single_fast_path_count=524288"
  require_report_line "$out" "select_page_single_fallback_count=0"
  require_report_line "$out" "release_known_page_fast_path_count=524288"
  require_report_line "$out" "release_known_page_fallback_count=0"
  require_report_line "$out" "provider_activation=0"
  require_report_line "$out" "host_replacement=0"
  require_report_line "$out" "hook_installed=0"
  require_report_line "$out" "global_allocator_installed=0"
  require_report_line "$out" "summary=ok"
}

SAFE_OUT="$TMP_DIR/safe_rwlock.out"
EXACT_OUT="$TMP_DIR/single_thread_exact.out"
run_smoke safe_rwlock "$SAFE_OUT"
run_smoke single_thread_exact "$EXACT_OUT"

set +e
HAKO_ARRAY_SLOT_STORE=invalid HAKO_TYPED_OBJECT_STORE=single_thread_exact \
  bash "$ROOT/tools/allocator/hako_exe_memory_runner.sh" \
    --app "$ROOT/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako" \
    --workload representative-object-lifecycle-small-block-v0 \
    --runtime-config empty \
    --operation-repeat 1 \
    --out "$TMP_DIR/invalid.out" >"$TMP_DIR/invalid.stdout" 2>"$TMP_DIR/invalid.stderr"
INVALID_CODE=$?
set -e

if [[ "$INVALID_CODE" -eq 0 ]]; then
  echo "[row205-array-store-backend] invalid backend unexpectedly succeeded" >&2
  cat "$TMP_DIR/invalid.stdout" >&2
  exit 1
fi
require_text "$TMP_DIR/invalid.stderr" "[freeze:contract][array-slot-store/backend] unsupported HAKO_ARRAY_SLOT_STORE=invalid"

echo "[row205-array-store-backend] ok"
