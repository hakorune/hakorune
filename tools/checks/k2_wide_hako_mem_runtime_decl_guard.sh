#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="k2-wide-hako-mem-runtime-decl"
cd "$ROOT_DIR"

echo "[$TAG] running hako_mem native pointer runtime-decl guard"

python3 tools/backend_runtime_decl_manifest_codegen.py --check

python3 - <<'PY'
import pathlib
import sys
import tomllib

ROOT = pathlib.Path.cwd()
MANIFEST = ROOT / "docs/development/current/main/design/runtime-decl-manifest-v0.toml"
GENERATED = ROOT / "lang/src/shared/backend/ll_emit/generated/runtime_decl_defaults.hako"
SSOT = ROOT / "docs/development/current/main/design/return-proof-vocabulary-ssot.md"
TASKBOARD = ROOT / "docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
REALLOC_CARD = ROOT / "docs/development/current/main/phases/phase-293x/293x-053-HAKO-MEM-REALLOC-RUNTIME-DECL.md"
ARG_EMIT_CARD = ROOT / "docs/development/current/main/phases/phase-293x/293x-054-NATIVE-PTR-CALL-ARG-EMIT.md"
FREE_CARD = ROOT / "docs/development/current/main/phases/phase-293x/293x-055-HAKO-MEM-FREE-VOID-RUNTIME-DECL.md"
SHIMS_README = ROOT / "lang/c-abi/shims/README.md"
CALL_POLICY = ROOT / "lang/src/shared/backend/ll_emit/call_policy_box.hako"
LL_TEXT = ROOT / "lang/src/shared/backend/ll_emit/ll_text_emit_box.hako"
REGISTRY = ROOT / "lang/src/shared/backend/ll_emit/runtime_decl_registry_box.hako"

ALLOWED_NATIVE_PTR_ROWS = {
    "hako_mem_alloc": {
        "args": ["imm_i64"],
        "ret": "native_ptr_nullable",
        "attrs": ["nounwind", "willreturn"],
        "memory": "readwrite",
        "lanes": ["hako-ll-min-v0", "compare"],
    },
    "hako_mem_realloc": {
        "args": ["native_ptr_nullable", "imm_i64"],
        "ret": "native_ptr_nullable",
        "attrs": ["nounwind", "willreturn"],
        "memory": "readwrite",
        "lanes": ["hako-ll-min-v0", "compare"],
    },
    "hako_mem_free": {
        "args": ["native_ptr_nullable"],
        "ret": "void",
        "attrs": ["nounwind", "willreturn"],
        "memory": "readwrite",
        "lanes": ["hako-ll-min-v0", "compare"],
    },
}
NATIVE_CLASSES = {
    "native_ptr_nonnull",
    "native_ptr_nullable",
    "native_ptr_dereferenceable",
}
STRONG_ATTRS = ("noalias", "nonnull", "dereferenceable", "align")


def fail(message: str) -> None:
    print(f"[k2-wide-hako-mem-runtime-decl][fail] {message}", file=sys.stderr)
    raise SystemExit(1)


data = tomllib.loads(MANIFEST.read_text())
rows = data.get("rows", [])
by_symbol = {row.get("symbol"): row for row in rows}

for symbol, expected in ALLOWED_NATIVE_PTR_ROWS.items():
    row = by_symbol.get(symbol)
    if row is None:
        fail(f"{MANIFEST}: missing {symbol} row")
    for key, value in expected.items():
        if row.get(key) != value:
            fail(f"{MANIFEST}:{symbol}: {key} drifted: {row.get(key)!r}")
    for forbidden_key in ("ret_proofs", "ret_proof_export"):
        if forbidden_key in row:
            fail(f"{MANIFEST}:{symbol}: active row must not carry {forbidden_key}")

for row in rows:
    symbol = row.get("symbol", "<unknown>")
    values = [row.get("ret", "")] + list(row.get("args", []))
    if any(value in NATIVE_CLASSES for value in values):
        if symbol not in ALLOWED_NATIVE_PTR_ROWS:
            fail(f"{MANIFEST}:{symbol}: unexpected active native pointer row")
    for attr in row.get("attrs", []):
        normalized = attr.strip()
        if any(normalized == strong or normalized.startswith(strong + " ") for strong in STRONG_ATTRS):
            fail(f"{MANIFEST}:{symbol}: strong attr still blocked before M10c: {attr}")

generated = GENERATED.read_text()
for needle in [
    '.set("symbol", "hako_mem_alloc")',
    '.set("symbol", "hako_mem_realloc")',
    '.set("symbol", "hako_mem_free")',
    '.set("ret", "native_ptr_nullable")',
    '.set("ret", "void")',
    '.push("native_ptr_nullable")',
    '.push("imm_i64")',
    '.push("nounwind")',
    '.push("willreturn")',
]:
    if needle not in generated:
        fail(f"{GENERATED}: missing generated hako_mem token {needle}")
for forbidden in ['"noalias"', '"nonnull"', '"dereferenceable"', '"align"', "ret_proof"]:
    if forbidden in generated:
        fail(f"{GENERATED}: generated defaults must not contain {forbidden}")

for path, needle in [
    (SSOT, "Decision: accepted M10c-hako-mem-alloc-row lock."),
    (SSOT, "Decision: accepted M10c-hako-mem-realloc-row lock."),
    (SSOT, "Decision: accepted M10c-hako-mem-free-void-row lock."),
    (SSOT, "hako_mem_alloc -> native_ptr_nullable"),
    (SSOT, "hako_mem_realloc -> native_ptr_nullable"),
    (SSOT, "hako_mem_free(native_ptr_nullable) -> void"),
    (TASKBOARD, "`M10c-hako-mem-alloc-row` | `live-narrow`"),
    (TASKBOARD, "`M10c-hako-mem-realloc-row` | `live-narrow`"),
    (TASKBOARD, "`M10c-hako-mem-free-void-row` | `live-narrow`"),
    (REALLOC_CARD, "M10c-hako-mem-realloc-row is live as the second active native pointer runtime-decl row."),
    (ARG_EMIT_CARD, "M10c-native-ptr-call-arg-emit is live for `.hako` ll_emit."),
    (FREE_CARD, "M10c-hako-mem-free-void-row is live as the third active hako.mem"),
    (SHIMS_README, "`native_ptr_*` spelling may appear only in MIR-owned LoweringPlan/extern-route"),
]:
    if needle not in path.read_text():
        fail(f"{path}: missing lock text: {needle}")

for path, needle in [
    (REGISTRY, "is_native_ptr_class(value_class)"),
    (CALL_POLICY, "_accept_manifest_extern_call"),
    (CALL_POLICY, "_manifest_arg_accepts"),
    (CALL_POLICY, "extern_arg_class_mismatch"),
    (LL_TEXT, "RuntimeDeclRegistryBox.is_native_ptr_class(arg_kind)"),
    (LL_TEXT, 'out = out + "ptr " + me._reg(me._resolve_alias(facts, src_reg))'),
    (REGISTRY, 'if vc == "void" { return "void" }'),
    (LL_TEXT, 'if ret_ty == "void"'),
    (LL_TEXT, 'call void @'),
]:
    if needle not in path.read_text():
        fail(f"{path}: missing native pointer arg emit lock: {needle}")
PY

python3 - <<'PY'
import pathlib
import sys

ROOT = pathlib.Path.cwd()
SHIMS = ROOT / "lang/c-abi/shims"
ALLOWED_ROUTE_FACT_CONSUMERS = {
    "hako_llvmc_ffi_lowering_plan_metadata.inc",
    "hako_llvmc_ffi_mir_call_shell.inc",
    "hako_llvmc_ffi_same_module_prepass.inc",
}

violations = []
for path in SHIMS.glob("*.inc"):
    if path.name in ALLOWED_ROUTE_FACT_CONSUMERS:
        continue
    if "native_ptr_nullable" in path.read_text():
        violations.append(path.relative_to(ROOT))

if violations:
    print(
        "[k2-wide-hako-mem-runtime-decl] ERROR: raw C shim .inc must not infer native pointer row semantics",
        file=sys.stderr,
    )
    for path in violations:
        print(f"  {path}", file=sys.stderr)
    raise SystemExit(1)
PY

echo "[$TAG] ok"
