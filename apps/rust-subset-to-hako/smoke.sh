#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

APP_DIR="apps/rust-subset-to-hako"
EXAMPLES_DIR="$APP_DIR/examples"
PY_SELFTEST="$APP_DIR/selftest.py"
JSON_PROBE="$APP_DIR/probes/stable/json_probe.hako"
SYN_ADAPTER_MANIFEST="$APP_DIR/tools/syn_adapter/Cargo.toml"

HCFI_BASE_TMP="/tmp/hako_rust_subset"

emit_mir_json() {
  local label="$1"
  local source="$2"
  local out="$3"

  echo "[rust-subset/smoke] emit MIR JSON: $label"
  NYASH_FILEBOX_MODE=core-ro \
    ./target/release/hakorune --emit-mir-json "$out" "$source" \
    >"${out%.json}.emit.log"
}

run_exe_diff() {
  local label="$1"
  local source="$2"
  local expected="$3"
  local tmp_name="$4"

  local exe="${HCFI_BASE_TMP}_${tmp_name}"
  local raw="$exe.out.raw"
  local out="$exe.out"

  echo "[rust-subset/smoke] EXE: $label"
  rm -f "$exe"
  rm -f tmp/nyash_cli_emit.json
  NYASH_FILEBOX_MODE=core-ro \
    ./target/release/hakorune --emit-exe "$exe" "$source" \
    >"$exe.exe.log" 2>&1
  "$exe" >"$raw" 2>"$exe.err"
  sed '/^Result: /d' "$raw" >"$out"
  diff -u "$expected" "$out"
}

run_generated_hako_mir_acceptance() {
  local label="$1"
  local source="$2"
  local tmp_name="$3"

  local exe="${HCFI_BASE_TMP}_${tmp_name}_generator"
  local raw="$exe.out.raw"
  local generated="/tmp/rust_subset_${tmp_name}_generated.hako"
  local mir="/tmp/rust_subset_${tmp_name}_generated.mir.json"

  echo "[rust-subset/smoke] generated Hako MIR: $label"
  rm -f "$exe" "$generated" "$mir"
  rm -f tmp/nyash_cli_emit.json
  NYASH_FILEBOX_MODE=core-ro \
    ./target/release/hakorune --emit-exe "$exe" "$source" \
    >"$exe.exe.log" 2>&1
  "$exe" >"$raw" 2>"$exe.err"
  sed '/^Result: /d' "$raw" >"$generated"
  NYASH_FILEBOX_MODE=core-ro \
    ./target/release/hakorune --emit-mir-json "$mir" "$generated" \
    >"${mir%.json}.emit.log"
}

run_adapter_json_diff() {
  local label="$1"
  local input="$2"
  local module="$3"
  local expected_json="$4"
  local tmp_name="$5"

  local actual="/tmp/rust_subset_syn_${tmp_name}.json"

  echo "[rust-subset/smoke] host adapter: $label"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    "$input" \
    --module "$module" \
    -o "$actual"
  diff -u "$expected_json" "$actual"
}

run_adapter_to_python_hako_diff() {
  local label="$1"
  local input="$2"
  local module="$3"
  local expected_hako="$4"
  local tmp_name="$5"

  local json_out="/tmp/rust_subset_syn_${tmp_name}.json"
  local hako_out="/tmp/rust_subset_syn_${tmp_name}.hako"

  echo "[rust-subset/smoke] host adapter: $label"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    "$input" \
    --module "$module" \
    -o "$json_out"
  python3 "$APP_DIR/convert.py" "$json_out" -o "$hako_out"
  diff -u "$expected_hako" "$hako_out"
}

run_adapter_crate_diff() {
  local label="$1"
  local crate_root="$2"
  local expected_dir="$3"
  local tmp_name="$4"

  local actual="/tmp/rust_subset_syn_${tmp_name}_crate"

  echo "[rust-subset/smoke] host adapter crate: $label"
  rm -rf "$actual"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    --crate-root "$crate_root" \
    --out-dir "$actual" \
    --crate-name mini_crate \
    --target-kind lib \
    --target-name mini_crate
  diff -u "$expected_dir/crate-manifest.json" "$actual/crate-manifest.json"
  diff -u "$expected_dir/modules/0000.json" "$actual/modules/0000.json"
  diff -u "$expected_dir/modules/0001.json" "$actual/modules/0001.json"
  diff -u "$expected_dir/modules/0002.json" "$actual/modules/0002.json"
}

run_simple_semantic_parity() {
  local actual="/tmp/rust_subset_syn_simple.json"

  echo "[rust-subset/smoke] host adapter: simple semantic parity"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    "$EXAMPLES_DIR/simple_input.rs" \
    --module simple \
    -o "$actual"
  python3 - <<'PY'
import json
from pathlib import Path

expected = json.loads(Path("apps/rust-subset-to-hako/examples/simple_subset.json").read_text())
actual = json.loads(Path("/tmp/rust_subset_syn_simple.json").read_text())
if expected != actual:
    raise SystemExit("syn adapter simple_input semantic parity failed")
PY
}

echo "[rust-subset/smoke] python reference selftest"
python3 "$PY_SELFTEST"

echo "[rust-subset/smoke] ensure ny-llvmc FFI"
bash tools/build_hako_llvmc_ffi.sh >/dev/null

if [[ "${RUST_SUBSET_RUN_ADAPTER:-0}" == "1" ]]; then
  run_adapter_json_diff \
    "syn fixture" \
    "$EXAMPLES_DIR/adapter_fixture_input.rs" \
    "adapter_fixture" \
    "$EXAMPLES_DIR/adapter_fixture_subset.json" \
    "adapter_fixture"

  run_simple_semantic_parity

  run_adapter_json_diff \
    "unsupported trait handoff" \
    "$EXAMPLES_DIR/unsupported_trait_input.rs" \
    "unsupported_trait" \
    "$EXAMPLES_DIR/unsupported_trait_subset.json" \
    "unsupported_trait"
  run_adapter_to_python_hako_diff \
    "unsupported trait handoff python parity" \
    "$EXAMPLES_DIR/unsupported_trait_input.rs" \
    "unsupported_trait" \
    "$EXAMPLES_DIR/unsupported_trait_expected.hako" \
    "unsupported_trait"

  ADAPTER_FIXTURES=(
    "if statement fixture|if|if_fixture"
    "assign statement fixture|assign|assign_fixture"
    "while statement fixture|while|while_fixture"
    "vec literal fixture|vec|vec_fixture"
    "else-if fixture|else_if|else_if_fixture"
    "returnless void body fixture|void_body|void_body_fixture"
    "Vec method-call fixture|vec_method|vec_method_fixture"
    "index expression fixture|index|index_fixture"
    "loop-without-break fixture|loop_forever|loop_forever_fixture"
    "break/continue unsupported handoff fixture|break_continue|break_continue_fixture"
    "generic function skeleton fixture|generic_function|generic_function_fixture"
    "match unsupported handoff fixture|match_unsupported|match_unsupported_fixture"
    "explicit unit return fixture|unit_return|unit_return_fixture"
    "for-loop unsupported handoff fixture|for_loop_unsupported|for_loop_unsupported_fixture"
    "path/name normalization fixture|path_name|path_name"
    "tuple struct constructor fixture|tuple_struct_constructor|tuple_struct_constructor_fixture"
    "compound assignment unsupported handoff fixture|compound_assign|compound_assign_fixture"
    "Self-qualified call unsupported handoff fixture|self_qualified_call|self_qualified_call_fixture"
    "enum variant value unsupported handoff fixture|enum_variant_value|enum_variant_value_fixture"
    "Vec::new call unsupported handoff fixture|vec_new_call|vec_new_call_fixture"
  )

  for entry in "${ADAPTER_FIXTURES[@]}"; do
    IFS='|' read -r label stem module <<<"$entry"
    run_adapter_json_diff \
      "$label" \
      "$EXAMPLES_DIR/${stem}_input.rs" \
      "$module" \
      "$EXAMPLES_DIR/${stem}_subset.json" \
      "$stem"
  done

  run_adapter_crate_diff \
    "mini crate manifest handoff" \
    "$EXAMPLES_DIR/mini_crate" \
    "$EXAMPLES_DIR/mini_crate_expected" \
    "mini_crate"
fi

emit_mir_json "json probe" "$JSON_PROBE" "/tmp/hako_json_probe.mir.json"

CONVERTER_FIXTURES=(
  "converter|$APP_DIR/convert.hako|$EXAMPLES_DIR/simple_expected.hako|convert"
  "file converter|$APP_DIR/convert_file.hako|$EXAMPLES_DIR/simple_expected.hako|convert_file"
  "adapter fixture converter|$APP_DIR/convert_adapter_fixture.hako|$EXAMPLES_DIR/adapter_fixture_expected.hako|convert_adapter_fixture"
  "unsupported trait fixture converter|$APP_DIR/convert_unsupported_trait_fixture.hako|$EXAMPLES_DIR/unsupported_trait_expected.hako|convert_unsupported_trait_fixture"
  "if fixture converter|$APP_DIR/convert_if_fixture.hako|$EXAMPLES_DIR/if_expected.hako|convert_if_fixture"
  "assign fixture converter|$APP_DIR/convert_assign_fixture.hako|$EXAMPLES_DIR/assign_expected.hako|convert_assign_fixture"
  "while fixture converter|$APP_DIR/convert_while_fixture.hako|$EXAMPLES_DIR/while_expected.hako|convert_while_fixture"
  "vec fixture converter|$APP_DIR/convert_vec_fixture.hako|$EXAMPLES_DIR/vec_expected.hako|convert_vec_fixture"
  "else-if fixture converter|$APP_DIR/convert_else_if_fixture.hako|$EXAMPLES_DIR/else_if_expected.hako|convert_else_if_fixture"
  "returnless void body fixture converter|$APP_DIR/convert_void_body_fixture.hako|$EXAMPLES_DIR/void_body_expected.hako|convert_void_body_fixture"
  "Vec method-call fixture converter|$APP_DIR/convert_vec_method_fixture.hako|$EXAMPLES_DIR/vec_method_expected.hako|convert_vec_method_fixture"
  "index fixture converter|$APP_DIR/convert_index_fixture.hako|$EXAMPLES_DIR/index_expected.hako|convert_index_fixture"
  "loop-without-break fixture converter|$APP_DIR/convert_loop_forever_fixture.hako|$EXAMPLES_DIR/loop_forever_expected.hako|convert_loop_forever_fixture"
  "break/continue unsupported fixture converter|$APP_DIR/convert_break_continue_fixture.hako|$EXAMPLES_DIR/break_continue_expected.hako|convert_break_continue_fixture"
  "generic function fixture converter|$APP_DIR/convert_generic_function_fixture.hako|$EXAMPLES_DIR/generic_function_expected.hako|convert_generic_function_fixture"
  "match unsupported handoff fixture converter|$APP_DIR/convert_match_unsupported_fixture.hako|$EXAMPLES_DIR/match_unsupported_expected.hako|convert_match_unsupported_fixture"
  "explicit unit return fixture converter|$APP_DIR/convert_unit_return_fixture.hako|$EXAMPLES_DIR/unit_return_expected.hako|convert_unit_return_fixture"
  "for-loop unsupported handoff fixture converter|$APP_DIR/convert_for_loop_unsupported_fixture.hako|$EXAMPLES_DIR/for_loop_unsupported_expected.hako|convert_for_loop_unsupported_fixture"
  "path/name fixture converter|$APP_DIR/convert_path_name_fixture.hako|$EXAMPLES_DIR/path_name_expected.hako|convert_path_name_fixture"
  "tuple struct constructor fixture converter|$APP_DIR/convert_tuple_struct_constructor_fixture.hako|$EXAMPLES_DIR/tuple_struct_constructor_expected.hako|convert_tuple_struct_constructor_fixture"
  "compound assignment fixture converter|$APP_DIR/convert_compound_assign_fixture.hako|$EXAMPLES_DIR/compound_assign_expected.hako|convert_compound_assign_fixture"
  "Self-qualified call fixture converter|$APP_DIR/convert_self_qualified_call_fixture.hako|$EXAMPLES_DIR/self_qualified_call_expected.hako|convert_self_qualified_call_fixture"
  "enum variant value fixture converter|$APP_DIR/convert_enum_variant_value_fixture.hako|$EXAMPLES_DIR/enum_variant_value_expected.hako|convert_enum_variant_value_fixture"
  "Vec::new call fixture converter|$APP_DIR/convert_vec_new_call_fixture.hako|$EXAMPLES_DIR/vec_new_call_expected.hako|convert_vec_new_call_fixture"
  "crate handoff fixture converter|$APP_DIR/convert_crate_file.hako|$EXAMPLES_DIR/mini_crate_expected.hako|convert_crate_file"
  "hakorune_mir_core ID modules fixture converter|$APP_DIR/convert_hakorune_mir_core_id_modules_crate_file.hako|$EXAMPLES_DIR/hakorune_mir_core_id_modules_expected.hako|convert_hakorune_mir_core_id_modules_crate_file"
  "hakorune_mir_core value_kind fixture converter|$APP_DIR/convert_hakorune_mir_core_value_kind_crate_file.hako|$EXAMPLES_DIR/hakorune_mir_core_value_kind_expected.hako|convert_hakorune_mir_core_value_kind_crate_file"
)

for entry in "${CONVERTER_FIXTURES[@]}"; do
  IFS='|' read -r label source _expected tmp_name <<<"$entry"
  emit_mir_json "$label" "$source" "/tmp/rust_subset_${tmp_name}.mir.json"
done

echo "[rust-subset/smoke] EXE: json probe"
rm -f /tmp/hako_json_probe
rm -f tmp/nyash_cli_emit.json
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_json_probe "$JSON_PROBE" \
  >/tmp/hako_json_probe.exe.log 2>&1
/tmp/hako_json_probe >/tmp/hako_json_probe.out 2>/tmp/hako_json_probe.err
grep -Fq "field.kind.value=Program" /tmp/hako_json_probe.out
grep -Fq "items.length=0" /tmp/hako_json_probe.out

for entry in "${CONVERTER_FIXTURES[@]}"; do
  IFS='|' read -r label source expected tmp_name <<<"$entry"
  run_exe_diff "$label parity" "$source" "$expected" "$tmp_name"
done

run_generated_hako_mir_acceptance \
  "crate handoff generated skeleton" \
  "$APP_DIR/convert_crate_file.hako" \
  "mini_crate_handoff"

run_generated_hako_mir_acceptance \
  "hakorune_mir_core ID modules generated skeleton" \
  "$APP_DIR/convert_hakorune_mir_core_id_modules_crate_file.hako" \
  "hakorune_mir_core_id_modules_handoff"

run_generated_hako_mir_acceptance \
  "hakorune_mir_core value_kind generated skeleton" \
  "$APP_DIR/convert_hakorune_mir_core_value_kind_crate_file.hako" \
  "hakorune_mir_core_value_kind_handoff"

if [[ "${RUST_SUBSET_RUN_REGRESSION:-0}" == "1" ]]; then
  echo "[rust-subset/smoke] EXE: regression probes"
  for probe in "$APP_DIR"/probes/regression/*.hako; do
    name="$(basename "$probe" .hako)"
    exe="/tmp/hako_${name}"
    rm -f "$exe"
    rm -f tmp/nyash_cli_emit.json
    NYASH_FILEBOX_MODE=core-ro \
      ./target/release/hakorune --emit-exe "$exe" "$probe" \
      >"/tmp/hako_${name}.exe.log" 2>&1
    "$exe" >"/tmp/hako_${name}.out" 2>"/tmp/hako_${name}.err"
  done
fi

echo "summary=ok"
