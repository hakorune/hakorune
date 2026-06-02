#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
REPEATED_RUNNER="${ROOT_DIR}/tools/allocator/mimalloc_repeated_measurement_runner.py"
PROVIDER_LADDER="${ROOT_DIR}/tools/allocator/hako_mimalloc_provider_package_explicit_ladder.sh"
COMPARISON_ADAPTER="${ROOT_DIR}/tools/allocator/provider_explicit_comparison_adapter.py"

OUT_FILE=""
OUT_DIR=""
BUILD_RELEASE=1
TMP_KEEP=0
WORKLOAD="representative-small-block-v0"
SAMPLE_COUNT=3
WARMUP_COUNT=1
OPERATION_REPEAT=128
HAKO_RUNTIME_CONFIG="empty"

usage() {
  cat >&2 <<'USAGE'
usage: tools/allocator/hako_mimalloc_provider_package_3way_comparison_ladder.sh --out FILE [options]

Runs the no-replacement 3-way comparison ladder:
  Hako exact-EXE/C repeated measurement
  + selected .hako-derived provider explicit ladder
  + provider_explicit_comparison_adapter.py

This keeps provider activation, allocator replacement, hooks, global allocator
integration, LD_PRELOAD replacement, and winner claims closed.

Options:
  --out FILE                 final comparison report path
  --out-dir DIR              artifact directory; defaults to FILE.artifacts.d
  --workload ID              hako/C workload id (default: representative-small-block-v0)
  --sample-count N           sample count for both ladders (default: 3)
  --warmup-count N           warmup count for both ladders (default: 1)
  --operation-repeat N       operation repeat for both ladders (default: 128)
  --hako-runtime-config NAME Hako runtime config for hako/C runner (default: empty)
  --skip-build-release       use existing target/release/hakorune in provider ladder
  --tmp-keep                 keep temporary artifacts and print their directory
USAGE
}

positive_int() {
  local value="$1"
  local label="$2"
  case "$value" in
    ''|*[!0-9]*)
      echo "[provider-package-3way-ladder] ERROR: $label must be a positive integer" >&2
      exit 2
      ;;
  esac
  if [[ "$value" -lt 1 ]]; then
    echo "[provider-package-3way-ladder] ERROR: $label must be >= 1" >&2
    exit 2
  fi
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
      echo "[provider-package-3way-ladder] ERROR: unknown argument: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [[ -z "$OUT_FILE" ]]; then
  echo "[provider-package-3way-ladder] ERROR: --out FILE is required" >&2
  usage
  exit 2
fi
positive_int "$SAMPLE_COUNT" "--sample-count"
positive_int "$OPERATION_REPEAT" "--operation-repeat"
case "$WARMUP_COUNT" in
  ''|*[!0-9]*)
    echo "[provider-package-3way-ladder] ERROR: --warmup-count must be a non-negative integer" >&2
    exit 2
    ;;
esac
case "$HAKO_RUNTIME_CONFIG" in
  empty|root)
    ;;
  *)
    echo "[provider-package-3way-ladder] ERROR: --hako-runtime-config must be empty|root" >&2
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
if [[ "$TMP_KEEP" -eq 0 && "$OUT_DIR" == /tmp/hakorune_provider_package_3way.* ]]; then
  trap 'rm -rf "$OUT_DIR"' EXIT
else
  echo "[provider-package-3way-ladder] out_dir=$OUT_DIR" >&2
fi

hako_c_report="$OUT_DIR/hako_c.out"
provider_report="$OUT_DIR/provider_ladder.out"
provider_repeated_report="$OUT_DIR/provider/repeated.out"
comparison_report="$OUT_DIR/comparison.out"

python3 "$REPEATED_RUNNER" \
  --out "$hako_c_report" \
  --workload "$WORKLOAD" \
  --sample-count "$SAMPLE_COUNT" \
  --warmup-count "$WARMUP_COUNT" \
  --operation-repeat "$OPERATION_REPEAT" \
  --hako-runtime-config "$HAKO_RUNTIME_CONFIG" \
  --allow-ldconfig-discovery >/dev/null

provider_args=(
  --out "$provider_report"
  --out-dir "$OUT_DIR/provider"
  --sample-count "$SAMPLE_COUNT"
  --warmup-count "$WARMUP_COUNT"
  --operation-repeat "$OPERATION_REPEAT"
)
if [[ "$BUILD_RELEASE" -eq 0 ]]; then
  provider_args+=(--skip-build-release)
fi
"$PROVIDER_LADDER" "${provider_args[@]}" >/dev/null

python3 "$COMPARISON_ADAPTER" \
  --hako-c-report "$hako_c_report" \
  --provider-report "$provider_repeated_report" \
  --out "$comparison_report"

{
  cat "$comparison_report"
  echo "provider_package_3way_ladder_tool=hako_mimalloc_provider_package_3way_comparison_ladder"
  echo "hako_c_report=$hako_c_report"
  echo "provider_ladder_report=$provider_report"
  echo "provider_report=$provider_repeated_report"
  echo "comparison_report=$comparison_report"
  echo "provider_activation=0"
  echo "replacement_active=0"
  echo "hook_installed=0"
  echo "global_allocator=0"
  echo "winner_claim=0"
} > "$OUT_FILE"

cat "$OUT_FILE"
