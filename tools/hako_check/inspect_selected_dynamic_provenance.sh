#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
TEMP_DIR="$(mktemp -d /tmp/hako-inspect-selected-dynamic-driver.XXXXXX)"
trap 'rm -rf -- "$TEMP_DIR"' EXIT
CC_CMD="${CC:-cc}"

for arg in "$@"; do
  case "$arg" in
    --repo-root|--repo-root=*|--driver|--driver=*)
      echo "[hako-check/selected-dynamic-provenance] reserved option: $arg" >&2
      exit 2
      ;;
  esac
done

"$CC_CMD" -I"$ROOT/plugins/nyash-json-plugin/c/yyjson" \
  -o "$TEMP_DIR/driver" \
  "$ROOT/lang/c-abi/tests/selected_dynamic_lowered_llvm_provenance_driver.c" \
  "$ROOT/lang/c-abi/shims/hako_aot.c" \
  "$ROOT/lang/c-abi/shims/hako_json_v1.c" \
  "$ROOT/plugins/nyash-json-plugin/c/yyjson/yyjson.c" -ldl

PYTHONPATH="$ROOT/tools/hako_check" python3 \
  "$ROOT/tools/hako_check/inspect_selected_dynamic_provenance.py" \
  "$@" --repo-root "$ROOT" --driver "$TEMP_DIR/driver"
