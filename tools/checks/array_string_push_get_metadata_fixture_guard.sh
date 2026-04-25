#!/usr/bin/env bash
# Guard: array-string boundary fixtures carry and consume RuntimeDataBox push/get metadata.

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
NY_LLVM_C="$ROOT_DIR/target/release/ny-llvmc"

fixtures=(
  "apps/tests/mir_shape_guard/array_string_indexof_interleaved_branch_min_v1.mir.json"
  "apps/tests/mir_shape_guard/array_string_len_piecewise_concat3_source_only_min_v1.mir.json"
  "apps/tests/mir_shape_guard/array_string_indexof_branch_live_after_get_min_v1.mir.json"
  "apps/tests/mir_shape_guard/array_string_len_live_after_get_min_v1.mir.json"
  "apps/tests/mir_shape_guard/array_string_indexof_interleaved_select_min_v1.mir.json"
  "apps/tests/mir_shape_guard/array_string_indexof_branch_min_v1.mir.json"
  "apps/tests/mir_shape_guard/array_string_indexof_cross_block_select_min_v1.mir.json"
  "apps/tests/mir_shape_guard/array_string_indexof_select_min_v1.mir.json"
  "apps/tests/mir_shape_guard/array_string_len_insert_mid_source_only_min_v1.mir.json"
)

if [ ! -x "$NY_LLVM_C" ]; then
  echo "[array-string-push-get-metadata-fixture-guard] ny-llvmc missing: $NY_LLVM_C" >&2
  exit 2
fi

echo "[array-string-push-get-metadata-fixture-guard] checking fixtures"

for rel in "${fixtures[@]}"; do
  fixture="$ROOT_DIR/$rel"
  if [ ! -f "$fixture" ]; then
    echo "[array-string-push-get-metadata-fixture-guard] fixture missing: $rel" >&2
    exit 1
  fi
  jq -e '
    .functions[0].metadata.generic_method_routes as $routes
    | ($routes | type == "array")
    and any($routes[]; .route_id == "generic_method.push"
      and .box_name == "RuntimeDataBox"
      and .receiver_origin_box == "ArrayBox"
      and .route_kind == "array_append_any"
      and .helper_symbol == "nyash.array.slot_append_hh"
      and .core_method.op == "ArrayPush")
    and any($routes[]; .route_id == "generic_method.get"
      and .box_name == "RuntimeDataBox"
      and .receiver_origin_box == "ArrayBox"
      and .route_kind == "array_slot_load_any"
      and .helper_symbol == "nyash.array.slot_load_hi"
      and .core_method.op == "ArrayGet")
  ' "$fixture" >/dev/null

  log="$(mktemp "/tmp/array_string_push_get_metadata.XXXXXX.log")"
  obj="$(mktemp "/tmp/array_string_push_get_metadata.XXXXXX.o")"
  set +e
  NYASH_LLVM_ROUTE_TRACE=1 \
    NYASH_NY_LLVM_COMPILER=/__missing__/ny-llvmc \
    "$NY_LLVM_C" --in "$fixture" --out "$obj" >"$log" 2>&1
  rc=$?
  set -e

  if [ "$rc" -ne 0 ] && ! grep -Fq "unsupported pure shape for current backend recipe" "$log"; then
    echo "[array-string-push-get-metadata-fixture-guard] unexpected compile failure: $rel rc=$rc" >&2
    tail -n 80 "$log" >&2 || true
    rm -f "$log" "$obj"
    exit 1
  fi
  if ! grep -Fq 'stage=mir_call_method result=seen reason=push' "$log" ||
     ! grep -Fq 'push:1' "$log"; then
    echo "[array-string-push-get-metadata-fixture-guard] missing push metadata route state: $rel" >&2
    tail -n 80 "$log" >&2 || true
    rm -f "$log" "$obj"
    exit 1
  fi
  if ! grep -Fq 'stage=mir_call_method result=seen reason=get' "$log" ||
     ! grep -Fq 'get:1' "$log"; then
    echo "[array-string-push-get-metadata-fixture-guard] missing get metadata route state: $rel" >&2
    tail -n 80 "$log" >&2 || true
    rm -f "$log" "$obj"
    exit 1
  fi
  rm -f "$log" "$obj"
done

echo "[array-string-push-get-metadata-fixture-guard] ok fixtures=${#fixtures[@]}"
