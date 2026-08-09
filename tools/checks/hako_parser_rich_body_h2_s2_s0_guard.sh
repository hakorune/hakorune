#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="hako-parser-rich-body-h2-s2-s0"
SCANNER="$ROOT/lang/src/compiler/parser/scan/parser_number_scan_box.hako"
PARTS="$ROOT/lang/src/compiler/parser/scan/parser_number_lexical_parts_v1.hako"
README="$ROOT/lang/src/compiler/parser/scan/README.md"
EXPR="$ROOT/lang/src/compiler/parser/expr/parser_expr_box.hako"
FIXTURE="$ROOT/tools/checks/fixtures/parser_rich_body_h2_s2_s0_v1.hako"
BIN="$ROOT/target/release/hakorune"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" cargo
guard_require_command "$TAG" python3
guard_require_command "$TAG" timeout
guard_require_files "$TAG" "$SCANNER" "$PARTS" "$README" "$EXPR" "$FIXTURE"

cd "$ROOT"
cargo build -q --release --features vm-reference --bin hakorune

vm_log=/tmp/hakorune-parser-rich-body-h2-s2-s0.vm.log
timeout 10s env NYASH_DISABLE_PLUGINS=1 "$BIN" --backend vm "$FIXTURE" >"$vm_log" 2>&1 || {
  tail -n 100 "$vm_log" >&2
  guard_fail "$TAG" "S0 numeric lexical-parts fixture failed or exceeded 10 seconds"
}
rg -q 'RC: 0' "$vm_log" || {
  tail -n 100 "$vm_log" >&2
  guard_fail "$TAG" "S0 numeric lexical-parts fixture returned nonzero"
}

python3 - "$SCANNER" "$PARTS" "$EXPR" <<'PY'
import re
import sys
from pathlib import Path

scanner_path, parts_path, expr_path = map(Path, sys.argv[1:])
scanner = scanner_path.read_text(encoding="utf-8")
parts = parts_path.read_text(encoding="utf-8")
expr = expr_path.read_text(encoding="utf-8")

for path, text in ((scanner_path, scanner), (parts_path, parts)):
    lines = len(text.splitlines())
    if lines >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path}={lines}")

if scanner.count("scan_parts(src, i)") != 2:
    raise SystemExit("scan_parts must have one definition and one scan_int call")

scan_int_match = re.search(r"  scan_int\(src, i\) \{(?P<body>.*?)\n  \}\n\}", scanner, re.S)
if not scan_int_match:
    raise SystemExit("scan_int body not found")
scan_int = scan_int_match.group("body")
for forbidden in ("loop(", "substring(", "src.length", "JsonParser"):
    if forbidden in scan_int:
        raise SystemExit(f"scan_int must remain projection-only: {forbidden}")
if scan_int.count("me.scan_parts(src, i)") != 1:
    raise SystemExit("scan_int must consume exactly one scan_parts result")

if "new ParserNumberLexicalPartsV1(" in parts or "new ParserNumberScanOutcomeV1(" in parts:
    raise SystemExit("lexical-parts constructors must remain confined to the scanner owner")
if scanner.count("new ParserNumberLexicalPartsV1(") != 3:
    raise SystemExit("scanner must own exactly three lexical-parts constructors")
if scanner.count("new ParserNumberScanOutcomeV1(") != 4:
    raise SystemExit("scanner must own Ready and InvalidStart outcome construction")
if "is_exact_unsuffixed_decimal()" not in parts:
    raise SystemExit("missing exact integer lexical admission")
for forbidden in ('scan_parts(src:', 'scan_int(src:', 'local scan_src =', 'local start = 0 + i'):
    if forbidden in scanner:
        raise SystemExit(f"retired source acceptance workaround returned: {forbidden}")

if expr.count("ParserNumberScanBox.scan_int(src, i)") != 1:
    raise SystemExit("live parse_number2 must remain on compatibility scan_int in S0")
if "scan_parts" in expr:
    raise SystemExit("S0 must not connect typed lexical parts to expression parsing")

combined = scanner + parts
for forbidden in (
    "ParserNodeProductV1", "SourceBodyV1", "ReturnSource", "TakeParameter",
    "VerifiedHomeAbi", "FunctionOwnerId", "method_transaction",
):
    if forbidden in combined:
        raise SystemExit(f"numeric scanner gained forbidden authority: {forbidden}")
for line in combined.splitlines():
    if line.lstrip().startswith("using ") and any(
        forbidden in line for forbidden in (".resolver.", ".mir.", ".loop_recipe.")
    ):
        raise SystemExit(f"numeric scanner imported forbidden authority: {line.strip()}")

print("numeric_traversal_owner=1")
print("compat_projection_from_parts=1")
print("exact_unsuffixed_integer_admission=1")
print("expression_connection=0")
print("return_body_method_connection=0")
print("semantic_physical_authority=0")
print("summary=ok")
PY

echo "[$TAG] ok"
