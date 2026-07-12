#!/usr/bin/env python3
"""Generate/check SourceAstVocabularyInventoryV1 from canonical Rust enums."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
OUT = ROOT / "tools/checks/fixtures/source_ast_vocabulary_inventory_v1.json"


AST_CONTEXT = {
    "Program": "root_body_only",
    "Assignment": "variable_target_only",
    "Literal": "literal_variant_policy",
    "UnaryOp": "unary_operator_policy",
    "Local": "untyped_bindings_only",
}
AST_ACCEPTED = {
    "If", "Loop", "Return", "Break", "Continue", "Variable", "BinaryOp",
    "MethodCall", "This", "Me",
}

CHILD_ORDER = {
    "Program": ["statements[]"],
    "Assignment": ["target", "value"],
    "CompoundAssignment": ["target", "value"],
    "Print": ["expression"],
    "If": ["condition", "then_body[]", "else_body[]?"],
    "Loop": ["condition", "body[]"],
    "LoopRange": ["start", "end", "body[]"],
    "Return": ["value?"],
    "BuildGate": ["then_items[]", "else_items[]?"],
    "Nowait": ["expression"],
    "TaskScope": ["body[]"],
    "ContextScope": ["value", "body[]"],
    "FastMemRegion": ["body[]"],
    "AwaitExpression": ["expression"],
    "QMarkPropagate": ["expression"],
    "MatchExpr": ["scrutinee", "arms[].1", "else_expr"],
    "EnumMatchExpr": ["scrutinee", "arms[].body[]", "else_expr?"],
    "ArrayLiteral": ["elements[]"],
    "MapLiteral": ["entries[].1"],
    "RecordLiteral": ["fields[].1"],
    "RecordUpdate": ["base", "updates[].1"],
    "Lambda": ["body[]"],
    "BlockExpr": ["prelude_stmts[]", "tail_expr"],
    "Arrow": ["sender", "receiver"],
    "TryCatch": ["try_body[]", "catch_clauses[].body[]", "finally_body[]?"],
    "Throw": ["expression"],
    "BoxDeclaration": ["invariants[]", "static_init[]?", "methods{}", "constructors{}"],
    "FunctionDeclaration": ["contracts[].condition", "body[]"],
    "GlobalVar": ["value"],
    "UnaryOp": ["operand"],
    "BinaryOp": ["left", "right"],
    "CheckExpr": ["items[].expression"],
    "GroupedAssignmentExpr": ["rhs"],
    "MethodCall": ["object", "arguments[]"],
    "FieldAccess": ["object"],
    "Index": ["target", "index"],
    "New": ["arguments[]", "field_initializers[].1"],
    "FromCall": ["arguments[]"],
    "Local": ["initial_values[]?"],
    "ScopeBox": ["body[]"],
    "Outbox": ["initial_values[]?"],
    "FunctionCall": ["arguments[]"],
    "Call": ["callee", "arguments[]"],
}

LITERAL_POLICY = {
    "String": ("Accepted", "source_literal"),
    "Integer": ("Accepted", "source_literal"),
    "TypedInteger": ("KnownUnsupported", "source.type_syntax_deferred"),
    "Float": ("KnownUnsupported", "source.literal_float_deferred"),
    "Bool": ("Accepted", "source_literal"),
    "Null": ("Accepted", "source_literal"),
    "Void": ("KnownUnsupported", "source.literal_void_deferred"),
}

HAKO_CARRIER = [
    {
        "distinction": "private_typed_source_carrier",
        "status": "Missing",
        "evidence": "parser functions return JSON fragment strings directly",
    },
    {
        "distinction": "UnaryOp",
        "status": "Lost",
        "evidence": "Minus becomes Binary(Int(0), rhs); Not/BitNot source nodes absent",
    },
    {
        "distinction": "Local_vs_Assignment",
        "status": "Lost",
        "evidence": "both statement routes emit ProgramV0 Local",
    },
    {
        "distinction": "Local_initializer_presence",
        "status": "Lost",
        "evidence": "missing initializer becomes Int(0)",
    },
    {
        "distinction": "Return_value_presence",
        "status": "Lost",
        "evidence": "bare return becomes Return(Int(0))",
    },
    {
        "distinction": "Me_This_Variable_kind",
        "status": "Lost",
        "evidence": "all identifiers emit Var(name)",
    },
    {
        "distinction": "MethodCall_receiver_argument_order",
        "status": "WireOnly",
        "evidence": "recv and args order survive only in JSON fragment strings",
    },
]


def enum_body(path: Path, enum_name: str) -> str:
    text = path.read_text(encoding="utf-8")
    match = re.search(rf"pub enum {re.escape(enum_name)}\s*\{{", text)
    if not match:
        raise SystemExit(f"missing enum {enum_name}: {path}")
    start = match.end()
    depth = 1
    for index in range(start, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return text[start:index]
    raise SystemExit(f"unterminated enum {enum_name}: {path}")


def top_level_chunks(body: str) -> list[str]:
    chunks: list[str] = []
    start = 0
    brace = paren = bracket = angle = 0
    for index, char in enumerate(body):
        if char == "{": brace += 1
        elif char == "}": brace -= 1
        elif char == "(": paren += 1
        elif char == ")": paren -= 1
        elif char == "[": bracket += 1
        elif char == "]": bracket -= 1
        elif char == "<": angle += 1
        elif char == ">" and angle: angle -= 1
        elif char == "," and brace == paren == bracket == angle == 0:
            chunk = body[start:index].strip()
            if chunk:
                chunks.append(chunk)
            start = index + 1
    tail = body[start:].strip()
    if tail:
        chunks.append(tail)
    return chunks


def strip_comments(text: str) -> str:
    return re.sub(r"(?m)//.*$", "", text).strip()


def variants(path: Path, enum_name: str) -> list[dict[str, object]]:
    rows = []
    for raw in top_level_chunks(strip_comments(enum_body(path, enum_name))):
        chunk = strip_comments(raw)
        name_match = re.match(r"([A-Za-z_][A-Za-z0-9_]*)", chunk)
        if not name_match:
            raise SystemExit(f"cannot parse {enum_name} variant: {raw[:80]!r}")
        name = name_match.group(1)
        rest = chunk[name_match.end():].strip()
        fields: list[str] = []
        if rest.startswith("{"):
            inner = rest[1:rest.rfind("}")]
            for field in top_level_chunks(inner):
                field = strip_comments(field)
                field_match = re.match(r"(?:pub\s+)?([A-Za-z_][A-Za-z0-9_]*)\s*:", field)
                if not field_match:
                    raise SystemExit(f"cannot parse {enum_name}.{name} field: {field!r}")
                fields.append(field_match.group(1))
        elif rest.startswith("("):
            inner = rest[1:rest.rfind(")")]
            fields = [f"${index}" for index, _ in enumerate(top_level_chunks(inner))]
        rows.append({"variant": name, "fields": fields})
    return rows


def build() -> dict[str, object]:
    ast_rows = variants(
        ROOT / "crates/hakorune_frontend_ast/src/ast_node.rs", "ASTNode"
    )
    for row in ast_rows:
        name = str(row["variant"])
        if name in AST_CONTEXT:
            row["classification"] = "ContextRequired"
            row["reason"] = AST_CONTEXT[name]
        elif name in AST_ACCEPTED:
            row["classification"] = "Accepted"
            row["reason"] = "initial_bool_predicate_subset"
        else:
            row["classification"] = "KnownUnsupported"
            row["reason"] = "source.kind_deferred"
        row["child_order"] = CHILD_ORDER.get(name, [])

    unary = variants(
        ROOT / "crates/hakorune_frontend_ast/src/operators.rs", "UnaryOperator"
    )
    for row in unary:
        row.update(classification="Accepted", reason="exact_source_operator")

    binary = variants(
        ROOT / "crates/hakorune_frontend_ast/src/operators.rs", "BinaryOperator"
    )
    for row in binary:
        row.update(classification="Accepted", reason="exact_source_operator")

    literals = variants(
        ROOT / "crates/hakorune_frontend_ast/src/literal.rs", "LiteralValue"
    )
    for row in literals:
        classification, reason = LITERAL_POLICY[str(row["variant"])]
        row.update(classification=classification, reason=reason)

    expected = {"ASTNode": 57, "UnaryOperator": 4, "BinaryOperator": 18, "LiteralValue": 7}
    actual = {
        "ASTNode": len(ast_rows), "UnaryOperator": len(unary),
        "BinaryOperator": len(binary), "LiteralValue": len(literals),
    }
    if actual != expected:
        raise SystemExit(f"inventory drift: expected={expected} actual={actual}")
    if set(LITERAL_POLICY) != {str(row["variant"]) for row in literals}:
        raise SystemExit("LiteralValue policy drift")

    return {
        "schema": "SourceAstVocabularyInventoryV1",
        "authority": {
            "ast": "crates/hakorune_frontend_ast/src/ast_node.rs",
            "operators": "crates/hakorune_frontend_ast/src/operators.rs",
            "literals": "crates/hakorune_frontend_ast/src/literal.rs",
        },
        "enums": {
            "ASTNode": ast_rows,
            "UnaryOperator": unary,
            "BinaryOperator": binary,
            "LiteralValue": literals,
        },
        "hako_source_carrier": HAKO_CARRIER,
        "decision": "HAKO-SOURCE-CARRIER-DESIGN-STOP-001",
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    rendered = json.dumps(build(), ensure_ascii=False, indent=2) + "\n"
    if args.write:
        OUT.parent.mkdir(parents=True, exist_ok=True)
        OUT.write_text(rendered, encoding="utf-8")
        print(f"[source-ast-vocabulary-v1] wrote {OUT.relative_to(ROOT)}")
        return 0
    if not OUT.exists() or OUT.read_text(encoding="utf-8") != rendered:
        print("[source-ast-vocabulary-v1] drift; run with --write")
        return 1
    print("[source-ast-vocabulary-v1] ok: AST=57 unary=4 binary=18 literal=7 carrier=stop")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
