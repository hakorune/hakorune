#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="${ROOT_DIR}/apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
HAKO_RUNNER="${ROOT_DIR}/tools/allocator/hako_exe_memory_runner.sh"
C_RUNNER="${ROOT_DIR}/tools/allocator/c_mimalloc_explicit_runner.sh"
PAIR_ADAPTER="${ROOT_DIR}/tools/allocator/hako_mimalloc_object_lifecycle_body_timing_pair_adapter.py"
ENV_PRESET="${ROOT_DIR}/tools/allocator/mimalloc_direct_exact_env.sh"

OUT_FILE=""
TMP_KEEP=0
OPERATION_REPEAT=1
IN_PROCESS_REPEAT=8192

usage() {
  cat >&2 <<'USAGE'
usage: tools/allocator/hako_mimalloc_direct_exact_pair.sh --out FILE [--app FILE] [--tmp-keep]

Runs the current representative object-lifecycle Hako/C pair through the
canonical direct-exact front:
  HAKO_TYPED_OBJECT_STORE=direct_slot_exact
  HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact

This wrapper exists so workers do not accidentally measure default/safe or
legacy single_thread_exact fronts while investigating mimalloc parity.
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
    --tmp-keep)
      TMP_KEEP=1
      shift
      ;;
    --operation-repeat)
      OPERATION_REPEAT="${2:-}"
      shift 2
      ;;
    --in-process-repeat)
      IN_PROCESS_REPEAT="${2:-}"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[mimalloc-direct-exact-pair] ERROR: unknown argument: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [[ -z "$OUT_FILE" ]]; then
  echo "[mimalloc-direct-exact-pair] ERROR: --out FILE is required" >&2
  usage
  exit 2
fi
if [[ ! -f "$APP" ]]; then
  echo "[mimalloc-direct-exact-pair] ERROR: app not found: $APP" >&2
  exit 2
fi
case "$OPERATION_REPEAT" in
  ''|*[!0-9]*)
    echo "[mimalloc-direct-exact-pair] ERROR: --operation-repeat must be a positive integer" >&2
    exit 2
    ;;
esac
case "$IN_PROCESS_REPEAT" in
  ''|*[!0-9]*)
    echo "[mimalloc-direct-exact-pair] ERROR: --in-process-repeat must be a positive integer" >&2
    exit 2
    ;;
esac
if [[ "$OPERATION_REPEAT" -lt 1 || "$IN_PROCESS_REPEAT" -lt 1 ]]; then
  echo "[mimalloc-direct-exact-pair] ERROR: repeats must be >= 1" >&2
  exit 2
fi

# shellcheck source=tools/allocator/mimalloc_direct_exact_env.sh
source "$ENV_PRESET"
mimalloc_direct_exact_env_check

tmp_dir="$(mktemp -d /tmp/hakorune_mimalloc_direct_exact_pair.XXXXXX)"
if [[ "$TMP_KEEP" -eq 0 ]]; then
  trap 'rm -rf "$tmp_dir"' EXIT
else
  echo "[mimalloc-direct-exact-pair] tmp_dir=$tmp_dir" >&2
fi

hako_report="$tmp_dir/hako.out"
c_report="$tmp_dir/c.out"
pair_report="$tmp_dir/pair.out"

bash "$HAKO_RUNNER" \
  --app "$APP" \
  --workload representative-object-lifecycle-small-block-v0 \
  --runtime-config empty \
  --operation-repeat "$OPERATION_REPEAT" \
  --out "$hako_report" >/dev/null

bash "$C_RUNNER" \
  --workload representative-object-lifecycle-small-block-v0 \
  --in-process-repeat "$IN_PROCESS_REPEAT" \
  --operation-repeat "$OPERATION_REPEAT" \
  --allow-ldconfig-discovery \
  --out "$c_report" >/dev/null

python3 "$PAIR_ADAPTER" \
  --hako-report "$hako_report" \
  --c-report "$c_report" \
  --out "$pair_report"

{
  cat "$pair_report"
  echo "hako_app=$APP"
  echo "direct_exact_env_contract=mimalloc-direct-exact-env-v0"
  echo "NYASH_FEATURES=$NYASH_FEATURES"
  echo "NYASH_DISABLE_PLUGINS=$NYASH_DISABLE_PLUGINS"
  echo "NYASH_SKIP_TOML_ENV=$NYASH_SKIP_TOML_ENV"
  echo "NYASH_GC_MODE=$NYASH_GC_MODE"
  echo "NYASH_SCHED_POLL_IN_SAFEPOINT=$NYASH_SCHED_POLL_IN_SAFEPOINT"
  echo "HAKO_TYPED_OBJECT_STORE=$HAKO_TYPED_OBJECT_STORE"
  echo "HAKO_ARRAY_SLOT_STORE=$HAKO_ARRAY_SLOT_STORE"
  echo "worker_front_mismatch_guard=1"
} > "$OUT_FILE"

cat "$OUT_FILE"
