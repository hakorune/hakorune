#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
source "$(dirname "$0")/lib/guard_common.sh"

TAG="windows-wsl-cmd-smoke"
TARGET_TRIPLE="${WINDOWS_WSL_TARGET:-x86_64-pc-windows-msvc}"
DO_BUILD="${WINDOWS_WSL_BUILD:-0}"
DO_CMD_SMOKE="${WINDOWS_WSL_CMD_SMOKE:-0}"

usage() {
  cat <<'USAGE'
Usage:
  bash tools/checks/windows_wsl_cmd_smoke.sh [--build] [--cmd-smoke] [--target <triple>]

Options:
  --build        Run `cargo xwin build --release --bin hakorune`.
  --cmd-smoke    Run `hakorune.exe --help` through Windows cmd.exe from WSL.
  --target       Windows target triple (default: x86_64-pc-windows-msvc).

Environment overrides:
  WINDOWS_WSL_BUILD=0|1
  WINDOWS_WSL_CMD_SMOKE=0|1
  WINDOWS_WSL_TARGET=<triple>
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --build)
      DO_BUILD=1
      ;;
    --cmd-smoke)
      DO_CMD_SMOKE=1
      ;;
    --target)
      shift
      if [[ $# -eq 0 ]]; then
        guard_fail "$TAG" "--target requires a value"
      fi
      TARGET_TRIPLE="$1"
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      usage >&2
      guard_fail "$TAG" "unknown argument: $1"
      ;;
  esac
  shift
done

cd "$ROOT_DIR"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" rg
if ! rg -qi 'microsoft' /proc/version 2>/dev/null; then
  echo "[$TAG] skip: non-WSL environment"
  exit 0
fi

echo "[$TAG] target=$TARGET_TRIPLE build=$DO_BUILD cmd_smoke=$DO_CMD_SMOKE"

if [[ "$DO_BUILD" == "1" ]]; then
  if ! cargo xwin --version >/dev/null 2>&1; then
    guard_fail "$TAG" "cargo-xwin is required for --build (install: cargo install cargo-xwin)"
  fi
  echo "[$TAG] building windows binary via cargo-xwin"
  cargo xwin build --target "$TARGET_TRIPLE" --release --bin hakorune
fi

exe_rel=""
for candidate in \
  "target/$TARGET_TRIPLE/release/hakorune.exe" \
  "target/$TARGET_TRIPLE/release/nyash.exe"
do
  if [[ -f "$candidate" ]]; then
    exe_rel="$candidate"
    break
  fi
done

if [[ -z "$exe_rel" ]]; then
  if [[ "$DO_BUILD" == "1" ]]; then
    guard_fail "$TAG" "windows executable missing after build (expected hakorune.exe or nyash.exe)"
  fi
  echo "[$TAG] note: windows executable not found; run with --build to produce one"
else
  echo "[$TAG] found executable: $exe_rel"
fi

if [[ "$DO_CMD_SMOKE" == "1" ]]; then
  guard_require_command "$TAG" cmd.exe
  guard_require_command "$TAG" wslpath
  if [[ -z "$exe_rel" ]]; then
    guard_fail "$TAG" "--cmd-smoke requested but executable not found (use --build first)"
  fi
  repo_win="$(wslpath -w "$ROOT_DIR")"
  exe_win="${exe_rel//\//\\}"
  echo "[$TAG] running cmd smoke: $exe_win --help"
  if [[ "$repo_win" == \\\\wsl.localhost\\* || "$repo_win" == \\\\wsl\\* ]]; then
    stage_dir_wsl="${WINDOWS_WSL_STAGE_DIR:-/mnt/c/Temp/hakorune_wsl_smoke}"
    if [[ ! -d /mnt/c ]]; then
      guard_fail "$TAG" "UNC path detected but /mnt/c is unavailable (set WINDOWS_WSL_STAGE_DIR to a Windows-mounted path)"
    fi
    mkdir -p "$stage_dir_wsl"
    cp "$exe_rel" "$stage_dir_wsl/hakorune.exe"
    stage_dir_win="$(wslpath -w "$stage_dir_wsl")"
    if [[ "$stage_dir_win" == *" "* ]]; then
      guard_fail "$TAG" "stage path contains spaces (set WINDOWS_WSL_STAGE_DIR without spaces)"
    fi
    echo "[$TAG] UNC repo detected; staging exe into $stage_dir_win"
    cmd.exe /C "cd /d $stage_dir_win && hakorune.exe --help >NUL"
  else
    if [[ "$repo_win" == *" "* || "$exe_win" == *" "* ]]; then
      guard_fail "$TAG" "repo or executable path contains spaces (unsupported in cmd smoke command)"
    fi
    cmd.exe /C "cd /d $repo_win && $exe_win --help >NUL"
  fi
  echo "[$TAG] cmd smoke ok"
fi

echo "[$TAG] ok"
