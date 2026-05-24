#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP=""
OUT_FILE=""
WORKLOAD="hako-exe-workload-v0"
RUNTIME_CONFIG="root"

usage() {
  cat >&2 <<'USAGE'
usage: tools/allocator/hako_exe_memory_runner.sh --app FILE --out FILE [--workload ID] [--runtime-config root|empty]

Builds a selected .hako app through the exact-MIR EXE route, runs that EXE as
an external process, and records stable memory-use evidence. This tool is
measurement evidence only; it does not replace the process allocator.

--runtime-config empty runs the exact EXE from a generated working directory
containing an empty nyash.toml [libraries] profile. It is diagnostic-only and
does not change default NyRT behavior.
USAGE
}

while [ "$#" -gt 0 ]; do
  case "$1" in
    --app)
      APP="${2:-}"
      shift 2
      ;;
    --out)
      OUT_FILE="${2:-}"
      shift 2
      ;;
    --workload)
      WORKLOAD="${2:-}"
      shift 2
      ;;
    --runtime-config)
      RUNTIME_CONFIG="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[hako-exe-memory-runner] ERROR: unknown argument: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [ -z "$APP" ] || [ -z "$OUT_FILE" ]; then
  echo "[hako-exe-memory-runner] ERROR: --app and --out are required" >&2
  usage
  exit 2
fi

if [ ! -f "$APP" ]; then
  echo "[hako-exe-memory-runner] ERROR: app not found: $APP" >&2
  exit 2
fi

if [ ! -x /usr/bin/time ]; then
  echo "[hako-exe-memory-runner] ERROR: /usr/bin/time is required for peak RSS evidence" >&2
  exit 2
fi

case "$RUNTIME_CONFIG" in
  root|empty) ;;
  *)
    echo "[hako-exe-memory-runner] ERROR: --runtime-config must be root or empty" >&2
    exit 2
    ;;
esac

if [ ! -x "$ROOT_DIR/target/release/hakorune" ] || [ ! -x "$ROOT_DIR/target/release/ny-llvmc" ]; then
  cargo build --release --bin hakorune --bin ny-llvmc
fi

tmp_dir="$(mktemp -d /tmp/hakorune_hako_exe_memory_runner.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT

mir_json="$tmp_dir/app.mir.json"
exe_out="$tmp_dir/app.exe"
run_out="$tmp_dir/run.out"
run_err="$tmp_dir/run.err"
time_out="$tmp_dir/time.out"
tmp_report="$tmp_dir/report.out"
run_cwd="$ROOT_DIR"
if [ "$RUNTIME_CONFIG" = "empty" ]; then
  run_cwd="$tmp_dir/runtime-empty-config"
  mkdir -p "$run_cwd"
  cat > "$run_cwd/nyash.toml" <<'TOML'
[libraries]
TOML
fi

NYASH_FEATURES="${NYASH_FEATURES:-rune}" \
NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "$ROOT_DIR/target/release/hakorune" --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$ROOT_DIR/tools/checks/pure_first_route_preflight.py" "$mir_json" >/dev/null

NYASH_DISABLE_PLUGINS="${NYASH_DISABLE_PLUGINS:-1}" \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$mir_json" --exe "$exe_out" >/dev/null

set +e
(
  cd "$run_cwd"
  /usr/bin/time -f '%e %M' -o "$time_out" "$exe_out" >"$run_out" 2>"$run_err"
)
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
peak_rss_bytes=$((peak_rss_kb * 1024))

python3 - "$WORKLOAD" "$APP" "$RUNTIME_CONFIG" "$run_rc" "$peak_rss_bytes" "$external_elapsed_ms" "$run_out" >"$tmp_report" <<'PY'
import sys
from pathlib import Path

workload, app, runtime_config, rc_text, peak_rss_text, elapsed_ms_text, run_out_path = sys.argv[1:8]
rc = int(rc_text)
peak_rss = int(peak_rss_text)
elapsed_ms = int(elapsed_ms_text)
lines = Path(run_out_path).read_text(encoding="utf-8", errors="replace").splitlines()

summary_ok = 1 if any(line == "summary=ok" for line in lines) else 0
requested_bytes = 0
committed_bytes = 0
allocation_count = 0
free_count = 0
operation_family = ""
operation_sequence_id = ""
free_order_id = ""
realloc_count = 0
aligned_alloc_count = 0
alignment_request_count = 0
alignment_ok_count = 0
alignment_reject_count = 0
large_request_count = 0
realloc_same_ptr_count = 0
realloc_moved_count = 0
copied_bytes = 0

for line in lines:
    if line.startswith("operation_family="):
        operation_family = line.split("=", 1)[1]
    if line.startswith("operation_sequence_id="):
        operation_sequence_id = line.split("=", 1)[1]
    if line.startswith("free_order_id="):
        free_order_id = line.split("=", 1)[1]
    if line.startswith("allocation_count="):
        allocation_count = int(line.split("=", 1)[1])
    if line.startswith("free_count="):
        free_count = int(line.split("=", 1)[1])
    if line.startswith("realloc_count="):
        realloc_count = int(line.split("=", 1)[1])
    if line.startswith("aligned_alloc_count="):
        aligned_alloc_count = int(line.split("=", 1)[1])
    if line.startswith("alignment_request_count="):
        alignment_request_count = int(line.split("=", 1)[1])
    if line.startswith("alignment_ok_count="):
        alignment_ok_count = int(line.split("=", 1)[1])
    if line.startswith("alignment_reject_count="):
        alignment_reject_count = int(line.split("=", 1)[1])
    if line.startswith("large_request_count="):
        large_request_count = int(line.split("=", 1)[1])
    if line.startswith("realloc_same_ptr_count="):
        realloc_same_ptr_count = int(line.split("=", 1)[1])
    if line.startswith("realloc_moved_count="):
        realloc_moved_count = int(line.split("=", 1)[1])
    if line.startswith("copied_bytes="):
        copied_bytes = int(line.split("=", 1)[1])
    if line.startswith("page="):
        fields = line.split("=", 1)[1].split(",")
        if fields:
            allocation_count = int(fields[0])
        if len(fields) > 1:
            free_count = int(fields[1])
    if line.startswith("hako_requested="):
        fields = line.split("=", 1)[1].split(",")
        if fields:
            requested_bytes = int(fields[-1])
    if line.startswith("hako_evidence="):
        fields = line.split("=", 1)[1].split(",")
        if fields:
            committed_bytes = int(fields[0])
    if line.startswith("summary_fields="):
        fields = line.split("=", 1)[1].split(",")
        if fields:
            requested_bytes = int(fields[0])
        if len(fields) > 1:
            committed_bytes = int(fields[1])

print("hako_exe_runner=1")
print("output_contract=hako-exe-memory-evidence-v0")
print(f"workload={workload}")
print(f"operation_family={operation_family}")
print(f"operation_sequence_id={operation_sequence_id}")
print(f"free_order_id={free_order_id}")
print(f"app_path={app}")
print(f"runtime_config_profile={runtime_config}")
print(f"result_code={rc}")
print("run_count=1")
print(f"allocation_count={allocation_count}")
print(f"free_count={free_count}")
print(f"requested_bytes={requested_bytes}")
print(f"committed_bytes={committed_bytes}")
print(f"realloc_count={realloc_count}")
print(f"aligned_alloc_count={aligned_alloc_count}")
print(f"alignment_request_count={alignment_request_count}")
print(f"alignment_ok_count={alignment_ok_count}")
print(f"alignment_reject_count={alignment_reject_count}")
print(f"large_request_count={large_request_count}")
print(f"realloc_same_ptr_count={realloc_same_ptr_count}")
print(f"realloc_moved_count={realloc_moved_count}")
print(f"copied_bytes={copied_bytes}")
print(f"external_elapsed_ms={elapsed_ms}")
print(f"peak_rss_bytes={peak_rss}")
print(f"external_peak_rss_bytes={peak_rss}")
print("memory_usage_evidence=1")
print(f"output_summary_ok={summary_ok}")
print("provider_activation=0")
print("host_replacement=0")
print("hook_installed=0")
print("global_allocator_installed=0")
print("summary=ok" if rc == 0 and summary_ok == 1 and peak_rss > 0 else "summary=fail")
PY

mv "$tmp_report" "$OUT_FILE"
cat "$OUT_FILE"

if [ "$run_rc" -ne 0 ]; then
  echo "[hako-exe-memory-runner] app exited with $run_rc" >&2
  cat "$run_err" >&2 || true
  exit "$run_rc"
fi
