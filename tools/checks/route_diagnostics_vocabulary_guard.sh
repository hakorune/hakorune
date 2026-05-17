#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
TAG="route-diagnostics-vocabulary"
cd "$ROOT_DIR"
source "$ROOT_DIR/tools/checks/lib/guard_common.sh"

PREFLIGHT="tools/checks/pure_first_route_preflight.py"
VOCAB="docs/reference/mir/route-diagnostics-vocabulary.md"
INDEX="docs/tools/check-scripts-index.md"
CARD="docs/development/current/main/phases/phase-293x/293x-627-ROUTE-DIAG-VOCAB-002-PREFLIGHT-VOCABULARY-GUARD.md"

echo "[$TAG] checking route diagnostics vocabulary"

guard_require_command "$TAG" python3
guard_require_command "$TAG" rg
guard_require_files "$TAG" "$PREFLIGHT" "$VOCAB" "$INDEX" "$CARD"
guard_expect_in_file "$TAG" '## Preflight Reasons' "$VOCAB" "vocabulary SSOT must list preflight reasons"
guard_expect_in_file "$TAG" '## Planner-Local Reasons' "$VOCAB" "vocabulary SSOT must separate planner-local reasons"
guard_expect_in_file "$TAG" '## Proof Vocabulary Boundary' "$VOCAB" "vocabulary SSOT must separate proof names"
guard_expect_in_file "$TAG" 'tools/checks/route_diagnostics_vocabulary_guard.sh' "$INDEX" "check index must list this guard"
guard_expect_in_file "$TAG" 'route-diagnostics-vocabulary.md' "$CARD" "current card must name vocabulary SSOT"

python3 - "$PREFLIGHT" "$VOCAB" <<'PY'
import ast
import re
import sys
from pathlib import Path

preflight_path = Path(sys.argv[1])
vocab_path = Path(sys.argv[2])


def const_str(node: ast.AST) -> str | None:
    if isinstance(node, ast.Constant) and isinstance(node.value, str):
        return node.value
    return None


class ReasonVisitor(ast.NodeVisitor):
    def __init__(self) -> None:
        self.reasons: set[str] = set()

    def visit_Call(self, node: ast.Call) -> None:  # noqa: N802 - ast visitor API
        name = node.func.id if isinstance(node.func, ast.Name) else ""
        if name == "failure" and len(node.args) >= 4:
            reason = const_str(node.args[3])
            if reason:
                self.reasons.add(reason)
        if name == "classify_unsupported_capability_plans":
            for keyword in node.keywords:
                if keyword.arg != "reason":
                    continue
                reason = const_str(keyword.value)
                if reason:
                    self.reasons.add(reason)
        self.generic_visit(node)


def preflight_reasons(path: Path) -> set[str]:
    tree = ast.parse(path.read_text(encoding="utf-8"), filename=str(path))
    visitor = ReasonVisitor()
    visitor.visit(tree)
    return visitor.reasons


def vocab_reasons(path: Path) -> set[str]:
    text = path.read_text(encoding="utf-8")
    match = re.search(
        r"^## Preflight Reasons\n(?P<section>.*?)(?:^## |\Z)",
        text,
        flags=re.MULTILINE | re.DOTALL,
    )
    if not match:
        raise SystemExit("missing ## Preflight Reasons section")
    section = match.group("section")
    return set(re.findall(r"^\| `([^`]+)` \|", section, flags=re.MULTILINE))


script_reasons = preflight_reasons(preflight_path)
ssot_reasons = vocab_reasons(vocab_path)

missing_from_ssot = sorted(script_reasons - ssot_reasons)
stale_in_ssot = sorted(ssot_reasons - script_reasons)

if missing_from_ssot or stale_in_ssot:
    if missing_from_ssot:
        print("missing_from_ssot=" + ",".join(missing_from_ssot), file=sys.stderr)
    if stale_in_ssot:
        print("stale_in_ssot=" + ",".join(stale_in_ssot), file=sys.stderr)
    raise SystemExit(1)

print(f"[route-diagnostics-vocabulary] ok reasons={len(script_reasons)}")
PY

echo "[$TAG] ok"
