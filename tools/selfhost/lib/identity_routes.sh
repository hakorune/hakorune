if [[ -z "${ROOT:-}" ]]; then
  ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
fi
source "${ROOT}/tools/selfhost/lib/stage1_contract.sh"

read_artifact_kind() {
  stage1_contract_artifact_kind "$1"
}

cleanup_stage_temp_files() {
  rm -f "$@" 2>/dev/null || true
}

cleanup_stage_temp_dir() {
  rm -rf "$1" 2>/dev/null || true
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
    echo "                build a stage1-cli artifact (tools/selfhost/mainline/build_stage1.sh --artifact-kind stage1-cli)" >&2
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

stage_emit_marker() {
  local subcmd="$1"
  case "$subcmd" in
    program-json) printf '%s' '"kind"[[:space:]]*:[[:space:]]*"Program"' ;;
    mir-json) printf '%s' '"functions"[[:space:]]*:' ;;
    *) return 1 ;;
  esac
}

stage_entry_source_text() {
  local entry="$1"
  stage1_contract_source_text "$entry"
}

route_file_value() {
  local route_file="$1"
  cat "$route_file" 2>/dev/null || echo unknown
}

stage1_env_program_route_id() {
  # Legacy reduced-artifact diagnostics route id.
  # Bootstrap acceptance now lives in stage1_contract_verify_stage1_cli_bootstrap_capability().
  printf '%s' 'stage1-env-program'
}

stage1_env_mir_source_route_id() {
  # Legacy reduced-artifact diagnostics route id.
  # Bootstrap acceptance now lives in stage1_contract_verify_stage1_cli_bootstrap_capability().
  printf '%s' 'stage1-env-mir-source'
}

require_stage1_route_for_full_mode() {
  local subcmd="$1"
  local stage1_route="$2"
  local stage2_route="$3"
  local hint="$4"
  if [[ ! "$stage1_route" =~ ^stage1 || ! "$stage2_route" =~ ^stage1 ]]; then
    echo "[G1:FAIL] full mode requires stage1 ${subcmd} route on both binaries" >&2
    echo "          detected routes: stage1_bin=$stage1_route stage2_bin=$stage2_route" >&2
    echo "          ${hint}" >&2
    return 1
  fi
  return 0
}

require_exact_stage1_route_for_full_mode() {
  local subcmd="$1"
  local expected_route="$2"
  local stage1_route="$3"
  local stage2_route="$4"
  local hint="$5"
  if [[ "$stage1_route" != "$expected_route" || "$stage2_route" != "$expected_route" ]]; then
    echo "[G1:FAIL] full mode requires exact stage1 ${subcmd} route on both binaries" >&2
    echo "          expected route: ${expected_route}" >&2
    echo "          detected routes: stage1_bin=$stage1_route stage2_bin=$stage2_route" >&2
    echo "          ${hint}" >&2
    return 1
  fi
  return 0
}

require_current_stage1_env_route_for_full_mode() {
  local subcmd="$1"
  local stage1_route="$2"
  local stage2_route="$3"

  case "$subcmd" in
    program-json)
      require_exact_stage1_route_for_full_mode \
        "$subcmd" \
        "$(stage1_env_program_route_id)" \
        "$stage1_route" \
        "$stage2_route" \
        "current reduced authority pins program-json on stage1 env mainline"
      ;;
    mir-json)
      require_exact_stage1_route_for_full_mode \
        "$subcmd" \
        "$(stage1_env_mir_source_route_id)" \
        "$stage1_route" \
        "$stage2_route" \
        "current reduced authority pins mir-json on single-step source->MIR env mainline"
      ;;
    *)
      echo "[G1:FAIL] unsupported current stage1 route check: ${subcmd}" >&2
      return 1
      ;;
  esac
}

require_exact_stage1_route_for_bin() {
  local subcmd="$1"
  local expected_route="$2"
  local detected_route="$3"
  local hint="$4"
  if [[ "$detected_route" != "$expected_route" ]]; then
    echo "[G1:FAIL] exact stage1 ${subcmd} route mismatch" >&2
    echo "          expected route: ${expected_route}" >&2
    echo "          detected route: ${detected_route}" >&2
    echo "          ${hint}" >&2
    return 1
  fi
  return 0
}

require_current_stage1_env_route_for_bin() {
  local subcmd="$1"
  local detected_route="$2"

  case "$subcmd" in
    program-json)
      require_exact_stage1_route_for_bin \
        "$subcmd" \
        "$(stage1_env_program_route_id)" \
        "$detected_route" \
        "current reduced authority pins program-json on stage1 env mainline"
      ;;
    mir-json)
      require_exact_stage1_route_for_bin \
        "$subcmd" \
        "$(stage1_env_mir_source_route_id)" \
        "$detected_route" \
        "current reduced authority pins mir-json on single-step source->MIR env mainline"
      ;;
    *)
      echo "[G1:FAIL] unsupported current stage1 route check: ${subcmd}" >&2
      return 1
      ;;
  esac
}

run_and_extract_stage_payload() {
  local subcmd="$1"
  local outfile="$2"
  shift 2

  local marker tmp_raw rc=0
  marker="$(stage_emit_marker "$subcmd")" || return 1
  tmp_raw="$(mktemp)"

  set +e
  "$@" >"$tmp_raw" 2>/dev/null
  rc=$?
  set -e
  if [[ "$rc" -ne 0 ]]; then
    rm -f "$tmp_raw"
    return "$rc"
  fi
  if ! extract_json_object_line_to_file "$marker" "$tmp_raw" "$outfile"; then
    rm -f "$tmp_raw"
    return 1
  fi
  rm -f "$tmp_raw"
  return 0
}

run_stage1_env_route() {
  local bin="$1"
  local subcmd="$2"
  local entry="$3"
  local outfile="$4"
  local route_file="${5:-}"
  local source_text
  source_text="$(stage_entry_source_text "$entry")"

  if [[ "$subcmd" == "program-json" ]]; then
    run_and_extract_stage_payload \
      "program-json" \
      "$outfile" \
      stage1_contract_exec_mode "$bin" "emit-program" "$entry" "$source_text"
    if [[ $? -eq 0 && -n "$route_file" ]]; then
      echo "$(stage1_env_program_route_id)" >"$route_file"
      return 0
    fi
    return $?
  fi

  if [[ "$subcmd" != "mir-json" ]]; then
    return 1
  fi

  if run_and_extract_stage_payload \
    "mir-json" \
    "$outfile" \
    stage1_contract_exec_mode "$bin" "emit-mir" "$entry" "$source_text"; then
    if [[ -n "$route_file" ]]; then
      echo "$(stage1_env_mir_source_route_id)" >"$route_file"
    fi
    return 0
  fi
  return 1
}

probe_exact_stage1_env_authority() {
  # Route evidence probe for legacy env diagnostics.
  # Stage1 bootstrap acceptance is handled separately by
  # stage1_contract_verify_stage1_cli_bootstrap_capability().
  # Build-stage bootstrap capability is verified separately by
  # stage1_contract_verify_stage1_cli_bootstrap_capability() so the two
  # contracts do not drift into one another.
  local bin="$1"
  local entry="$2"
  local program_out="$3"
  local mir_out="$4"
  local tmp_dir
  tmp_dir="$(mktemp -d)"
  local program_route_file="${tmp_dir}/program.route"
  local mir_route_file="${tmp_dir}/mir.route"

  if ! run_stage1_env_route "$bin" "program-json" "$entry" "$program_out" "$program_route_file"; then
    cleanup_stage_temp_dir "$tmp_dir"
    return 1
  fi
  if ! require_current_stage1_env_route_for_bin \
    "program-json" \
    "$(route_file_value "$program_route_file")"; then
    cleanup_stage_temp_dir "$tmp_dir"
    return 1
  fi

  if ! run_stage1_env_route "$bin" "mir-json" "$entry" "$mir_out" "$mir_route_file"; then
    cleanup_stage_temp_dir "$tmp_dir"
    return 1
  fi
  if ! require_current_stage1_env_route_for_bin \
    "mir-json" \
    "$(route_file_value "$mir_route_file")"; then
    cleanup_stage_temp_dir "$tmp_dir"
    return 1
  fi
  cleanup_stage_temp_dir "$tmp_dir"
  return 0
}

run_stage1_subcmd_route() {
  local bin="$1"
  local subcmd="$2"
  local entry="$3"
  local outfile="$4"
  bash "${ROOT}/tools/selfhost/compat/run_stage1_cli.sh" --bin "$bin" emit "$subcmd" "$entry" >"$outfile" 2>/dev/null
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
    if run_stage1_env_route "$bin" "$subcmd" "$entry" "$outfile" "$route_file"; then
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

  if [[ "$subcmd" == "program-json" && "$CLI_MODE" != "stage1" ]]; then
    echo "[identity/compat-retired] program-json cli-mode=${CLI_MODE} is retired for G1 identity" >&2
    echo "                         use tools/dev/phase29ch_program_json_compat_route_probe.sh for explicit compat proof" >&2
    return 2
  fi

  if [[ "$CLI_MODE" == "stage1" ]]; then
    if run_stage1_route "$bin" "$subcmd" "$entry" "$outfile" "$route_file"; then
      return 0
    fi
    return 1
  fi

  if [[ "$CLI_MODE" == "stage0" ]]; then
    "$bin" --emit-mir-json "$outfile" "$entry" >/dev/null 2>&1
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

  # `auto` -> `stage0` is compatibility-only recovery.
  # It is useful for diagnostics, but it is not accepted as full-mode identity
  # evidence while `stage1` remains the mainline selfhost route.
  echo "[identity/compat-fallback] route=stage0 subcmd=${subcmd} reason=stage1-route-failed bin=$(basename "$bin")" >&2
  "$bin" --emit-mir-json "$outfile" "$entry" >/dev/null 2>&1
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
  local kind
  kind="$(read_artifact_kind "$bin")"
  if [[ "$kind" == "stage1-cli" ]]; then
    require_stage1_env_mainline_capability "$stage_label" "$bin" "$ENTRY_SMOKE"
    return $?
  fi
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
  if ! grep -Eq "$(stage_emit_marker program-json)" "$tmp_out"; then
    rm -f "$tmp_out"
    rm -f "$tmp_route"
    report_stage1_cli_capability_hint "$stage_label" "$bin"
    return 1
  fi
  rm -f "$tmp_out"
  rm -f "$tmp_route"
  return 0
}

require_stage1_env_mainline_capability() {
  local stage_label="$1"
  local bin="$2"
  local entry="$3"
  local tmp_prog tmp_mir
  tmp_prog="$(mktemp)"
  tmp_mir="$(mktemp)"

  if ! probe_exact_stage1_env_authority "$bin" "$entry" "$tmp_prog" "$tmp_mir"; then
    rm -f "$tmp_prog" "$tmp_mir"
    report_stage1_cli_capability_hint "$stage_label" "$bin"
    return 1
  fi

  rm -f "$tmp_prog" "$tmp_mir"
  return 0
}
