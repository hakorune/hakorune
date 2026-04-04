#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
STAGE1_BIN="${STAGE1_BIN:-target/selfhost/hakorune.stage1_cli}"
STAGE2_BIN="${STAGE2_BIN:-target/selfhost/hakorune.stage1_cli.stage2}"
ENTRY="${ENTRY:-apps/tests/hello_simple_llvm.hako}"

for bin in "$STAGE1_BIN" "$STAGE2_BIN"; do
  if [[ ! -x "$bin" ]]; then
    echo "[FAIL] missing selfhost bin: $bin" >&2
    exit 2
  fi
done

if [[ ! -f "$ENTRY" ]]; then
  echo "[FAIL] missing entry: $ENTRY" >&2
  exit 2
fi

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

program_json="$tmp_dir/program.json"
helper_src="$tmp_dir/phase29ch_selfhost_program_json_helper_probe.hako"
stage1_mir="$tmp_dir/stage1.mir.json"
stage2_mir="$tmp_dir/stage2.mir.json"
stage1_exe="$tmp_dir/stage1_probe"
stage2_exe="$tmp_dir/stage2_probe"

bash "$ROOT/tools/selfhost/compat/run_stage1_cli.sh" --bin "$STAGE1_BIN" emit program-json "$ENTRY" >"$program_json"

python3 - "$program_json" "$helper_src" <<'PY'
import json
import pathlib
import sys

program_json_path = pathlib.Path(sys.argv[1])
helper_path = pathlib.Path(sys.argv[2])
program_json_text = program_json_path.read_text(encoding="utf-8")
program_literal = json.dumps(program_json_text, ensure_ascii=False)

helper_path.write_text(
    "using lang.mir.builder.MirBuilderBox as MirBuilderBox\n"
    "using selfhost.shared.common.box_type_inspector as BoxTypeInspectorBox\n\n"
    "static box Main {\n"
    "  main() {\n"
    f"    local program_json_text = {program_literal}\n"
    "    local mir = MirBuilderBox.emit_from_program_json_v0(program_json_text, null)\n"
    "    local kind = BoxTypeInspectorBox.kind(mir)\n"
    "    local desc = BoxTypeInspectorBox.describe(mir)\n"
    "    local mir_text = \"\" + mir\n"
    "    if mir == null { print(\"MIR_NULL\") } else { print(\"MIR_NONNULL\") }\n"
    "    if mir == \"\" { print(\"MIR_EMPTY\") } else { print(\"MIR_NONEMPTY\") }\n"
    "    if mir_text == \"\" { print(\"TEXT_EMPTY\") } else { print(\"TEXT_NONEMPTY\") }\n"
    "    if mir_text.length() > 0 { print(\"LEN_POS\") } else { print(\"LEN_NONPOS\") }\n"
    "    if mir_text.substring(0, 1) == \"{\" { print(\"HEAD_OK\") } else { print(\"HEAD_BAD\") }\n"
    "    if mir_text.indexOf(\"functions\") >= 0 { print(\"IDX_OK\") } else { print(\"IDX_BAD\") }\n"
    "    print(\"K=\" + kind)\n"
    "    print(\"D=\" + desc)\n"
    "    print(\"T=\" + mir_text)\n"
    "    return 0\n"
    "  }\n"
    "}\n",
    encoding="utf-8",
)
PY

bash "$ROOT/tools/selfhost/compat/run_stage1_cli.sh" --bin "$STAGE1_BIN" emit mir-json "$helper_src" >"$stage1_mir"
bash "$ROOT/tools/selfhost/compat/run_stage1_cli.sh" --bin "$STAGE2_BIN" emit mir-json "$helper_src" >"$stage2_mir"

if ! diff -q "$stage1_mir" "$stage2_mir" >/dev/null; then
  echo "[FAIL] Stage1/Stage2 helper MIR mismatch" >&2
  diff -u "$stage1_mir" "$stage2_mir" | sed -n '1,120p' >&2 || true
  exit 1
fi

bash "$ROOT/tools/ny_mir_builder.sh" --in "$stage1_mir" --emit exe -o "$stage1_exe" >/dev/null
bash "$ROOT/tools/ny_mir_builder.sh" --in "$stage2_mir" --emit exe -o "$stage2_exe" >/dev/null

expected_flags=$'MIR_NONNULL\nMIR_NONEMPTY\nTEXT_NONEMPTY\nLEN_POS\nHEAD_OK\nIDX_OK'

run_probe_exe() {
  local exe="$1"
  local label="$2"
  local out="$tmp_dir/${label}.out"
  NYASH_NYRT_SILENT_RESULT="${NYASH_NYRT_SILENT_RESULT:-1}" \
  NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  NYASH_FILEBOX_MODE="${NYASH_FILEBOX_MODE:-core-ro}" \
  "$exe" >"$out" 2>&1
  echo "[selfhost-program-json-helper] ${label}=$(sed -n '1,8p' "$out" | tr '\n' ';')"
  if ! diff -u <(printf '%s\n' "$expected_flags") <(grep -E '^(MIR_|TEXT_|LEN_|HEAD_|IDX_)' "$out") >/dev/null; then
    echo "[FAIL] unexpected flags for ${label}" >&2
    sed -n '1,40p' "$out" >&2
    exit 1
  fi
}

run_probe_exe "$stage1_exe" "stage1"
run_probe_exe "$stage2_exe" "stage2"

echo "[selfhost-program-json-helper] stage1_stage2_mir=exact-match"
echo "[selfhost-program-json-helper] result=PASS"
