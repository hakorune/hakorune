"""Small helpers for focused lifecycle adapter source extraction."""

from __future__ import annotations

import json
import re
from pathlib import Path
from typing import Any


class ExtractionError(AssertionError):
    pass


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ExtractionError(message)


def normalized_rust_type(text: str) -> str:
    compact = re.sub(r"\s+", " ", text.strip())
    compact = compact.replace(" <", "<").replace("< ", "<")
    compact = compact.replace(" >", ">").replace("> ", ">")
    compact = compact.replace(" ,", ",")
    compact = re.sub(r",\s*", ", ", compact)
    return compact


def extract_btree_map_type(source: str, field_name: str, value_type: str) -> str:
    pattern = (
        rf"{re.escape(field_name)}\s*:\s*"
        rf"(?P<ty>BTreeMap\s*<\s*String\s*,\s*{re.escape(value_type)}\s*>)"
    )
    match = re.search(pattern, source)
    require(match is not None, f"missing BTreeMap field: {field_name}")
    return normalized_rust_type(match.group("ty"))


def extract_method_signatures(source: str) -> dict[str, dict[str, Any]]:
    pattern = re.compile(
        r"pub\s+fn\s+(?P<name>\w+)\s*"
        r"\((?P<params>[^)]*)\)\s*"
        r"(?:->\s*(?P<ret>[^{]+))?\{",
        re.MULTILINE,
    )
    methods: dict[str, dict[str, Any]] = {}
    for match in pattern.finditer(source):
        methods[match.group("name")] = {
            "params": normalized_rust_type(match.group("params")),
            "ret": normalized_rust_type(match.group("ret") or ""),
        }
    return methods


def extract_method_body(source: str, name: str) -> str:
    marker = f"pub fn {name}"
    start = source.find(marker)
    require(start >= 0, f"missing method body: {name}")
    brace = source.find("{", start)
    require(brace >= 0, f"missing method body brace: {name}")
    depth = 0
    for index in range(brace, len(source)):
        char = source[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return normalized_rust_type(source[brace + 1 : index])
    raise ExtractionError(f"unterminated method body: {name}")


def receiver_fact(params: str) -> dict[str, Any]:
    if params.startswith("&mut self"):
        return {
            "borrow_kind": "UniqueWrite",
            "borrow_escape": "CallOnly",
            "mutation": True,
        }
    if params.startswith("&self"):
        return {
            "borrow_kind": "SharedRead",
            "borrow_escape": "CallOnly",
            "mutation": False,
        }
    raise ExtractionError(f"unsupported receiver params: {params}")


def immediate_return() -> dict[str, str]:
    return {"copy_class": "ImmediateValue", "drop_class": "TrivialMemory"}


def assert_no_hako_policy_spelling(facts: dict[str, Any]) -> None:
    text = json.dumps(facts, sort_keys=True)
    for forbidden in [
        "OrderedMapBox",
        "BorrowView",
        "TransferOwned",
        "LocalBox",
        "HakoLifecyclePlan",
        ".hako source",
        "backend lowering",
    ]:
        require(forbidden not in text, f"Hako policy spelling leaked: {forbidden}")


def report_or_emit(
    *,
    facts: dict[str, Any],
    reference: Path,
    check_reference: bool,
    emit_json: bool,
    report: list[tuple[str, str]],
) -> int:
    assert_no_hako_policy_spelling(facts)
    if check_reference:
        expected = json.loads(reference.read_text())
        require(facts == expected, "extracted facts differ from reference fixture")

    if emit_json:
        print(json.dumps(facts, indent=2))
    else:
        for key, value in report:
            print(f"{key}={value}")
        print("summary=ok")
    return 0
