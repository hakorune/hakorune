#!/usr/bin/env bash
# bug_origin_triage.sh
#
# Internal engineering triage helper to identify likely bug origin across vm-family lanes:
# - Rust VM route (--backend vm)
# - Stage1 .hako route (--backend vm --hako-run)
# - vm-hako route (--backend vm-hako)
#
# This is not a front-door runtime surface. Use it for focused route triage only.
#
# Usage:
#   tools/dev/bug_origin_triage.sh <source.hako> [--expect '<regex>'] [--expect-rc <n>] [--timeout <sec>] [--stage1-entry <path>] [--keep-temp]

set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
BIN="${NYASH_BIN:-$ROOT_DIR/target/release/hakorune}"

if [ $# -lt 1 ]; then
  echo "usage: tools/dev/bug_origin_triage.sh <source.hako> [--expect '<regex>'] [--expect-rc <n>] [--timeout <sec>] [--stage1-entry <path>] [--keep-temp]" >&2
  exit 2
fi

SOURCE="$1"
shift

EXPECT_REGEX=""
EXPECT_RC=""
TIMEOUT_SECS=30
STAGE1_ENTRY=""
KEEP_TEMP=0
PRESERVE_ON_EXIT=0

while [ $# -gt 0 ]; do
  case "$1" in
    --expect)
      EXPECT_REGEX="${2:-}"
      shift 2
      ;;
    --expect-rc)
      EXPECT_RC="${2:-}"
      shift 2
      ;;
    --timeout)
      TIMEOUT_SECS="${2:-}"
      shift 2
      ;;
    --stage1-entry)
      STAGE1_ENTRY="${2:-}"
      shift 2
      ;;
    --keep-temp)
      KEEP_TEMP=1
      shift
      ;;
    *)
      echo "unknown option: $1" >&2
      exit 2
      ;;
  esac
done

if [ ! -x "$BIN" ]; then
  echo "[triage] hakorune binary not found/executable: $BIN" >&2
  exit 2
fi

if [[ "$SOURCE" != /* ]]; then
  SOURCE="$ROOT_DIR/$SOURCE"
fi
if [ ! -f "$SOURCE" ]; then
  echo "[triage] source missing: $SOURCE" >&2
  exit 2
fi

if ! [[ "$TIMEOUT_SECS" =~ ^[0-9]+$ ]]; then
  echo "[triage] --timeout must be integer seconds: $TIMEOUT_SECS" >&2
  exit 2
fi

if [ -n "$EXPECT_RC" ] && ! [[ "$EXPECT_RC" =~ ^-?[0-9]+$ ]]; then
  echo "[triage] --expect-rc must be integer: $EXPECT_RC" >&2
  exit 2
fi

if [ -z "$STAGE1_ENTRY" ] && [ -f "$ROOT_DIR/tools/hako_check/cli.hako" ]; then
  STAGE1_ENTRY="$ROOT_DIR/tools/hako_check/cli.hako"
fi

TMP_DIR="$(mktemp -d /tmp/bug_origin_triage.XXXXXX)"
cleanup() {
  if [ "$KEEP_TEMP" -eq 1 ] || [ "$PRESERVE_ON_EXIT" -eq 1 ]; then
    echo "[triage] temp kept: $TMP_DIR" >&2
    return
  fi
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

run_route() {
  local route="$1"
  shift
  local out_file="$TMP_DIR/${route}.out"
  local rc_file="$TMP_DIR/${route}.rc"
  local status_file="$TMP_DIR/${route}.status"

  set +e
  timeout "$TIMEOUT_SECS" "$@" >"$out_file" 2>&1
  local rc=$?
  set -e

  printf '%s' "$rc" >"$rc_file"
  if [ "$rc" -eq 124 ]; then
    echo "timeout" >"$status_file"
  else
    echo "done" >"$status_file"
  fi
}

check_expect() {
  local route="$1"
  local out_file="$TMP_DIR/${route}.out"
  local rc
  rc="$(cat "$TMP_DIR/${route}.rc")"
  local status
  status="$(cat "$TMP_DIR/${route}.status")"

  if [ "$status" = "timeout" ]; then
    echo "unknown"
    return
  fi

  local ok=1
  if [ -n "$EXPECT_RC" ] && [ "$rc" != "$EXPECT_RC" ]; then
    ok=0
  fi

  if [ -n "$EXPECT_REGEX" ]; then
    if ! rg -q -- "$EXPECT_REGEX" "$out_file"; then
      ok=0
    fi
  fi

  if [ "$ok" -eq 1 ]; then
    echo "pass"
  else
    echo "fail"
  fi
}

has_tag() {
  local route="$1"
  local pattern="$2"
  rg -q -- "$pattern" "$TMP_DIR/${route}.out"
}

print_route_summary() {
  local route="$1"
  local title="$2"
  local rc status verdict
  rc="$(cat "$TMP_DIR/${route}.rc")"
  status="$(cat "$TMP_DIR/${route}.status")"
  verdict="$(check_expect "$route")"
  echo "[route] $title"
  echo "  rc=$rc status=$status verdict=$verdict out=$TMP_DIR/${route}.out"
}

stage1_env=()
if [ -n "$STAGE1_ENTRY" ]; then
  stage1_env+=(STAGE1_CLI_ENTRY="$STAGE1_ENTRY")
fi

run_route "rust_vm" env \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  "$BIN" --backend vm "$SOURCE"

run_route "stage1_run" env \
  NYASH_DISABLE_PLUGINS=1 \
  NYASH_VM_HAKO_PREFER_STRICT_DEV=0 \
  NYASH_JOINIR_DEV=0 \
  NYASH_JOINIR_STRICT=0 \
  HAKO_JOINIR_STRICT=0 \
  HAKO_JOINIR_PLANNER_REQUIRED=0 \
  "${stage1_env[@]}" \
  "$BIN" --backend vm --hako-run "$SOURCE"

run_route "vm_hako" env \
  NYASH_DISABLE_PLUGINS=1 \
  "$BIN" --backend vm-hako "$SOURCE"

echo "[triage] source=$SOURCE"
if [ -n "$EXPECT_REGEX" ]; then
  echo "[triage] expect regex: $EXPECT_REGEX"
fi
if [ -n "$EXPECT_RC" ]; then
  echo "[triage] expect rc: $EXPECT_RC"
fi
echo "[triage] timeout: ${TIMEOUT_SECS}s"
if [ -n "$STAGE1_ENTRY" ]; then
  echo "[triage] stage1-entry: $STAGE1_ENTRY"
fi
echo

print_route_summary "rust_vm" "vm (--backend vm)"
print_route_summary "stage1_run" "stage1-route (--hako-run)"
print_route_summary "vm_hako" "vm-hako (--backend vm-hako)"
echo

rust_v="$(check_expect rust_vm)"
stage1_v="$(check_expect stage1_run)"
vmh_v="$(check_expect vm_hako)"

rust_timeout="$(cat "$TMP_DIR/rust_vm.status")"
stage1_timeout="$(cat "$TMP_DIR/stage1_run.status")"
vmh_timeout="$(cat "$TMP_DIR/vm_hako.status")"

echo "[diagnosis]"

if [ "$rust_timeout" = "timeout" ]; then
  PRESERVE_ON_EXIT=1
  echo "  - vm route timed out; cannot classify. Check parser/compile recursion first."
elif [ -z "$EXPECT_REGEX" ] && [ -z "$EXPECT_RC" ]; then
  PRESERVE_ON_EXIT=1
  echo "  - no expectation provided; verdict is tag-based only."
  if has_tag "vm_hako" '^\[vm-hako/unimplemented\]'; then
    echo "  - vm-hako lane reports subset unimplemented (runtime capability gap)."
  fi
  if has_tag "stage1_run" '\[stage1-cli\]'; then
    echo "  - stage1 route emits stage1-cli contract tags (stage1 lane issue likely)."
  fi
  echo "  - add --expect/--expect-rc for stronger origin classification."
else
  if [ "$rust_v" != "pass" ] || [ "$stage1_v" != "pass" ] || [ "$vmh_v" != "pass" ]; then
    PRESERVE_ON_EXIT=1
  fi

  if [ "$rust_v" = "pass" ]; then
    if [ "$stage1_v" = "fail" ]; then
      echo "  - vm matches expectation but stage1-route does not."
      echo "  - likely origin: .hako compiler/stage1 lane."
    elif [ "$stage1_v" = "unknown" ]; then
      echo "  - stage1-route timed out/unknown; stage1 stability issue may hide .hako lane bugs."
    fi

    if [ "$vmh_v" = "fail" ]; then
      if has_tag "vm_hako" '^\[vm-hako/unimplemented\]'; then
        echo "  - vm-hako mismatch is due to subset unimplemented."
        echo "  - likely origin: vm-hako capability gap (not vm semantics)."
      else
        echo "  - vm-hako mismatches without unimplemented tag."
        echo "  - likely origin: vm-hako runtime semantics."
      fi
    elif [ "$vmh_v" = "unknown" ]; then
      echo "  - vm-hako timed out/unknown."
    fi

    if [ "$stage1_v" = "pass" ] && [ "$vmh_v" = "pass" ]; then
      echo "  - all routes match expectation (no repro)."
    fi
  else
    if [ "$stage1_v" = "pass" ]; then
      echo "  - stage1-route matches expectation but vm does not."
      echo "  - likely origin: Rust compiler/runtime lane."
    elif [ "$stage1_v" = "fail" ] || [ "$stage1_v" = "unknown" ]; then
      echo "  - vm also fails expectation and stage1 is not clearly healthy."
      echo "  - likely origin: shared frontend/core lowering contract (SSOT-level bug)."
    fi
  fi
fi

echo
if [ "$KEEP_TEMP" -eq 1 ] || [ "$PRESERVE_ON_EXIT" -eq 1 ]; then
  echo "[hint] inspect raw logs:"
  echo "  - vm:         $TMP_DIR/rust_vm.out"
  echo "  - stage1_run: $TMP_DIR/stage1_run.out"
  echo "  - vm_hako:    $TMP_DIR/vm_hako.out"
else
  echo "[hint] all routes passed; re-run with --keep-temp if you need raw logs."
fi
