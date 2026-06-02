#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
APP="${ROOT_DIR}/apps/provider-package/hako-derived-mimalloc-real-entrypoint-fixture/main.hako"
HAKORUNE_BIN="${ROOT_DIR}/target/release/hakorune"
MODE="object-lifecycle-small-alloc-release-v0"

OUT_FILE=""
OUT_DIR=""
TMP_KEEP=0
BUILD_RELEASE=1
SAMPLE_COUNT=3
WARMUP_COUNT=1
OPERATION_REPEAT=128
SIZE=32
ALIGN=8

usage() {
  cat >&2 <<'USAGE'
usage: tools/allocator/hako_mimalloc_provider_package_explicit_ladder.sh --out FILE [options]

Builds the selected .hako-derived mimalloc provider package and runs the
current no-replacement explicit provider ladder:
  package build -> metadata preflight -> descriptor -> API bind -> alloc/free
  -> repeated explicit provider measurement -> native-fusion adapter.

This is not provider activation, process allocator replacement, hooks,
global allocator integration, or LD_PRELOAD replacement.

Options:
  --out FILE                 final normalized report path
  --out-dir DIR              artifact directory; defaults to FILE.artifacts.d
  --app FILE                 selected .hako provider fixture
  --semantic-codegen MODE    semantic provider codegen mode
                             (default: object-lifecycle-small-alloc-release-v0)
  --sample-count N           repeated measurement sample count (default: 3)
  --warmup-count N           repeated measurement warmup count (default: 1)
  --operation-repeat N       repeated provider alloc/free operations (default: 128)
  --size N                   provider allocation size (default: 32)
  --align N                  provider allocation alignment (default: 8)
  --skip-build-release       use the existing target/release/hakorune binary
  --tmp-keep                 keep temporary artifacts and print their directory
USAGE
}

positive_int() {
  local value="$1"
  local label="$2"
  case "$value" in
    ''|*[!0-9]*)
      echo "[provider-package-explicit-ladder] ERROR: $label must be a positive integer" >&2
      exit 2
      ;;
  esac
  if [[ "$value" -lt 1 ]]; then
    echo "[provider-package-explicit-ladder] ERROR: $label must be >= 1" >&2
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
    --app)
      APP="${2:-}"
      shift 2
      ;;
    --semantic-codegen)
      MODE="${2:-}"
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
    --size)
      SIZE="${2:-}"
      shift 2
      ;;
    --align)
      ALIGN="${2:-}"
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
      echo "[provider-package-explicit-ladder] ERROR: unknown argument: $1" >&2
      usage
      exit 2
      ;;
  esac
done

if [[ -z "$OUT_FILE" ]]; then
  echo "[provider-package-explicit-ladder] ERROR: --out FILE is required" >&2
  usage
  exit 2
fi
if [[ ! -f "$APP" ]]; then
  echo "[provider-package-explicit-ladder] ERROR: app not found: $APP" >&2
  exit 2
fi
positive_int "$SAMPLE_COUNT" "--sample-count"
positive_int "$OPERATION_REPEAT" "--operation-repeat"
positive_int "$SIZE" "--size"
positive_int "$ALIGN" "--align"
case "$WARMUP_COUNT" in
  ''|*[!0-9]*)
    echo "[provider-package-explicit-ladder] ERROR: --warmup-count must be a non-negative integer" >&2
    exit 2
    ;;
esac

if [[ "$BUILD_RELEASE" -eq 1 ]]; then
  cargo build --release --bin hakorune >/dev/null
elif [[ ! -x "$HAKORUNE_BIN" ]]; then
  echo "[provider-package-explicit-ladder] ERROR: missing $HAKORUNE_BIN; remove --skip-build-release" >&2
  exit 2
fi

if [[ -z "$OUT_DIR" ]]; then
  OUT_DIR="${OUT_FILE}.artifacts.d"
  rm -rf "$OUT_DIR"
  mkdir -p "$OUT_DIR"
else
  mkdir -p "$OUT_DIR"
fi
if [[ "$TMP_KEEP" -eq 0 && "$OUT_DIR" == /tmp/hakorune_provider_package_ladder.* ]]; then
  trap 'rm -rf "$OUT_DIR"' EXIT
else
  echo "[provider-package-explicit-ladder] out_dir=$OUT_DIR" >&2
fi

pkg_dir="$OUT_DIR/pkg"
mkdir -p "$pkg_dir"
build_report="$OUT_DIR/build.out"
preflight_report="$OUT_DIR/preflight.out"
descriptor_report="$OUT_DIR/descriptor.out"
api_report="$OUT_DIR/api.out"
allocfree_report="$OUT_DIR/allocfree.out"
repeated_report="$OUT_DIR/repeated.out"
native_fusion_report="$OUT_DIR/native_fusion.out"

"$HAKORUNE_BIN" \
  --provider-package-hako-derived-build-fixture "$APP" \
  --provider-package-hako-semantic-codegen "$MODE" \
  --provider-package-out-dir "$pkg_dir" \
  --provider-package-id org.hakorune.provider.hako.mimalloc.real-entrypoint \
  --provider-package-name hako-mimalloc-real-entrypoint-provider \
  --provider-package-target-triple x86_64-unknown-linux-gnu \
  --provider-package-platform linux \
  --provider-package-provider-call-allowed \
  --provider-package-force > "$build_report"

python3 "$ROOT_DIR/tools/allocator/provider_package_metadata_preflight.py" \
  --manifest "$pkg_dir/hakorune_provider.json" > "$preflight_report"
python3 "$ROOT_DIR/tools/allocator/provider_package_descriptor_smoke.py" \
  --manifest "$pkg_dir/hakorune_provider.json" > "$descriptor_report"
python3 "$ROOT_DIR/tools/allocator/provider_package_api_bind_smoke.py" \
  --manifest "$pkg_dir/hakorune_provider.json" > "$api_report"
python3 "$ROOT_DIR/tools/allocator/provider_package_alloc_free_smoke.py" \
  --manifest "$pkg_dir/hakorune_provider.json" > "$allocfree_report"
python3 "$ROOT_DIR/tools/allocator/provider_package_explicit_repeated_measurement.py" \
  --manifest "$pkg_dir/hakorune_provider.json" \
  --operation-repeat "$OPERATION_REPEAT" \
  --sample-count "$SAMPLE_COUNT" \
  --warmup-count "$WARMUP_COUNT" \
  --size "$SIZE" \
  --align "$ALIGN" \
  --out "$repeated_report"
python3 "$ROOT_DIR/tools/allocator/hako_mimalloc_provider_package_native_fusion_explicit_measurement.py" \
  --build-report "$build_report" \
  --measurement-report "$repeated_report" \
  --out "$native_fusion_report"

{
  cat "$native_fusion_report"
  echo "provider_package_ladder_tool=hako_mimalloc_provider_package_explicit_ladder"
  echo "build_report=$build_report"
  echo "preflight_report=$preflight_report"
  echo "descriptor_report=$descriptor_report"
  echo "api_report=$api_report"
  echo "allocfree_report=$allocfree_report"
  echo "repeated_report=$repeated_report"
  echo "manifest=$pkg_dir/hakorune_provider.json"
} > "$OUT_FILE"

cat "$OUT_FILE"
