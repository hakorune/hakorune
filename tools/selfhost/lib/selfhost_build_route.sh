#!/usr/bin/env bash
# selfhost_build_route.sh — route-main orchestration for selfhost_build.sh
#
# Purpose:
# - Own the top-level arg parsing and route orchestration for selfhost_build.sh.
# - Keep the shell facade thin while helper files own direct / run / exe details.

exit_program_json_wrapper_retired() {
  echo "[selfhost] --json is retired from selfhost_build.sh" >&2
  echo "           use --mir for public/bootstrap output" >&2
  echo "           use tools/dev/phase29ch_program_json_compat_route_probe.sh for explicit compat proof" >&2
  exit 2
}

exit_bare_stageb_route_retired() {
  echo "[selfhost] bare --in Program(JSON v0) output is retired" >&2
  echo "           use --mir <file>, --exe <file>, or --run" >&2
  echo "           use tools/dev/program_json_v0/stageb_artifact_probe.sh for Program(JSON v0) diagnostics" >&2
  exit 2
}

exit_stageb_artifact_route_retired() {
  echo "[selfhost] Stage-B Program(JSON v0) artifact output is retired from selfhost_build.sh" >&2
  echo "           use tools/dev/program_json_v0/stageb_artifact_probe.sh --in <source.hako> [--out <program.json>]" >&2
  exit 2
}

direct_mir_only_route_requested() {
  [ -n "$MIR_OUT" ] \
    && [ -z "$EXE_OUT" ]
}

direct_exe_route_requested() {
  [ -n "$EXE_OUT" ]
}

direct_run_route_requested() {
  [ "$DO_RUN" = "1" ] \
    && [ -z "$EXE_OUT" ]
}

selfhost_build_output_route_requested() {
  [ -n "$MIR_OUT" ] \
    || [ -n "$EXE_OUT" ] \
    || [ "$DO_RUN" = "1" ]
}

apply_selfhost_env() {
  export NYASH_FEATURES="${NYASH_FEATURES:-stage3}"
  export NYASH_PARSER_ALLOW_SEMICOLON=1
  export NYASH_ALLOW_USING_FILE=0
  export HAKO_ALLOW_USING_FILE=0
  export NYASH_USING_AST=1
  export NYASH_VARMAP_GUARD_STRICT=0
  export NYASH_BLOCK_SCHEDULE_VERIFY=0
  export NYASH_QUIET=0 HAKO_QUIET=0 NYASH_CLI_VERBOSE=0
  # Ensure core plugins (Console/Array/Map/String/Integer) are discoverable
  export NYASH_PLUGIN_PATH="${NYASH_PLUGIN_PATH:-$ROOT/target/release}"
  export NYASH_PLUGIN_PATHS="${NYASH_PLUGIN_PATHS:-$NYASH_PLUGIN_PATH}"
  # Phase 80/81: JoinIR Core/Strict mode propagation
  # NYASH_JOINIR_CORE は常時 ON のため no-op（警告のみ）。STRICT は互換のため通す。
  if [ -n "${NYASH_JOINIR_STRICT:-}" ]; then
    export NYASH_JOINIR_STRICT
  fi
}

selfhost_build_main() {
  IN=""
  JSON_OUT=""
  MIR_OUT=""
  EXE_OUT=""
  DO_RUN=0
  KEEP_TMP=0

  RAW_KEEP="${NYASH_SELFHOST_KEEP_RAW:-0}"

  while [ $# -gt 0 ]; do
    case "$1" in
      --in) IN="$2"; shift 2;;
      --json) JSON_OUT="$2"; shift 2;;
      --run) DO_RUN=1; shift;;
      --mir) MIR_OUT="$2"; shift 2;;
      --keep-tmp) KEEP_TMP=1; shift;;
      --exe) EXE_OUT="$2"; shift 2;;
      --strict) export NYASH_JOINIR_STRICT=1; shift;; # Phase 81: Fail-Fast mode
      --core) echo "[selfhost] --core is deprecated (JoinIR is always on); ignoring" >&2; shift;;
      *) echo "[selfhost] unknown arg: $1" >&2; exit 2;;
    esac
  done

  if [ -z "$IN" ]; then echo "[selfhost] --in <file.hako> is required" >&2; exit 2; fi
  if [ ! -f "$IN" ]; then echo "[selfhost] input not found: $IN" >&2; exit 2; fi

  if [ -n "$JSON_OUT" ]; then
    exit_program_json_wrapper_retired
  fi

  if [ "$KEEP_TMP" = "1" ] || [ "$RAW_KEEP" = "1" ]; then
    exit_stageb_artifact_route_retired
  fi

  if ! selfhost_build_output_route_requested; then
    exit_bare_stageb_route_retired
  fi

  if direct_run_route_requested; then
    apply_selfhost_env
    run_requested_direct_mir
    exit $?
  fi

  if direct_mir_only_route_requested; then
    apply_selfhost_env
    emit_direct_mir_only_output
    exit $?
  fi

  if direct_exe_route_requested; then
    apply_selfhost_env
    emit_requested_direct_exe_output
    exit $?
  fi
}
