#!/usr/bin/env python3
"""
RustSubset JSON v0 -> .hako skeleton converter.

Reads a RustSubset JSON v0 document (from file or stdin) and emits
.hako skeleton text. This is the Hakorune-side emitter only;
Rust parsing is delegated to an external adapter (syn / tree-sitter-rust /
rust-analyzer) that produces the RustSubset JSON.

Usage:
    python3 convert.py input.json              # write .hako to stdout
    python3 convert.py input.json -o out.hako  # write to file
    cat input.json | python3 convert.py        # read from stdin

Schema: apps/rust-subset-to-hako/schema/RustSubset-v0.md
"""

import json
import sys
from pathlib import Path


# ── fail-fast helpers ──────────────────────────────────────────────

def fail_fast(msg: str):
    """Honor the schema's fail-fast compatibility rule."""
    print(f"[rust-subset-to-hako] ERROR: {msg}", file=sys.stderr)
    sys.exit(1)


def require_key(obj: dict, key: str, context: str):
    if key not in obj:
        fail_fast(f"missing key '{key}' in {context}: {obj}")
    return obj[key]


def map_type(ty: str) -> str:
    """Map RustSubset type spelling to conservative .hako skeleton spelling."""
    if ty in {"i8", "i16", "i32", "i64", "isize"}:
        return "i64"
    if ty in {"u8", "u16", "u32", "u64"}:
        return "i64"
    if ty == "usize":
        return "usize"
    if ty in {"String", "&str", "str"}:
        return "String"
    if ty == "bool":
        return "bool"
    if ty.startswith("Vec<") and ty.endswith(">"):
        return "Array"
    return ty


def emit_literal_value(value) -> str:
    if isinstance(value, str):
        return json.dumps(value)
    if isinstance(value, bool):
        return "true" if value else "false"
    if value is None:
        return "null"
    return str(value)


# ── expression emitter ─────────────────────────────────────────────

def emit_expr(expr: dict) -> str:
    """Convert a RustSubset expression node to .hako expression text."""
    if not isinstance(expr, dict):
        fail_fast(f"expression is not a dict: {expr}")

    kind = require_key(expr, "kind", "expression")

    if kind == "Literal":
        # { "kind": "Literal", "type": "i64", "value": 1 }
        return emit_literal_value(require_key(expr, "value", "Literal"))

    if kind == "Name":
        # { "kind": "Name", "name": "x" }
        # Rust `self` maps to .hako `me`
        name = require_key(expr, "name", "Name")
        return "me" if name == "self" else name

    if kind == "Field":
        # { "kind": "Field", "base": {...}, "field": "x" }
        base = emit_expr(require_key(expr, "base", "Field"))
        field = require_key(expr, "field", "Field")
        return f"{base}.{field}"

    if kind == "Binary":
        # { "kind": "Binary", "op": "+", "left": {...}, "right": {...} }
        op = require_key(expr, "op", "Binary")
        left = emit_expr(require_key(expr, "left", "Binary"))
        right = emit_expr(require_key(expr, "right", "Binary"))
        return f"{left} {op} {right}"

    if kind == "Call":
        # { "kind": "Call", "callee": "add", "args": [...] }
        callee = require_key(expr, "callee", "Call")
        args = expr.get("args", [])
        args_str = ", ".join(emit_expr(a) for a in args)
        return f"{callee}({args_str})"

    if kind == "MethodCall":
        # { "kind": "MethodCall", "receiver": {...}, "method": "len2", "args": [...] }
        receiver = emit_expr(require_key(expr, "receiver", "MethodCall"))
        method = require_key(expr, "method", "MethodCall")
        args = expr.get("args", [])
        args_str = ", ".join(emit_expr(a) for a in args)
        return f"{receiver}.{method}({args_str})"

    if kind == "Unsupported":
        reason = expr.get("reason", "unsupported")
        return f"/* TODO: {reason} */"

    fail_fast(f"unknown expression kind: {kind}")


# ── statement emitter ──────────────────────────────────────────────

INDENT = "    "


def emit_stmt(stmt: dict) -> str:
    """Convert a RustSubset statement node to one .hako line."""
    if not isinstance(stmt, dict):
        fail_fast(f"statement is not a dict: {stmt}")

    kind = require_key(stmt, "kind", "statement")

    if kind == "Let":
        # { "kind": "Let", "name": "v", "type": "i64", "value": {...} }
        name = require_key(stmt, "name", "Let")
        ty = map_type(require_key(stmt, "type", "Let"))
        value = emit_expr(require_key(stmt, "value", "Let"))
        return f"{INDENT}local {name}: {ty} = {value}"

    if kind == "Return":
        # { "kind": "Return", "value": {...} }
        if "value" in stmt:
            value = emit_expr(stmt["value"])
            return f"{INDENT}return {value}"
        return f"{INDENT}return"

    if kind == "Expr":
        # { "kind": "Expr", "value": {...} }
        value = emit_expr(require_key(stmt, "value", "Expr"))
        return f"{INDENT}{value}"

    if kind == "Unsupported":
        reason = stmt.get("reason", "unsupported")
        return f"{INDENT}/* TODO: {reason} */"

    fail_fast(f"unknown statement kind: {kind}")


# ── function emitter ───────────────────────────────────────────────

def emit_function(func: dict, target_prefix: str = None) -> str:
    """
    Convert a RustSubset Function (or Impl method) to a .hako function.

    If *target_prefix* is given (e.g. "Point"), the function name is prefixed
    ("Point_len2") and a receiver parameter `me: Point` is inserted.
    """
    name = require_key(func, "name", "Function")
    if target_prefix:
        name = f"{target_prefix}_{name}"

    # Build parameter list
    params = []

    # Receiver handling: self_ref / self_mut / self_value -> me: TargetType
    receiver = func.get("receiver", "none")
    if receiver != "none" and target_prefix:
        params.append(f"me: {target_prefix}")

    # Explicit params
    for p in func.get("params", []):
        pname = require_key(p, "name", "Function param")
        ptype = map_type(require_key(p, "type", "Function param"))
        params.append(f"{pname}: {ptype}")

    params_str = ", ".join(params)

    # Return type (default void)
    return_type = map_type(func.get("return_type", "void"))

    # Body statements
    body_stmts = func.get("body", [])
    if body_stmts:
        body_lines = "\n".join(emit_stmt(s) for s in body_stmts)
    else:
        body_lines = f"{INDENT}// empty body"

    return f"function {name}({params_str}): {return_type} {{\n{body_lines}\n}}"


# ── item emitter ───────────────────────────────────────────────────

def emit_item(item: dict) -> str:
    """Convert a top-level RustSubset item to .hako text."""
    if not isinstance(item, dict):
        fail_fast(f"item is not a dict: {item}")

    kind = require_key(item, "kind", "item")

    if kind == "Struct":
        name = require_key(item, "name", "Struct")
        identity = item.get("identity", False)
        fields = item.get("fields", [])

        keyword = "box" if identity else "record"
        field_lines = "\n".join(
            f"{INDENT}{require_key(f, 'name', 'Struct field')}: {map_type(require_key(f, 'type', 'Struct field'))}"
            for f in fields
        )
        return f"{keyword} {name} {{\n{field_lines}\n}}"

    if kind == "Enum":
        # v0: emit as comments (no native .hako enum yet)
        name = require_key(item, "name", "Enum")
        variants = item.get("variants", [])
        lines = [f"// enum {name}"]
        for v in variants:
            vname = require_key(v, "name", "Enum variant")
            vfields = v.get("fields", [])
            if vfields:
                types = ", ".join(f.get("type", "?") for f in vfields)
                lines.append(f"//   {vname}({types})")
            else:
                lines.append(f"//   {vname}")
        return "\n".join(lines)

    if kind == "Impl":
        target = require_key(item, "target", "Impl")
        methods = item.get("methods", [])
        if not methods:
            return f"// impl {target} (no methods)"
        return "\n\n".join(emit_function(m, target_prefix=target) for m in methods)

    if kind == "Function":
        return emit_function(item)

    if kind == "Unsupported":
        reason = item.get("reason", "unsupported")
        return f"// TODO: {reason}"

    fail_fast(f"unknown item kind: {kind}")


# ── top-level converter ────────────────────────────────────────────

def convert(json_str: str) -> str:
    """Convert a RustSubset JSON v0 string to .hako skeleton text."""
    try:
        doc = json.loads(json_str)
    except json.JSONDecodeError as e:
        fail_fast(f"invalid JSON: {e}")

    # Validate document shape
    schema_version = require_key(doc, "schema_version", "document")
    if schema_version != 0:
        fail_fast(f"unsupported schema_version: {schema_version} (expected 0)")

    doc_kind = require_key(doc, "kind", "document")
    if doc_kind != "RustSubsetModule":
        fail_fast(f"unsupported document kind: {doc_kind} (expected RustSubsetModule)")

    # Convert items
    items = doc.get("items", [])
    if not items:
        return "// empty module\n"

    blocks = []
    for item in items:
        blocks.append(emit_item(item))

    return "\n\n".join(blocks) + "\n"


# ── CLI ────────────────────────────────────────────────────────────

def main():
    args = sys.argv[1:]

    # Determine input source
    input_arg = None
    output_path = None
    i = 0
    while i < len(args):
        arg = args[i]
        if arg == "-o":
            if i + 1 >= len(args):
                fail_fast("missing output path after -o")
            output_path = Path(args[i + 1])
            i += 2
            continue
        if arg == "-":
            input_arg = "-"
        elif input_arg is None:
            input_arg = arg
        else:
            fail_fast(f"unexpected argument: {arg}")
        i += 1

    if input_arg and input_arg != "-":
        input_path = Path(input_arg)
        if not input_path.exists():
            fail_fast(f"input file not found: {input_path}")
        json_str = input_path.read_text()
    else:
        json_str = sys.stdin.read()

    # Convert
    hako_text = convert(json_str)

    if output_path:
        output_path.write_text(hako_text)
        print(f"[rust-subset-to-hako] wrote {len(hako_text)} bytes to {output_path}",
              file=sys.stderr)
    else:
        sys.stdout.write(hako_text)


if __name__ == "__main__":
    main()
