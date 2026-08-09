#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-parser-rich-body-h2-s2-r0"
PARSER="$ROOT/lang/src/compiler/parser/parser_box.hako"
SUPPORT="$ROOT/lang/src/compiler/parser/support/parser_compat_text_box.hako"
FIXTURE="$ROOT/tools/checks/fixtures/parser_rich_body_h2_s2_r0_v1.hako"
BIN="$ROOT/target/release/hakorune"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$PARSER" "$SUPPORT" "$FIXTURE"

cd "$ROOT"
cargo build -q --release --features vm-reference --bin hakorune

vm_log=/tmp/hakorune-parser-rich-body-h2-s2-r0.vm.log
timeout 10s env NYASH_DISABLE_PLUGINS=1 "$BIN" --backend vm "$FIXTURE" >"$vm_log" 2>&1 || {
  tail -n 100 "$vm_log" >&2
  guard_fail "$TAG" "R0 support fixture failed or exceeded 10 seconds"
}
rg -q 'RC: 0' "$vm_log" || {
  tail -n 100 "$vm_log" >&2
  guard_fail "$TAG" "R0 support fixture returned nonzero"
}

python3 - "$ROOT" "$PARSER" "$SUPPORT" <<'PY'
import sys
from pathlib import Path

root, parser, support = map(Path, sys.argv[1:])
parser_text = parser.read_text(encoding="utf-8")
support_text = support.read_text(encoding="utf-8")

parser_lines = len(parser_text.splitlines())
support_lines = len(support_text.splitlines())
if parser_lines >= 760:
    raise SystemExit(f"ParserBox facade must remain below 760 lines: {parser_lines}")
if support_lines >= 800:
    raise SystemExit(f"support source must remain below 800 lines: {support_lines}")

required_parser = (
    "using lang.compiler.parser.support.parser_compat_text_box",
    "return ParserCompatTextBox.escape_json(s)",
)
for needle in required_parser:
    if needle not in parser_text:
        raise SystemExit(f"missing ParserBox facade delegation: {needle}")

required_support = (
    "static box ParserCompatTextBox",
    "escape_json(value)",
)
for needle in required_support:
    if needle not in support_text:
        raise SystemExit(f"missing compatibility text owner: {needle}")

for forbidden in (
    "ParserNodeProductV1", "SourceCarrierBuilderV1", "Take",
    "HomeAbi", "Resolver", "Recipe", "MIR", "FuncScanner", "StageB",
    "JsonParser", "parse_stmt_product", "parse_block_product",
):
    if forbidden in support_text:
        raise SystemExit(f"support helper gained forbidden authority: {forbidden}")

print(f"parser_box_lines={parser_lines}")
print(f"support_lines={support_lines}")
print("grammar_change=0")
print("rich_result_activation=0")
print("parser_state_duplication=0")
print("compat_projection_owner=1")
print("summary=ok")
PY

echo "[$TAG] ok"
