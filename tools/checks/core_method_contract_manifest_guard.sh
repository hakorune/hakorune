#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="core-method-contract-manifest-guard"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" \
  "$ROOT_DIR/tools/core_method_contract_manifest_codegen.py" \
  "$ROOT_DIR/lang/src/runtime/meta/core_method_contract_box.hako" \
  "$ROOT_DIR/lang/src/runtime/meta/generated/core_method_contract_manifest.json"

python3 "$ROOT_DIR/tools/core_method_contract_manifest_codegen.py" --check
