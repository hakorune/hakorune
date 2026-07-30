#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="resolved-region-flow-r0-seam"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" \
  "$ROOT/crates/hakorune_mir_core/src/binding_id.rs" \
  "$ROOT/crates/hakorune_mir_builder/src/binding_context.rs" \
  "$ROOT/src/mir/builder/vars/lexical_scope.rs" \
  "$ROOT/crates/hakorune_frontend_ast/src/ast_node.rs" \
  "$ROOT/src/mir/builder/control_flow/plan/recipe_tree/common.rs" \
  "$ROOT/src/mir/builder/control_flow/plan/lowerer/exit_lowering.rs"

python3 - "$ROOT" <<'PY'
from pathlib import Path
import re
import sys

root = Path(sys.argv[1])
canonical = (root / "crates/hakorune_mir_core/src/binding_id.rs").read_text()
binding_ctx = (root / "crates/hakorune_mir_builder/src/binding_context.rs").read_text()
lexical = (root / "src/mir/builder/vars/lexical_scope.rs").read_text()
ast = (root / "crates/hakorune_frontend_ast/src/ast_node.rs").read_text()
recipe_exit = (root / "src/mir/builder/control_flow/plan/recipe_tree/common.rs").read_text()
lower_exit = (root / "src/mir/builder/control_flow/plan/lowerer/exit_lowering.rs").read_text()
facts = (root / "src/mir/builder/control_flow/plan/generic_loop/facts/extract/v1.rs").read_text()

required = {
    "canonical BindingId": (canonical, r"pub struct BindingId\(pub u32\)"),
    "BindingContext snapshot": (binding_ctx, r"pub fn snapshot\(&self\) -> Self"),
    "canonical declaration allocator": (lexical, r"allocate_binding_id\(\)"),
    "source Break without target": (ast, r"Break \{ span: Span \}"),
    "source Continue without target": (ast, r"Continue \{ span: Span \}"),
    "Recipe depth carrier": (recipe_exit, r"Break \{ depth: u32 \}"),
    "Lower depth recount": (lower_exit, r"stack\.len\(\).*depth|checked_sub\(depth\)"),
    "Facts AST-only signature": (facts, r"condition: &ASTNode,\s*body: &\[ASTNode\]"),
}
for label, (text, pattern) in required.items():
    if not re.search(pattern, text, flags=re.DOTALL):
        raise SystemExit(f"[{label}] inventory drift")

if "MirBuilder" in facts or "BindingContext" in facts:
    raise SystemExit("generic Facts timing changed; reclassify R0 binding seam")

print("canonical_binding_id_owner=present")
print("preplan_resolved_binding_view=missing")
print("ownership_private_binding_id=retired")
print("resolved_control_target_owner=missing")
print("r1_implementation_connection=0")
print("summary=ok")
PY

echo "[$TAG] ok"
