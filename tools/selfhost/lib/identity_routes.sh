if [[ -z "${ROOT:-}" ]]; then
  ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
fi
source "${ROOT}/tools/selfhost/lib/stage1_contract.sh"

read_artifact_kind() {
  local bin="$1"
  local meta="${bin}.artifact_kind"
  if [[ ! -f "$meta" ]]; then
    echo "unknown"
    return 0
  fi
  local kind
  kind="$(awk -F= '/^artifact_kind=/{print $2; exit}' "$meta" 2>/dev/null || true)"
  if [[ -z "$kind" ]]; then
    echo "unknown"
    return 0
  fi
  echo "$kind"
}

report_stage1_cli_capability_hint() {
  local stage_label="$1"
  local bin="$2"
  local kind
  kind="$(read_artifact_kind "$bin")"
  echo "[G1:FAIL] ${stage_label} does not provide Stage1 CLI emit capability" >&2
  echo "          bin=${bin}" >&2
  echo "          artifact_kind=${kind}" >&2
  if [[ "$kind" == "launcher-exe" ]]; then
    echo "          hint: launcher-exe cannot satisfy G1 identity emit contract" >&2
    echo "                build a stage1-cli artifact (tools/selfhost/build_stage1.sh --artifact-kind stage1-cli)" >&2
  else
    echo "          hint: provide prebuilt stage1-cli compatible binary via --bin-stage{1,2}" >&2
  fi
}

extract_json_object_line_to_file() {
  local marker="$1"
  local input_file="$2"
  local output_file="$3"
  local line
  line="$(grep -m1 -E "^[[:space:]]*\\{.*${marker}.*\\}[[:space:]]*$" "$input_file" || true)"
  if [[ -z "$line" ]]; then
    return 1
  fi
  printf '%s\n' "$line" >"$output_file"
  return 0
}

run_stage1_env_route() {
  local bin="$1"
  local subcmd="$2"
  local entry="$3"
  local outfile="$4"
  local tmp_raw
  local rc=0
  tmp_raw="$(mktemp)"
  if [[ "$subcmd" == "program-json" ]]; then
    local source_text
    source_text="$(stage1_contract_source_text "$entry")"
    set +e
    stage1_contract_exec_mode "$bin" "emit-program" "$entry" "$source_text" >"$tmp_raw" 2>/dev/null
    rc=$?
    set -e
    if [[ "$rc" -ne 0 ]]; then
      rm -f "$tmp_raw"
      return "$rc"
    fi
    if ! extract_json_object_line_to_file '"kind"[[:space:]]*:[[:space:]]*"Program"' "$tmp_raw" "$outfile"; then
      rm -f "$tmp_raw"
      return 1
    fi
    rm -f "$tmp_raw"
    return 0
  fi

  if [[ "$subcmd" != "mir-json" ]]; then
    rm -f "$tmp_raw"
    return 1
  fi

  local tmp_prog
  local source_text
  tmp_prog="$(mktemp)"
  source_text="$(stage1_contract_source_text "$entry")"
  if ! run_stage1_env_route "$bin" "program-json" "$entry" "$tmp_prog"; then
    rm -f "$tmp_raw" "$tmp_prog"
    return 1
  fi

  # Preferred contract: emit-mir consumes prebuilt Program(JSON v0) path.
  set +e
  stage1_contract_exec_mode "$bin" "emit-mir" "$entry" "$source_text" "$tmp_prog" >"$tmp_raw" 2>/dev/null
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] && extract_json_object_line_to_file '"functions"[[:space:]]*:' "$tmp_raw" "$outfile"; then
    rm -f "$tmp_raw" "$tmp_prog"
    return 0
  fi

  # Compatibility toggle for artifacts that still key off legacy STAGE1_EMIT_MIR_JSON.
  set +e
  stage1_contract_exec_legacy_emit_mir "$bin" "$entry" "$source_text" "$tmp_prog" >"$tmp_raw" 2>/dev/null
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] && extract_json_object_line_to_file '"functions"[[:space:]]*:' "$tmp_raw" "$outfile"; then
    rm -f "$tmp_raw" "$tmp_prog"
    return 0
  fi

  # Last attempt inside stage1 route: explicit subcommand with --from-program-json.
  set +e
  bash "${ROOT}/tools/selfhost/run_stage1_cli.sh" --bin "$bin" emit mir-json --from-program-json "$tmp_prog" "$entry" >"$tmp_raw" 2>/dev/null
  rc=$?
  set -e
  if [[ "$rc" -eq 0 ]] && extract_json_object_line_to_file '"functions"[[:space:]]*:' "$tmp_raw" "$outfile"; then
    rm -f "$tmp_raw" "$tmp_prog"
    return 0
  fi
  rm -f "$tmp_raw" "$tmp_prog"
  return 1
}

run_stage1_subcmd_route() {
  local bin="$1"
  local subcmd="$2"
  local entry="$3"
  local outfile="$4"
  bash "${ROOT}/tools/selfhost/run_stage1_cli.sh" --bin "$bin" emit "$subcmd" "$entry" >"$outfile" 2>/dev/null
}

run_stage1_route() {
  local bin="$1"
  local subcmd="$2"
  local entry="$3"
  local outfile="$4"
  local route_file="$5"
  local kind
  kind="$(read_artifact_kind "$bin")"

  if [[ "$kind" == "stage1-cli" ]]; then
    if run_stage1_env_route "$bin" "$subcmd" "$entry" "$outfile"; then
      echo "stage1-env" >"$route_file"
      return 0
    fi
  fi

  if run_stage1_subcmd_route "$bin" "$subcmd" "$entry" "$outfile"; then
    echo "stage1-subcmd" >"$route_file"
    return 0
  fi

  return 1
}

run_stage_cli() {
  local bin="$1"
  local subcmd="$2"
  local entry="$3"
  local outfile="$4"
  local route_file="$5"
  local rc=0

  if [[ "$CLI_MODE" == "stage1" ]]; then
    if run_stage1_route "$bin" "$subcmd" "$entry" "$outfile" "$route_file"; then
      return 0
    fi
    return 1
  fi

  if [[ "$CLI_MODE" == "stage0" ]]; then
    if [[ "$subcmd" == "program-json" ]]; then
      "$bin" --emit-program-json-v0 "$outfile" "$entry" >/dev/null 2>&1
    else
      "$bin" --emit-mir-json "$outfile" "$entry" >/dev/null 2>&1
    fi
    rc=$?
    if [[ "$rc" -ne 0 ]]; then
      return "$rc"
    fi
    echo "stage0" >"$route_file"
    return 0
  fi

  if run_stage1_route "$bin" "$subcmd" "$entry" "$outfile" "$route_file"; then
    return 0
  fi

  echo "[identity/compat-fallback] route=stage0 subcmd=${subcmd} reason=stage1-route-failed bin=$(basename "$bin")" >&2
  if [[ "$subcmd" == "program-json" ]]; then
    "$bin" --emit-program-json-v0 "$outfile" "$entry" >/dev/null 2>&1
  else
    "$bin" --emit-mir-json "$outfile" "$entry" >/dev/null 2>&1
  fi
  rc=$?
  if [[ "$rc" -ne 0 ]]; then
    return "$rc"
  fi
  echo "stage0" >"$route_file"
  return 0
}

preflight_stage1_cli() {
  local stage_label="$1"
  local bin="$2"
  local tmp_out
  tmp_out="$(mktemp)"
  local tmp_route
  tmp_route="$(mktemp)"
  if ! run_stage1_route "$bin" "program-json" "$ENTRY_SMOKE" "$tmp_out" "$tmp_route"; then
    rm -f "$tmp_out"
    rm -f "$tmp_route"
    report_stage1_cli_capability_hint "$stage_label" "$bin"
    return 1
  fi
  if ! grep -Eq '"kind"[[:space:]]*:[[:space:]]*"Program"' "$tmp_out"; then
    rm -f "$tmp_out"
    rm -f "$tmp_route"
    report_stage1_cli_capability_hint "$stage_label" "$bin"
    return 1
  fi
  rm -f "$tmp_out"
  rm -f "$tmp_route"
  return 0
}
