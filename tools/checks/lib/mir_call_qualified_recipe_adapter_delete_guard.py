#!/usr/bin/env python3
"""Guard the caller-zero QualifiedMethodRecipePortV1 delete-set."""
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parents[3]
BUILDER = ROOT / "src/mir/builder"
ADAPTER = BUILDER / "normal_callable_semantic_loan_port.rs"
MAIN_ROOT = BUILDER / "normal_callable_semantic_loan_port/main_root.rs"
LOWERER = BUILDER / "resolved_lowering/mod.rs"

impl_pattern = re.compile(
    r"impl\s+(?:crate::mir::builder::resolved_lowering::)?QualifiedMethodRecipePortV1\s*"
    r"for\s+([A-Za-z0-9_]+)"
)
impls = []
for path in BUILDER.rglob("*.rs"):
    impls.extend((path, owner) for owner in impl_pattern.findall(path.read_text()))

if len(impls) != 1 or impls[0][1] != "MainQualifiedMethodRecipePort":
    print(f"[qualified-recipe-delete-guard] unexpected implementations: {impls}", file=sys.stderr)
    raise SystemExit(1)

if "QualifiedMethodRecipePortV1" in ADAPTER.read_text():
    print("[qualified-recipe-delete-guard] deleted adapter still mentions the trait", file=sys.stderr)
    raise SystemExit(1)

call_files = []
for path in ROOT.joinpath("src").rglob("*.rs"):
    if "lower_resolved_trivial_body_with_qualified_method_port_v1" in path.read_text():
        call_files.append(path.relative_to(ROOT).as_posix())
expected_files = {
    "src/mir/builder/resolved_lowering/mod.rs",
    "src/mir/builder/normal_callable_semantic_loan_port/main_root.rs",
}
if set(call_files) != expected_files:
    print(f"[qualified-recipe-delete-guard] unexpected lowerer references: {call_files}", file=sys.stderr)
    raise SystemExit(1)
if MAIN_ROOT.read_text().count("lower_resolved_trivial_body_with_qualified_method_port_v1(") != 1:
    print("[qualified-recipe-delete-guard] Main lowerer caller count changed", file=sys.stderr)
    raise SystemExit(1)
if LOWERER.read_text().count("fn lower_resolved_trivial_body_with_qualified_method_port_v1(") != 1:
    print("[qualified-recipe-delete-guard] lowerer definition count changed", file=sys.stderr)
    raise SystemExit(1)

print("[qualified-recipe-delete-guard] one Main implementation and one lowerer caller")
