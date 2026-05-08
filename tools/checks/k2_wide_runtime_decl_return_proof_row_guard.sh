#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-runtime-decl-return-proof-row"
cd "$ROOT_DIR"

echo "[$TAG] running M10c-proof-row runtime-decl return proof row guard"

cargo test -q runtime_decl_return_proof

python3 - <<'PY'
import pathlib
import sys
import tomllib

ROOT = pathlib.Path.cwd()
FIXTURE = ROOT / "docs/development/current/main/design/runtime-decl-return-proof-fixture-v0.toml"
RUNTIME_DECL = ROOT / "docs/development/current/main/design/runtime-decl-manifest-v0.toml"
GENERATED = ROOT / "lang/src/shared/backend/ll_emit/generated/runtime_decl_defaults.hako"
SSOT = ROOT / "docs/development/current/main/design/return-proof-vocabulary-ssot.md"
TASKBOARD = ROOT / "docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CARD = ROOT / "docs/development/current/main/phases/phase-293x/293x-050-M10C-RUNTIME-DECL-RETURN-PROOF-ROW.md"
RUST = ROOT / "src/abi/runtime_decl_return_proof.rs"

STRONG_ATTRS = ("noalias", "nonnull", "dereferenceable", "align")


def fail(message: str) -> None:
    print(f"[k2-wide-runtime-decl-return-proof-row][fail] {message}", file=sys.stderr)
    raise SystemExit(1)


fixture = tomllib.loads(FIXTURE.read_text())
if fixture.get("status") != "schema-fixture-only":
    fail(f"{FIXTURE}: status must be schema-fixture-only")

rows = {row.get("symbol"): row for row in fixture.get("rows", [])}
expected_symbols = {
    "fixture.handle.borrowed",
    "fixture.native.nonnull",
    "fixture.native.dereferenceable",
}
if set(rows) != expected_symbols:
    fail(f"{FIXTURE}: fixture rows drifted")

handle = rows["fixture.handle.borrowed"]
if handle.get("ret") != "handle_existing_borrowed":
    fail(f"{FIXTURE}: handle row ret drifted")
if handle.get("ret_proofs") != ["no_refcount_mutation", "no_registry_write"]:
    fail(f"{FIXTURE}: handle row proofs drifted")
if handle.get("ret_proof_export") != "disabled":
    fail(f"{FIXTURE}: handle row must stay export-disabled")

nonnull = rows["fixture.native.nonnull"]
if nonnull.get("ret") != "native_ptr_nonnull":
    fail(f"{FIXTURE}: native nonnull ret drifted")
if nonnull.get("ret_proofs") != ["nonnull"]:
    fail(f"{FIXTURE}: native nonnull proofs drifted")
if nonnull.get("ret_proof_export") != "disabled":
    fail(f"{FIXTURE}: native nonnull row must stay export-disabled")

deref = rows["fixture.native.dereferenceable"]
if deref.get("ret") != "native_ptr_dereferenceable":
    fail(f"{FIXTURE}: native dereferenceable ret drifted")
if deref.get("ret_dereferenceable_len") != "len":
    fail(f"{FIXTURE}: dereferenceable len token drifted")
if deref.get("ret_dereferenceable_align") != 16:
    fail(f"{FIXTURE}: dereferenceable align drifted")
if deref.get("ret_proofs") != ["nonnull", "dereferenceable_bytes", "alignment"]:
    fail(f"{FIXTURE}: native dereferenceable proofs drifted")
if deref.get("ret_proof_export") != "disabled":
    fail(f"{FIXTURE}: native dereferenceable row must stay export-disabled")

runtime_decl = tomllib.loads(RUNTIME_DECL.read_text())
for row in runtime_decl.get("rows", []):
    symbol = row.get("symbol", "<unknown>")
    for forbidden_key in ("ret_proofs", "ret_proof_export"):
        if forbidden_key in row:
            fail(f"{RUNTIME_DECL}:{symbol}: active runtime-decl proof rows are still blocked")
    for attr in row.get("attrs", []):
        normalized = attr.strip()
        if any(normalized == strong or normalized.startswith(strong + " ") for strong in STRONG_ATTRS):
            fail(f"{RUNTIME_DECL}:{symbol}: strong attr still blocked before M10c: {attr}")

generated = GENERATED.read_text()
for needle in ('"noalias"', '"nonnull"', '"dereferenceable"', '"align"', "ret_proof"):
    if needle in generated:
        fail(f"{GENERATED}: generated defaults must not contain {needle}")

rust = RUST.read_text()
for token in [
    "RuntimeDeclReturnProofRow",
    "ReturnProofExportMode",
    "validate_runtime_decl_return_proof_row",
    "VerifierRequired",
    "Exported",
    "strong attr export is still blocked",
]:
    if token not in rust:
        fail(f"{RUST}: missing validator token {token}")

for path, needle in [
    (SSOT, "Decision: accepted M10c-proof-row schema lock."),
    (SSOT, "runtime-decl return proof row schema"),
    (TASKBOARD, "`M10c-proof-row runtime-decl return proof row` | `live-narrow`"),
    (CARD, "M10c-proof-row is live as schema/validator only."),
]:
    if needle not in path.read_text():
        fail(f"{path}: missing lock text: {needle}")
PY

if rg -F -q 'ret_proof' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: C shim .inc must not consume return proof rows in M10c-proof-row" >&2
  exit 1
fi

if rg -F -q 'native_ptr_nonnull' lang/c-abi/shims -g '*.inc'; then
  echo "[$TAG] ERROR: C shim .inc must not infer native pointer return proof in M10c-proof-row" >&2
  exit 1
fi

echo "[$TAG] ok"
