#!/usr/bin/env bash
# build_stage1.sh — Build Hakorune selfhost stage artifact
#
# Purpose
# - Produce a selfhost executable artifact from Nyash/Hako sources.
# - Artifact kind is explicit (`launcher-exe` or `stage1-cli`) to avoid contract drift.
# - This script keeps existing build wiring and makes artifact intent machine-readable.
#
# Output
# - launcher-exe (default): target/selfhost/hakorune
# - stage1-cli:            target/selfhost/hakorune.stage1_cli
#
# Env / args
# - HAKORUNE_STAGE1_ENTRY: override Stage1 entry .hako (optional).
# - HAKORUNE_STAGE1_OUT:   override output path (optional).
# - NYASH_LLVM_SKIP_BUILD: set to 1 to skip cargo build in ny_mir_builder.sh when artifacts already exist.
# - HAKORUNE_STAGE1_REUSE_IF_FRESH: set to 1 to reuse existing artifact when up-to-date (default: 1).
# - HAKORUNE_STAGE1_ARTIFACT_KIND: default artifact kind override (`launcher-exe`|`stage1-cli`)
# - Args:
#     --out <path>           : override output path (same as HAKORUNE_STAGE1_OUT).
#     --entry <file>         : override entry .hako (same as HAKORUNE_STAGE1_ENTRY).
#     --artifact-kind <kind> : launcher-exe | stage1-cli (default: launcher-exe)
#     --timeout-secs <secs>  : fail-fast timeout for Stage-B→MIR→EXE build (default: 900, 0 disables).
#     --reuse-if-fresh <0|1> : reuse existing output if metadata/deps are unchanged (default: 1).
#     --force-rebuild         : disable reuse check for this invocation.
#     -h|--help      : show usage and exit.
#
set -euo pipefail

is_truthy() {
  case "${1:-}" in
    1|true|TRUE|yes|YES|on|ON) return 0 ;;
    *) return 1 ;;
  esac
}

artifact_metadata_matches() {
  local meta_path="$1"
  [ -f "$meta_path" ] || return 1
  local meta_kind meta_entry
  meta_kind="$(awk -F= '$1=="artifact_kind"{print $2}' "$meta_path" | tail -n 1)"
  meta_entry="$(awk -F= '$1=="entry"{print substr($0, index($0, "=")+1)}' "$meta_path" | tail -n 1)"
  [ "$meta_kind" = "$ARTIFACT_KIND" ] || return 1
  [ "$meta_entry" = "$ENTRY" ] || return 1
  return 0
}

artifact_is_fresh() {
  local out_path="$1"
  shift
  [ -x "$out_path" ] || return 1
  local dep
  for dep in "$@"; do
    [ -e "$dep" ] || continue
    if [ "$dep" -nt "$out_path" ]; then
      return 1
    fi
  done
  return 0
}

usage() {
  cat <<'USAGE'
build_stage1.sh — Build Hakorune selfhost stage artifact

Usage:
  tools/selfhost/build_stage1.sh [--artifact-kind <launcher-exe|stage1-cli>] [--out <exe_path>] [--entry <entry.hako>] [--timeout-secs <secs>] [--reuse-if-fresh <0|1>] [--force-rebuild]

Defaults:
  artifact-kind: launcher-exe
  launcher-exe entry/out:
    entry .hako : lang/src/runner/launcher.hako
    output exe  : target/selfhost/hakorune
  stage1-cli entry/out:
    entry .hako : lang/src/runner/stage1_cli_env.hako
    output exe  : target/selfhost/hakorune.stage1_cli

Notes:
  - This script uses selfhost_exe_stageb helper-free emit + ny_mir_builder pipeline:
      <entry.hako>
        → tools/selfhost_exe_stageb.sh (route selectable via HAKORUNE_STAGE1_EMIT_ROUTE)
        → tools/ny_mir_builder.sh --emit exe
  - The Rust binary (Stage0) is treated as bootstrap and is resolved via
    selfhost_exe_stageb.sh / ny_mir_builder.sh.
  - Default timeout is 900 seconds to avoid hanging forever on large launcher builds.
    Set --timeout-secs 0 to disable timeout.
  - Default reuse mode is enabled (`--reuse-if-fresh 1`) to speed up daily loops.
    Use `--force-rebuild` when you need a full rebuild.
USAGE
}

ROOT="${NYASH_ROOT:-$(cd "$(dirname "$0")/../.." && pwd)}"
source "$ROOT/tools/selfhost/lib/stage1_contract.sh"

read_bootstrap_artifact_kind() {
  local bin="${NYASH_BIN:-}"
  local meta
  if [ -z "$bin" ]; then
    printf "unknown"
    return 0
  fi
  meta="${bin}.artifact_kind"
  if [ ! -f "$meta" ]; then
    printf "unknown"
    return 0
  fi
  awk -F= '$1=="artifact_kind"{print $2; exit}' "$meta" 2>/dev/null || printf "unknown"
}

build_with_stage1_cli_bootstrap() {
  local tmp_prog tmp_mir source_text
  tmp_prog="$(mktemp --suffix .stage1_cli_bootstrap.program.json)"
  tmp_mir="$(mktemp --suffix .stage1_cli_bootstrap.mir.json)"
  trap 'rm -f "$tmp_prog" "$tmp_mir" 2>/dev/null || true' RETURN

  source_text="$(stage1_contract_source_text "$ENTRY")"
  if ! NYASH_BRIDGE_ME_DUMMY="${NYASH_BRIDGE_ME_DUMMY:-1}" \
    stage1_contract_exec_mode "$NYASH_BIN" "emit-program" "$ENTRY" "$source_text" >"$tmp_prog"; then
    echo "[stage1] stage1-cli bootstrap emit-program failed: $ENTRY" >&2
    return 1
  fi
  if ! grep -q '"kind"[[:space:]]*:[[:space:]]*"Program"' "$tmp_prog"; then
    echo "[stage1] stage1-cli bootstrap produced non-Program payload: $ENTRY" >&2
    return 1
  fi
  if [[ "$ENTRY" == *"/lang/src/runner/stage1_cli_env.hako" ]]; then
    if ! python - "$tmp_prog" <<'PY'
import json, pathlib, sys
p = pathlib.Path(sys.argv[1])
obj = json.loads(p.read_text())
sys.exit(0 if len(obj.get("defs", [])) > 0 else 1)
PY
    then
      echo "[stage1] stage1-cli bootstrap emit-program missing helper defs: $ENTRY" >&2
      return 1
    fi
  fi
  if ! NYASH_BRIDGE_ME_DUMMY="${NYASH_BRIDGE_ME_DUMMY:-1}" \
    stage1_contract_exec_mode "$NYASH_BIN" "emit-mir" "$ENTRY" "$source_text" "$tmp_prog" >"$tmp_mir"; then
    echo "[stage1] stage1-cli bootstrap emit-mir failed: $ENTRY" >&2
    return 1
  fi
  if ! grep -q '"functions"' "$tmp_mir"; then
    echo "[stage1] stage1-cli bootstrap produced non-MIR payload: $ENTRY" >&2
    return 1
  fi

  NYASH_LLVM_BACKEND=crate \
  NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
  NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
    bash "$ROOT/tools/ny_mir_builder.sh" --in "$tmp_mir" --emit exe -o "$OUT" --quiet >/dev/null
}

ENTRY_DEFAULT_LAUNCHER="$ROOT/lang/src/runner/launcher.hako"
ENTRY_DEFAULT_STAGE1_CLI="$ROOT/lang/src/runner/stage1_cli_env.hako"
OUT_DEFAULT_LAUNCHER="$ROOT/target/selfhost/hakorune"
OUT_DEFAULT_STAGE1_CLI="$ROOT/target/selfhost/hakorune.stage1_cli"
TIMEOUT_SECS_DEFAULT=900

ARTIFACT_KIND="${HAKORUNE_STAGE1_ARTIFACT_KIND:-launcher-exe}"
ENTRY="${HAKORUNE_STAGE1_ENTRY:-}"
OUT="${HAKORUNE_STAGE1_OUT:-}"
TIMEOUT_SECS="$TIMEOUT_SECS_DEFAULT"
REUSE_IF_FRESH="${HAKORUNE_STAGE1_REUSE_IF_FRESH:-1}"

while [ $# -gt 0 ]; do
  case "$1" in
    --out)
      OUT="$2"
      shift 2
      ;;
    --entry)
      ENTRY="$2"
      shift 2
      ;;
    --artifact-kind)
      ARTIFACT_KIND="$2"
      shift 2
      ;;
    --timeout-secs)
      TIMEOUT_SECS="$2"
      shift 2
      ;;
    --reuse-if-fresh)
      REUSE_IF_FRESH="$2"
      shift 2
      ;;
    --force-rebuild|--no-reuse)
      REUSE_IF_FRESH=0
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[stage1] unknown arg: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [ "$ARTIFACT_KIND" != "launcher-exe" ] && [ "$ARTIFACT_KIND" != "stage1-cli" ]; then
  echo "[stage1] --artifact-kind must be launcher-exe|stage1-cli: $ARTIFACT_KIND" >&2
  exit 2
fi

if [ -z "$ENTRY" ]; then
  if [ "$ARTIFACT_KIND" = "stage1-cli" ]; then
    ENTRY="$ENTRY_DEFAULT_STAGE1_CLI"
  else
    ENTRY="$ENTRY_DEFAULT_LAUNCHER"
  fi
fi

if [ -z "$OUT" ]; then
  if [ "$ARTIFACT_KIND" = "stage1-cli" ]; then
    OUT="$OUT_DEFAULT_STAGE1_CLI"
  else
    OUT="$OUT_DEFAULT_LAUNCHER"
  fi
fi

if ! [[ "$TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  echo "[stage1] --timeout-secs must be a non-negative integer: $TIMEOUT_SECS" >&2
  exit 2
fi

if [ ! -f "$ENTRY" ]; then
  echo "[stage1] entry .hako not found: $ENTRY" >&2
  exit 2
fi

OUT_DIR="$(dirname "$OUT")"
mkdir -p "$OUT_DIR"
META_OUT="${OUT}.artifact_kind"

FRESH_DEPS=(
  "$ENTRY"
  "$ROOT/tools/selfhost/build_stage1.sh"
  "$ROOT/tools/selfhost_exe_stageb.sh"
  "$ROOT/tools/ny_mir_builder.sh"
  "$ROOT/tools/selfhost/lib/stage1_contract.sh"
  "$ROOT/lang/src/compiler/entry/compiler_stageb.hako"
  "$ROOT/target/release/hakorune"
  "$ROOT/target/release/ny-llvmc"
  "$ROOT/target/release/libnyash_kernel.a"
  "$ROOT/crates/nyash_kernel/target/release/libnyash_kernel.a"
)

echo "[stage1] building Stage1 selfhost binary" >&2
echo "         artifact: $ARTIFACT_KIND" >&2
echo "         entry : $ENTRY" >&2
echo "         output: $OUT" >&2
if [ "$TIMEOUT_SECS" -gt 0 ]; then
  echo "         timeout: ${TIMEOUT_SECS}s" >&2
else
  echo "         timeout: disabled" >&2
fi

# Use the Stage‑B → MirBuilder → ny-llvmc path
EXTRA_ENV=()
if [ -z "${NYASH_LLVM_SKIP_BUILD+x}" ]; then
  if [ -x "$ROOT/target/release/hakorune" ] && [ -x "$ROOT/target/release/ny-llvmc" ] \
     && { [ -f "$ROOT/target/release/libnyash_kernel.a" ] || [ -f "$ROOT/crates/nyash_kernel/target/release/libnyash_kernel.a" ]; }; then
    EXTRA_ENV+=("NYASH_LLVM_SKIP_BUILD=1")
    echo "         build-opt: NYASH_LLVM_SKIP_BUILD=1 (auto)" >&2
  fi
fi
if [ "$ARTIFACT_KIND" = "stage1-cli" ]; then
  EXTRA_ENV+=("NYASH_BRIDGE_ME_DUMMY=${NYASH_BRIDGE_ME_DUMMY:-1}")
fi

SKIPPED_BUILD=0
if is_truthy "$REUSE_IF_FRESH"; then
  if artifact_metadata_matches "$META_OUT" && artifact_is_fresh "$OUT" "${FRESH_DEPS[@]}"; then
    SKIPPED_BUILD=1
    echo "[stage1] reuse: up-to-date artifact detected; skipping rebuild" >&2
  fi
fi

if [ "$SKIPPED_BUILD" -ne 1 ]; then
  BOOTSTRAP_KIND="$(read_bootstrap_artifact_kind)"
  if [ "$ARTIFACT_KIND" = "stage1-cli" ] && [ "$BOOTSTRAP_KIND" = "stage1-cli" ]; then
    echo "         bootstrap: stage1-cli bridge-first" >&2
    if [ "$TIMEOUT_SECS" -gt 0 ]; then
      set +e
      timeout --preserve-status "${TIMEOUT_SECS}s" bash -lc '
        set -euo pipefail
        ROOT="$1"
        ENTRY="$2"
        OUT="$3"
        NYASH_BIN="$4"
        source "$ROOT/tools/selfhost/lib/stage1_contract.sh"
        source_text="$(stage1_contract_source_text "$ENTRY")"
        tmp_prog="$(mktemp --suffix .stage1_cli_bootstrap.program.json)"
        tmp_mir="$(mktemp --suffix .stage1_cli_bootstrap.mir.json)"
        trap '\''rm -f "$tmp_prog" "$tmp_mir" 2>/dev/null || true'\'' EXIT
        NYASH_BRIDGE_ME_DUMMY="${NYASH_BRIDGE_ME_DUMMY:-1}" \
          stage1_contract_exec_mode "$NYASH_BIN" "emit-program" "$ENTRY" "$source_text" >"$tmp_prog"
        grep -q "\"kind\"[[:space:]]*:[[:space:]]*\"Program\"" "$tmp_prog"
        if [[ "$ENTRY" == *"/lang/src/runner/stage1_cli_env.hako" ]]; then
          if ! python - "$tmp_prog" <<'\''PY'\''
import json, pathlib, sys
p = pathlib.Path(sys.argv[1])
obj = json.loads(p.read_text())
sys.exit(0 if len(obj.get("defs", [])) > 0 else 1)
PY
          then
            echo "[stage1] stage1-cli bootstrap emit-program missing helper defs: $ENTRY" >&2
            exit 1
          fi
        fi
        NYASH_BRIDGE_ME_DUMMY="${NYASH_BRIDGE_ME_DUMMY:-1}" \
          stage1_contract_exec_mode "$NYASH_BIN" "emit-mir" "$ENTRY" "$source_text" "$tmp_prog" >"$tmp_mir"
        grep -q "\"functions\"" "$tmp_mir"
        NYASH_LLVM_BACKEND=crate \
        NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
        NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
          bash "$ROOT/tools/ny_mir_builder.sh" --in "$tmp_mir" --emit exe -o "$OUT" --quiet >/dev/null
      ' bash "$ROOT" "$ENTRY" "$OUT" "$NYASH_BIN"
      RC=$?
      set -e
      if [ "$RC" -eq 124 ] || [ "$RC" -eq 137 ] || [ "$RC" -eq 143 ]; then
        echo "[stage1] build timed out after ${TIMEOUT_SECS}s" >&2
        echo "         hint: rerun with larger --timeout-secs or use --skip-build with prebuilt binaries" >&2
        exit 2
      fi
      if [ "$RC" -ne 0 ]; then
        echo "[stage1] build failed (rc=$RC)" >&2
        exit "$RC"
      fi
    else
      build_with_stage1_cli_bootstrap
    fi
  elif [ "$TIMEOUT_SECS" -gt 0 ]; then
    set +e
    # Keep env overrides opt-in; stage1-cli correctness is validated post-build.
    timeout --preserve-status "${TIMEOUT_SECS}s" env NYASH_ROOT="$ROOT" "${EXTRA_ENV[@]}" \
      bash "$ROOT/tools/selfhost_exe_stageb.sh" "$ENTRY" -o "$OUT"
    RC=$?
    set -e
    if [ "$RC" -eq 124 ] || [ "$RC" -eq 137 ] || [ "$RC" -eq 143 ]; then
      echo "[stage1] build timed out after ${TIMEOUT_SECS}s" >&2
      echo "         hint: rerun with larger --timeout-secs or use --skip-build with prebuilt binaries" >&2
      exit 2
    fi
    if [ "$RC" -ne 0 ]; then
      echo "[stage1] build failed (rc=$RC)" >&2
      exit "$RC"
    fi
  else
    env NYASH_ROOT="$ROOT" "${EXTRA_ENV[@]}" bash "$ROOT/tools/selfhost_exe_stageb.sh" "$ENTRY" -o "$OUT"
  fi
  echo "[stage1] done: $OUT" >&2
else
  echo "[stage1] done (reused): $OUT" >&2
fi

if [ "$ARTIFACT_KIND" = "stage1-cli" ]; then
  PROBE_SRC="$ROOT/apps/tests/hello_simple_llvm.hako"
  if [ ! -f "$PROBE_SRC" ]; then
    echo "[stage1] stage1-cli probe source not found: $PROBE_SRC" >&2
    exit 2
  fi
  PROBE_SOURCE_TEXT="$(stage1_contract_source_text "$PROBE_SRC")"
  TMP_PROBE_OUT="$(mktemp)"
  set +e
  stage1_contract_exec_mode "$OUT" "emit-program" "$PROBE_SRC" "$PROBE_SOURCE_TEXT" >"$TMP_PROBE_OUT" 2>/dev/null
  PROBE_RC=$?
  set -e
  if [ "$PROBE_RC" -ne 0 ]; then
    rm -f "$TMP_PROBE_OUT" || true
    echo "[stage1] stage1-cli capability check failed" >&2
    echo "         artifact failed env-mode smoke: emit-program (rc=$PROBE_RC)" >&2
    echo "         hint: adjust build route/env and retry" >&2
    exit 2
  fi
  if grep -Eq '"kind"[[:space:]]*:[[:space:]]*"Program"' "$TMP_PROBE_OUT"; then
    rm -f "$TMP_PROBE_OUT" || true
    echo "[stage1] stage1-cli capability: OK (program-json)" >&2
  elif grep -Eq '^Result:[[:space:]]*0$' "$TMP_PROBE_OUT"; then
    rm -f "$TMP_PROBE_OUT" || true
    echo "[stage1] stage1-cli capability: SMOKE-OK (env route returned 0)" >&2
    echo "         note: program-json payload was not observed on stdout in this route" >&2
  else
    rm -f "$TMP_PROBE_OUT" || true
    echo "[stage1] stage1-cli capability check failed" >&2
    echo "         artifact did not produce Program(JSON) or Result: 0 on env-mode smoke" >&2
    echo "         hint: adjust build route/env and retry" >&2
    exit 2
  fi
fi

{
  echo "artifact_kind=${ARTIFACT_KIND}"
  echo "entry=${ENTRY}"
} > "$META_OUT"
echo "[stage1] metadata: $META_OUT" >&2
