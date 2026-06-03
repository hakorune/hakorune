#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
ENV_PRESET="${ROOT_DIR}/tools/allocator/mimalloc_direct_exact_env.sh"
APP=""
OUT_FILE=""
SYMBOL="ny_main"
RUNS=150
IN_PROCESS_REPEAT=8192
PERF_FREQ=999

usage() {
  cat >&2 <<'USAGE'
usage: tools/allocator/hako_mimalloc_direct_exact_app_perf_asm.sh --app FILE --out FILE [--symbol SYMBOL] [--runs N] [--in-process-repeat N] [--perf-freq N]

Builds one .hako app through the canonical mimalloc direct-exact EXE route,
records repeated perf samples with a tiny direct C runner, and writes:

  OUT_FILE
    key-value report
  OUT_FILE.artifacts.d/
    app.exe, perf.data, perf-report.txt, perf-annotate.txt, objdump.txt

This is an owner-first investigation tool. It does not select a keeper, change
the source front, or widen compiler fast-path contracts.
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
    --symbol)
      SYMBOL="${2:-}"
      shift 2
      ;;
    --runs)
      RUNS="${2:-}"
      shift 2
      ;;
    --in-process-repeat)
      IN_PROCESS_REPEAT="${2:-}"
      shift 2
      ;;
    --perf-freq)
      PERF_FREQ="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[mimalloc-direct-exact-app-asm] ERROR: unknown argument: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [[ -z "$APP" || -z "$OUT_FILE" ]]; then
  echo "[mimalloc-direct-exact-app-asm] ERROR: --app and --out are required" >&2
  usage
  exit 2
fi
if [[ ! -f "$APP" ]]; then
  echo "[mimalloc-direct-exact-app-asm] ERROR: app not found: $APP" >&2
  exit 2
fi
for value_name in RUNS IN_PROCESS_REPEAT PERF_FREQ; do
  value="${!value_name}"
  case "$value" in
    ''|*[!0-9]*)
      echo "[mimalloc-direct-exact-app-asm] ERROR: ${value_name} must be a positive integer" >&2
      exit 2
      ;;
  esac
  if [[ "$value" -lt 1 ]]; then
    echo "[mimalloc-direct-exact-app-asm] ERROR: ${value_name} must be >= 1" >&2
    exit 2
  fi
done
if ! command -v perf >/dev/null 2>&1; then
  echo "[mimalloc-direct-exact-app-asm] ERROR: perf is required" >&2
  exit 2
fi
if ! command -v objdump >/dev/null 2>&1; then
  echo "[mimalloc-direct-exact-app-asm] ERROR: objdump is required" >&2
  exit 2
fi
if ! command -v "${CC:-cc}" >/dev/null 2>&1; then
  echo "[mimalloc-direct-exact-app-asm] ERROR: C compiler required (\$CC or cc)" >&2
  exit 2
fi

# shellcheck source=tools/allocator/mimalloc_direct_exact_env.sh
source "$ENV_PRESET"
mimalloc_direct_exact_env_check

if [[ ! -x "$ROOT_DIR/target/release/hakorune" || ! -x "$ROOT_DIR/target/release/ny-llvmc" ]]; then
  cargo build --release --bin hakorune --bin ny-llvmc
fi

out_dir="$(dirname "$OUT_FILE")"
mkdir -p "$out_dir"
OUT_FILE="$(cd "$out_dir" && pwd)/$(basename "$OUT_FILE")"
artifact_dir="${OUT_FILE}.artifacts.d"
rm -rf "$artifact_dir"
mkdir -p "$artifact_dir"

work_dir="$(mktemp -d /tmp/hakorune_mimalloc_direct_exact_app_asm.XXXXXX)"
trap 'rm -rf "$work_dir"' EXIT

mir_json="$artifact_dir/app.mir.json"
exe_out="$artifact_dir/app.exe"
perf_data="$artifact_dir/perf.data"
perf_report="$artifact_dir/perf-report.txt"
perf_annotate="$artifact_dir/perf-annotate.txt"
perf_attribution="$artifact_dir/perf-attribution.txt"
objdump_txt="$artifact_dir/objdump.txt"
runner_c="$work_dir/runner.c"
runner_bin="$work_dir/runner.bin"
run_cwd="$work_dir/runtime-empty-config"
run_out="$work_dir/run.out"
run_err="$work_dir/run.err"
mkdir -p "$run_cwd"
cat > "$run_cwd/nyash.toml" <<'TOML'
[libraries]
TOML

hako_app="$APP"
if [[ "$IN_PROCESS_REPEAT" != "8192" ]]; then
  hako_app="$work_dir/app.in_process_repeat_${IN_PROCESS_REPEAT}.hako"
  python3 - "$APP" "$hako_app" "$IN_PROCESS_REPEAT" <<'PY'
import sys
from pathlib import Path

src_path = Path(sys.argv[1])
out_path = Path(sys.argv[2])
repeat = int(sys.argv[3])
text = src_path.read_text(encoding="utf-8")
replacements = {
    "local operation_repeat = 8192": f"local operation_repeat = {repeat}",
    "524288": str(64 * repeat),
    "272416768": str(33254 * repeat),
    "276824064": str(33792 * repeat),
}
for old, new in replacements.items():
    if old not in text:
        raise SystemExit(f"missing expected representative repeat token: {old}")
    text = text.replace(old, new)
out_path.write_text(text, encoding="utf-8")
PY
fi

NYASH_FEATURES="$NYASH_FEATURES" \
NYASH_DISABLE_PLUGINS="$NYASH_DISABLE_PLUGINS" \
  "$ROOT_DIR/target/release/hakorune" --backend mir --emit-mir-json "$mir_json" "$hako_app" >/dev/null

python3 "$ROOT_DIR/tools/checks/pure_first_route_preflight.py" "$mir_json" >/dev/null

NYASH_DISABLE_PLUGINS="$NYASH_DISABLE_PLUGINS" \
  "$ROOT_DIR/tools/selfhost/selfhost_build.sh" --mir-in "$mir_json" --exe "$exe_out" >/dev/null

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
    "$exe_out" >"$run_out" 2>"$run_err"
)
if ! rg -q '^summary=ok$' "$run_out"; then
  echo "[mimalloc-direct-exact-app-asm] ERROR: app summary was not ok" >&2
  cat "$run_out" >&2
  cat "$run_err" >&2
  exit 1
fi

cat >"$runner_c" <<'EOF'
#include <errno.h>
#include <spawn.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/wait.h>
#include <unistd.h>

extern char **environ;

int main(int argc, char **argv) {
  if (argc != 3) {
    fprintf(stderr, "usage: %s <runs> <exe>\n", argv[0]);
    return 2;
  }
  char *end = NULL;
  long runs = strtol(argv[1], &end, 10);
  if (!end || *end != '\0' || runs < 1) {
    fprintf(stderr, "invalid runs: %s\n", argv[1]);
    return 2;
  }

  char *const child_argv[] = { argv[2], NULL };
  for (long i = 0; i < runs; ++i) {
    pid_t pid = 0;
    int rc = posix_spawn(&pid, argv[2], NULL, NULL, child_argv, environ);
    if (rc != 0) {
      fprintf(stderr, "posix_spawn failed: %s\n", strerror(rc));
      return 1;
    }
    int status = 0;
    if (waitpid(pid, &status, 0) < 0) {
      fprintf(stderr, "waitpid failed: %s\n", strerror(errno));
      return 1;
    }
    if (!WIFEXITED(status) || WEXITSTATUS(status) != 0) {
      return 1;
    }
  }
  return 0;
}
EOF

"${CC:-cc}" -O2 -std=c11 -Wall -Wextra -o "$runner_bin" "$runner_c"

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
    perf record -o "$perf_data" -F "$PERF_FREQ" -- "$runner_bin" "$RUNS" "$exe_out" >/dev/null 2>&1
)

perf report --stdio --no-children -i "$perf_data" >"$perf_report"
perf annotate --stdio -i "$perf_data" --symbol "$SYMBOL" >"$perf_annotate" || true
objdump -d --demangle "$exe_out" >"$objdump_txt"
python3 "$ROOT_DIR/tools/allocator/hako_mimalloc_perf_attribution.py" \
  --perf-report "$perf_report" \
  --perf-annotate "$perf_annotate" \
  --symbol "$SYMBOL" >"$perf_attribution"

body_elapsed_ns="$(awk -F= '$1 == "body_elapsed_ns" { print $2; exit }' "$run_out")"
observed_repeat="$(awk -F= '$1 == "in_process_operation_repeat" { print $2; exit }' "$run_out")"
nonzero_annotate_count="$(
  awk '
    /^[[:space:]]*[0-9]+([.][0-9]+)?[[:space:]]*:/ { count++ }
    END { print count + 0 }
  ' "$perf_annotate"
)"
top_symbol="$(awk '
  /^[[:space:]]*[0-9]+([.][0-9]+)?%/ {
    for (i = 1; i <= NF; i++) {
      if ($i == "[.]") {
        print $(i + 1)
        exit
      }
    }
  }
' "$perf_report")"

cat >"$OUT_FILE" <<EOF
output_contract=hako-mimalloc-direct-exact-app-perf-asm-v0
hako_app=$APP
runs=$RUNS
symbol=$SYMBOL
perf_freq=$PERF_FREQ
in_process_operation_repeat=${observed_repeat:-0}
requested_in_process_operation_repeat=$IN_PROCESS_REPEAT
body_elapsed_ns=${body_elapsed_ns:-0}
top_symbol=${top_symbol:-}
nonzero_annotate_line_count=$nonzero_annotate_count
artifact_dir=$artifact_dir
exe=$exe_out
perf_data=$perf_data
perf_report=$perf_report
perf_annotate=$perf_annotate
perf_attribution=$perf_attribution
objdump=$objdump_txt
direct_exact_env_contract=mimalloc-direct-exact-env-v0
NYASH_FEATURES=$NYASH_FEATURES
NYASH_DISABLE_PLUGINS=$NYASH_DISABLE_PLUGINS
NYASH_SKIP_TOML_ENV=$NYASH_SKIP_TOML_ENV
NYASH_GC_MODE=$NYASH_GC_MODE
NYASH_SCHED_POLL_IN_SAFEPOINT=$NYASH_SCHED_POLL_IN_SAFEPOINT
HAKO_TYPED_OBJECT_STORE=$HAKO_TYPED_OBJECT_STORE
HAKO_ARRAY_SLOT_STORE=$HAKO_ARRAY_SLOT_STORE
worker_front_mismatch_guard=1
EOF
awk -F= '
  $1 != "output_contract" && $1 != "summary" { print }
' "$perf_attribution" >>"$OUT_FILE"
cat >>"$OUT_FILE" <<'EOF'
summary=ok
EOF

cat "$OUT_FILE"
