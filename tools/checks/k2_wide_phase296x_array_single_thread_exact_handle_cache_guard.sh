#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
DOC="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-346-ARRAY-SINGLE-THREAD-EXACT-HANDLE-CACHE.md"
PREV="$ROOT_DIR/docs/development/current/main/phases/phase-296x/296x-345-POST-DIRECT-SLOT-SUPPORTED-STORAGE-OWNER-REFRESH.md"
SRC="$ROOT_DIR/crates/nyash_kernel/src/plugin/array_slot_backend.rs"
APP="$ROOT_DIR/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
TMP_DIR="$(mktemp -d /tmp/hakorune_row346_array_cache.XXXXXX)"
trap 'rm -rf "$TMP_DIR"' EXIT

REPORT="$TMP_DIR/report.out"

require_line() {
  local file="$1"
  local expected="$2"
  if ! grep -Fqx "$expected" "$file"; then
    echo "[row346-array-handle-cache] missing line in ${file#$ROOT_DIR/}: $expected" >&2
    exit 1
  fi
}

require_pattern() {
  local file="$1"
  local pattern="$2"
  if ! grep -Fq "$pattern" "$file"; then
    echo "[row346-array-handle-cache] missing pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

forbid_pattern() {
  local file="$1"
  local pattern="$2"
  if grep -Fq "$pattern" "$file"; then
    echo "[row346-array-handle-cache] forbidden pattern in ${file#$ROOT_DIR/}: $pattern" >&2
    exit 1
  fi
}

require_positive_key() {
  local file="$1"
  local key="$2"
  if ! awk -F= -v key="$key" '$1 == key { found=1; exit !($2 + 0 > 0) } END { if (!found) exit 1 }' "$file"; then
    echo "[row346-array-handle-cache] expected positive ${key} in ${file#$ROOT_DIR/}" >&2
    exit 1
  fi
}

require_line "$DOC" "Status: Landed"
require_line "$PREV" "Status: Landed"
require_line "$DOC" "output_contract=array-single-thread-exact-handle-cache-v0"
require_line "$DOC" "input_contract=direct-slot-post-supported-storage-owner-refresh-v0"
require_line "$DOC" "implemented_owner=array_slot_backend_single_thread_exact_handle_cache"
require_line "$DOC" "implemented_owner_file=crates/nyash_kernel/src/plugin/array_slot_backend.rs"
require_line "$DOC" "selected_backend=single_thread_exact"
require_line "$DOC" "hashmap_lookup_removed=1"
require_line "$DOC" "small_handle_entry_cache=1"
require_line "$DOC" "default_backend_semantics_change=0"
require_line "$DOC" "public_arraybox_storage_change=0"
require_line "$DOC" "safe_rwlock_path_preserved=1"
require_line "$DOC" "numeric_i64_slot_semantics_preserved=1"
require_line "$DOC" "append_at_end_semantics_preserved=1"
require_line "$DOC" "oob_semantics_preserved=1"
require_line "$DOC" "invalid_handle_idx_semantics_preserved=1"
require_line "$DOC" "unsupported_storage_failfast_preserved=1"
require_line "$DOC" "mirbuilder_changes_allowed=0"
require_line "$DOC" "hako_source_changes_allowed=0"
require_line "$DOC" "summary=ok"

require_pattern "$SRC" "struct ArraySlotCacheEntry"
require_pattern "$SRC" "handle: i64"
require_pattern "$SRC" "values: Vec<i64>"
require_pattern "$SRC" "RefCell<Vec<ArraySlotCacheEntry>>"
require_pattern "$SRC" "slots.iter().position(|entry| entry.handle == handle)"
require_pattern "$SRC" "initialize_exact_i64_slots(handle)"
forbid_pattern "$SRC" "std::collections::HashMap"
forbid_pattern "$SRC" ".entry(handle)"

cargo test -p nyash_kernel array --lib >/dev/null
cargo build --release --bin hakorune >/dev/null
cargo build --release -p nyash-llvm-compiler --bin ny-llvmc >/dev/null
cargo build --release -p nyash_kernel >/dev/null
bash "$ROOT_DIR/tools/build_hako_llvmc_ffi.sh" >/dev/null

HAKO_ARRAY_SLOT_STORE=single_thread_exact \
HAKO_TYPED_OBJECT_STORE=direct_slot_exact \
NYASH_DISABLE_PLUGINS=1 \
  "$ROOT_DIR/tools/allocator/hako_exe_memory_runner.sh" \
    --app "$APP" \
    --workload representative-object-lifecycle-small-block-v0 \
    --runtime-config empty \
    --operation-repeat 1 \
    --out "$REPORT" >/dev/null

require_line "$REPORT" "output_contract=hako-exe-memory-evidence-v0"
require_line "$REPORT" "workload=representative-object-lifecycle-small-block-v0"
require_line "$REPORT" "runtime_config_profile=empty"
require_line "$REPORT" "result_code=0"
require_line "$REPORT" "operation_repeat=1"
require_line "$REPORT" "allocation_count=524288"
require_line "$REPORT" "free_count=524288"
require_line "$REPORT" "select_page_single_fast_path_count=524288"
require_line "$REPORT" "select_page_single_fallback_count=0"
require_line "$REPORT" "release_known_page_fast_path_count=524288"
require_line "$REPORT" "release_known_page_fallback_count=0"
require_line "$REPORT" "output_summary_ok=1"
require_line "$REPORT" "provider_activation=0"
require_line "$REPORT" "host_replacement=0"
require_line "$REPORT" "hook_installed=0"
require_line "$REPORT" "global_allocator_installed=0"
require_line "$REPORT" "summary=ok"
require_positive_key "$REPORT" "body_elapsed_ns"

cat <<REPORT_TEXT
output_contract=array-single-thread-exact-handle-cache-v0
input_contract=direct-slot-post-supported-storage-owner-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
measurement_scope=exact_exe_object_lifecycle_direct_slot_exact_backend
body_elapsed_ns=$(awk -F= '$1 == "body_elapsed_ns" { print $2 }' "$REPORT")
semantic_proof_summary=ok
provider_activation=0
host_replacement=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
REPORT_TEXT
