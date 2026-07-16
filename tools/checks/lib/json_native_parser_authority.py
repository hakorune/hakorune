#!/usr/bin/env python3
"""Guard the single json_native text-to-tree authority after ITER0-CUT0."""

from __future__ import annotations

import re
import sys
from pathlib import Path


def fail(message: str) -> None:
    print(f"[json-native/parser-authority] FAIL: {message}")
    raise SystemExit(1)


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    parser = root / "apps/lib/json_native/parser/parser.hako"
    engine = root / "apps/lib/json_native/parser/iterative_engine_v1.hako"
    node = root / "apps/lib/json_native/core/node.hako"
    frame = root / "src/backend/mir_interpreter/exec/frame_transaction.rs"

    parser_text = parser.read_text()
    engine_text = engine.read_text()
    node_text = node.read_text()
    frame_text = frame.read_text()

    if parser_text.count("new JsonIterativeParserEngineV1()") != 1:
        fail("parser facade must select exactly one iterative engine")
    if "iterative_engine_v1.hako" not in parser_text:
        fail("parser facade must import the iterative engine")
    if "JsonTokenizer" in parser_text or "token_cursor_v1.hako" in parser_text:
        fail("parser facade must not inspect tokenizer/cursor state")

    retired_methods = (
        "parse_with_current_policy",
        "parse_value",
        "parse_number",
        "parse_string",
        "parse_object",
        "parse_array",
        "current_token",
        "peek_token",
        "match_token",
    )
    for method in retired_methods:
        pattern = rf"(?m)^\s*{re.escape(method)}\s*\("
        if re.search(pattern, parser_text):
            fail(f"retired recursive/token helper remains: {method}")
    if "JsonParserTrace" in parser_text:
        fail("retired JsonParserTrace remains")

    if re.search(r"(?m)^\s*parse\s*\(", node_text):
        fail("JsonNode text parser remains")
    if re.search(r"(?m)^\s*using\s+.*parser", node_text) or "JsonParser" in node_text:
        fail("Core JsonNode must not import the parser")

    hako_files = [path for path in root.rglob("*.hako") if ".git" not in path.parts]
    json_node_parse = []
    for path in hako_files:
        text = path.read_text(errors="replace")
        if re.search(r"JsonNode\s*\.\s*parse\s*\(", text):
            json_node_parse.append(str(path.relative_to(root)))
    if json_node_parse:
        fail("JsonNode.parse callers remain: " + ", ".join(json_node_parse))

    if "class JsonIterativeParserEngineV1" in engine_text:
        fail("unexpected second host-language engine declaration")
    if frame_text.count("const MAX_CALL_DEPTH: usize = 16;") != 1:
        fail("VM MAX_CALL_DEPTH must remain exactly 16")

    oversized = []
    json_root = root / "apps/lib/json_native"
    for path in json_root.rglob("*.hako"):
        line_count = len(path.read_text().splitlines())
        if line_count >= 800:
            oversized.append(f"{path.relative_to(root)}:{line_count}")
    if oversized:
        fail("json_native source at or above 800 lines: " + ", ".join(oversized))

    print(
        "[json-native/parser-authority] ok "
        "engine_selectors=1 recursive_helpers=0 json_node_parse=0 max_depth=16"
    )


if __name__ == "__main__":
    main()
