#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-return-proof-vocab"
cd "$ROOT_DIR"

echo "[$TAG] running M10c-pre return proof vocabulary guard"

cargo test -q return_proof

python3 - <<'PY'
import pathlib
import sys
import tomllib

ROOT = pathlib.Path.cwd()
MANIFEST = ROOT / "docs/development/current/main/design/return-proof-vocabulary-v0.toml"
RUST = ROOT / "src/abi/return_proof.rs"
RUNTIME_DECL = ROOT / "docs/development/current/main/design/runtime-decl-manifest-v0.toml"
TASKBOARD = ROOT / "docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
SSOT = ROOT / "docs/development/current/main/design/return-proof-vocabulary-ssot.md"
CURRENT = ROOT / "docs/development/current/main/CURRENT_STATE.toml"

EXPECTED_CLASSES = [
    "imm_i64",
    "handle_existing_borrowed",
    "handle_existing_owned_ref",
    "handle_fresh_owned",
    "native_ptr_nonnull",
    "native_ptr_nullable",
    "native_ptr_dereferenceable",
]
EXPECTED_HANDLE_CLASSES = [
    "handle_existing_borrowed",
    "handle_existing_owned_ref",
    "handle_fresh_owned",
]
EXPECTED_NATIVE_PTR_CLASSES = [
    "native_ptr_nonnull",
    "native_ptr_nullable",
    "native_ptr_dereferenceable",
]
EXPECTED_PROOFS = [
    "fresh",
    "nonnull",
    "dereferenceable_bytes",
    "alignment",
    "noalias_scope",
    "no_refcount_mutation",
    "no_registry_write",
]
STRONG_ATTRS = ("noalias", "nonnull", "dereferenceable", "align")


def fail(message: str) -> None:
    print(f"[k2-wide-return-proof-vocab][fail] {message}", file=sys.stderr)
    raise SystemExit(1)


data = tomllib.loads(MANIFEST.read_text())
if data.get("return_classes") != EXPECTED_CLASSES:
    fail("return_classes drifted")
if data.get("handle_return_classes") != EXPECTED_HANDLE_CLASSES:
    fail("handle_return_classes drifted")
if data.get("native_pointer_return_classes") != EXPECTED_NATIVE_PTR_CLASSES:
    fail("native_pointer_return_classes drifted")
if data.get("proofs") != EXPECTED_PROOFS:
    fail("proof vocabulary drifted")

rust = RUST.read_text()
for token in EXPECTED_CLASSES + EXPECTED_PROOFS:
    if token not in rust:
        fail(f"{RUST}: missing token {token}")
for token in [
    "may_export_llvm_pointer_attr",
    "is_handle_class",
    "is_native_pointer_class",
]:
    if token not in rust:
        fail(f"{RUST}: missing API token {token}")

runtime_decl = tomllib.loads(RUNTIME_DECL.read_text())
for row in runtime_decl.get("rows", []):
    symbol = row.get("symbol", "<unknown>")
    for attr in row.get("attrs", []):
        normalized = attr.strip()
        if any(normalized == strong or normalized.startswith(strong + " ") for strong in STRONG_ATTRS):
            fail(f"{RUNTIME_DECL}:{symbol}: strong attr still blocked before M10c: {attr}")

for path, needle in [
    (TASKBOARD, "`M10c-pre pointer/handle return proof vocabulary` | `live-narrow`"),
    (SSOT, "handle return proof:"),
    (SSOT, "native pointer return proof:"),
    (CURRENT, "293x-049 M10c-pre return proof vocabulary landed"),
]:
    if needle not in path.read_text():
        fail(f"{path}: missing lock text: {needle}")
PY

if rg -F -q 'native_ptr_nonnull' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: .inc must not infer native pointer return proof in M10c-pre" >&2
  exit 1
fi

echo "[$TAG] ok"
