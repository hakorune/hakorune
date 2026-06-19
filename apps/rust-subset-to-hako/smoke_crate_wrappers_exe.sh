#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

APP_DIR="apps/rust-subset-to-hako"
TMP_PREFIX="${TMPDIR:-/tmp}/hako_rust_subset_crate_wrapper_exe_$$"

run_wrapper_exe() {
  local label="$1"
  local source="$2"
  local exe="${TMP_PREFIX}_${label}"

  echo "[rust-subset/crate-wrapper-exe] $label"
  rm -f "$exe" "$exe.log"
  NYASH_FILEBOX_MODE=core-ro \
    ./target/release/hakorune --emit-exe "$exe" "$source" \
    >"$exe.log" 2>&1
  test -x "$exe"
}

echo "[rust-subset/crate-wrapper-exe] ensure ny-llvmc FFI"
bash tools/build_hako_llvmc_ffi.sh >/dev/null

run_wrapper_exe \
  "mini_crate" \
  "$APP_DIR/convert_crate_file.hako"

run_wrapper_exe \
  "hakorune_box_core" \
  "$APP_DIR/convert_hakorune_box_core_crate_file.hako"

run_wrapper_exe \
  "hakorune_mir_core_selected" \
  "$APP_DIR/convert_hakorune_mir_core_selected_crate_file.hako"

run_wrapper_exe \
  "hakorune_mir_core_id_modules" \
  "$APP_DIR/convert_hakorune_mir_core_id_modules_crate_file.hako"

run_wrapper_exe \
  "hakorune_mir_core_value_kind" \
  "$APP_DIR/convert_hakorune_mir_core_value_kind_crate_file.hako"

run_wrapper_exe \
  "hakorune_mir_core_effect" \
  "$APP_DIR/convert_hakorune_mir_core_effect_crate_file.hako"

run_wrapper_exe \
  "hakorune_mir_builder_binding_context" \
  "$APP_DIR/convert_hakorune_mir_builder_binding_context_crate_file.hako"

echo "[rust-subset/crate-wrapper-exe] summary=ok"
