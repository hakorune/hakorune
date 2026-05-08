#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-native-ptr-decl-type"
cd "$ROOT_DIR"

echo "[$TAG] running M10c-native-ptr-declare-type guard"

REGISTRY="lang/src/shared/backend/ll_emit/runtime_decl_registry_box.hako"
RUNTIME_DECL="docs/development/current/main/design/runtime-decl-manifest-v0.toml"
GENERATED="lang/src/shared/backend/ll_emit/generated/runtime_decl_defaults.hako"
SSOT="docs/development/current/main/design/return-proof-vocabulary-ssot.md"
TASKBOARD="docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD="docs/development/current/main/phases/phase-293x/293x-051-M10C-NATIVE-PTR-DECLARE-TYPE.md"

rg -F -q 'if vc == "native_ptr_nonnull" { return "ptr" }' "$REGISTRY"
rg -F -q 'if vc == "native_ptr_nullable" { return "ptr" }' "$REGISTRY"
rg -F -q 'if vc == "native_ptr_dereferenceable" { return "ptr" }' "$REGISTRY"

python3 - <<'PY'
import pathlib
import sys
import tomllib

ROOT = pathlib.Path.cwd()
RUNTIME_DECL = ROOT / "docs/development/current/main/design/runtime-decl-manifest-v0.toml"
GENERATED = ROOT / "lang/src/shared/backend/ll_emit/generated/runtime_decl_defaults.hako"
SSOT = ROOT / "docs/development/current/main/design/return-proof-vocabulary-ssot.md"
TASKBOARD = ROOT / "docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD = ROOT / "docs/development/current/main/phases/phase-293x/293x-051-M10C-NATIVE-PTR-DECLARE-TYPE.md"

NATIVE_CLASSES = (
    "native_ptr_nonnull",
    "native_ptr_nullable",
    "native_ptr_dereferenceable",
)
STRONG_ATTRS = ("noalias", "nonnull", "dereferenceable", "align")


def fail(message: str) -> None:
    print(f"[k2-wide-native-ptr-decl-type][fail] {message}", file=sys.stderr)
    raise SystemExit(1)


runtime_decl = tomllib.loads(RUNTIME_DECL.read_text())
for row in runtime_decl.get("rows", []):
    symbol = row.get("symbol", "<unknown>")
    values = [row.get("ret", "")] + list(row.get("args", []))
    for value in values:
        if value in NATIVE_CLASSES:
            fail(f"{RUNTIME_DECL}:{symbol}: active native pointer rows remain blocked")
    for attr in row.get("attrs", []):
        normalized = attr.strip()
        if any(normalized == strong or normalized.startswith(strong + " ") for strong in STRONG_ATTRS):
            fail(f"{RUNTIME_DECL}:{symbol}: strong attr still blocked before M10c: {attr}")

generated = GENERATED.read_text()
for needle in NATIVE_CLASSES + ('"noalias"', '"nonnull"', '"dereferenceable"', '"align"'):
    if needle in generated:
        fail(f"{GENERATED}: generated defaults must not contain {needle}")

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
