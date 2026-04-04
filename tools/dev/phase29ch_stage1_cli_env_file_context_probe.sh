#!/usr/bin/env bash
set -euo pipefail

ROOT="$(git rev-parse --show-toplevel)"
STAGE1_BIN="${STAGE1_BIN:-target/selfhost/hakorune.stage1_cli}"
STAGE2_BIN="${STAGE2_BIN:-target/selfhost/hakorune.stage1_cli.stage2}"
ENTRY_SOURCE="${ENTRY_SOURCE:-apps/tests/hello_simple_llvm.hako}"

for bin in "$STAGE1_BIN" "$STAGE2_BIN"; do
  if [[ ! -x "$bin" ]]; then
    echo "[FAIL] missing selfhost bin: $bin" >&2
    exit 2
  fi
done
if [[ ! -f "$ENTRY_SOURCE" ]]; then
  echo "[FAIL] missing entry source: $ENTRY_SOURCE" >&2
  exit 2
fi

tmp_dir="$(mktemp -d)"
cleanup() {
  if [[ "${KEEP_TMP:-0}" == "1" ]]; then
    echo "[stage1-cli-env-context] keep-tmp=$tmp_dir"
    return
  fi
  rm -rf "$tmp_dir"
}
trap cleanup EXIT

build_case_clone() {
  local case_name="$1"
  local out="$2"
  python3 - "$case_name" "$out" <<'PY'
from pathlib import Path
import sys

case_name = sys.argv[1]
out = Path(sys.argv[2])

if case_name == "env_source_only":
    out.write_text("""// env source-only probe
using lang.mir.builder.MirBuilderBox as MirBuilderBox
using selfhost.shared.common.box_type_inspector as BoxTypeInspectorBox
using sh_core as StringHelpers

static box Main {
  main() {
    local stage1_debug = env.get("STAGE1_CLI_DEBUG")
    local stage1_debug_on = stage1_debug != null && ("" + stage1_debug) == "1"
    local source_text = me._clean_env_value(env.get("STAGE1_SOURCE_TEXT"))
    local selected_input = source_text
    local mir = MirBuilderBox.emit_from_source_v0(source_text, null)
    if stage1_debug_on {
      print("[stage1-cli/debug] emit-mir selected_input.len=" + me._debug_len_inline(selected_input) + " preview=" + me._debug_preview_inline(selected_input))
    }
    local mir_kind = BoxTypeInspectorBox.kind(mir)
    local mir_desc = BoxTypeInspectorBox.describe(mir)
    local mir_text = "" + mir
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
      print("[stage1-cli/debug] emit-mir result.kind=" + ("" + mir_kind) +
            " is_null=" + ("" + mir_is_null) +
            " is_empty=" + ("" + mir_is_empty) +
            " text_kind=" + ("" + mir_text_kind) +
            " text_is_empty=" + ("" + mir_text_is_empty) +
            " text_len_sign=" + mir_text_len_sign +
            " text_len=" + me._debug_len_inline(mir_text) +
            " text_head_preview=" + me._debug_preview_inline(mir_text_head) +
            " text_head_is_lbrace=" + ("" + mir_text_head_is_lbrace) +
            " text_has_functions=" + ("" + mir_text_has_functions_flag) +
            " desc=" + me._debug_preview_inline(mir_desc) +
            " preview=" + me._debug_preview_inline(mir) +
            " text_preview=" + me._debug_preview_inline(mir_text))
      if mir_text_len < 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=neg")
      } else if mir_text_len == 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=zero")
      } else {
        print("[stage1-cli/debug] emit-mir text-len-sign=pos")
      }
      if mir == "" {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=false")
      }
      if mir_text == "" {
        print("[stage1-cli/debug] emit-mir text-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-empty-compare=false")
      }
      if mir_text_head == "{" {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=false")
      }
      if mir_text_has_functions >= 0 {
        print("[stage1-cli/debug] emit-mir text-has-functions=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-has-functions=false")
      }
      if mir_is_null == 1 {
        print("[stage1-cli/debug] emit-mir current-route null compare matched; ignoring and using structural JSON validation")
      }
      if mir_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route direct compare matched empty string; ignoring and using structural JSON validation")
      }
      if mir_text_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route materialized empty string; ignoring and using structural JSON validation")
      }
      print("[stage1-cli/debug] emit-mir current-route validating materialized payload")
    }
    mir = mir_text
    local mir_head = mir.substring(0, 1)
    if mir_head != "{" {
      print("[freeze:contract][stage1-cli/emit-mir] output is not MIR JSON")
      print(mir)
      return 96
    }
    local has_functions = mir.indexOf("functions")
    if has_functions < 0 { return 96 }
    print(mir)
    return 0
  }

  method _debug_len_inline(value) {
    if value == null { return "null" }
    local s = "" + value
    return StringHelpers.int_to_str(s.length())
  }

  method _debug_preview_inline(value) {
    if value == null { return "<null>" }
    local s = "" + value
    if s.length() <= 64 { return s }
    return s.substring(0, 64)
  }

  method _clean_env_value(v) {
    local s = "" + v
    if s == "null" || s == "Null" || s == "void" || s == "Void" {
      return ""
    }
    return s
  }
}
""")
    raise SystemExit(0)

if case_name == "env_mode_no_supplied":
    out.write_text("""// env mode without supplied-program-json branch
using lang.mir.builder.MirBuilderBox as MirBuilderBox
using selfhost.shared.common.box_type_inspector as BoxTypeInspectorBox
using sh_core as StringHelpers

static box Main {
  main() {
    local mode = me._resolve_mode()
    if mode == "emit-mir" {
      return me._run_emit_mir_mode("[freeze:contract][stage1-cli/emit-mir]")
    }
    return 97
  }

  method _resolve_mode() {
    local explicit = me._clean_env_value(env.get("NYASH_STAGE1_MODE"))
    if explicit == "" {
      explicit = me._clean_env_value(env.get("HAKO_STAGE1_MODE"))
    }
    if explicit != "" { return explicit }
    if me._clean_env_value(env.get("STAGE1_EMIT_MIR_JSON")) == "1" {
      return "emit-mir"
    }
    return ""
  }

  method _run_emit_mir_mode(tag) {
    local stage1_debug = env.get("STAGE1_CLI_DEBUG")
    local stage1_debug_on = stage1_debug != null && ("" + stage1_debug) == "1"
    local source_text = me._resolve_emit_program_source_text()
    local selected_input = source_text
    if source_text == null || source_text == "" {
      print(tag + " input missing source/program-json contract")
      return 96
    }
    local mir = MirBuilderBox.emit_from_source_v0(source_text, null)
    if stage1_debug_on {
      print("[stage1-cli/debug] emit-mir selected_input.len=" + me._debug_len_inline(selected_input) + " preview=" + me._debug_preview_inline(selected_input))
    }
    local mir_kind = BoxTypeInspectorBox.kind(mir)
    local mir_desc = BoxTypeInspectorBox.describe(mir)
    local mir_text = "" + mir
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
      print("[stage1-cli/debug] emit-mir result.kind=" + ("" + mir_kind) +
            " is_null=" + ("" + mir_is_null) +
            " is_empty=" + ("" + mir_is_empty) +
            " text_kind=" + ("" + mir_text_kind) +
            " text_is_empty=" + ("" + mir_text_is_empty) +
            " text_len_sign=" + mir_text_len_sign +
            " text_len=" + me._debug_len_inline(mir_text) +
            " text_head_preview=" + me._debug_preview_inline(mir_text_head) +
            " text_head_is_lbrace=" + ("" + mir_text_head_is_lbrace) +
            " text_has_functions=" + ("" + mir_text_has_functions_flag) +
            " desc=" + me._debug_preview_inline(mir_desc) +
            " preview=" + me._debug_preview_inline(mir) +
            " text_preview=" + me._debug_preview_inline(mir_text))
      if mir_text_len < 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=neg")
      } else if mir_text_len == 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=zero")
      } else {
        print("[stage1-cli/debug] emit-mir text-len-sign=pos")
      }
      if mir == "" {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=false")
      }
      if mir_text == "" {
        print("[stage1-cli/debug] emit-mir text-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-empty-compare=false")
      }
      if mir_text_head == "{" {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=false")
      }
      if mir_text_has_functions >= 0 {
        print("[stage1-cli/debug] emit-mir text-has-functions=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-has-functions=false")
      }
      if mir_is_null == 1 {
        print("[stage1-cli/debug] emit-mir current-route null compare matched; ignoring and using structural JSON validation")
      }
      if mir_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route direct compare matched empty string; ignoring and using structural JSON validation")
      }
      if mir_text_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route materialized empty string; ignoring and using structural JSON validation")
      }
      print("[stage1-cli/debug] emit-mir current-route validating materialized payload")
    }
    mir = mir_text
    local mir_head = mir.substring(0, 1)
    if mir_head != "{" {
      print("[freeze:contract][stage1-cli/emit-mir] output is not MIR JSON")
      print(mir)
      return 96
    }
    local has_functions = mir.indexOf("functions")
    if has_functions < 0 { return 96 }
    print(mir)
    return 0
  }

  method _resolve_emit_program_source_text() {
    local text = me._clean_env_value(env.get("STAGE1_SOURCE_TEXT"))
    if env.get("STAGE1_CLI_DEBUG") == "1" {
      print("[stage1-cli/debug] emit-program resolved-text.len=" + me._debug_len_inline(text) + " preview=" + me._debug_preview_inline(text))
    }
    return text
  }

  method _debug_len_inline(value) {
    if value == null { return "null" }
    local s = "" + value
    return StringHelpers.int_to_str(s.length())
  }

  method _debug_preview_inline(value) {
    if value == null { return "<null>" }
    local s = "" + value
    if s.length() <= 64 { return s }
    return s.substring(0, 64)
  }

  method _clean_env_value(v) {
    local s = "" + v
    if s == "null" || s == "Null" || s == "void" || s == "Void" {
      return ""
    }
    return s
  }
}
""")
    raise SystemExit(0)

if case_name == "mini_env":
    out.write_text("""// mini env-style source-route probe
using lang.mir.builder.MirBuilderBox as MirBuilderBox
using selfhost.shared.common.box_type_inspector as BoxTypeInspectorBox
using sh_core as StringHelpers

static box Main {
  main() {
    local mode = me._resolve_mode()
    if mode == "emit-mir" {
      return me._run_emit_mir_mode("[freeze:contract][stage1-cli/emit-mir]")
    }
    return 97
  }

  method _resolve_mode() {
    local explicit = me._clean_env_value(env.get("NYASH_STAGE1_MODE"))
    if explicit == "" {
      explicit = me._clean_env_value(env.get("HAKO_STAGE1_MODE"))
    }
    if explicit != "" { return explicit }
    if me._clean_env_value(env.get("STAGE1_EMIT_MIR_JSON")) == "1" {
      return "emit-mir"
    }
    return ""
  }

  method _run_emit_mir_mode(tag) {
    local stage1_debug = env.get("STAGE1_CLI_DEBUG")
    local stage1_debug_on = stage1_debug != null && ("" + stage1_debug) == "1"
    local supplied_program_json = me._resolve_supplied_program_json_text(tag)
    local selected_input = supplied_program_json
    local mir = null
    if supplied_program_json != null && supplied_program_json != "" {
      mir = MirBuilderBox.emit_from_program_json_v0(supplied_program_json, null)
    } else {
      local source_text = me._resolve_emit_program_source_text()
      if source_text == null || source_text == "" {
        print(tag + " input missing source/program-json contract")
        return 96
      }
      selected_input = source_text
      mir = MirBuilderBox.emit_from_source_v0(source_text, null)
    }
    if stage1_debug_on {
      print("[stage1-cli/debug] emit-mir selected_input.len=" + me._debug_len_inline(selected_input) + " preview=" + me._debug_preview_inline(selected_input))
    }
    local mir_kind = BoxTypeInspectorBox.kind(mir)
    local mir_desc = BoxTypeInspectorBox.describe(mir)
    local mir_text = "" + mir
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
      print("[stage1-cli/debug] emit-mir result.kind=" + ("" + mir_kind) +
            " is_null=" + ("" + mir_is_null) +
            " is_empty=" + ("" + mir_is_empty) +
            " text_kind=" + ("" + mir_text_kind) +
            " text_is_empty=" + ("" + mir_text_is_empty) +
            " text_len_sign=" + mir_text_len_sign +
            " text_len=" + me._debug_len_inline(mir_text) +
            " text_head_preview=" + me._debug_preview_inline(mir_text_head) +
            " text_head_is_lbrace=" + ("" + mir_text_head_is_lbrace) +
            " text_has_functions=" + ("" + mir_text_has_functions_flag) +
            " desc=" + me._debug_preview_inline(mir_desc) +
            " preview=" + me._debug_preview_inline(mir) +
            " text_preview=" + me._debug_preview_inline(mir_text))
      if mir_text_len < 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=neg")
      } else if mir_text_len == 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=zero")
      } else {
        print("[stage1-cli/debug] emit-mir text-len-sign=pos")
      }
      if mir == "" {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=false")
      }
      if mir_text == "" {
        print("[stage1-cli/debug] emit-mir text-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-empty-compare=false")
      }
      if mir_text_head == "{" {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=false")
        print("[stage1-cli/debug] emit-mir text-head-preview=" + me._debug_preview_inline(mir_text_head))
      }
      if mir_text_has_functions >= 0 {
        print("[stage1-cli/debug] emit-mir text-has-functions=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-has-functions=false")
      }
      if mir_is_null == 1 {
        print("[stage1-cli/debug] emit-mir current-route null compare matched; ignoring and using structural JSON validation")
      }
      if mir_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route direct compare matched empty string; ignoring and using structural JSON validation")
      }
      if mir_text_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route materialized empty string; ignoring and using structural JSON validation")
      }
      print("[stage1-cli/debug] emit-mir current-route validating materialized payload")
    }
    mir = mir_text
    local mir_head = mir.substring(0, 1)
    if mir_head != "{" {
      print("[freeze:contract][stage1-cli/emit-mir] output is not MIR JSON")
      print(mir)
      return 96
    }
    local has_functions = mir.indexOf("functions")
    if has_functions < 0 { return 96 }
    print(mir)
    return 0
  }

  method _resolve_supplied_program_json_text(tag) {
    return me._clean_env_value(env.get("STAGE1_PROGRAM_JSON_TEXT"))
  }

  method _resolve_emit_program_source_text() {
    local text = me._resolve_source_text()
    if env.get("STAGE1_CLI_DEBUG") == "1" {
      print("[stage1-cli/debug] emit-program resolved-text.len=" + me._debug_len_inline(text) + " preview=" + me._debug_preview_inline(text))
    }
    if text != null && text != "" { return text }
    return ""
  }

  method _resolve_source_text() {
    return me._clean_env_value(env.get("STAGE1_SOURCE_TEXT"))
  }

  method _debug_len_inline(value) {
    if value == null { return "null" }
    local s = "" + value
    return StringHelpers.int_to_str(s.length())
  }

  method _debug_preview_inline(value) {
    if value == null { return "<null>" }
    local s = "" + value
    if s.length() <= 64 { return s }
    return s.substring(0, 64)
  }

  method _clean_env_value(v) {
    local s = "" + v
    if s == "null" || s == "Null" || s == "void" || s == "Void" {
      return ""
    }
    return s
  }
}
""")
    raise SystemExit(0)

if case_name == "env_branch_literal_empty":
    out.write_text("""// env branch with literal empty supplied-program-json
using lang.mir.builder.MirBuilderBox as MirBuilderBox
using selfhost.shared.common.box_type_inspector as BoxTypeInspectorBox
using sh_core as StringHelpers

static box Main {
  main() {
    local mode = me._resolve_mode()
    if mode == "emit-mir" {
      return me._run_emit_mir_mode("[freeze:contract][stage1-cli/emit-mir]")
    }
    return 97
  }

  method _resolve_mode() {
    local explicit = me._clean_env_value(env.get("NYASH_STAGE1_MODE"))
    if explicit == "" {
      explicit = me._clean_env_value(env.get("HAKO_STAGE1_MODE"))
    }
    if explicit != "" { return explicit }
    if me._clean_env_value(env.get("STAGE1_EMIT_MIR_JSON")) == "1" {
      return "emit-mir"
    }
    return ""
  }

  method _run_emit_mir_mode(tag) {
    local stage1_debug = env.get("STAGE1_CLI_DEBUG")
    local stage1_debug_on = stage1_debug != null && ("" + stage1_debug) == "1"
    local supplied_program_json = ""
    local selected_input = supplied_program_json
    local mir = null
    if supplied_program_json != null && supplied_program_json != "" {
      mir = MirBuilderBox.emit_from_program_json_v0(supplied_program_json, null)
    } else {
      local source_text = me._resolve_emit_program_source_text()
      if source_text == null || source_text == "" {
        print(tag + " input missing source/program-json contract")
        return 96
      }
      selected_input = source_text
      mir = MirBuilderBox.emit_from_source_v0(source_text, null)
    }
    if stage1_debug_on {
      print("[stage1-cli/debug] emit-mir selected_input.len=" + me._debug_len_inline(selected_input) + " preview=" + me._debug_preview_inline(selected_input))
    }
    local mir_kind = BoxTypeInspectorBox.kind(mir)
    local mir_desc = BoxTypeInspectorBox.describe(mir)
    local mir_text = "" + mir
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
      print("[stage1-cli/debug] emit-mir result.kind=" + ("" + mir_kind) +
            " is_null=" + ("" + mir_is_null) +
            " is_empty=" + ("" + mir_is_empty) +
            " text_kind=" + ("" + mir_text_kind) +
            " text_is_empty=" + ("" + mir_text_is_empty) +
            " text_len_sign=" + mir_text_len_sign +
            " text_len=" + me._debug_len_inline(mir_text) +
            " text_head_preview=" + me._debug_preview_inline(mir_text_head) +
            " text_head_is_lbrace=" + ("" + mir_text_head_is_lbrace) +
            " text_has_functions=" + ("" + mir_text_has_functions_flag) +
            " desc=" + me._debug_preview_inline(mir_desc) +
            " preview=" + me._debug_preview_inline(mir) +
            " text_preview=" + me._debug_preview_inline(mir_text))
      if mir_text_len < 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=neg")
      } else if mir_text_len == 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=zero")
      } else {
        print("[stage1-cli/debug] emit-mir text-len-sign=pos")
      }
      if mir == "" {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=false")
      }
      if mir_text == "" {
        print("[stage1-cli/debug] emit-mir text-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-empty-compare=false")
      }
      if mir_text_head == "{" {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=false")
      }
      if mir_text_has_functions >= 0 {
        print("[stage1-cli/debug] emit-mir text-has-functions=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-has-functions=false")
      }
      if mir_is_null == 1 {
        print("[stage1-cli/debug] emit-mir current-route null compare matched; ignoring and using structural JSON validation")
      }
      if mir_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route direct compare matched empty string; ignoring and using structural JSON validation")
      }
      if mir_text_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route materialized empty string; ignoring and using structural JSON validation")
      }
      print("[stage1-cli/debug] emit-mir current-route validating materialized payload")
    }
    mir = mir_text
    local mir_head = mir.substring(0, 1)
    if mir_head != "{" {
      print("[freeze:contract][stage1-cli/emit-mir] output is not MIR JSON")
      print(mir)
      return 96
    }
    local has_functions = mir.indexOf("functions")
    if has_functions < 0 { return 96 }
    print(mir)
    return 0
  }

  method _resolve_emit_program_source_text() {
    local text = me._clean_env_value(env.get("STAGE1_SOURCE_TEXT"))
    if env.get("STAGE1_CLI_DEBUG") == "1" {
      print("[stage1-cli/debug] emit-program resolved-text.len=" + me._debug_len_inline(text) + " preview=" + me._debug_preview_inline(text))
    }
    return text
  }

  method _debug_len_inline(value) {
    if value == null { return "null" }
    local s = "" + value
    return StringHelpers.int_to_str(s.length())
  }

  method _debug_preview_inline(value) {
    if value == null { return "<null>" }
    local s = "" + value
    if s.length() <= 64 { return s }
    return s.substring(0, 64)
  }

  method _clean_env_value(v) {
    local s = "" + v
    if s == "null" || s == "Null" || s == "void" || s == "Void" {
      return ""
    }
    return s
  }
}
""")
    raise SystemExit(0)

if case_name == "env_branch_helper_empty":
    out.write_text("""// env branch with helper returning empty supplied-program-json
using lang.mir.builder.MirBuilderBox as MirBuilderBox
using selfhost.shared.common.box_type_inspector as BoxTypeInspectorBox
using sh_core as StringHelpers

static box Main {
  main() {
    local mode = me._resolve_mode()
    if mode == "emit-mir" {
      return me._run_emit_mir_mode("[freeze:contract][stage1-cli/emit-mir]")
    }
    return 97
  }

  method _resolve_mode() {
    local explicit = me._clean_env_value(env.get("NYASH_STAGE1_MODE"))
    if explicit == "" {
      explicit = me._clean_env_value(env.get("HAKO_STAGE1_MODE"))
    }
    if explicit != "" { return explicit }
    if me._clean_env_value(env.get("STAGE1_EMIT_MIR_JSON")) == "1" {
      return "emit-mir"
    }
    return ""
  }

  method _run_emit_mir_mode(tag) {
    local stage1_debug = env.get("STAGE1_CLI_DEBUG")
    local stage1_debug_on = stage1_debug != null && ("" + stage1_debug) == "1"
    local supplied_program_json = me._resolve_supplied_program_json_text(tag)
    local selected_input = supplied_program_json
    local mir = null
    if supplied_program_json != null && supplied_program_json != "" {
      mir = MirBuilderBox.emit_from_program_json_v0(supplied_program_json, null)
    } else {
      local source_text = me._resolve_emit_program_source_text()
      if source_text == null || source_text == "" {
        print(tag + " input missing source/program-json contract")
        return 96
      }
      selected_input = source_text
      mir = MirBuilderBox.emit_from_source_v0(source_text, null)
    }
    if stage1_debug_on {
      print("[stage1-cli/debug] emit-mir selected_input.len=" + me._debug_len_inline(selected_input) + " preview=" + me._debug_preview_inline(selected_input))
    }
    local mir_kind = BoxTypeInspectorBox.kind(mir)
    local mir_desc = BoxTypeInspectorBox.describe(mir)
    local mir_text = "" + mir
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
      print("[stage1-cli/debug] emit-mir result.kind=" + ("" + mir_kind) +
            " is_null=" + ("" + mir_is_null) +
            " is_empty=" + ("" + mir_is_empty) +
            " text_kind=" + ("" + mir_text_kind) +
            " text_is_empty=" + ("" + mir_text_is_empty) +
            " text_len_sign=" + mir_text_len_sign +
            " text_len=" + me._debug_len_inline(mir_text) +
            " text_head_preview=" + me._debug_preview_inline(mir_text_head) +
            " text_head_is_lbrace=" + ("" + mir_text_head_is_lbrace) +
            " text_has_functions=" + ("" + mir_text_has_functions_flag) +
            " desc=" + me._debug_preview_inline(mir_desc) +
            " preview=" + me._debug_preview_inline(mir) +
            " text_preview=" + me._debug_preview_inline(mir_text))
      if mir_text_len < 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=neg")
      } else if mir_text_len == 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=zero")
      } else {
        print("[stage1-cli/debug] emit-mir text-len-sign=pos")
      }
      if mir == "" {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=false")
      }
      if mir_text == "" {
        print("[stage1-cli/debug] emit-mir text-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-empty-compare=false")
      }
      if mir_text_head == "{" {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=false")
      }
      if mir_text_has_functions >= 0 {
        print("[stage1-cli/debug] emit-mir text-has-functions=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-has-functions=false")
      }
      if mir_is_null == 1 {
        print("[stage1-cli/debug] emit-mir current-route null compare matched; ignoring and using structural JSON validation")
      }
      if mir_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route direct compare matched empty string; ignoring and using structural JSON validation")
      }
      if mir_text_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route materialized empty string; ignoring and using structural JSON validation")
      }
      print("[stage1-cli/debug] emit-mir current-route validating materialized payload")
    }
    mir = mir_text
    local mir_head = mir.substring(0, 1)
    if mir_head != "{" {
      print("[freeze:contract][stage1-cli/emit-mir] output is not MIR JSON")
      print(mir)
      return 96
    }
    local has_functions = mir.indexOf("functions")
    if has_functions < 0 { return 96 }
    print(mir)
    return 0
  }

  method _resolve_supplied_program_json_text(tag) {
    return ""
  }

  method _resolve_emit_program_source_text() {
    local text = me._clean_env_value(env.get("STAGE1_SOURCE_TEXT"))
    if env.get("STAGE1_CLI_DEBUG") == "1" {
      print("[stage1-cli/debug] emit-program resolved-text.len=" + me._debug_len_inline(text) + " preview=" + me._debug_preview_inline(text))
    }
    return text
  }

  method _debug_len_inline(value) {
    if value == null { return "null" }
    local s = "" + value
    return StringHelpers.int_to_str(s.length())
  }

  method _debug_preview_inline(value) {
    if value == null { return "<null>" }
    local s = "" + value
    if s.length() <= 64 { return s }
    return s.substring(0, 64)
  }

  method _clean_env_value(v) {
    local s = "" + v
    if s == "null" || s == "Null" || s == "void" || s == "Void" {
      return ""
    }
    return s
  }
}
""")
    raise SystemExit(0)

if case_name == "env_branch_helper_env_text":
    out.write_text("""// env branch with helper reading STAGE1_PROGRAM_JSON_TEXT only
using lang.mir.builder.MirBuilderBox as MirBuilderBox
using selfhost.shared.common.box_type_inspector as BoxTypeInspectorBox
using sh_core as StringHelpers

static box Main {
  main() {
    local mode = me._resolve_mode()
    if mode == "emit-mir" {
      return me._run_emit_mir_mode("[freeze:contract][stage1-cli/emit-mir]")
    }
    return 97
  }

  method _resolve_mode() {
    local explicit = me._clean_env_value(env.get("NYASH_STAGE1_MODE"))
    if explicit == "" {
      explicit = me._clean_env_value(env.get("HAKO_STAGE1_MODE"))
    }
    if explicit != "" { return explicit }
    if me._clean_env_value(env.get("STAGE1_EMIT_MIR_JSON")) == "1" {
      return "emit-mir"
    }
    return ""
  }

  method _run_emit_mir_mode(tag) {
    local stage1_debug = env.get("STAGE1_CLI_DEBUG")
    local stage1_debug_on = stage1_debug != null && ("" + stage1_debug) == "1"
    local supplied_program_json = me._resolve_supplied_program_json_text(tag)
    local selected_input = supplied_program_json
    local mir = null
    if supplied_program_json != null && supplied_program_json != "" {
      mir = MirBuilderBox.emit_from_program_json_v0(supplied_program_json, null)
    } else {
      local source_text = me._resolve_emit_program_source_text()
      if source_text == null || source_text == "" {
        print(tag + " input missing source/program-json contract")
        return 96
      }
      selected_input = source_text
      mir = MirBuilderBox.emit_from_source_v0(source_text, null)
    }
    if stage1_debug_on {
      print("[stage1-cli/debug] emit-mir selected_input.len=" + me._debug_len_inline(selected_input) + " preview=" + me._debug_preview_inline(selected_input))
    }
    local mir_kind = BoxTypeInspectorBox.kind(mir)
    local mir_desc = BoxTypeInspectorBox.describe(mir)
    local mir_text = "" + mir
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
      print("[stage1-cli/debug] emit-mir result.kind=" + ("" + mir_kind) +
            " is_null=" + ("" + mir_is_null) +
            " is_empty=" + ("" + mir_is_empty) +
            " text_kind=" + ("" + mir_text_kind) +
            " text_is_empty=" + ("" + mir_text_is_empty) +
            " text_len_sign=" + mir_text_len_sign +
            " text_len=" + me._debug_len_inline(mir_text) +
            " text_head_preview=" + me._debug_preview_inline(mir_text_head) +
            " text_head_is_lbrace=" + ("" + mir_text_head_is_lbrace) +
            " text_has_functions=" + ("" + mir_text_has_functions_flag) +
            " desc=" + me._debug_preview_inline(mir_desc) +
            " preview=" + me._debug_preview_inline(mir) +
            " text_preview=" + me._debug_preview_inline(mir_text))
      if mir_text_len < 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=neg")
      } else if mir_text_len == 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=zero")
      } else {
        print("[stage1-cli/debug] emit-mir text-len-sign=pos")
      }
      if mir == "" {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=false")
      }
      if mir_text == "" {
        print("[stage1-cli/debug] emit-mir text-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-empty-compare=false")
      }
      if mir_text_head == "{" {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=false")
      }
      if mir_text_has_functions >= 0 {
        print("[stage1-cli/debug] emit-mir text-has-functions=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-has-functions=false")
      }
      if mir_is_null == 1 {
        print("[stage1-cli/debug] emit-mir current-route null compare matched; ignoring and using structural JSON validation")
      }
      if mir_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route direct compare matched empty string; ignoring and using structural JSON validation")
      }
      if mir_text_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route materialized empty string; ignoring and using structural JSON validation")
      }
      print("[stage1-cli/debug] emit-mir current-route validating materialized payload")
    }
    mir = mir_text
    local mir_head = mir.substring(0, 1)
    if mir_head != "{" {
      print("[freeze:contract][stage1-cli/emit-mir] output is not MIR JSON")
      print(mir)
      return 96
    }
    local has_functions = mir.indexOf("functions")
    if has_functions < 0 { return 96 }
    print(mir)
    return 0
  }

  method _resolve_supplied_program_json_text(tag) {
    return me._clean_env_value(env.get("STAGE1_PROGRAM_JSON_TEXT"))
  }

  method _resolve_emit_program_source_text() {
    local text = me._clean_env_value(env.get("STAGE1_SOURCE_TEXT"))
    if env.get("STAGE1_CLI_DEBUG") == "1" {
      print("[stage1-cli/debug] emit-program resolved-text.len=" + me._debug_len_inline(text) + " preview=" + me._debug_preview_inline(text))
    }
    return text
  }

  method _debug_len_inline(value) {
    if value == null { return "null" }
    local s = "" + value
    return StringHelpers.int_to_str(s.length())
  }

  method _debug_preview_inline(value) {
    if value == null { return "<null>" }
    local s = "" + value
    if s.length() <= 64 { return s }
    return s.substring(0, 64)
  }

  method _clean_env_value(v) {
    local s = "" + v
    if s == "null" || s == "Null" || s == "void" || s == "Void" {
      return ""
    }
    return s
  }
}
""")
    raise SystemExit(0)

if case_name == "env_branch_select_then_call":
    out.write_text("""// env branch selects input first, then performs one source-route call
using lang.mir.builder.MirBuilderBox as MirBuilderBox
using selfhost.shared.common.box_type_inspector as BoxTypeInspectorBox
using sh_core as StringHelpers

static box Main {
  main() {
    local mode = me._resolve_mode()
    if mode == "emit-mir" {
      return me._run_emit_mir_mode("[freeze:contract][stage1-cli/emit-mir]")
    }
    return 97
  }

  method _resolve_mode() {
    local explicit = me._clean_env_value(env.get("NYASH_STAGE1_MODE"))
    if explicit == "" {
      explicit = me._clean_env_value(env.get("HAKO_STAGE1_MODE"))
    }
    if explicit != "" { return explicit }
    if me._clean_env_value(env.get("STAGE1_EMIT_MIR_JSON")) == "1" {
      return "emit-mir"
    }
    return ""
  }

  method _run_emit_mir_mode(tag) {
    local stage1_debug = env.get("STAGE1_CLI_DEBUG")
    local stage1_debug_on = stage1_debug != null && ("" + stage1_debug) == "1"
    local supplied_program_json = ""
    local selected_input = supplied_program_json
    if supplied_program_json == null || supplied_program_json == "" {
      local source_text = me._resolve_emit_program_source_text()
      if source_text == null || source_text == "" {
        print(tag + " input missing source/program-json contract")
        return 96
      }
      selected_input = source_text
    }
    local mir = MirBuilderBox.emit_from_source_v0(selected_input, null)
    if stage1_debug_on {
      print("[stage1-cli/debug] emit-mir selected_input.len=" + me._debug_len_inline(selected_input) + " preview=" + me._debug_preview_inline(selected_input))
    }
    local mir_kind = BoxTypeInspectorBox.kind(mir)
    local mir_desc = BoxTypeInspectorBox.describe(mir)
    local mir_text = "" + mir
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
      print("[stage1-cli/debug] emit-mir result.kind=" + ("" + mir_kind) +
            " is_null=" + ("" + mir_is_null) +
            " is_empty=" + ("" + mir_is_empty) +
            " text_kind=" + ("" + mir_text_kind) +
            " text_is_empty=" + ("" + mir_text_is_empty) +
            " text_len_sign=" + mir_text_len_sign +
            " text_len=" + me._debug_len_inline(mir_text) +
            " text_head_preview=" + me._debug_preview_inline(mir_text_head) +
            " text_head_is_lbrace=" + ("" + mir_text_head_is_lbrace) +
            " text_has_functions=" + ("" + mir_text_has_functions_flag) +
            " desc=" + me._debug_preview_inline(mir_desc) +
            " preview=" + me._debug_preview_inline(mir) +
            " text_preview=" + me._debug_preview_inline(mir_text))
      if mir_text_len < 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=neg")
      } else if mir_text_len == 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=zero")
      } else {
        print("[stage1-cli/debug] emit-mir text-len-sign=pos")
      }
      if mir == "" {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=false")
      }
      if mir_text == "" {
        print("[stage1-cli/debug] emit-mir text-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-empty-compare=false")
      }
      if mir_text_head == "{" {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=false")
      }
      if mir_text_has_functions >= 0 {
        print("[stage1-cli/debug] emit-mir text-has-functions=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-has-functions=false")
      }
      if mir_is_null == 1 {
        print("[stage1-cli/debug] emit-mir current-route null compare matched; ignoring and using structural JSON validation")
      }
      if mir_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route direct compare matched empty string; ignoring and using structural JSON validation")
      }
      if mir_text_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route materialized empty string; ignoring and using structural JSON validation")
      }
      print("[stage1-cli/debug] emit-mir current-route validating materialized payload")
    }
    mir = mir_text
    local mir_head = mir.substring(0, 1)
    if mir_head != "{" {
      print("[freeze:contract][stage1-cli/emit-mir] output is not MIR JSON")
      print(mir)
      return 96
    }
    local has_functions = mir.indexOf("functions")
    if has_functions < 0 { return 96 }
    print(mir)
    return 0
  }

  method _resolve_emit_program_source_text() {
    local text = me._clean_env_value(env.get("STAGE1_SOURCE_TEXT"))
    if env.get("STAGE1_CLI_DEBUG") == "1" {
      print("[stage1-cli/debug] emit-program resolved-text.len=" + me._debug_len_inline(text) + " preview=" + me._debug_preview_inline(text))
    }
    return text
  }

  method _debug_len_inline(value) {
    if value == null { return "null" }
    local s = "" + value
    return StringHelpers.int_to_str(s.length())
  }

  method _debug_preview_inline(value) {
    if value == null { return "<null>" }
    local s = "" + value
    if s.length() <= 64 { return s }
    return s.substring(0, 64)
  }

  method _clean_env_value(v) {
    local s = "" + v
    if s == "null" || s == "Null" || s == "void" || s == "Void" {
      return ""
    }
    return s
  }
}
""")
    raise SystemExit(0)

if case_name == "env_branch_same_callee_two_calls":
    out.write_text("""// env branch keeps two source-route call sites to the same callee
using lang.mir.builder.MirBuilderBox as MirBuilderBox
using selfhost.shared.common.box_type_inspector as BoxTypeInspectorBox
using sh_core as StringHelpers

static box Main {
  main() {
    local mode = me._resolve_mode()
    if mode == "emit-mir" {
      return me._run_emit_mir_mode("[freeze:contract][stage1-cli/emit-mir]")
    }
    return 97
  }

  method _resolve_mode() {
    local explicit = me._clean_env_value(env.get("NYASH_STAGE1_MODE"))
    if explicit == "" {
      explicit = me._clean_env_value(env.get("HAKO_STAGE1_MODE"))
    }
    if explicit != "" { return explicit }
    if me._clean_env_value(env.get("STAGE1_EMIT_MIR_JSON")) == "1" {
      return "emit-mir"
    }
    return ""
  }

  method _run_emit_mir_mode(tag) {
    local stage1_debug = env.get("STAGE1_CLI_DEBUG")
    local stage1_debug_on = stage1_debug != null && ("" + stage1_debug) == "1"
    local supplied_program_json = ""
    local selected_input = supplied_program_json
    local mir = null
    if supplied_program_json != null && supplied_program_json != "" {
      mir = MirBuilderBox.emit_from_source_v0(supplied_program_json, null)
    } else {
      local source_text = me._resolve_emit_program_source_text()
      if source_text == null || source_text == "" {
        print(tag + " input missing source/program-json contract")
        return 96
      }
      selected_input = source_text
      mir = MirBuilderBox.emit_from_source_v0(source_text, null)
    }
    if stage1_debug_on {
      print("[stage1-cli/debug] emit-mir selected_input.len=" + me._debug_len_inline(selected_input) + " preview=" + me._debug_preview_inline(selected_input))
    }
    local mir_kind = BoxTypeInspectorBox.kind(mir)
    local mir_desc = BoxTypeInspectorBox.describe(mir)
    local mir_text = "" + mir
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
      print("[stage1-cli/debug] emit-mir result.kind=" + ("" + mir_kind) +
            " is_null=" + ("" + mir_is_null) +
            " is_empty=" + ("" + mir_is_empty) +
            " text_kind=" + ("" + mir_text_kind) +
            " text_is_empty=" + ("" + mir_text_is_empty) +
            " text_len_sign=" + mir_text_len_sign +
            " text_len=" + me._debug_len_inline(mir_text) +
            " text_head_preview=" + me._debug_preview_inline(mir_text_head) +
            " text_head_is_lbrace=" + ("" + mir_text_head_is_lbrace) +
            " text_has_functions=" + ("" + mir_text_has_functions_flag) +
            " desc=" + me._debug_preview_inline(mir_desc) +
            " preview=" + me._debug_preview_inline(mir) +
            " text_preview=" + me._debug_preview_inline(mir_text))
      if mir_text_len < 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=neg")
      } else if mir_text_len == 0 {
        print("[stage1-cli/debug] emit-mir text-len-sign=zero")
      } else {
        print("[stage1-cli/debug] emit-mir text-len-sign=pos")
      }
      if mir == "" {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir direct-empty-compare=false")
      }
      if mir_text == "" {
        print("[stage1-cli/debug] emit-mir text-empty-compare=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-empty-compare=false")
      }
      if mir_text_head == "{" {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-head-is-lbrace=false")
      }
      if mir_text_has_functions >= 0 {
        print("[stage1-cli/debug] emit-mir text-has-functions=true")
      } else {
        print("[stage1-cli/debug] emit-mir text-has-functions=false")
      }
      if mir_is_null == 1 {
        print("[stage1-cli/debug] emit-mir current-route null compare matched; ignoring and using structural JSON validation")
      }
      if mir_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route direct compare matched empty string; ignoring and using structural JSON validation")
      }
      if mir_text_is_empty == 1 {
        print("[stage1-cli/debug] emit-mir current-route materialized empty string; ignoring and using structural JSON validation")
      }
      print("[stage1-cli/debug] emit-mir current-route validating materialized payload")
    }
    mir = mir_text
    local mir_head = mir.substring(0, 1)
    if mir_head != "{" {
      print("[freeze:contract][stage1-cli/emit-mir] output is not MIR JSON")
      print(mir)
      return 96
    }
    local has_functions = mir.indexOf("functions")
    if has_functions < 0 { return 96 }
    print(mir)
    return 0
  }

  method _resolve_emit_program_source_text() {
    local text = me._clean_env_value(env.get("STAGE1_SOURCE_TEXT"))
    if env.get("STAGE1_CLI_DEBUG") == "1" {
      print("[stage1-cli/debug] emit-program resolved-text.len=" + me._debug_len_inline(text) + " preview=" + me._debug_preview_inline(text))
    }
    return text
  }

  method _debug_len_inline(value) {
    if value == null { return "null" }
    local s = "" + value
    return StringHelpers.int_to_str(s.length())
  }

  method _debug_preview_inline(value) {
    if value == null { return "<null>" }
    local s = "" + value
    if s.length() <= 64 { return s }
    return s.substring(0, 64)
  }

  method _clean_env_value(v) {
    local s = "" + v
    if s == "null" || s == "Null" || s == "void" || s == "Void" {
      return ""
    }
    return s
  }
}
""")
    raise SystemExit(0)

src = Path("lang/src/runner/stage1_cli_env.hako").read_text()

old = '''  method _run_emit_mir_mode(tag) {
    local stage1_debug = env.get("STAGE1_CLI_DEBUG")
    local stage1_debug_on = stage1_debug != null && ("" + stage1_debug) == "1"
    local src_json = me._resolve_emit_mir_program_json_text(tag)
    if src_json == null { return 96 }
    if stage1_debug_on {
      print("[stage1-cli/debug] emit-mir selected_input.len=" + me._debug_len_inline(src_json) + " preview=" + me._debug_preview_inline(src_json))
    }
    local mir = MirBuilderBox.emit_from_program_json_v0(src_json, null)
'''

new = '''  method _run_emit_mir_mode(tag) {
    local stage1_debug = env.get("STAGE1_CLI_DEBUG")
    local stage1_debug_on = stage1_debug != null && ("" + stage1_debug) == "1"
    local supplied_program_json = me._resolve_supplied_program_json_text(tag)
    local selected_input = supplied_program_json
    local mir = null
    if supplied_program_json != null && supplied_program_json != "" {
      mir = MirBuilderBox.emit_from_program_json_v0(supplied_program_json, null)
    } else {
      local source_text = me._resolve_emit_program_source_text()
      if source_text == null || source_text == "" {
        print(tag + " input missing source/program-json contract")
        return 96
      }
      selected_input = source_text
      mir = MirBuilderBox.emit_from_source_v0(source_text, null)
    }
    if stage1_debug_on {
      print("[stage1-cli/debug] emit-mir selected_input.len=" + me._debug_len_inline(selected_input) + " preview=" + me._debug_preview_inline(selected_input))
    }
'''

src = src.replace(old, new, 1)

remove1 = '''  // Current authority route still passes Program(JSON v0) into MirBuilderBox.
  // Keep the transient/supplied boundary explicit in one helper so the next
  // MIR-direct reduction slice can replace it without widening route authority.
  method _resolve_emit_mir_program_json_text(tag) {
    local supplied = me._resolve_supplied_program_json_text(tag)
    if supplied != null && supplied != "" {
      return supplied
    }
    return me._build_emit_mir_program_json_transient(tag)
  }

'''
remove2 = '''  method _build_emit_mir_program_json_transient(tag) {
    local prog_json = me._build_program_json()
    if prog_json != null && prog_json != "" {
      return prog_json
    }
    print(tag + " input missing source/program-json contract")
    return null
  }

'''
src = src.replace(remove1, "", 1)
src = src.replace(remove2, "", 1)

def replace_method(text: str, name: str, body: str) -> str:
    needle = f"  method {name}("
    start = text.find(needle)
    if start < 0:
        return text
    next_pos = text.find("\n  method ", start + 1)
    box_end = text.rfind("\n}")
    if next_pos < 0:
      next_pos = box_end
    if next_pos < 0:
      raise SystemExit(f"failed to locate end for method {name}")
    header_end = text.find("{", start)
    if header_end < 0:
      raise SystemExit(f"failed to locate header for method {name}")
    header = text[start:header_end + 1]
    return text[:start] + header + "\n" + body + "\n  }\n" + text[next_pos + 1:]

if case_name in {"thin", "thin_imports"}:
    for name, body in [
        ("_build_program_json", '    return ""'),
        ("_should_synthesize_entry_defs_inline", '    return ""'),
        ("_build_main_defs_fragment_inline", '    return ""'),
        ("_build_all_defs_fragment_inline", '    return ""'),
        ("_build_box_defs_array_inline", '    return ""'),
        ("_extract_box_body_inline", '    return ""'),
        ("_lower_method_body_inline", '    return ""'),
        ("_extract_program_body_inline", '    return ""'),
        ("_build_params_json_inline", '    return ""'),
        ("_inject_json_fragment_inline", '    return ""'),
        ("_skip_ws_inline", '    return 0'),
        ("_scan_ident_end_inline", '    return 0'),
        ("_is_ident_char_inline", '    return 0'),
        ("_find_matching_pair_inline", '    return -1'),
        ("_trim_inline", '    return "" + s'),
        ("_find_last_significant_char_inline", '    return -1'),
        ("_build_user_box_decls_json_inline", '    return ""'),
        ("_box_name_from_decl_line_inline", '    return ""'),
        ("_build_user_box_decls_fallback_inline", '    return ""'),
    ]:
        src = replace_method(src, name, body)

if case_name == "thin_imports":
    for raw in [
        'using lang.compiler.build.build_box as BuildBox\n',
        'using lang.compiler.entry.using_resolver_box as Stage1UsingResolverBox\n',
        'using lang.compiler.entry.func_scanner as FuncScannerBox\n',
        'using lang.compiler.entry.stageb.stageb_json_builder_box as StageBJsonBuilderBox\n',
    ]:
        src = src.replace(raw, '', 1)

out.write_text(src)
PY
}

run_case() {
  local case_name="$1"
  local src="$tmp_dir/${case_name}.hako"
  local s1_mir="$tmp_dir/${case_name}.stage1.mir.json"
  local s2_mir="$tmp_dir/${case_name}.stage2.mir.json"
  local s1_exe="$tmp_dir/${case_name}.stage1.exe"
  local s2_exe="$tmp_dir/${case_name}.stage2.exe"
  build_case_clone "$case_name" "$src"
  bash "$ROOT/tools/selfhost/compat/run_stage1_cli.sh" --bin "$STAGE1_BIN" emit mir-json "$src" >"$s1_mir"
  bash "$ROOT/tools/selfhost/compat/run_stage1_cli.sh" --bin "$STAGE2_BIN" emit mir-json "$src" >"$s2_mir"
  bash "$ROOT/tools/ny_mir_builder.sh" --in "$s1_mir" --emit exe -o "$s1_exe" >/dev/null
  bash "$ROOT/tools/ny_mir_builder.sh" --in "$s2_mir" --emit exe -o "$s2_exe" >/dev/null
  local s1_out="$tmp_dir/${case_name}.stage1.out"
  local s2_out="$tmp_dir/${case_name}.stage2.out"
  STAGE1_CLI_DEBUG=1 \
  NYASH_NYRT_SILENT_RESULT=1 \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_FILEBOX_MODE=core-ro \
  HAKO_SELFHOST_NO_DELEGATE=1 \
  HAKO_MIR_BUILDER_DELEGATE=0 \
  NYASH_USE_STAGE1_CLI=1 \
  NYASH_STAGE1_MODE=emit-mir \
  HAKO_STAGE1_MODE=emit-mir \
  STAGE1_EMIT_MIR_JSON=1 \
  HAKO_STAGE1_INPUT="$ENTRY_SOURCE" \
  NYASH_STAGE1_INPUT="$ENTRY_SOURCE" \
  STAGE1_SOURCE="$ENTRY_SOURCE" \
  STAGE1_INPUT="$ENTRY_SOURCE" \
  STAGE1_SOURCE_TEXT="$(cat "$ENTRY_SOURCE")" \
  "$s1_exe" >"$s1_out" 2>&1 || true
  STAGE1_CLI_DEBUG=1 \
  NYASH_NYRT_SILENT_RESULT=1 \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_FILEBOX_MODE=core-ro \
  HAKO_SELFHOST_NO_DELEGATE=1 \
  HAKO_MIR_BUILDER_DELEGATE=0 \
  NYASH_USE_STAGE1_CLI=1 \
  NYASH_STAGE1_MODE=emit-mir \
  HAKO_STAGE1_MODE=emit-mir \
  STAGE1_EMIT_MIR_JSON=1 \
  HAKO_STAGE1_INPUT="$ENTRY_SOURCE" \
  NYASH_STAGE1_INPUT="$ENTRY_SOURCE" \
  STAGE1_SOURCE="$ENTRY_SOURCE" \
  STAGE1_INPUT="$ENTRY_SOURCE" \
  STAGE1_SOURCE_TEXT="$(cat "$ENTRY_SOURCE")" \
  "$s2_exe" >"$s2_out" 2>&1 || true
  echo "[stage1-cli-env-context] case=${case_name}"
  echo "[stage1-cli-env-context] stage1=$(rg -n 'selected_input|current-route|freeze:contract|emit-mir result.kind|text-empty-compare|text-head-is-lbrace|text-has-functions' "$s1_out" | tr '\n' ';')"
  echo "[stage1-cli-env-context] stage2=$(rg -n 'selected_input|current-route|freeze:contract|emit-mir result.kind|text-empty-compare|text-head-is-lbrace|text-has-functions' "$s2_out" | tr '\n' ';')"
}

cases=(
  env_source_only
  env_mode_no_supplied
  env_branch_select_then_call
  env_branch_literal_empty
  env_branch_helper_empty
  env_branch_helper_env_text
  env_branch_same_callee_two_calls
  mini_env
  full
  thin
  thin_imports
)

if [[ -n "${CASE_FILTER:-}" ]]; then
  cases=("$CASE_FILTER")
fi

for case_name in "${cases[@]}"; do
  run_case "$case_name"
done
