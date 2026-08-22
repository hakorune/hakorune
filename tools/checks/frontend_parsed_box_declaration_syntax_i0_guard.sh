#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="frontend-parsed-box-declaration-syntax-i0"
AUTHORITY="$ROOT/src/parser/source_authority.rs"
SYNTAX="$ROOT/src/parser/source_authority/declaration_syntax.rs"
MODEL="$ROOT/src/parser/source_seal/model.rs"
FINALIZE="$ROOT/src/parser/source_seal/finalize.rs"
BOX="$ROOT/src/parser/declarations/box_def/mod.rs"
RESOLVER="$ROOT/src/parser/source_resolver_handoff.rs"
TESTS="$ROOT/src/parser/source_seal_finalizer_tests.rs"
README="$ROOT/src/parser/README.md"
REFERENCE="$ROOT/docs/reference/language/callable-contracts.md"
CARD="$ROOT/docs/development/current/main/investigations/normal-module-parser-box-declaration-syntax-d0-2026-08-23.md"
INDEX="$ROOT/docs/tools/check-scripts-index.md"
source "$ROOT/tools/checks/lib/guard_common.sh"

guard_require_command "$TAG" python3
guard_require_files "$TAG" "$AUTHORITY" "$SYNTAX" "$MODEL" "$FINALIZE" "$BOX" "$RESOLVER" "$TESTS" "$README" "$REFERENCE" "$CARD" "$INDEX"

python3 - "$AUTHORITY" "$SYNTAX" "$MODEL" "$FINALIZE" "$BOX" "$RESOLVER" "$TESTS" "$README" "$REFERENCE" "$CARD" "$INDEX" <<'PY'
import sys
from pathlib import Path

authority, syntax, model, finalize, box, resolver, tests, readme, reference, card, index = map(
    Path, sys.argv[1:]
)
authority_text = authority.read_text(encoding="utf-8")
syntax_text = syntax.read_text(encoding="utf-8")
model_text = model.read_text(encoding="utf-8")
finalize_text = finalize.read_text(encoding="utf-8")
box_text = box.read_text(encoding="utf-8")
resolver_text = resolver.read_text(encoding="utf-8")
tests_text = tests.read_text(encoding="utf-8")
readme_text = readme.read_text(encoding="utf-8")
reference_text = reference.read_text(encoding="utf-8")
card_text = card.read_text(encoding="utf-8")
index_text = index.read_text(encoding="utf-8")

for needle in (
    "mod declaration_syntax;",
    "declaration_syntax: ParserBoxDeclarationSyntaxV1",
    "declaration_syntax: self.declaration_syntax",
):
    if needle not in authority_text:
        raise SystemExit(f"missing parser transaction syntax seam: {needle}")

if syntax_text.count("\n    Ordinary,") != 1:
    raise SystemExit("current syntax cohort must have one Ordinary kind")
if syntax_text.count("ParserBoxDeclarationKindV1::Ordinary") != 1:
    raise SystemExit("current syntax cohort must have one Ordinary constructor")
for forbidden in ("Static", "Interface", "Record"):
    if forbidden in syntax_text:
        raise SystemExit(f"unsupported declaration kind entered the current seal: {forbidden}")

for needle in (
    "declaration_syntax: ParserBoxDeclarationSyntaxV1",
    "fn declaration_syntax",
):
    if needle not in model_text:
        raise SystemExit(f"missing final seal syntax transport: {needle}")

for needle in (
    "final_name: &str",
    "final_is_sync: bool",
    "DeclarationNameMismatch",
    "DeclarationKindMismatch",
    "DeclarationSyncMismatch",
    "prepared.finalize_against(",
):
    if needle not in finalize_text:
        raise SystemExit(f"missing finalizer syntax validation: {needle}")

if box_text.count("ParserBoxDeclarationSyntaxV1::ordinary(") != 1:
    raise SystemExit("Box declaration syntax must have exactly one production capture")
if "ParserBoxDeclarationSyntaxV1" in resolver_text:
    raise SystemExit("resolver handoff must not issue or reconstruct Box declaration syntax")
if "ASTNode::BoxDeclaration" in authority_text:
    raise SystemExit("parser authority transaction must not re-issue syntax from AST")

if "r6_s3b_b4_captures_sync_box_syntax" not in tests_text:
    raise SystemExit("missing sync syntax transport test")
if not ("Ordinary-only" in card_text or "Ordinary only" in card_text) or (
    "parser-header capture" not in card_text and "parser header" not in card_text
):
    raise SystemExit("active card must retain the bounded source-syntax contract")
if "ParserBoxSourceSealV1" not in readme_text or "declaration syntax" not in readme_text:
    raise SystemExit("parser README must document the declaration-syntax seal")
if "R6-S3B-B4" not in reference_text:
    raise SystemExit("language reference must record the declaration-syntax seal")
if "frontend_parsed_box_declaration_syntax_i0_guard.sh" not in index_text:
    raise SystemExit("check index must list the reusable declaration-syntax guard")

for path in (authority, syntax, model, finalize, box):
    lines = len(path.read_text(encoding="utf-8").splitlines())
    if lines >= 800:
        raise SystemExit(f"source must remain below 800 lines: {path} ({lines})")
if len(authority.read_text(encoding="utf-8").splitlines()) >= 760:
    raise SystemExit("source_authority.rs crossed the 760-line split trigger")

print("one_parser_header_capture=1")
print("ordinary_only_current_seal=1")
print("finalizer_name_sync_validation=1")
print("resolver_reconstruction=0")
print("focused_syntax_evidence=1")
print("source_size_limits=1")
print("summary=ok")
PY

echo "[$TAG] ok"
