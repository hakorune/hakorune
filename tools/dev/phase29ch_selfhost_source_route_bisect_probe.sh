#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
STAGE1_BIN="${STAGE1_BIN:-target/selfhost/hakorune.stage1_cli}"
STAGE2_BIN="${STAGE2_BIN:-target/selfhost/hakorune.stage1_cli.stage2}"

for bin in "$STAGE1_BIN" "$STAGE2_BIN"; do
  if [[ ! -x "$bin" ]]; then
    echo "[FAIL] missing selfhost bin: $bin" >&2
    exit 2
  fi
done

tmp_dir="$(mktemp -d)"
cleanup() {
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

build_case_source() {
  local case_name="$1"
  local out="$2"
  python3 - "$case_name" "$out" <<'PY'
import pathlib
import sys

case_name = sys.argv[1]
out = pathlib.Path(sys.argv[2])

COMMON_TOP = """using lang.mir.builder.MirBuilderBox as MirBuilderBox
using selfhost.shared.common.box_type_inspector as BoxTypeInspectorBox

static box Main {
  main() {
    local source_text = "static box Main { main() { print(42) return 0 } }"
    local mir = MirBuilderBox.emit_from_source_v0(source_text, null)
    local mir_kind = BoxTypeInspectorBox.kind(mir)
    local mir_desc = BoxTypeInspectorBox.describe(mir)
    local mir_text = "" + mir
"""

COMMON_END = """
    print("K=" + mir_kind)
    print("D=" + mir_desc)
    print("T=" + mir_text)
    return 0
  }
}
"""

cases = {
    "base": """
    if mir == null { print("MIR_NULL") } else { print("MIR_NONNULL") }
    if mir == "" { print("MIR_EMPTY") } else { print("MIR_NONEMPTY") }
    if mir_text == "" { print("TEXT_EMPTY") } else { print("TEXT_NONEMPTY") }
    if mir_text.length() > 0 { print("LEN_POS") } else { print("LEN_NONPOS") }
    if mir_text.substring(0, 1) == "{" { print("HEAD_OK") } else { print("HEAD_BAD") }
    if mir_text.indexOf("functions") >= 0 { print("IDX_OK") } else { print("IDX_BAD") }
""",
    "len_sign": """
    local mir_text_len = mir_text.length()
    local mir_text_len_sign = "zero"
    if mir_text_len < 0 {
      mir_text_len_sign = "neg"
    } else if mir_text_len > 0 {
      mir_text_len_sign = "pos"
    }
    print("LS=" + mir_text_len_sign)
    if mir == null { print("MIR_NULL") } else { print("MIR_NONNULL") }
    if mir == "" { print("MIR_EMPTY") } else { print("MIR_NONEMPTY") }
    if mir_text_len <= 0 { print("TEXT_EMPTY") } else { print("TEXT_NONEMPTY") }
    if mir_text.substring(0, 1) == "{" { print("HEAD_OK") } else { print("HEAD_BAD") }
    if mir_text.indexOf("functions") >= 0 { print("IDX_OK") } else { print("IDX_BAD") }
""",
    "debug_gate": """
    local stage1_debug = "1"
    local stage1_debug_on = stage1_debug != null && ("" + stage1_debug) == "1"
    local mir_text_len = mir_text.length()
    local mir_text_head = mir_text.substring(0, 1)
    local mir_text_has_functions = mir_text.indexOf("functions")
    if stage1_debug_on {
      print("DBG=ON")
      if mir_text_len < 0 {
        print("NEG")
      } else if mir_text_len == 0 {
        print("ZERO")
      } else {
        print("POS")
      }
      if mir == "" {
        print("DE=true")
      } else {
        print("DE=false")
      }
      if mir_text == "" {
        print("TE=true")
      } else {
        print("TE=false")
      }
      if mir_text_head == "{" {
        print("HB=true")
      } else {
        print("HB=false")
      }
      if mir_text_has_functions >= 0 {
        print("HF=true")
      } else {
        print("HF=false")
      }
    }
    if mir == null { print("MIR_NULL") } else { print("MIR_NONNULL") }
    if mir == "" { print("MIR_EMPTY") } else { print("MIR_NONEMPTY") }
    if mir_text == "" { print("TEXT_EMPTY") } else { print("TEXT_NONEMPTY") }
    if mir_text.length() > 0 { print("LEN_POS") } else { print("LEN_NONPOS") }
    if mir_text_head == "{" { print("HEAD_OK") } else { print("HEAD_BAD") }
    if mir_text_has_functions >= 0 { print("IDX_OK") } else { print("IDX_BAD") }
""",
    "full_guard": """
    local stage1_debug = "1"
    local stage1_debug_on = stage1_debug != null && ("" + stage1_debug) == "1"
    local mir_text_kind = BoxTypeInspectorBox.kind(mir_text)
    local mir_text_len = mir_text.length()
    local mir_text_len_sign = "zero"
    if mir_text_len < 0 {
      mir_text_len_sign = "neg"
    } else if mir_text_len > 0 {
      mir_text_len_sign = "pos"
    }
    local mir_is_null = 0
    if mir == null { mir_is_null = 1 }
    local mir_is_empty = 0
    if mir == "" { mir_is_empty = 1 }
    local mir_text_is_empty = 0
    if mir_text_len <= 0 { mir_text_is_empty = 1 }
    local mir_text_head = mir_text.substring(0, 1)
    local mir_text_head_is_lbrace = 0
    if mir_text_head == "{" { mir_text_head_is_lbrace = 1 }
    local mir_text_has_functions = mir_text.indexOf("functions")
    local mir_text_has_functions_flag = 0
    if mir_text_has_functions >= 0 { mir_text_has_functions_flag = 1 }
    if stage1_debug_on {
      print("TK=" + ("" + mir_text_kind))
      print("LS=" + mir_text_len_sign)
      print("MN=" + ("" + mir_is_null))
      print("ME=" + ("" + mir_is_empty))
      print("TE=" + ("" + mir_text_is_empty))
      print("HB=" + ("" + mir_text_head_is_lbrace))
      print("HF=" + ("" + mir_text_has_functions_flag))
      if mir_is_null == 1 { print("NULL_MATCH") }
      if mir_is_empty == 1 { print("EMPTY_MATCH") }
      if mir_text_is_empty == 1 { print("TEXT_EMPTY_MATCH") }
    }
    local mir_head = mir_text.substring(0, 1)
    if mir_head != "{" {
      print("FREEZE_HEAD")
      print(mir_text)
      return 96
    }
    local has_functions = mir_text.indexOf("functions")
    if has_functions < 0 {
      print("FREEZE_FUNCS")
      return 96
    }
    if mir == null { print("MIR_NULL") } else { print("MIR_NONNULL") }
    if mir == "" { print("MIR_EMPTY") } else { print("MIR_NONEMPTY") }
    if mir_text == "" { print("TEXT_EMPTY") } else { print("TEXT_NONEMPTY") }
    if mir_text.length() > 0 { print("LEN_POS") } else { print("LEN_NONPOS") }
    if mir_head == "{" { print("HEAD_OK") } else { print("HEAD_BAD") }
    if has_functions >= 0 { print("IDX_OK") } else { print("IDX_BAD") }
""",
}

body = cases.get(case_name)
if body is None:
    raise SystemExit(f"unknown case: {case_name}")
out.write_text(COMMON_TOP + body + COMMON_END, encoding="utf-8")
PY
}

run_case() {
  local case_name="$1"
  local src="$tmp_dir/${case_name}.hako"
  local stage1_mir="$tmp_dir/${case_name}.stage1.mir.json"
  local stage2_mir="$tmp_dir/${case_name}.stage2.mir.json"
  local stage1_exe="$tmp_dir/${case_name}.stage1.exe"
  local stage2_exe="$tmp_dir/${case_name}.stage2.exe"
  build_case_source "$case_name" "$src"
  bash "$ROOT/tools/selfhost/run_stage1_cli.sh" --bin "$STAGE1_BIN" emit mir-json "$src" >"$stage1_mir"
  bash "$ROOT/tools/selfhost/run_stage1_cli.sh" --bin "$STAGE2_BIN" emit mir-json "$src" >"$stage2_mir"
  if diff -q "$stage1_mir" "$stage2_mir" >/dev/null; then
    local mir_status="exact"
  else
    local mir_status="diff"
  fi
  bash "$ROOT/tools/ny_mir_builder.sh" --in "$stage1_mir" --emit exe -o "$stage1_exe" >/dev/null
  bash "$ROOT/tools/ny_mir_builder.sh" --in "$stage2_mir" --emit exe -o "$stage2_exe" >/dev/null
  local stage1_out="$tmp_dir/${case_name}.stage1.out"
  local stage2_out="$tmp_dir/${case_name}.stage2.out"
  NYASH_NYRT_SILENT_RESULT=1 NYASH_DISABLE_PLUGINS=1 NYASH_FILEBOX_MODE=core-ro "$stage1_exe" >"$stage1_out" 2>&1
  NYASH_NYRT_SILENT_RESULT=1 NYASH_DISABLE_PLUGINS=1 NYASH_FILEBOX_MODE=core-ro "$stage2_exe" >"$stage2_out" 2>&1
  local stage1_flags
  local stage2_flags
  stage1_flags="$(grep -E '^(MIR_|TEXT_|LEN_|HEAD_|IDX_|DBG=|NEG|ZERO|POS|DE=|TE=|HB=|HF=|TK=|LS=|MN=|ME=|FREEZE_)' "$stage1_out" | tr '\n' ';')"
  stage2_flags="$(grep -E '^(MIR_|TEXT_|LEN_|HEAD_|IDX_|DBG=|NEG|ZERO|POS|DE=|TE=|HB=|HF=|TK=|LS=|MN=|ME=|FREEZE_)' "$stage2_out" | tr '\n' ';')"
  echo "[selfhost-source-route-bisect] case=${case_name} mir=${mir_status}"
  echo "[selfhost-source-route-bisect] stage1=${stage1_flags}"
  echo "[selfhost-source-route-bisect] stage2=${stage2_flags}"
}

for case_name in base len_sign debug_gate full_guard; do
  run_case "$case_name"
done
