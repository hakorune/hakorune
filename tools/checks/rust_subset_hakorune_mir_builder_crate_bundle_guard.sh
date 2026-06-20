#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

APP_DIR="apps/rust-subset-to-hako"
EXAMPLES_DIR="$APP_DIR/examples"
EXPECTED_DIR="$EXAMPLES_DIR/hakorune_mir_builder_crate_expected"
WRAPPER="$APP_DIR/convert_hakorune_mir_builder_crate_file.hako"
EXPECTED_HAKO="$EXAMPLES_DIR/hakorune_mir_builder_crate_expected.hako"
SYN_ADAPTER_MANIFEST="$APP_DIR/tools/syn_adapter/Cargo.toml"

TMP_BASE="${TMPDIR:-/tmp}/rust_subset_hakorune_mir_builder_crate_bundle"
ACTUAL_BUNDLE="${TMP_BASE}_adapter"
WRAPPER_MIR="${TMP_BASE}.wrapper.mir.json"
WRAPPER_EXE="${TMP_BASE}.wrapper"
RAW_OUT="${TMP_BASE}.out.raw"
OUT="${TMP_BASE}.out"
GENERATED_MIR="${TMP_BASE}.generated.mir.json"

bash tools/build_hako_llvmc_ffi.sh >/dev/null

rm -rf "$ACTUAL_BUNDLE"
cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
  --crate-root crates/hakorune_mir_builder \
  --out-dir "$ACTUAL_BUNDLE" \
  --crate-name hakorune_mir_builder \
  --target-kind lib \
  --target-name hakorune_mir_builder

diff -u "$EXPECTED_DIR/crate-manifest.json" "$ACTUAL_BUNDLE/crate-manifest.json"
for i in 0 1 2 3 4 5 6; do
  file="$(printf "%04d.json" "$i")"
  diff -u "$EXPECTED_DIR/modules/$file" "$ACTUAL_BUNDLE/modules/$file"
done

NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json "$WRAPPER_MIR" "$WRAPPER" \
  >"${TMP_BASE}.wrapper.emit.log" 2>&1

rm -f "$WRAPPER_EXE"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe "$WRAPPER_EXE" "$WRAPPER" \
  >"${TMP_BASE}.wrapper.exe.log" 2>&1

"$WRAPPER_EXE" >"$RAW_OUT" 2>"${TMP_BASE}.wrapper.err"
sed '/^Result: /d' "$RAW_OUT" >"$OUT"
diff -u "$EXPECTED_HAKO" "$OUT"

NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json "$GENERATED_MIR" "$OUT" \
  >"${TMP_BASE}.generated.emit.log" 2>&1

cat <<'REPORT'
output_contract=rust-subset-hakorune-mir-builder-crate-bundle-v0
adapter_crate_mode_bundle_golden=green
manifest_bundle_checked_in=1
module_count=7
wrapper_mir_emit=green
wrapper_exe_parity=green
aggregate_text_mir_emit=green
generated_program_execution_claim=0
cross_module_linking_claim=0
combined_namespace_semantics_claim=0
summary=ok
REPORT
