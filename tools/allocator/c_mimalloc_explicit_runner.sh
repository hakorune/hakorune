#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
SRC="$ROOT_DIR/tools/allocator/c_mimalloc_explicit_runner.c"
OUT_FILE=""
LIBRARY_PATH=""
WORKLOAD="representative-small-block-v0"
ALLOC_COUNT="64"
BLOCK_SIZE="512"
OPERATION_REPEAT="1"
ALLOW_LDCONFIG_DISCOVERY=0

usage() {
  cat >&2 <<'USAGE'
usage: tools/allocator/c_mimalloc_explicit_runner.sh --out FILE [--library PATH] [--workload ID] [--operation-repeat N] [--allow-ldconfig-discovery]

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
    --operation-repeat)
      OPERATION_REPEAT="${2:-}"
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

case "$OPERATION_REPEAT" in
  ''|*[!0-9]*)
    echo "[c-mimalloc-runner] ERROR: --operation-repeat must be a positive integer" >&2
    exit 2
    ;;
esac
if [ "$OPERATION_REPEAT" -lt 1 ]; then
  echo "[c-mimalloc-runner] ERROR: --operation-repeat must be >= 1" >&2
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
last_out="$tmp_dir/runner.last.out"
time_out="$tmp_dir/time.out"

cc -std=c11 -O2 -Wall -Wextra "$SRC" -ldl -o "$bin"

echo "[c-mimalloc-runner] library=$LIBRARY_PATH" >&2
set +e
/usr/bin/time -f '%e %M' -o "$time_out" bash -c '
  i=0
  repeat="$1"
  bin="$2"
  library_path="$3"
  workload="$4"
  alloc_count="$5"
  block_size="$6"
  last_out="$7"
  while [ "$i" -lt "$repeat" ]; do
    "$bin" --library "$library_path" --workload "$workload" --alloc-count "$alloc_count" --block-size "$block_size" >"$last_out" || exit "$?"
    i=$((i + 1))
  done
' _ "$OPERATION_REPEAT" "$bin" "$LIBRARY_PATH" "$WORKLOAD" "$ALLOC_COUNT" "$BLOCK_SIZE" "$last_out"
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
python3 - "$last_out" "$tmp_out" "$OPERATION_REPEAT" <<'PY'
import sys
from pathlib import Path

src = Path(sys.argv[1])
dst = Path(sys.argv[2])
repeat = int(sys.argv[3])
scale_keys = {
    "allocation_count",
    "free_count",
    "requested_bytes",
    "realloc_count",
    "aligned_alloc_count",
    "alignment_request_count",
    "alignment_ok_count",
    "alignment_reject_count",
    "large_request_count",
    "realloc_same_ptr_count",
    "realloc_moved_count",
    "copied_bytes",
}
out = []
for raw in src.read_text(encoding="utf-8", errors="replace").splitlines():
    if "=" not in raw:
        out.append(raw)
        continue
    key, value = raw.split("=", 1)
    if key == "run_count":
        out.append(f"run_count={repeat}")
    elif key in scale_keys:
        out.append(f"{key}={int(value) * repeat}")
    else:
        out.append(raw)
out.append(f"operation_repeat={repeat}")
out.append("timing_repeat_kind=process-invocation-v0")
dst.write_text("\n".join(out) + "\n", encoding="utf-8")
PY
echo "external_elapsed_ms=$external_elapsed_ms" >>"$tmp_out"
echo "external_peak_rss_bytes=$((peak_rss_kb * 1024))" >>"$tmp_out"

mv "$tmp_out" "$OUT_FILE"
cat "$OUT_FILE"
exit "$run_rc"
