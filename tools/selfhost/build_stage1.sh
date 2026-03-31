#!/usr/bin/env bash
# build_stage1.sh — Build Hakorune Stage1 bootstrap artifact
#
# Purpose
# - Produce Stage1 bootstrap artifacts from Nyash/Hako sources.
# - Artifact kind is explicit (`launcher-exe` or `stage1-cli`) to avoid contract drift.
# - Stage2 distribution packaging is a future SSOT concern and is not implemented here.
#
# Output
# - launcher-exe (default): target/selfhost/hakorune
# - stage1-cli:             target/selfhost/hakorune.stage1_cli
#
# Env / args
# - HAKORUNE_STAGE1_ENTRY: override Stage1 entry .hako (optional).
# - HAKORUNE_STAGE1_OUT:   override output path (optional).
# - NYASH_LLVM_SKIP_BUILD: set to 1 to skip cargo build in ny_mir_builder.sh when artifacts already exist.
# - HAKORUNE_STAGE1_REUSE_IF_FRESH: set to 1 to reuse existing artifact when up-to-date (default: 1).
# - HAKORUNE_STAGE1_ARTIFACT_KIND: default artifact kind override (`launcher-exe`|`stage1-cli`)
#   (Stage1 artifact kinds only; Stage2 packaging is separate.)
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

tree_has_newer_file() {
  local tree="$1"
  local out_path="$2"
  [ -e "$tree" ] || return 1
  if [ -d "$tree" ]; then
    find "$tree" -type f -newer "$out_path" -print -quit | grep -q .
    return $?
  fi
  [ "$tree" -nt "$out_path" ]
}

release_artifacts_are_fresh_for_skip() {
  local hakorune="$ROOT/target/release/hakorune"
  local nyllvmc="$ROOT/target/release/ny-llvmc"
  local ffi="$ROOT/target/release/libhako_llvmc_ffi.so"
  local kernel="$ROOT/target/release/libnyash_kernel.a"
  [ -x "$hakorune" ] || return 1
  [ -x "$nyllvmc" ] || return 1
  [ -f "$ffi" ] || return 1
  [ -f "$kernel" ] || return 1

  local freshness_roots=(
    "$ROOT/Cargo.toml"
    "$ROOT/Cargo.lock"
    "$ROOT/src"
    "$ROOT/lang/c-abi"
    "$ROOT/crates/nyash_kernel/Cargo.toml"
    "$ROOT/crates/nyash_kernel/src"
    "$ROOT/tools/build_hako_llvmc_ffi.sh"
  )
  local dep
  for dep in "${freshness_roots[@]}"; do
    if tree_has_newer_file "$dep" "$hakorune"; then
      return 1
    fi
    if tree_has_newer_file "$dep" "$kernel"; then
      return 1
    fi
  done
  return 0
}

build_hako_llvmc_ffi() {
  bash "$ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null
}

usage() {
  cat <<'USAGE'
build_stage1.sh — Build Hakorune Stage1 bootstrap artifact

Usage:
  tools/selfhost/build_stage1.sh [--artifact-kind <launcher-exe|stage1-cli>] [--out <exe_path>] [--entry <entry.hako>] [--timeout-secs <secs>] [--reuse-if-fresh <0|1>] [--force-rebuild]

Defaults:
  artifact-kind: launcher-exe
  launcher-exe entry/out:
    entry .hako : lang/src/runner/launcher_native_entry.hako
    output exe  : target/selfhost/hakorune
  stage1-cli entry/out:
    entry .hako : lang/src/runner/stage1_cli_env_entry.hako
    output exe  : target/selfhost/hakorune.stage1_cli
  Artifact semantics:
    launcher-exe / stage1-cli are Stage1 artifact kinds; Stage2 packaging is separate.

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
source "$ROOT/tools/selfhost/lib/identity_routes.sh"

export HAKO_BACKEND_COMPILE_RECIPE="${HAKO_BACKEND_COMPILE_RECIPE:-pure-first}"
export HAKO_BACKEND_COMPAT_REPLAY="${HAKO_BACKEND_COMPAT_REPLAY:-none}"

ARTIFACT_KIND="${HAKORUNE_STAGE1_ARTIFACT_KIND:-launcher-exe}"

if [ -z "${NYASH_BIN:-}" ]; then
  if [ -x "$ROOT/target/release/hakorune" ]; then
    NYASH_BIN="$ROOT/target/release/hakorune"
  elif [ -x "$ROOT/target/release/nyash" ]; then
    NYASH_BIN="$ROOT/target/release/nyash"
  elif [ "$ARTIFACT_KIND" = "stage1-cli" ]; then
    if [ -x "$ROOT/target/selfhost/hakorune.stage1_cli.stage2" ]; then
      NYASH_BIN="$ROOT/target/selfhost/hakorune.stage1_cli.stage2"
    elif [ -x "$ROOT/target/selfhost/hakorune.stage1_cli.next" ]; then
      NYASH_BIN="$ROOT/target/selfhost/hakorune.stage1_cli.next"
    fi
  fi
  if [ -z "${NYASH_BIN:-}" ]; then
    echo "[stage1] error: NYASH_BIN not set and no bootstrap binary found under target/release" >&2
    exit 2
  fi
fi

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
  local tmp_prog_raw tmp_prog tmp_mir_raw tmp_mir
  tmp_prog_raw="$(mktemp --suffix .stage1_cli_bootstrap.program.raw.json)"
  tmp_prog="$(mktemp --suffix .stage1_cli_bootstrap.program.json)"
  tmp_mir_raw="$(mktemp --suffix .stage1_cli_bootstrap.mir.raw.json)"
  tmp_mir="$(mktemp --suffix .stage1_cli_bootstrap.mir.json)"

  if ! stage1_contract_exec_direct_emit_mode \
    "$NYASH_BIN" \
    "emit-program" \
    "$ENTRY" >"$tmp_prog_raw" 2>/dev/null; then
    cleanup_stage_temp_files "$tmp_prog_raw" "$tmp_prog" "$tmp_mir_raw" "$tmp_mir"
    return 1
  fi

  cp "$tmp_prog_raw" "$tmp_prog"

  if ! stage1_contract_exec_direct_emit_mode \
    "$NYASH_BIN" \
    "emit-mir" \
    "$ENTRY" >"$tmp_mir_raw" 2>/dev/null; then
    echo "[stage1] stage1-cli bootstrap direct mir-json route failed: $ENTRY" >&2
    cleanup_stage_temp_files "$tmp_prog_raw" "$tmp_prog" "$tmp_mir_raw" "$tmp_mir"
    return 1
  fi

  cp "$tmp_mir_raw" "$tmp_mir"

  build_hako_llvmc_ffi

  if ! NYASH_LLVM_BACKEND=crate \
    NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
    NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
      bash "$ROOT/tools/ny_mir_builder.sh" --in "$tmp_mir" --emit exe -o "$OUT" --quiet >/dev/null; then
    echo "[stage1] stage1-cli bootstrap ny_mir_builder failed: $ENTRY" >&2
    cleanup_stage_temp_files "$tmp_prog_raw" "$tmp_prog" "$tmp_mir_raw" "$tmp_mir"
    return 1
  fi
  cleanup_stage_temp_files "$tmp_prog_raw" "$tmp_prog" "$tmp_mir_raw" "$tmp_mir"
}

build_with_launcher_bootstrap() {
  local tmp_mir
  tmp_mir="$(mktemp --suffix .launcher_bootstrap.mir.json)"
  trap 'rm -f "$tmp_mir" 2>/dev/null || true' RETURN

  if ! env NYASH_USE_STAGE1_CLI=1 \
    "$NYASH_BIN" --emit-mir-json "$tmp_mir" "$ENTRY" >/dev/null 2>&1; then
    echo "[stage1] launcher direct mir-json route failed: $ENTRY" >&2
    return 1
  fi

  build_hako_llvmc_ffi

  if ! NYASH_LLVM_BACKEND=crate \
    NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
    NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
      bash "$ROOT/tools/ny_mir_builder.sh" --in "$tmp_mir" --emit exe -o "$OUT" --quiet >/dev/null; then
    echo "[stage1] launcher bootstrap ny_mir_builder failed: $ENTRY" >&2
    return 1
  fi
}

ENTRY_DEFAULT_LAUNCHER="$ROOT/lang/src/runner/launcher_native_entry.hako"
ENTRY_DEFAULT_STAGE1_CLI="$ROOT/lang/src/runner/stage1_cli_env_entry.hako"
OUT_DEFAULT_LAUNCHER="$ROOT/target/selfhost/hakorune"
OUT_DEFAULT_STAGE1_CLI="$ROOT/target/selfhost/hakorune.stage1_cli"
TIMEOUT_SECS_DEFAULT=900

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
  "$ROOT/tools/build_hako_llvmc_ffi.sh"
  "$ROOT/lang/src/compiler/entry/compiler_stageb.hako"
  "$ROOT/lang/c-abi"
  "$ROOT/target/release/hakorune"
  "$ROOT/target/release/ny-llvmc"
  "$ROOT/target/release/libhako_llvmc_ffi.so"
  "$ROOT/target/release/libnyash_kernel.a"
  "$ROOT/crates/nyash_kernel/target/release/libnyash_kernel.a"
)

echo "[stage1] building Stage1 bootstrap artifact" >&2
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
  if release_artifacts_are_fresh_for_skip; then
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
  if [ "$ARTIFACT_KIND" = "stage1-cli" ] && [ "$BOOTSTRAP_KIND" != "stage1-cli" ] && [ "${NYASH_BIN:-}" != "$ROOT/target/release/hakorune" ] && [ "${NYASH_BIN:-}" != "$ROOT/target/release/nyash" ]; then
    if [ -x "$ROOT/target/selfhost/hakorune.stage1_cli.stage2" ]; then
      NYASH_BIN="$ROOT/target/selfhost/hakorune.stage1_cli.stage2"
      BOOTSTRAP_KIND="stage1-cli"
    elif [ -x "$ROOT/target/selfhost/hakorune.stage1_cli.next" ]; then
      NYASH_BIN="$ROOT/target/selfhost/hakorune.stage1_cli.next"
      BOOTSTRAP_KIND="stage1-cli"
    fi
  fi
  # stage1-cli artifacts are a contract-specific lane; they should use the
  # dedicated bridge-first bootstrap path even when the bootstrap binary does
  # not carry explicit artifact metadata.
  if [ "$ARTIFACT_KIND" = "stage1-cli" ] || [ "$BOOTSTRAP_KIND" = "stage1-cli" ]; then
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
        source "$ROOT/tools/selfhost/lib/identity_routes.sh"
        tmp_prog_raw="$(mktemp --suffix .stage1_cli_bootstrap.program.raw.json)"
        tmp_prog="$(mktemp --suffix .stage1_cli_bootstrap.program.json)"
        tmp_mir_raw="$(mktemp --suffix .stage1_cli_bootstrap.mir.raw.json)"
        tmp_mir="$(mktemp --suffix .stage1_cli_bootstrap.mir.json)"
        trap '\''rm -f "$tmp_prog_raw" "$tmp_prog" "$tmp_mir_raw" "$tmp_mir" 2>/dev/null || true'\'' EXIT
        if ! stage1_contract_exec_direct_emit_mode "$NYASH_BIN" "emit-program" "$ENTRY" >"$tmp_prog_raw" 2>/dev/null; then
          echo "[stage1] stage1-cli bootstrap direct program-json route failed: $ENTRY" >&2
          exit 1
        fi
        cp "$tmp_prog_raw" "$tmp_prog"
        if ! stage1_contract_exec_direct_emit_mode "$NYASH_BIN" "emit-mir" "$ENTRY" >"$tmp_mir_raw" 2>/dev/null; then
          echo "[stage1] stage1-cli bootstrap direct mir-json route failed: $ENTRY" >&2
          exit 1
        fi
        cp "$tmp_mir_raw" "$tmp_mir"
        bash "$ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null
        if ! NYASH_LLVM_BACKEND=crate \
          NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
          NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
      bash "$ROOT/tools/ny_mir_builder.sh" --in "$tmp_mir" --emit exe -o "$OUT" --quiet >/dev/null; then
          echo "[stage1] stage1-cli bootstrap ny_mir_builder failed: $ENTRY" >&2
          exit 1
        fi
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
  elif [ "$BOOTSTRAP_KIND" = "launcher-exe" ]; then
    echo "         bootstrap: launcher-exe env-build" >&2
    if [ "$TIMEOUT_SECS" -gt 0 ]; then
      set +e
      timeout --preserve-status "${TIMEOUT_SECS}s" bash -lc '
        set -euo pipefail
        ROOT="$1"
        ENTRY="$2"
        OUT="$3"
        NYASH_BIN="$4"
        tmp_mir="$(mktemp --suffix .launcher_bootstrap.mir.json)"
        trap '\''rm -f "$tmp_mir" 2>/dev/null || true'\'' EXIT
        if ! env NYASH_USE_STAGE1_CLI=1 \
          "$NYASH_BIN" --emit-mir-json "$tmp_mir" "$ENTRY" >/dev/null 2>&1; then
          echo "[stage1] launcher direct mir-json route failed: $ENTRY" >&2
          exit 1
        fi
        bash "$ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null
        if ! NYASH_LLVM_BACKEND=crate \
          NYASH_NY_LLVM_COMPILER="${NYASH_NY_LLVM_COMPILER:-$ROOT/target/release/ny-llvmc}" \
          NYASH_EMIT_EXE_NYRT="${NYASH_EMIT_EXE_NYRT:-$ROOT/target/release}" \
      bash "$ROOT/tools/ny_mir_builder.sh" --in "$tmp_mir" --emit exe -o "$OUT" --quiet >/dev/null; then
          echo "[stage1] launcher bootstrap ny_mir_builder failed: $ENTRY" >&2
          exit 1
        fi
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
      build_with_launcher_bootstrap
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

    # The bootstrap payload proof stays on the stage0 compatibility route.
    # The reduced stage1-cli artifact itself is treated as a runnable bootstrap
    # output and is checked with the same input-bearing contract used by G1.
    if stage1_contract_verify_stage1_cli_bootstrap_capability \
      "$ROOT/target/release/hakorune" \
      "$PROBE_SRC" \
      "$OUT"; then
      :
    else
      rc=$?
      echo "[stage1] stage1-cli capability check failed" >&2
      case "$rc" in
        1) echo "         stage0 bootstrap route failed Program(JSON v0) proof" >&2 ;;
        2) echo "         stage0 bootstrap route failed MIR(JSON v0) proof" >&2 ;;
        3) echo "         reduced artifact failed runnable liveness contract" >&2 ;;
        *) echo "         bootstrap capability helper failed (rc=$rc)" >&2 ;;
      esac
      exit 2
    fi
    echo "[stage1] stage1-cli capability: OK (stage0 bootstrap proof + runnable reduced artifact)" >&2
  fi

{
  echo "artifact_kind=${ARTIFACT_KIND}"
  echo "entry=${ENTRY}"
} > "$META_OUT"
echo "[stage1] metadata: $META_OUT" >&2
