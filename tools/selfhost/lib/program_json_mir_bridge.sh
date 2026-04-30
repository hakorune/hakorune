#!/usr/bin/env bash
# program_json_mir_bridge.sh — non-raw Program(JSON v0) -> MIR(JSON) shell bridge
#
# Purpose:
# - Own shell-side Program(JSON v0) -> MIR(JSON) conversion without the raw
#   Program(JSON)->MIR CLI surface.
# - Keep EXE helpers on the same env.mirbuilder.emit route vocabulary while the
#   remaining raw CLI implementation is retired caller-by-caller.

program_json_mir_bridge_quote_file_text() {
  local json_path="$1"
  python3 - "$json_path" <<'PY'
import json
import pathlib
import sys

path = pathlib.Path(sys.argv[1])
print(json.dumps(path.read_text(encoding="utf-8")))
PY
}

program_json_mir_bridge_render_v1_wrapper() {
  local wrapper_path="$1" program_json_path="$2"
  local program_json_quoted
  program_json_quoted="$(program_json_mir_bridge_quote_file_text "$program_json_path")"

  cat >"$wrapper_path" <<JSON
{
  "schema_version": "1.0",
  "functions": [
    {
      "name": "main",
      "blocks": [
        { "id": 0, "instructions": [
          {"op":"const","dst":0, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": ${program_json_quoted}}},
          {"op":"mir_call","dst":1, "callee": {"type":"Extern","name":"env.mirbuilder.emit"}, "args": [0], "effects": [] },
          {"op":"const","dst":2, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "[MIR_OUT_BEGIN]"}},
          {"op":"mir_call","callee": {"type":"Extern","name":"print"}, "args": [2], "effects": [] },
          {"op":"mir_call","callee": {"type":"Extern","name":"print"}, "args": [1], "effects": [] },
          {"op":"const","dst":3, "value": {"type": {"kind":"handle","box_type":"StringBox"}, "value": "[MIR_OUT_END]"}},
          {"op":"mir_call","callee": {"type":"Extern","name":"print"}, "args": [3], "effects": [] },
          {"op":"const","dst":4, "value": {"type":"i64", "value": 0}},
          {"op":"ret", "value": 4}
        ] }
      ]
    }
  ]
}
JSON
}

program_json_mir_bridge_extract_payload() {
  local stdout_path="$1" mir_out_path="$2"
  awk '/\[MIR_OUT_BEGIN\]/{flag=1;next}/\[MIR_OUT_END\]/{flag=0}flag' "$stdout_path" > "$mir_out_path"
}

program_json_mir_bridge_output_looks_like_mir() {
  local mir_out_path="$1"
  [ -s "$mir_out_path" ] &&
    grep -q '"functions"' "$mir_out_path" &&
    grep -q '"blocks"' "$mir_out_path"
}

program_json_mir_bridge_cleanup() {
  rm -f "$@" 2>/dev/null || true
}

program_json_mir_bridge_emit() {
  local bin="$1" program_json_path="$2" mir_out_path="$3"
  local label="${4:-[program-json-mir-bridge]}"
  local tmp_wrapper tmp_stdout
  tmp_wrapper="$(mktemp --suffix .mirv1.json)"
  tmp_stdout="$(mktemp)"

  if ! program_json_mir_bridge_render_v1_wrapper "$tmp_wrapper" "$program_json_path"; then
    echo "$label failed to render Program(JSON)->MIR wrapper" >&2
    program_json_mir_bridge_cleanup "$tmp_wrapper" "$tmp_stdout"
    return 1
  fi

  local rc=0
  set +e
  HAKO_V1_EXTERN_PROVIDER=0 \
  NYASH_MIR_UNIFIED_CALL="${NYASH_MIR_UNIFIED_CALL:-1}" \
    "$bin" --mir-json-file "$tmp_wrapper" >"$tmp_stdout" 2>&1
  rc=$?
  set -e
  if [ "$rc" -ne 0 ]; then
    echo "$label env.mirbuilder.emit wrapper failed (rc=$rc)" >&2
    tail -n 80 "$tmp_stdout" >&2 || true
    program_json_mir_bridge_cleanup "$tmp_wrapper" "$tmp_stdout"
    return "$rc"
  fi

  program_json_mir_bridge_extract_payload "$tmp_stdout" "$mir_out_path"
  if ! program_json_mir_bridge_output_looks_like_mir "$mir_out_path"; then
    echo "$label env.mirbuilder.emit produced non-MIR payload" >&2
    tail -n 80 "$tmp_stdout" >&2 || true
    program_json_mir_bridge_cleanup "$tmp_wrapper" "$tmp_stdout"
    return 1
  fi

  program_json_mir_bridge_cleanup "$tmp_wrapper" "$tmp_stdout"
  return 0
}
