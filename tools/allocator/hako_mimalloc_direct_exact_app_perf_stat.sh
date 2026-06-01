#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
ENV_PRESET="${ROOT_DIR}/tools/allocator/mimalloc_direct_exact_env.sh"
APP=""
OUT_FILE=""
RUNS=3

usage() {
  cat >&2 <<'USAGE'
usage: tools/allocator/hako_mimalloc_direct_exact_app_perf_stat.sh --app FILE --out FILE [--runs N]

Builds one .hako app through the direct-exact EXE route and records perf stat
instruction/cycle medians. This is Hako-only; use it to compare production
facade vs comparison-only observer-light apps under the same direct-exact front.
USAGE
}

while [[ "$#" -gt 0 ]]; do
  case "$1" in
    --app)
      APP="${2:-}"
      shift 2
      ;;
    --out)
      OUT_FILE="${2:-}"
      shift 2
      ;;
    --runs)
      RUNS="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[mimalloc-direct-exact-app-stat] ERROR: unknown argument: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [[ -z "$APP" || -z "$OUT_FILE" ]]; then
  echo "[mimalloc-direct-exact-app-stat] ERROR: --app and --out are required" >&2
  usage
  exit 2
fi
if [[ ! -f "$APP" ]]; then
  echo "[mimalloc-direct-exact-app-stat] ERROR: app not found: $APP" >&2
  exit 2
fi
case "$RUNS" in
  ''|*[!0-9]*)
    echo "[mimalloc-direct-exact-app-stat] ERROR: --runs must be a positive integer" >&2
    exit 2
    ;;
esac
if [[ "$RUNS" -lt 1 ]]; then
  echo "[mimalloc-direct-exact-app-stat] ERROR: --runs must be >= 1" >&2
  exit 2
fi
if ! command -v perf >/dev/null 2>&1; then
  echo "[mimalloc-direct-exact-app-stat] ERROR: perf is required" >&2
  exit 2
fi

# shellcheck source=tools/allocator/mimalloc_direct_exact_env.sh
source "$ENV_PRESET"
mimalloc_direct_exact_env_check

if [[ ! -x "$ROOT_DIR/target/release/hakorune" || ! -x "$ROOT_DIR/target/release/ny-llvmc" ]]; then
  cargo build --release --bin hakorune --bin ny-llvmc
fi

tmp_dir="$(mktemp -d /tmp/hakorune_mimalloc_direct_exact_app_stat.XXXXXX)"
trap 'rm -rf "$tmp_dir"' EXIT
mir_json="$tmp_dir/app.mir.json"
exe_out="$tmp_dir/app.exe"
run_cwd="$tmp_dir/runtime-empty-config"
mkdir -p "$run_cwd"
cat > "$run_cwd/nyash.toml" <<'TOML'
[libraries]
TOML

NYASH_FEATURES="$NYASH_FEATURES" \
NYASH_DISABLE_PLUGINS="$NYASH_DISABLE_PLUGINS" \
  "$ROOT_DIR/target/release/hakorune" --backend mir --emit-mir-json "$mir_json" "$APP" >/dev/null

python3 "$ROOT_DIR/tools/checks/pure_first_route_preflight.py" "$mir_json" >/dev/null

NYASH_DISABLE_PLUGINS="$NYASH_DISABLE_PLUGINS" \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$mir_json" --exe "$exe_out" >/dev/null

sample_file="$tmp_dir/samples.tsv"
for i in $(seq 1 "$RUNS"); do
  stat_file="$tmp_dir/perf.$i.txt"
  run_out="$tmp_dir/run.$i.out"
  run_err="$tmp_dir/run.$i.err"
  (
    cd "$run_cwd"
    env \
      NYASH_FEATURES="$NYASH_FEATURES" \
      NYASH_DISABLE_PLUGINS="$NYASH_DISABLE_PLUGINS" \
      NYASH_SKIP_TOML_ENV="$NYASH_SKIP_TOML_ENV" \
      NYASH_GC_MODE="$NYASH_GC_MODE" \
      NYASH_SCHED_POLL_IN_SAFEPOINT="$NYASH_SCHED_POLL_IN_SAFEPOINT" \
      HAKO_TYPED_OBJECT_STORE="$HAKO_TYPED_OBJECT_STORE" \
      HAKO_ARRAY_SLOT_STORE="$HAKO_ARRAY_SLOT_STORE" \
      perf stat -x, -e instructions,cycles -o "$stat_file" -- "$exe_out" >"$run_out" 2>"$run_err"
  )
  if ! rg -q '^summary=ok$' "$run_out"; then
    echo "[mimalloc-direct-exact-app-stat] ERROR: app summary was not ok" >&2
    cat "$run_out" >&2
    cat "$run_err" >&2
    exit 1
  fi
  instructions="$(awk -F, '$3 ~ /^instructions/ { gsub(/ /, "", $1); print $1; exit }' "$stat_file")"
  cycles="$(awk -F, '$3 ~ /^cycles/ { gsub(/ /, "", $1); print $1; exit }' "$stat_file")"
  if [[ -z "$instructions" || -z "$cycles" ]]; then
    echo "[mimalloc-direct-exact-app-stat] ERROR: failed to parse perf stat" >&2
    cat "$stat_file" >&2
    exit 1
  fi
  body_elapsed_ns="$(awk -F= '$1 == "body_elapsed_ns" { print $2; exit }' "$run_out")"
  printf "%s\t%s\t%s\n" "$instructions" "$cycles" "${body_elapsed_ns:-0}" >>"$sample_file"
done

python3 - "$sample_file" "$APP" "$RUNS" "$OUT_FILE" <<'PY'
import statistics
import sys
from pathlib import Path

sample_path = Path(sys.argv[1])
app = sys.argv[2]
runs = int(sys.argv[3])
out_path = Path(sys.argv[4])

instructions = []
cycles = []
body_ns = []
for line in sample_path.read_text(encoding="utf-8").splitlines():
    instr, cyc, body = line.split("\t")
    instructions.append(int(instr))
    cycles.append(int(cyc))
    body_ns.append(int(body))

lines = [
    "output_contract=hako-mimalloc-direct-exact-app-perf-stat-v0",
    f"hako_app={app}",
    f"runs={runs}",
    f"hako_instructions_median={int(statistics.median(instructions))}",
    f"hako_cycles_median={int(statistics.median(cycles))}",
    f"hako_body_elapsed_ns_median={int(statistics.median(body_ns))}",
    "direct_exact_env_contract=mimalloc-direct-exact-env-v0",
    "NYASH_FEATURES=rune",
    "NYASH_DISABLE_PLUGINS=1",
    "NYASH_SKIP_TOML_ENV=1",
    "NYASH_GC_MODE=off",
    "NYASH_SCHED_POLL_IN_SAFEPOINT=0",
    "HAKO_TYPED_OBJECT_STORE=direct_slot_exact",
    "HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact",
    "worker_front_mismatch_guard=1",
    "summary=ok",
]
out_path.parent.mkdir(parents=True, exist_ok=True)
out_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
print(out_path.read_text(encoding="utf-8"), end="")
PY
