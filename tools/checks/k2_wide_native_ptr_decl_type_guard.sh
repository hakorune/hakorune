#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-native-ptr-decl-type"
cd "$ROOT_DIR"

echo "[$TAG] running M10c-native-ptr-declare-type guard"

REGISTRY="lang/src/shared/backend/ll_emit/runtime_decl_registry_box.hako"
SSOT="docs/development/current/main/design/return-proof-vocabulary-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-051-M10C-NATIVE-PTR-DECLARE-TYPE.md"

rg -F -q 'if vc == "native_ptr_nonnull" { return "ptr" }' "$REGISTRY"
rg -F -q 'if vc == "native_ptr_nullable" { return "ptr" }' "$REGISTRY"
rg -F -q 'if vc == "native_ptr_dereferenceable" { return "ptr" }' "$REGISTRY"

python3 - <<'PY'
import pathlib
import sys

ROOT = pathlib.Path.cwd()
SSOT = ROOT / "docs/development/current/main/design/return-proof-vocabulary-ssot.md"
TASKBOARD = ROOT / "docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD = ROOT / "docs/development/current/main/phases/phase-293x/293x-051-M10C-NATIVE-PTR-DECLARE-TYPE.md"


def fail(message: str) -> None:
    print(f"[k2-wide-native-ptr-decl-type][fail] {message}", file=sys.stderr)
    raise SystemExit(1)


for path, needle in [
    (SSOT, "Decision: accepted M10c-native-ptr-declare-type lock."),
    (SSOT, "native_ptr_nonnull -> ptr"),
    (TASKBOARD, "`M10c-native-ptr-declare-type` | `live-narrow`"),
    (CARD, "M10c-native-ptr-declare-type is live as type spelling only."),
]:
    if needle not in path.read_text():
        fail(f"{path}: missing lock text: {needle}")
PY

if rg -F -q 'native_ptr_nonnull' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: C shim .inc must not infer native pointer declaration types" >&2
  exit 1
fi

echo "[$TAG] ok"
