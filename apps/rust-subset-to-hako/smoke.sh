#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
cd "$ROOT_DIR"

PY_SELFTEST="apps/rust-subset-to-hako/selftest.py"
JSON_PROBE="apps/rust-subset-to-hako/probes/stable/json_probe.hako"
CONVERTER="apps/rust-subset-to-hako/convert.hako"
FILE_CONVERTER="apps/rust-subset-to-hako/convert_file.hako"
ADAPTER_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_adapter_fixture.hako"
IF_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_if_fixture.hako"
ASSIGN_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_assign_fixture.hako"
WHILE_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_while_fixture.hako"
VEC_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_vec_fixture.hako"
ELSE_IF_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_else_if_fixture.hako"
VOID_BODY_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_void_body_fixture.hako"
VEC_METHOD_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_vec_method_fixture.hako"
INDEX_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_index_fixture.hako"
LOOP_FOREVER_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_loop_forever_fixture.hako"
BREAK_CONTINUE_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_break_continue_fixture.hako"
GENERIC_FUNCTION_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_generic_function_fixture.hako"
MATCH_UNSUPPORTED_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_match_unsupported_fixture.hako"
UNIT_RETURN_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_unit_return_fixture.hako"
FOR_LOOP_UNSUPPORTED_FIXTURE_CONVERTER="apps/rust-subset-to-hako/convert_for_loop_unsupported_fixture.hako"
SYN_ADAPTER_MANIFEST="apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml"

echo "[rust-subset/smoke] python reference selftest"
python3 "$PY_SELFTEST"

echo "[rust-subset/smoke] ensure ny-llvmc FFI"
bash tools/build_hako_llvmc_ffi.sh >/dev/null

if [[ "${RUST_SUBSET_RUN_ADAPTER:-0}" == "1" ]]; then
  echo "[rust-subset/smoke] host adapter: syn fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/adapter_fixture_input.rs \
    --module adapter_fixture \
    -o /tmp/rust_subset_syn_adapter_fixture.json
  diff -u apps/rust-subset-to-hako/examples/adapter_fixture_subset.json \
    /tmp/rust_subset_syn_adapter_fixture.json

  echo "[rust-subset/smoke] host adapter: simple semantic parity"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/simple_input.rs \
    --module simple \
    -o /tmp/rust_subset_syn_simple.json
  python3 - <<'PY'
import json
from pathlib import Path

expected = json.loads(Path("apps/rust-subset-to-hako/examples/simple_subset.json").read_text())
actual = json.loads(Path("/tmp/rust_subset_syn_simple.json").read_text())
if expected != actual:
    raise SystemExit("syn adapter simple_input semantic parity failed")
PY

  echo "[rust-subset/smoke] host adapter: unsupported trait handoff"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/unsupported_trait_input.rs \
    --module unsupported_trait \
    -o /tmp/rust_subset_syn_unsupported_trait.json
  python3 apps/rust-subset-to-hako/convert.py \
    /tmp/rust_subset_syn_unsupported_trait.json \
    -o /tmp/rust_subset_syn_unsupported_trait.hako
  diff -u apps/rust-subset-to-hako/examples/unsupported_trait_expected.hako \
    /tmp/rust_subset_syn_unsupported_trait.hako

  echo "[rust-subset/smoke] host adapter: if statement fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/if_input.rs \
    --module if_fixture \
    -o /tmp/rust_subset_syn_if_fixture.json
  diff -u apps/rust-subset-to-hako/examples/if_subset.json \
    /tmp/rust_subset_syn_if_fixture.json

  echo "[rust-subset/smoke] host adapter: assign statement fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/assign_input.rs \
    --module assign_fixture \
    -o /tmp/rust_subset_syn_assign_fixture.json
  diff -u apps/rust-subset-to-hako/examples/assign_subset.json \
    /tmp/rust_subset_syn_assign_fixture.json

  echo "[rust-subset/smoke] host adapter: while statement fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/while_input.rs \
    --module while_fixture \
    -o /tmp/rust_subset_syn_while_fixture.json
  diff -u apps/rust-subset-to-hako/examples/while_subset.json \
    /tmp/rust_subset_syn_while_fixture.json

  echo "[rust-subset/smoke] host adapter: vec literal fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/vec_input.rs \
    --module vec_fixture \
    -o /tmp/rust_subset_syn_vec_fixture.json
  diff -u apps/rust-subset-to-hako/examples/vec_subset.json \
    /tmp/rust_subset_syn_vec_fixture.json

  echo "[rust-subset/smoke] host adapter: else-if fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/else_if_input.rs \
    --module else_if_fixture \
    -o /tmp/rust_subset_syn_else_if_fixture.json
  diff -u apps/rust-subset-to-hako/examples/else_if_subset.json \
    /tmp/rust_subset_syn_else_if_fixture.json

  echo "[rust-subset/smoke] host adapter: returnless void body fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/void_body_input.rs \
    --module void_body_fixture \
    -o /tmp/rust_subset_syn_void_body_fixture.json
  diff -u apps/rust-subset-to-hako/examples/void_body_subset.json \
    /tmp/rust_subset_syn_void_body_fixture.json

  echo "[rust-subset/smoke] host adapter: Vec method-call fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/vec_method_input.rs \
    --module vec_method_fixture \
    -o /tmp/rust_subset_syn_vec_method_fixture.json
  diff -u apps/rust-subset-to-hako/examples/vec_method_subset.json \
    /tmp/rust_subset_syn_vec_method_fixture.json

  echo "[rust-subset/smoke] host adapter: index expression fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/index_input.rs \
    --module index_fixture \
    -o /tmp/rust_subset_syn_index_fixture.json
  diff -u apps/rust-subset-to-hako/examples/index_subset.json \
    /tmp/rust_subset_syn_index_fixture.json

  echo "[rust-subset/smoke] host adapter: loop-without-break fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/loop_forever_input.rs \
    --module loop_forever_fixture \
    -o /tmp/rust_subset_syn_loop_forever_fixture.json
  diff -u apps/rust-subset-to-hako/examples/loop_forever_subset.json \
    /tmp/rust_subset_syn_loop_forever_fixture.json

  echo "[rust-subset/smoke] host adapter: break/continue unsupported handoff fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/break_continue_input.rs \
    --module break_continue_fixture \
    -o /tmp/rust_subset_syn_break_continue_fixture.json
  diff -u apps/rust-subset-to-hako/examples/break_continue_subset.json \
    /tmp/rust_subset_syn_break_continue_fixture.json

  echo "[rust-subset/smoke] host adapter: generic function skeleton fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/generic_function_input.rs \
    --module generic_function_fixture \
    -o /tmp/rust_subset_syn_generic_function_fixture.json
  diff -u apps/rust-subset-to-hako/examples/generic_function_subset.json \
    /tmp/rust_subset_syn_generic_function_fixture.json

  echo "[rust-subset/smoke] host adapter: match unsupported handoff fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/match_unsupported_input.rs \
    --module match_unsupported_fixture \
    -o /tmp/rust_subset_syn_match_unsupported_fixture.json
  diff -u apps/rust-subset-to-hako/examples/match_unsupported_subset.json \
    /tmp/rust_subset_syn_match_unsupported_fixture.json

  echo "[rust-subset/smoke] host adapter: explicit unit return fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/unit_return_input.rs \
    --module unit_return_fixture \
    -o /tmp/rust_subset_syn_unit_return_fixture.json
  diff -u apps/rust-subset-to-hako/examples/unit_return_subset.json \
    /tmp/rust_subset_syn_unit_return_fixture.json

  echo "[rust-subset/smoke] host adapter: for-loop unsupported handoff fixture"
  cargo run --manifest-path "$SYN_ADAPTER_MANIFEST" --quiet -- \
    apps/rust-subset-to-hako/examples/for_loop_unsupported_input.rs \
    --module for_loop_unsupported_fixture \
    -o /tmp/rust_subset_syn_for_loop_unsupported_fixture.json
  diff -u apps/rust-subset-to-hako/examples/for_loop_unsupported_subset.json \
    /tmp/rust_subset_syn_for_loop_unsupported_fixture.json
fi

echo "[rust-subset/smoke] emit MIR JSON: json probe"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/hako_json_probe.mir.json "$JSON_PROBE" >/tmp/hako_json_probe.emit.log

echo "[rust-subset/smoke] emit MIR JSON: converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert.mir.json "$CONVERTER" >/tmp/rust_subset_convert.emit.log

echo "[rust-subset/smoke] emit MIR JSON: file converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_file.mir.json "$FILE_CONVERTER" >/tmp/rust_subset_convert_file.emit.log

echo "[rust-subset/smoke] emit MIR JSON: adapter fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_adapter_fixture.mir.json "$ADAPTER_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_adapter_fixture.emit.log

echo "[rust-subset/smoke] emit MIR JSON: if fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_if_fixture.mir.json "$IF_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_if_fixture.emit.log

echo "[rust-subset/smoke] emit MIR JSON: assign fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_assign_fixture.mir.json "$ASSIGN_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_assign_fixture.emit.log

echo "[rust-subset/smoke] emit MIR JSON: while fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_while_fixture.mir.json "$WHILE_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_while_fixture.emit.log

echo "[rust-subset/smoke] emit MIR JSON: vec fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_vec_fixture.mir.json "$VEC_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_vec_fixture.emit.log

echo "[rust-subset/smoke] emit MIR JSON: else-if fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_else_if_fixture.mir.json "$ELSE_IF_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_else_if_fixture.emit.log

echo "[rust-subset/smoke] emit MIR JSON: returnless void body fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_void_body_fixture.mir.json "$VOID_BODY_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_void_body_fixture.emit.log

echo "[rust-subset/smoke] emit MIR JSON: Vec method-call fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_vec_method_fixture.mir.json "$VEC_METHOD_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_vec_method_fixture.emit.log

echo "[rust-subset/smoke] emit MIR JSON: index fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_index_fixture.mir.json "$INDEX_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_index_fixture.emit.log

echo "[rust-subset/smoke] emit MIR JSON: loop-without-break fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_loop_forever_fixture.mir.json "$LOOP_FOREVER_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_loop_forever_fixture.emit.log

echo "[rust-subset/smoke] emit MIR JSON: break/continue unsupported fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_break_continue_fixture.mir.json "$BREAK_CONTINUE_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_break_continue_fixture.emit.log

echo "[rust-subset/smoke] emit MIR JSON: generic function fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_generic_function_fixture.mir.json "$GENERIC_FUNCTION_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_generic_function_fixture.emit.log

echo "[rust-subset/smoke] emit MIR JSON: match unsupported handoff fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_match_unsupported_fixture.mir.json "$MATCH_UNSUPPORTED_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_match_unsupported_fixture.emit.log

echo "[rust-subset/smoke] emit MIR JSON: explicit unit return fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_unit_return_fixture.mir.json "$UNIT_RETURN_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_unit_return_fixture.emit.log

echo "[rust-subset/smoke] emit MIR JSON: for-loop unsupported handoff fixture converter"
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-mir-json /tmp/rust_subset_convert_for_loop_unsupported_fixture.mir.json "$FOR_LOOP_UNSUPPORTED_FIXTURE_CONVERTER" >/tmp/rust_subset_convert_for_loop_unsupported_fixture.emit.log

echo "[rust-subset/smoke] EXE: json probe"
rm -f /tmp/hako_json_probe
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_json_probe "$JSON_PROBE" \
  >/tmp/hako_json_probe.exe.log 2>&1
/tmp/hako_json_probe >/tmp/hako_json_probe.out 2>/tmp/hako_json_probe.err
grep -Fq "field.kind.value=Program" /tmp/hako_json_probe.out
grep -Fq "items.length=0" /tmp/hako_json_probe.out

echo "[rust-subset/smoke] EXE: converter parity"
rm -f /tmp/hako_rust_subset_convert
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert "$CONVERTER" \
  >/tmp/hako_rust_subset_convert.exe.log 2>&1
/tmp/hako_rust_subset_convert \
  >/tmp/hako_rust_subset_convert.out.raw \
  2>/tmp/hako_rust_subset_convert.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert.out.raw \
  >/tmp/hako_rust_subset_convert.out
diff -u apps/rust-subset-to-hako/examples/simple_expected.hako \
  /tmp/hako_rust_subset_convert.out

echo "[rust-subset/smoke] EXE: file converter parity"
rm -f /tmp/hako_rust_subset_convert_file
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_file "$FILE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_file.exe.log 2>&1
/tmp/hako_rust_subset_convert_file \
  >/tmp/hako_rust_subset_convert_file.out.raw \
  2>/tmp/hako_rust_subset_convert_file.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_file.out.raw \
  >/tmp/hako_rust_subset_convert_file.out
diff -u apps/rust-subset-to-hako/examples/simple_expected.hako \
  /tmp/hako_rust_subset_convert_file.out

echo "[rust-subset/smoke] EXE: adapter fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_adapter_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_adapter_fixture "$ADAPTER_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_adapter_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_adapter_fixture \
  >/tmp/hako_rust_subset_convert_adapter_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_adapter_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_adapter_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_adapter_fixture.out
diff -u apps/rust-subset-to-hako/examples/adapter_fixture_expected.hako \
  /tmp/hako_rust_subset_convert_adapter_fixture.out

echo "[rust-subset/smoke] EXE: if fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_if_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_if_fixture "$IF_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_if_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_if_fixture \
  >/tmp/hako_rust_subset_convert_if_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_if_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_if_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_if_fixture.out
diff -u apps/rust-subset-to-hako/examples/if_expected.hako \
  /tmp/hako_rust_subset_convert_if_fixture.out

echo "[rust-subset/smoke] EXE: assign fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_assign_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_assign_fixture "$ASSIGN_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_assign_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_assign_fixture \
  >/tmp/hako_rust_subset_convert_assign_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_assign_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_assign_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_assign_fixture.out
diff -u apps/rust-subset-to-hako/examples/assign_expected.hako \
  /tmp/hako_rust_subset_convert_assign_fixture.out

echo "[rust-subset/smoke] EXE: while fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_while_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_while_fixture "$WHILE_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_while_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_while_fixture \
  >/tmp/hako_rust_subset_convert_while_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_while_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_while_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_while_fixture.out
diff -u apps/rust-subset-to-hako/examples/while_expected.hako \
  /tmp/hako_rust_subset_convert_while_fixture.out

echo "[rust-subset/smoke] EXE: vec fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_vec_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_vec_fixture "$VEC_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_vec_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_vec_fixture \
  >/tmp/hako_rust_subset_convert_vec_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_vec_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_vec_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_vec_fixture.out
diff -u apps/rust-subset-to-hako/examples/vec_expected.hako \
  /tmp/hako_rust_subset_convert_vec_fixture.out

echo "[rust-subset/smoke] EXE: else-if fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_else_if_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_else_if_fixture "$ELSE_IF_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_else_if_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_else_if_fixture \
  >/tmp/hako_rust_subset_convert_else_if_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_else_if_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_else_if_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_else_if_fixture.out
diff -u apps/rust-subset-to-hako/examples/else_if_expected.hako \
  /tmp/hako_rust_subset_convert_else_if_fixture.out

echo "[rust-subset/smoke] EXE: returnless void body fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_void_body_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_void_body_fixture "$VOID_BODY_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_void_body_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_void_body_fixture \
  >/tmp/hako_rust_subset_convert_void_body_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_void_body_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_void_body_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_void_body_fixture.out
diff -u apps/rust-subset-to-hako/examples/void_body_expected.hako \
  /tmp/hako_rust_subset_convert_void_body_fixture.out

echo "[rust-subset/smoke] EXE: Vec method-call fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_vec_method_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_vec_method_fixture "$VEC_METHOD_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_vec_method_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_vec_method_fixture \
  >/tmp/hako_rust_subset_convert_vec_method_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_vec_method_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_vec_method_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_vec_method_fixture.out
diff -u apps/rust-subset-to-hako/examples/vec_method_expected.hako \
  /tmp/hako_rust_subset_convert_vec_method_fixture.out

echo "[rust-subset/smoke] EXE: index fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_index_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_index_fixture "$INDEX_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_index_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_index_fixture \
  >/tmp/hako_rust_subset_convert_index_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_index_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_index_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_index_fixture.out
diff -u apps/rust-subset-to-hako/examples/index_expected.hako \
  /tmp/hako_rust_subset_convert_index_fixture.out

echo "[rust-subset/smoke] EXE: loop-without-break fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_loop_forever_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_loop_forever_fixture "$LOOP_FOREVER_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_loop_forever_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_loop_forever_fixture \
  >/tmp/hako_rust_subset_convert_loop_forever_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_loop_forever_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_loop_forever_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_loop_forever_fixture.out
diff -u apps/rust-subset-to-hako/examples/loop_forever_expected.hako \
  /tmp/hako_rust_subset_convert_loop_forever_fixture.out

echo "[rust-subset/smoke] EXE: break/continue unsupported fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_break_continue_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_break_continue_fixture "$BREAK_CONTINUE_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_break_continue_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_break_continue_fixture \
  >/tmp/hako_rust_subset_convert_break_continue_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_break_continue_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_break_continue_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_break_continue_fixture.out
diff -u apps/rust-subset-to-hako/examples/break_continue_expected.hako \
  /tmp/hako_rust_subset_convert_break_continue_fixture.out

echo "[rust-subset/smoke] EXE: generic function fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_generic_function_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_generic_function_fixture "$GENERIC_FUNCTION_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_generic_function_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_generic_function_fixture \
  >/tmp/hako_rust_subset_convert_generic_function_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_generic_function_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_generic_function_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_generic_function_fixture.out
diff -u apps/rust-subset-to-hako/examples/generic_function_expected.hako \
  /tmp/hako_rust_subset_convert_generic_function_fixture.out

echo "[rust-subset/smoke] EXE: match unsupported handoff fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_match_unsupported_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_match_unsupported_fixture "$MATCH_UNSUPPORTED_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_match_unsupported_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_match_unsupported_fixture \
  >/tmp/hako_rust_subset_convert_match_unsupported_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_match_unsupported_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_match_unsupported_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_match_unsupported_fixture.out
diff -u apps/rust-subset-to-hako/examples/match_unsupported_expected.hako \
  /tmp/hako_rust_subset_convert_match_unsupported_fixture.out

echo "[rust-subset/smoke] EXE: explicit unit return fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_unit_return_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_unit_return_fixture "$UNIT_RETURN_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_unit_return_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_unit_return_fixture \
  >/tmp/hako_rust_subset_convert_unit_return_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_unit_return_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_unit_return_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_unit_return_fixture.out
diff -u apps/rust-subset-to-hako/examples/unit_return_expected.hako \
  /tmp/hako_rust_subset_convert_unit_return_fixture.out

echo "[rust-subset/smoke] EXE: for-loop unsupported handoff fixture converter parity"
rm -f /tmp/hako_rust_subset_convert_for_loop_unsupported_fixture
NYASH_FILEBOX_MODE=core-ro \
  ./target/release/hakorune --emit-exe /tmp/hako_rust_subset_convert_for_loop_unsupported_fixture "$FOR_LOOP_UNSUPPORTED_FIXTURE_CONVERTER" \
  >/tmp/hako_rust_subset_convert_for_loop_unsupported_fixture.exe.log 2>&1
/tmp/hako_rust_subset_convert_for_loop_unsupported_fixture \
  >/tmp/hako_rust_subset_convert_for_loop_unsupported_fixture.out.raw \
  2>/tmp/hako_rust_subset_convert_for_loop_unsupported_fixture.err
sed '/^Result: /d' /tmp/hako_rust_subset_convert_for_loop_unsupported_fixture.out.raw \
  >/tmp/hako_rust_subset_convert_for_loop_unsupported_fixture.out
diff -u apps/rust-subset-to-hako/examples/for_loop_unsupported_expected.hako \
  /tmp/hako_rust_subset_convert_for_loop_unsupported_fixture.out

if [[ "${RUST_SUBSET_RUN_REGRESSION:-0}" == "1" ]]; then
  echo "[rust-subset/smoke] EXE: regression probes"
  for probe in apps/rust-subset-to-hako/probes/regression/*.hako; do
    name="$(basename "$probe" .hako)"
    exe="/tmp/hako_${name}"
    rm -f "$exe"
    NYASH_FILEBOX_MODE=core-ro \
      ./target/release/hakorune --emit-exe "$exe" "$probe" \
      >"/tmp/hako_${name}.exe.log" 2>&1
    "$exe" >"/tmp/hako_${name}.out" 2>"/tmp/hako_${name}.err"
  done
fi

echo "summary=ok"
