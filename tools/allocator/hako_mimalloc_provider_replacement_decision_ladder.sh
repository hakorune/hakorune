#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
REPEATED_RUNNER="${ROOT_DIR}/tools/allocator/mimalloc_repeated_measurement_runner.py"
PROVIDER_LADDER="${ROOT_DIR}/tools/allocator/hako_mimalloc_provider_package_explicit_ladder.sh"
LDPRELOAD_REPEATED="${ROOT_DIR}/tools/allocator/hako_mimalloc_provider_backed_hakmem_ldpreload_repeated_measurement.py"
RUST_GLOBAL_SMOKE="${ROOT_DIR}/tools/allocator/provider_package_rust_global_allocator_smoke.py"
DECISION_ADAPTER="${ROOT_DIR}/tools/allocator/provider_replacement_decision_adapter.py"

OUT_FILE=""
OUT_DIR=""
BUILD_RELEASE=1
TMP_KEEP=0
WORKLOAD="representative-small-block-v0"
SAMPLE_COUNT=1
WARMUP_COUNT=0
OPERATION_REPEAT=1
HAKO_RUNTIME_CONFIG="empty"
HAKMEM_ITERATIONS=1000
HAKMEM_WORKING_SET=128
HAKMEM_SEED=42

usage() {
  cat >&2 <<'USAGE'
usage: tools/allocator/hako_mimalloc_provider_replacement_decision_ladder.sh --out FILE [options]

Runs the no-product-default provider replacement decision ladder:
  Hako exact-EXE/C repeated measurement
  + selected .hako-derived provider explicit ladder
  + provider-backed hakmem LD_PRELOAD repeated measurement
  + generated Rust #[global_allocator] pilot
  + provider_replacement_decision_adapter.py

This records replacement evidence while keeping product provider activation,
production hooks, production global allocator defaults, and winner claims closed.

Options:
  --out FILE                 final decision report path
  --out-dir DIR              artifact directory; defaults to FILE.artifacts.d
  --workload ID              hako/C workload id (default: representative-small-block-v0)
  --sample-count N           hako/C and provider explicit sample count (default: 1)
  --warmup-count N           hako/C and provider explicit warmup count (default: 0)
  --operation-repeat N       hako/C and provider explicit operation repeat (default: 1)
  --hako-runtime-config NAME Hako runtime config for hako/C runner (default: empty)
  --hakmem-iterations N      hakmem random-mixed iterations (default: 1000)
  --hakmem-working-set N     hakmem random-mixed working set (default: 128)
  --hakmem-seed N            hakmem random-mixed seed (default: 42)
  --skip-build-release       use existing target/release/hakorune in provider ladder
  --tmp-keep                 keep temporary artifacts and print their directory
USAGE
}

positive_int() {
  local value="$1"
  local label="$2"
  case "$value" in
    ''|*[!0-9]*)
      echo "[provider-replacement-decision-ladder] ERROR: $label must be a positive integer" >&2
      exit 2
      ;;
  esac
  if [[ "$value" -lt 1 ]]; then
    echo "[provider-replacement-decision-ladder] ERROR: $label must be >= 1" >&2
    exit 2
  fi
}

nonnegative_int() {
  local value="$1"
  local label="$2"
  case "$value" in
    ''|*[!0-9]*)
      echo "[provider-replacement-decision-ladder] ERROR: $label must be a non-negative integer" >&2
      exit 2
      ;;
  esac
}

kv_value() {
  local file="$1"
  local key="$2"
  awk -F= -v key="$key" '$1 == key { sub(/^[^=]*=/, ""); print; found=1 } END { if (!found) exit 1 }' "$file"
}

while [[ "$#" -gt 0 ]]; do
  case "$1" in
    --out)
      OUT_FILE="${2:-}"
      shift 2
      ;;
    --out-dir)
      OUT_DIR="${2:-}"
      shift 2
      ;;
    --workload)
      WORKLOAD="${2:-}"
      shift 2
      ;;
    --sample-count)
      SAMPLE_COUNT="${2:-}"
      shift 2
      ;;
    --warmup-count)
      WARMUP_COUNT="${2:-}"
      shift 2
      ;;
    --operation-repeat)
      OPERATION_REPEAT="${2:-}"
      shift 2
      ;;
    --hako-runtime-config)
      HAKO_RUNTIME_CONFIG="${2:-}"
      shift 2
      ;;
    --hakmem-iterations)
      HAKMEM_ITERATIONS="${2:-}"
      shift 2
      ;;
    --hakmem-working-set)
      HAKMEM_WORKING_SET="${2:-}"
      shift 2
      ;;
    --hakmem-seed)
      HAKMEM_SEED="${2:-}"
      shift 2
      ;;
    --skip-build-release)
      BUILD_RELEASE=0
      shift
      ;;
    --tmp-keep)
      TMP_KEEP=1
      shift
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "[provider-replacement-decision-ladder] ERROR: unknown argument: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [[ -z "$OUT_FILE" ]]; then
  echo "[provider-replacement-decision-ladder] ERROR: --out FILE is required" >&2
  usage
  exit 2
fi
positive_int "$SAMPLE_COUNT" "--sample-count"
nonnegative_int "$WARMUP_COUNT" "--warmup-count"
positive_int "$OPERATION_REPEAT" "--operation-repeat"
positive_int "$HAKMEM_ITERATIONS" "--hakmem-iterations"
positive_int "$HAKMEM_WORKING_SET" "--hakmem-working-set"
positive_int "$HAKMEM_SEED" "--hakmem-seed"
case "$HAKO_RUNTIME_CONFIG" in
  empty|root)
    ;;
  *)
    echo "[provider-replacement-decision-ladder] ERROR: --hako-runtime-config must be empty|root" >&2
    exit 2
    ;;
esac

if [[ -z "$OUT_DIR" ]]; then
  OUT_DIR="${OUT_FILE}.artifacts.d"
  rm -rf "$OUT_DIR"
  mkdir -p "$OUT_DIR"
else
  mkdir -p "$OUT_DIR"
fi
if [[ "$TMP_KEEP" -eq 0 && "$OUT_DIR" == /tmp/hakorune_provider_replacement_decision.* ]]; then
  trap 'rm -rf "$OUT_DIR"' EXIT
else
  echo "[provider-replacement-decision-ladder] out_dir=$OUT_DIR" >&2
fi

hako_c_report="$OUT_DIR/hako_c.out"
provider_ladder_report="$OUT_DIR/provider_ladder.out"
provider_repeated_report="$OUT_DIR/provider/repeated.out"
ldpreload_report="$OUT_DIR/hakmem_ldpreload.out"
rust_global_report="$OUT_DIR/rust_global.out"
decision_report="$OUT_DIR/decision.out"

python3 "$REPEATED_RUNNER" \
  --out "$hako_c_report" \
  --workload "$WORKLOAD" \
  --sample-count "$SAMPLE_COUNT" \
  --warmup-count "$WARMUP_COUNT" \
  --operation-repeat "$OPERATION_REPEAT" \
  --hako-runtime-config "$HAKO_RUNTIME_CONFIG" \
  --allow-ldconfig-discovery >/dev/null

provider_args=(
  --out "$provider_ladder_report"
  --out-dir "$OUT_DIR/provider"
  --sample-count "$SAMPLE_COUNT"
  --warmup-count "$WARMUP_COUNT"
  --operation-repeat "$OPERATION_REPEAT"
)
if [[ "$BUILD_RELEASE" -eq 0 ]]; then
  provider_args+=(--skip-build-release)
fi
"$PROVIDER_LADDER" "${provider_args[@]}" >/dev/null

manifest="$(kv_value "$provider_ladder_report" manifest)"

python3 "$LDPRELOAD_REPEATED" \
  --manifest "$manifest" \
  --out-dir "$OUT_DIR/hakmem_ldpreload" \
  --out "$ldpreload_report" \
  --sample-count "$SAMPLE_COUNT" \
  --warmup-count "$WARMUP_COUNT" \
  --iterations "$HAKMEM_ITERATIONS" \
  --working-set "$HAKMEM_WORKING_SET" \
  --seed "$HAKMEM_SEED"

python3 "$RUST_GLOBAL_SMOKE" \
  --manifest "$manifest" \
  --out-dir "$OUT_DIR/rust_global" \
  --out "$rust_global_report"

python3 "$DECISION_ADAPTER" \
  --hako-c-report "$hako_c_report" \
  --provider-report "$provider_repeated_report" \
  --ldpreload-report "$ldpreload_report" \
  --rust-global-report "$rust_global_report" \
  --out "$decision_report"

{
  cat "$decision_report"
  echo "provider_replacement_decision_ladder_tool=hako_mimalloc_provider_replacement_decision_ladder"
  echo "hako_c_report=$hako_c_report"
  echo "provider_ladder_report=$provider_ladder_report"
  echo "provider_report=$provider_repeated_report"
  echo "ldpreload_report=$ldpreload_report"
  echo "rust_global_report=$rust_global_report"
  echo "decision_report=$decision_report"
  echo "manifest=$manifest"
  echo "provider_activation=0"
  echo "production_replacement_active=0"
  echo "hook_installed=0"
  echo "global_allocator_product_claim=0"
  echo "winner_claim=0"
} > "$OUT_FILE"

cat "$OUT_FILE"
