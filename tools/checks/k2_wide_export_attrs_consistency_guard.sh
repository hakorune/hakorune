#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT_DIR"

echo "[k2-wide-export-attrs-consistency] running export attrs consistency guard"

python3 - <<'PY'
import ast
import pathlib
import re
import sys
import tomllib

ROOT = pathlib.Path.cwd()

LLVM_ATTRS = ROOT / "src/llvm_py/instructions/llvm_attrs.py"
RUNTIME_DECL_MANIFEST = ROOT / "docs/development/current/main/design/runtime-decl-manifest-v0.toml"
RUNTIME_DECL_DEFAULTS = ROOT / "lang/src/shared/backend/ll_emit/generated/runtime_decl_defaults.hako"
TASKBOARD = ROOT / "docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md"
CURRENT_OPT = ROOT / "docs/development/current/main/design/current-optimization-mechanisms-ssot.md"
ROADMAP = ROOT / "docs/development/current/main/design/optimization-layer-roadmap-ssot.md"

WEAK_LLVM_ATTRS = {"readonly", "nocapture"}
WEAK_RUNTIME_DECL_ATTRS = {"nounwind", "readonly", "willreturn"}
FORBIDDEN_STRONG_ATTR_PREFIXES = (
    "noalias",
    "nonnull",
    "dereferenceable",
    "align",
)


def fail(message: str) -> None:
    print(f"[k2-wide-export-attrs-consistency][fail] {message}", file=sys.stderr)
    raise SystemExit(1)


def check_forbidden_attr(attr: str, origin: str) -> None:
    normalized = attr.strip()
    for prefix in FORBIDDEN_STRONG_ATTR_PREFIXES:
        if normalized == prefix or normalized.startswith(prefix + " "):
            fail(f"{origin}: strong attr is not verifier-owned yet: {attr!r}")


def attr_call_name(node: ast.AST) -> str:
    if isinstance(node, ast.Name):
        return node.id
    if isinstance(node, ast.Attribute):
        return node.attr
    return ""


def string_arg(node: ast.AST) -> str | None:
    if isinstance(node, ast.Constant) and isinstance(node.value, str):
        return node.value
    return None


def check_llvm_py_policy() -> None:
    tree = ast.parse(LLVM_ATTRS.read_text(), filename=str(LLVM_ATTRS))
    attrs: list[str] = []
    for node in ast.walk(tree):
        if not isinstance(node, ast.Call):
            continue
        name = attr_call_name(node.func)
        if name in {"_add_function_attr", "_add_arg_attr"} and len(node.args) >= 2:
            attr = string_arg(node.args[1])
            if attr is None:
                fail(f"{LLVM_ATTRS}: dynamic runtime attr wiring is not allowed")
            attrs.append(attr)
        elif name in {"add", "add_attribute"} and node.args:
            attr = string_arg(node.args[0])
            if attr is not None:
                attrs.append(attr)

    for attr in attrs:
        check_forbidden_attr(attr, str(LLVM_ATTRS))
        if attr not in WEAK_LLVM_ATTRS:
            fail(f"{LLVM_ATTRS}: unexpected attr outside weak policy: {attr!r}")


def check_runtime_decl_manifest() -> None:
    data = tomllib.loads(RUNTIME_DECL_MANIFEST.read_text())
    for row in data.get("rows", []):
        symbol = row.get("symbol", "<unknown>")
        for attr in row.get("attrs", []):
            check_forbidden_attr(attr, f"{RUNTIME_DECL_MANIFEST}:{symbol}")
            if attr not in WEAK_RUNTIME_DECL_ATTRS:
                fail(f"{RUNTIME_DECL_MANIFEST}:{symbol}: unexpected attr {attr!r}")

    generated_attrs = set(
        re.findall(r'attrs_\d+\.push\("([^"]+)"\)', RUNTIME_DECL_DEFAULTS.read_text())
    )
    for attr in sorted(generated_attrs):
        check_forbidden_attr(attr, str(RUNTIME_DECL_DEFAULTS))
        if attr not in WEAK_RUNTIME_DECL_ATTRS:
            fail(f"{RUNTIME_DECL_DEFAULTS}: unexpected generated attr {attr!r}")


def require_text(path: pathlib.Path, needle: str) -> None:
    if needle not in path.read_text():
        fail(f"{path}: missing lock text: {needle}")


def check_docs_lock() -> None:
    require_text(TASKBOARD, "`M10a export attrs consistency gate`")
    require_text(TASKBOARD, "`M10b LLVM export attrs widening`")
    require_text(CURRENT_OPT, "export-attrs consistency guard")
    require_text(ROADMAP, "export-attrs consistency guard")


check_llvm_py_policy()
check_runtime_decl_manifest()
check_docs_lock()
print("[k2-wide-export-attrs-consistency] ok")
PY
