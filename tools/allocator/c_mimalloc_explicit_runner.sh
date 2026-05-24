#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
SRC="$ROOT_DIR/tools/allocator/c_mimalloc_explicit_runner.c"
OUT_FILE=""
LIBRARY_PATH=""
WORKLOAD="representative-small-block-v0"
ALLOC_COUNT="64"
BLOCK_SIZE="512"
ALLOW_LDCONFIG_DISCOVERY=0

usage() {
  cat >&2 <<'USAGE'
usage: tools/allocator/c_mimalloc_explicit_runner.sh --out FILE [--library PATH] [--workload ID] [--allow-ldconfig-discovery]

Runs the MIMAP-451A explicit C mimalloc runner. The preferred path is an
explicit --library PATH. --allow-ldconfig-discovery is a guard/tool convenience:
the resolved path is printed and passed to the C runner explicitly.
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --out)
      OUT_FILE="${2:-}"
      shift 2
      ;;
    --library)
      LIBRARY_PATH="${2:-}"
      shift 2
      ;;
    --workload)
      WORKLOAD="${2:-}"
      shift 2
      ;;
    --alloc-count)
      ALLOC_COUNT="${2:-}"
      shift 2
      ;;
    --block-size)
      BLOCK_SIZE="${2:-}"
      shift 2
      ;;
    --allow-ldconfig-discovery)
      ALLOW_LDCONFIG_DISCOVERY=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[c-mimalloc-runner] ERROR: unknown argument: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [ -z "$OUT_FILE" ]; then
  echo "[c-mimalloc-runner] ERROR: --out FILE is required" >&2
  usage
  exit 2
fi

if [ -z "$LIBRARY_PATH" ]; then
  if [ "$ALLOW_LDCONFIG_DISCOVERY" -ne 1 ]; then
    echo "[c-mimalloc-runner] ERROR: --library PATH is required unless --allow-ldconfig-discovery is explicit" >&2
    exit 2
  fi
  LIBRARY_PATH="$(ldconfig -p 2>/dev/null | awk '/libmimalloc\.so\.2[[:space:]]/ { print $NF; exit }' || true)"
fi

if [ -z "$LIBRARY_PATH" ] || [ ! -f "$LIBRARY_PATH" ]; then
  echo "[c-mimalloc-runner] ERROR: libmimalloc.so.2 not found; pass --library PATH" >&2
  exit 3
fi

if [ ! -x /usr/bin/time ]; then
  echo "[c-mimalloc-runner] ERROR: /usr/bin/time is required for external RSS evidence" >&2
  exit 2
fi

tmp_dir="$(mktemp -d /tmp/hakorune_c_mimalloc_runner.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
bin="$tmp_dir/c_mimalloc_explicit_runner"
tmp_out="$tmp_dir/runner.out"
time_out="$tmp_dir/time.out"

cc -std=c11 -O2 -Wall -Wextra "$SRC" -ldl -o "$bin"

echo "[c-mimalloc-runner] library=$LIBRARY_PATH" >&2
set +e
/usr/bin/time -f '%e %M' -o "$time_out" "$bin" --library "$LIBRARY_PATH" --workload "$WORKLOAD" --alloc-count "$ALLOC_COUNT" --block-size "$BLOCK_SIZE" >"$tmp_out"
run_rc="$?"
set -e

read -r external_elapsed_seconds peak_rss_kb < "$time_out" || true
external_elapsed_seconds="${external_elapsed_seconds:-0}"
peak_rss_kb="${peak_rss_kb:-0}"
case "$peak_rss_kb" in
  ''|*[!0-9]*) peak_rss_kb=0 ;;
esac
external_elapsed_ms="$(python3 - "$external_elapsed_seconds" <<'PY'
import sys
try:
    elapsed_ms = int(round(float(sys.argv[1]) * 1000))
    print(elapsed_ms if elapsed_ms > 0 else 1)
except Exception:
    print(0)
PY
)"
echo "external_elapsed_ms=$external_elapsed_ms" >>"$tmp_out"
echo "external_peak_rss_bytes=$((peak_rss_kb * 1024))" >>"$tmp_out"

mv "$tmp_out" "$OUT_FILE"
cat "$OUT_FILE"
exit "$run_rc"
