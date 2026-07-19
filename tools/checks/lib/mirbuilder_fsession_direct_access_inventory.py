#!/usr/bin/env python3
"""Freeze pre-S0b FunctionOwned direct-access observations.

This checker owns only source occurrence evidence. The selector-to-destination
map remains in the FSESSION Census fixture, so S0b has one state-authority map
and one disposable pre-cutover observation guard.
"""

from __future__ import annotations

import argparse
import json
import re
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
CENSUS = ROOT / "tools/checks/fixtures/mirbuilder_fsession_census_v1.json"
SNAPSHOT = ROOT / "tools/checks/fixtures/mirbuilder_fsession_direct_access_v1.json"
SOURCE_ROOT = ROOT / "src"
EXCLUDED = {"src/mir/builder/function_lowering_state.rs"}

# The receiver spelling is only a candidate. `ReceiverOwnerIndex` proves the
# lexical owner before it contributes an old FunctionOwned access.
RECEIVER = r"(?P<receiver>\bself(?:\s*\.\s*(?:0|builder))?|\bbuilder|\bb|\b[A-Za-z_]\w*\s*\.\s*builder)"


def fail(message: str) -> None:
    raise SystemExit(f"[mirbuilder-fsession-direct-access] {message}")


def skip_rust_literal_or_comment(text: str, index: int) -> int | None:
    initial = text[index]
    if initial == "/" and text.startswith("//", index):
        newline = text.find("\n", index + 2)
        return len(text) if newline < 0 else newline + 1
    if initial == "/" and text.startswith("/*", index):
        depth = 1
        cursor = index + 2
        while cursor < len(text) and depth:
            if text.startswith("/*", cursor):
                depth += 1
                cursor += 2
            elif text.startswith("*/", cursor):
                depth -= 1
                cursor += 2
            else:
                cursor += 1
        return cursor

    if initial in {"r", "b"}:
        raw = re.match(r"(?:br|r)(?P<hashes>#{0,16})\"", text[index:])
        if raw is not None:
            delimiter = '"' + raw.group("hashes")
            body = index + raw.end()
            close = text.find(delimiter, body)
            return len(text) if close < 0 else close + len(delimiter)

    quote_index = index + 1 if initial in {"b", "c"} and text.startswith('"', index + 1) else index
    if initial == '"' or quote_index != index:
        cursor = quote_index + 1
        while cursor < len(text):
            if text[cursor] == "\\":
                cursor += 2
            elif text[cursor] == '"':
                return cursor + 1
            else:
                cursor += 1
        return len(text)

    # Lifetimes are not character literals. Only consume a nearby closing quote.
    if initial == "'":
        cursor = index + 2 if text.startswith("'\\", index) else index + 1
        close = text.find("'", cursor)
        if 0 <= close - index <= 8:
            return close + 1
    return None


def strip_rust_literals_and_comments(text: str) -> str:
    output: list[str] = []
    cursor = 0
    while cursor < len(text):
        skipped = skip_rust_literal_or_comment(text, cursor)
        if skipped is None:
            output.append(text[cursor])
            cursor += 1
            continue
        output.extend("\n" if char == "\n" else " " for char in text[cursor:skipped])
        cursor = skipped
    return "".join(output)


def skip_space_and_comments(text: str, index: int) -> int:
    while index < len(text):
        if text[index].isspace():
            index += 1
            continue
        skipped = skip_rust_literal_or_comment(text, index)
        if skipped is None:
            return index
        index = skipped
    return index


def matching_delimiter(text: str, opening: int, left: str = "{", right: str = "}") -> int:
    depth = 1
    cursor = opening + 1
    while cursor < len(text):
        skipped = skip_rust_literal_or_comment(text, cursor)
        if skipped is not None:
            cursor = skipped
            continue
        if text[cursor] == left:
            depth += 1
        elif text[cursor] == right:
            depth -= 1
            if depth == 0:
                return cursor + 1
        cursor += 1
    fail("unterminated cfg(test) item")


def matching_brace(text: str, opening: int) -> int:
    return matching_delimiter(text, opening)


def cfg_test_item_end(text: str, attribute_start: int) -> int:
    close = text.find("]", attribute_start + 2)
    if close < 0:
        fail("unterminated cfg(test) attribute")
    cursor = skip_space_and_comments(text, close + 1)
    while text.startswith("#[", cursor):
        close = text.find("]", cursor + 2)
        if close < 0:
            fail("unterminated adjacent test attribute")
        cursor = skip_space_and_comments(text, close + 1)

    item_start = cursor
    use_item = re.match(r"(?:pub(?:\([^)]*\))?\s+)?use\b", text[item_start:])
    parens = 0
    brackets = 0
    while cursor < len(text):
        skipped = skip_rust_literal_or_comment(text, cursor)
        if skipped is not None:
            cursor = skipped
            continue
        token = text[cursor]
        if token == "(":
            parens += 1
        elif token == ")":
            parens -= 1
        elif token == "[":
            brackets += 1
        elif token == "]":
            brackets -= 1
        elif token == ";" and parens == 0 and brackets == 0:
            return cursor + 1
        elif token == "{" and parens == 0 and brackets == 0 and not use_item:
            return matching_brace(text, cursor)
        cursor += 1
    fail("unterminated cfg(test) item")


def blank_like(text: str) -> str:
    return "".join("\n" if char == "\n" else " " for char in text)


def cfg_source_domain(attribute: str) -> str:
    compact = re.sub(r"\s+", "", attribute)
    if compact == "test" or compact.startswith("all(test,"):
        return "test"
    if compact == "not(test)" or compact.startswith("all(not(test),"):
        return "production"
    return "shared"


def partition_cfg_items(text: str) -> dict[str, str]:
    """Classify only bounded cfg/test items; all other source stays production."""

    partitions = {domain: [] for domain in ("production", "test", "shared")}
    cursor = 0
    marker = re.compile(r"(?m)^[ \t]*#\[\s*(?P<attribute>cfg\((?P<cfg>[^\]]+)\)|test)\s*\]")
    while True:
        match = marker.search(text, cursor)
        if match is None:
            tail = text[cursor:]
            partitions["production"].append(tail)
            partitions["test"].append(blank_like(tail))
            partitions["shared"].append(blank_like(tail))
            return {domain: "".join(parts) for domain, parts in partitions.items()}
        start = match.start()
        prefix = text[cursor:start]
        partitions["production"].append(prefix)
        partitions["test"].append(blank_like(prefix))
        partitions["shared"].append(blank_like(prefix))
        end = cfg_test_item_end(text, start)
        item = text[start:end]
        domain = cfg_source_domain(match.group("cfg") or match.group("attribute"))
        for candidate, parts in partitions.items():
            parts.append(item if candidate == domain else blank_like(item))
        cursor = end


def is_test_path(path: Path) -> bool:
    return path.name == "tests.rs" or path.name.endswith("_tests.rs") or "tests" in path.parts


@dataclass(frozen=True)
class LexicalSpan:
    start: int
    end: int
    kind: str
    bare_receivers: frozenset[str] = frozenset()


def terminal_type_name(header: str) -> str | None:
    candidate = header.rsplit(" for ", 1)[-1].strip()
    candidate = re.sub(r"\s+where\b.*", "", candidate, flags=re.S)
    match = re.search(r"(?P<name>[A-Za-z_]\w*)(?:\s*<[^<>]*>)?\s*$", candidate)
    return None if match is None else match.group("name")


def struct_wrapper_kinds(text: str) -> dict[str, str]:
    wrappers: dict[str, str] = {}
    for match in re.finditer(r"\bstruct\s+(?P<name>[A-Za-z_]\w*)[^;{(]*", text):
        cursor = skip_space_and_comments(text, match.end())
        if cursor >= len(text):
            continue
        name = match.group("name")
        if text[cursor] == "{":
            body = text[cursor + 1 : matching_brace(text, cursor) - 1]
            if re.search(r"\bbuilder\s*:\s*&?\s*(?:'[A-Za-z_]\w*\s*)?(?:mut\s+)?(?:[A-Za-z_]\w*::)*MirBuilder\b", body):
                wrappers[name] = "builder"
        elif text[cursor] == "(":
            body = text[cursor + 1 : matching_delimiter(text, cursor, "(", ")") - 1]
            if re.match(r"\s*&?\s*(?:'[A-Za-z_]\w*\s*)?(?:mut\s+)?(?:[A-Za-z_]\w*::)*MirBuilder\b", body):
                wrappers[name] = "zero"
    return wrappers


def function_spans(text: str) -> list[LexicalSpan]:
    spans: list[LexicalSpan] = []
    for match in re.finditer(r"\bfn\s+[A-Za-z_]\w*", text):
        opening = text.find("(", match.end())
        if opening < 0:
            continue
        closing = matching_delimiter(text, opening, "(", ")")
        body_open = text.find("{", closing)
        semicolon = text.find(";", closing)
        if body_open < 0 or (0 <= semicolon < body_open):
            continue
        body_end = matching_brace(text, body_open)
        parameters = text[opening + 1 : closing - 1]
        names = frozenset(
            parameter.group("name")
            for parameter in re.finditer(
                r"\b(?P<name>builder|b)\s*:\s*&\s*(?:'[A-Za-z_]\w*\s*)?(?:mut\s+)?(?:[A-Za-z_]\w*::)*MirBuilder\b",
                parameters,
            )
        )
        spans.append(LexicalSpan(body_open, body_end, "function", names))
    return spans


def impl_spans(text: str, wrappers: dict[str, str]) -> list[LexicalSpan]:
    spans: list[LexicalSpan] = []
    for match in re.finditer(r"\bimpl\b(?P<header>[^{};]*)\{", text):
        target = terminal_type_name(match.group("header"))
        kind = "mir_builder" if target == "MirBuilder" else wrappers.get(target or "")
        if kind is not None:
            spans.append(LexicalSpan(match.end() - 1, matching_brace(text, match.end() - 1), kind))
    return spans


class ReceiverOwnerIndex:
    """A bounded lexical proof; it intentionally performs no alias analysis."""

    def __init__(
        self,
        text: str,
        wrappers: dict[str, str] | None = None,
        factories: frozenset[str] = frozenset(),
    ) -> None:
        self.text = text
        self.wrappers = struct_wrapper_kinds(text) if wrappers is None else wrappers
        self.factories = factories
        self.functions = function_spans(text)
        self.impls = impl_spans(text, self.wrappers)

    @staticmethod
    def enclosing(spans: list[LexicalSpan], position: int) -> LexicalSpan | None:
        contained = [span for span in spans if span.start <= position < span.end]
        return min(contained, key=lambda span: span.end - span.start, default=None)

    def exact_builder_binding(self, name: str, position: int, function: LexicalSpan) -> bool:
        if name in function.bare_receivers:
            return True
        prefix = self.text[function.start:position]
        birth = rf"\blet\s+(?:mut\s+)?{re.escape(name)}\s*=\s*(?:[A-Za-z_]\w*::)*MirBuilder\s*::\s*new\s*\("
        if re.search(birth, prefix):
            return True
        factory = re.search(
            rf"\blet\s+(?:\(\s*mut\s+{re.escape(name)}\b[^)]*\)|(?:mut\s+)?{re.escape(name)})\s*=\s*(?P<factory>[A-Za-z_]\w*)\s*\(",
            prefix,
        )
        return factory is not None and factory.group("factory") in self.factories

    def exact_array_iteration_binding(self, name: str, position: int, function: LexicalSpan) -> bool:
        prefix = self.text[function.start:position]
        match = re.search(rf"\bfor\s+{re.escape(name)}\s+in\s*\[(?P<items>[^\]]+)\]\s*\{{", prefix)
        if match is None:
            return False
        roots = re.findall(r"&\s*mut\s+(?P<name>[A-Za-z_]\w*)\b", match.group("items"))
        loop_position = function.start + match.start()
        return bool(roots) and all(self.exact_builder_binding(root, loop_position, function) for root in roots)

    def classify(self, receiver: str, position: int) -> str:
        compact = re.sub(r"\s+", "", receiver)
        owner = self.enclosing(self.impls, position)
        if compact == "self":
            return "accept" if owner is not None and owner.kind == "mir_builder" else "ignore"
        if compact == "self.builder":
            return "accept" if owner is not None and owner.kind == "builder" else "ignore"
        if compact == "self.0":
            return "accept" if owner is not None and owner.kind == "zero" else "ignore"
        if compact in {"builder", "b"}:
            function = self.enclosing(self.functions, position)
            if function is not None:
                if self.exact_builder_binding(compact, position, function):
                    return "accept"
                if self.exact_array_iteration_binding(compact, position, function):
                    return "accept"
                prefix = self.text[function.start:position]
                closure_start = max(prefix.rfind("|builder|"), prefix.rfind("| builder |"))
                if compact == "builder" and closure_start >= 0:
                    closure_context = prefix[max(0, closure_start - 512) : closure_start]
                    if any(
                        owner in closure_context
                        for owner in (
                            "with_function_lowering_session",
                            "with_resolved_function_lowering_session",
                            "with_resolved_function_draft_session",
                        )
                    ):
                        return "accept"
            return "unknown"
        if compact.endswith(".builder"):
            function = self.enclosing(self.functions, position)
            root = compact.removesuffix(".builder")
            if function is not None:
                prefix = self.text[function.start:position]
                wrapper_names = "|".join(re.escape(name) for name, kind in self.wrappers.items() if kind == "builder")
                if wrapper_names:
                    typed = rf"\b{re.escape(root)}\s*:\s*&?\s*(?:'[A-Za-z_]\w*\s*)?(?:mut\s+)?(?:{wrapper_names})\b"
                    born = rf"\blet\s+(?:mut\s+)?{re.escape(root)}\s*=\s*(?:{wrapper_names})\s*::"
                    if re.search(typed, self.text[function.start:function.end]) or re.search(born, prefix):
                        return "accept"
        return "unknown"


def direct(receiver: str) -> str:
    return rf"{RECEIVER}\s*\.\s*{receiver}\b(?!\s*\()"


def nested(context: str, member: str, *, call: bool = False) -> str:
    suffix = r"\s*\(" if call else r"\b(?!\s*\()"
    return rf"{RECEIVER}\s*\.\s*{context}\s*\.\s*{member}{suffix}"


# API families are assigned to the exact state leaf they currently mutate or
# observe. `scope.entry_clear` is deliberately a separate mixed route because
# it also clears debug observation state and therefore needs an S0b split.
ROUTE_PATTERNS = {
    "current_block": (direct("current_block"),),
    "type_ctx": (direct("type_ctx"),),
    "variable_ctx": (direct("variable_ctx"),),
    "binding_ctx": (direct("binding_ctx"),),
    "resolved_binding_state": (direct("resolved_binding_state"),),
    "scope.current_function": (nested("scope_ctx", "current_function"),),
    "scope.lexical_scope_stack": (
        nested("scope_ctx", "lexical_scope_stack"),
        nested("scope_ctx", "push_lexical_scope", call=True),
        nested("scope_ctx", "pop_lexical_scope", call=True),
        nested("scope_ctx", "current_scope_mut", call=True),
    ),
    "scope.loop_header_stack": (nested("scope_ctx", "loop_header_stack"),),
    "scope.loop_exit_stack": (nested("scope_ctx", "loop_exit_stack"),),
    "scope.if_merge_stack": (
        nested("scope_ctx", "if_merge_stack"),
        nested("scope_ctx", "push_if_merge", call=True),
        nested("scope_ctx", "pop_if_merge", call=True),
    ),
    "scope.function_param_names": (nested("scope_ctx", "function_param_names"),),
    "scope.fastmem_region_stack": (
        nested("scope_ctx", "fastmem_region_stack"),
        nested("scope_ctx", "push_fastmem_region", call=True),
        nested("scope_ctx", "pop_fastmem_region", call=True),
        nested("scope_ctx", "current_fastmem_region", call=True),
    ),
    "scope.entry_clear": (nested("scope_ctx", "clear_for_function_entry", call=True),),
    "compilation.reserved_value_ids": (
        nested("comp_ctx", "reserved_value_ids"),
        nested("comp_ctx", "is_reserved_value_id", call=True),
        nested("comp_ctx", "reserve_value_id", call=True),
        nested("comp_ctx", "clear_reserved_value_ids", call=True),
    ),
    "compilation.fn_body_ast": (
        nested("comp_ctx", "fn_body_ast"),
        nested("comp_ctx", "set_fn_body_ast", call=True),
        nested("comp_ctx", "take_fn_body_ast", call=True),
        nested("comp_ctx", "clear_fn_body_ast", call=True),
    ),
    "compilation.record_local_values": (
        nested("comp_ctx", "record_local_values"),
        nested("comp_ctx", "register_record_local_value", call=True),
        nested("comp_ctx", "record_local_value", call=True),
        nested("comp_ctx", "propagate_record_local_value", call=True),
        nested("comp_ctx", "propagate_record_local_value_from_phi", call=True),
        nested("comp_ctx", "clear_record_local_values", call=True),
    ),
    "value_origins.spans": (
        nested("metadata_ctx", "value_origin_spans"),
        nested("metadata_ctx", "record_value_span", call=True),
        nested("metadata_ctx", "value_span", call=True),
    ),
    "value_origins.callers": (
        nested("metadata_ctx", "value_origin_callers"),
        nested("metadata_ctx", "record_value_caller", call=True),
        nested("metadata_ctx", "value_caller", call=True),
        nested("metadata_ctx", "value_origin_callers", call=True),
    ),
    "pending_phis": (direct("pending_phis"),),
    "local_ssa_map": (direct("local_ssa_map"),),
    "schedule_mat_map": (direct("schedule_mat_map"),),
    "pin_slot_names": (direct("pin_slot_names"),),
    "frag_emit_session": (direct("frag_emit_session"),),
    "return_defer_active": (direct("return_defer_active"),),
    "return_defer_slot": (direct("return_defer_slot"),),
    "return_defer_target": (direct("return_defer_target"),),
    "return_deferred_emitted": (direct("return_deferred_emitted"),),
    "in_cleanup_block": (direct("in_cleanup_block"),),
    "cleanup_allow_return": (direct("cleanup_allow_return"),),
    "cleanup_allow_throw": (direct("cleanup_allow_throw"),),
    "suppress_pin_entry_copy_next": (direct("suppress_pin_entry_copy_next"),),
    "in_unified_boxcall_fallback": (direct("in_unified_boxcall_fallback"),),
}

ROUTE_HINTS = {
    "current_block": ("current_block",),
    "type_ctx": ("type_ctx",),
    "variable_ctx": ("variable_ctx",),
    "binding_ctx": ("binding_ctx",),
    "resolved_binding_state": ("resolved_binding_state",),
    "scope.current_function": ("current_function",),
    "scope.lexical_scope_stack": ("lexical_scope_stack", "lexical_scope", "current_scope_mut"),
    "scope.loop_header_stack": ("loop_header_stack",),
    "scope.loop_exit_stack": ("loop_exit_stack",),
    "scope.if_merge_stack": ("if_merge_stack", "push_if_merge", "pop_if_merge"),
    "scope.function_param_names": ("function_param_names",),
    "scope.fastmem_region_stack": ("fastmem_region_stack", "fastmem_region"),
    "scope.entry_clear": ("clear_for_function_entry",),
    "compilation.reserved_value_ids": ("reserved_value_ids", "reserved_value_id"),
    "compilation.fn_body_ast": ("fn_body_ast",),
    "compilation.record_local_values": ("record_local_value",),
    "value_origins.spans": ("value_origin_spans", "value_span"),
    "value_origins.callers": ("value_origin_callers", "value_caller"),
    "pending_phis": ("pending_phis",),
    "local_ssa_map": ("local_ssa_map",),
    "schedule_mat_map": ("schedule_mat_map",),
    "pin_slot_names": ("pin_slot_names",),
    "frag_emit_session": ("frag_emit_session",),
    "return_defer_active": ("return_defer_active",),
    "return_defer_slot": ("return_defer_slot",),
    "return_defer_target": ("return_defer_target",),
    "return_deferred_emitted": ("return_deferred_emitted",),
    "in_cleanup_block": ("in_cleanup_block",),
    "cleanup_allow_return": ("cleanup_allow_return",),
    "cleanup_allow_throw": ("cleanup_allow_throw",),
    "suppress_pin_entry_copy_next": ("suppress_pin_entry_copy_next",),
    "in_unified_boxcall_fallback": ("in_unified_boxcall_fallback",),
}
ALL_HINTS = frozenset(hint for hints in ROUTE_HINTS.values() for hint in hints)


def census_routes() -> dict[str, dict[str, str]]:
    data = json.loads(CENSUS.read_text(encoding="utf-8"))
    rows = data.get("function_state_routes")
    if not isinstance(rows, list):
        fail("Census lacks function_state_routes")
    routes: dict[str, dict[str, str]] = {}
    for row in rows:
        if not isinstance(row, dict):
            fail("Census route is not an object")
        selector = row.get("selector")
        old_storage = row.get("old_storage")
        destination = row.get("destination")
        if not all(isinstance(value, str) and value for value in (selector, old_storage, destination)):
            fail("Census route has invalid strings")
        if selector in routes:
            fail(f"duplicate Census selector: {selector}")
        routes[selector] = {"old_storage": old_storage, "destination": destination}
    expected = set(ROUTE_PATTERNS) | {"scope.entry_clear"}
    if set(routes) != expected:
        fail(f"Census/scanner selector drift missing={sorted(expected - set(routes))} extra={sorted(set(routes) - expected)}")
    return routes


def source_partitions(path: Path, text: str) -> dict[str, str]:
    if is_test_path(path):
        return {"production": blank_like(text), "test": text, "shared": blank_like(text)}
    return partition_cfg_items(text)


def wrapper_contracts() -> dict[str, str]:
    contracts: dict[str, str] = {}
    for path in sorted(SOURCE_ROOT.rglob("*.rs")):
        relative = path.relative_to(ROOT).as_posix()
        if relative in EXCLUDED:
            continue
        for name, kind in struct_wrapper_kinds(strip_rust_literals_and_comments(path.read_text(encoding="utf-8"))).items():
            previous = contracts.setdefault(name, kind)
            if previous != kind:
                fail(f"ambiguous MirBuilder wrapper contract: {name}")
    return contracts


def factory_contracts() -> frozenset[str]:
    factories: set[str] = set()
    pattern = re.compile(
        r"\bfn\s+(?P<name>[A-Za-z_]\w*)\s*\([^)]*\)\s*->\s*(?:\(\s*)?(?:[A-Za-z_]\w*::)*MirBuilder\b"
    )
    for path in SOURCE_ROOT.rglob("*.rs"):
        factories.update(match.group("name") for match in pattern.finditer(strip_rust_literals_and_comments(path.read_text(encoding="utf-8"))))
    return frozenset(factories)


def observe(routes: dict[str, dict[str, str]]) -> list[dict[str, object]]:
    compiled = {
        selector: tuple(re.compile(pattern) for pattern in patterns)
        for selector, patterns in ROUTE_PATTERNS.items()
    }
    evidence: dict[tuple[str, str], dict[str, object]] = {}
    wrappers = wrapper_contracts()
    factories = factory_contracts()
    for selector, route in routes.items():
        for domain in ("production", "test", "shared"):
            evidence[(selector, domain)] = {
                "selector": selector,
                "old_storage": route["old_storage"],
                "destination": route["destination"],
                "domain": domain,
                "files": [],
                "occurrences": 0,
            }

    for path in sorted(SOURCE_ROOT.rglob("*.rs")):
        relative = path.relative_to(ROOT).as_posix()
        if relative in EXCLUDED:
            continue
        raw_text = path.read_text(encoding="utf-8")
        if not any(hint in raw_text for hint in ALL_HINTS):
            continue
        source_text = strip_rust_literals_and_comments(raw_text)
        owners = ReceiverOwnerIndex(source_text, wrappers, factories)
        for domain, text in source_partitions(path, source_text).items():
            for selector, patterns in compiled.items():
                if not any(hint in text for hint in ROUTE_HINTS[selector]):
                    continue
                count = 0
                for pattern in patterns:
                    for match in pattern.finditer(text):
                        disposition = owners.classify(match.group("receiver"), match.start("receiver"))
                        if disposition == "accept":
                            count += 1
                        elif disposition == "unknown":
                            fail(
                                "unverified candidate receiver "
                                f"path={relative} line={text.count(chr(10), 0, match.start()) + 1} "
                                f"receiver={match.group('receiver').strip()} selector={selector}"
                            )
                if count == 0:
                    continue
                row = evidence[(selector, domain)]
                row["files"].append(relative)
                row["occurrences"] += count

    for row in evidence.values():
        row["files"] = sorted(row["files"])
    return [evidence[key] for key in sorted(evidence)]


def validate_snapshot(observed: list[dict[str, object]]) -> None:
    data = json.loads(SNAPSHOT.read_text(encoding="utf-8"))
    if data.get("schema") != "MirBuilderFunctionStateDirectAccessV1":
        fail("unexpected direct-access snapshot schema")
    if data.get("decision") != "pre_s0b_observation_only":
        fail("direct-access snapshot must remain pre-S0b observation only")
    expected = data.get("routes")
    if not isinstance(expected, list):
        fail("snapshot lacks routes")
    expected_rows = len(census_routes()) * 3
    if len(expected) != expected_rows:
        fail(f"snapshot row count must be {expected_rows}")
    if expected != sorted(expected, key=lambda row: (row.get("selector", ""), row.get("domain", ""))):
        fail("snapshot rows must be sorted by selector and domain")
    for row in expected:
        if row.get("domain") not in {"production", "test", "shared"}:
            fail("snapshot has an unknown source domain")
        files = row.get("files")
        if not isinstance(files, list) or files != sorted(set(files)):
            fail("snapshot files must be sorted and unique")
        if not isinstance(row.get("occurrences"), int) or row["occurrences"] < 0:
            fail("snapshot occurrence count must be non-negative")
    totals = data.get("totals")
    if not isinstance(totals, dict):
        fail("snapshot lacks totals")
    actual_totals = {domain: sum(int(row["occurrences"]) for row in expected if row["domain"] == domain) for domain in ("production", "test", "shared")}
    if totals != {"all": sum(actual_totals.values()), **actual_totals}:
        fail("snapshot totals drift from rows")
    if expected != observed:
        fail("direct-access inventory drift; regenerate only through the selected S0a-G0 row")


def reference_document(observed: list[dict[str, object]]) -> str:
    total = sum(int(row["occurrences"]) for row in observed)
    domain_totals = {
        domain: sum(int(row["occurrences"]) for row in observed if row["domain"] == domain)
        for domain in ("production", "test", "shared")
    }
    lines = [
        "{",
        '  "schema": "MirBuilderFunctionStateDirectAccessV1",',
        '  "decision": "pre_s0b_observation_only",',
        f'  "totals": {{"all": {total}, "production": {domain_totals["production"]}, "test": {domain_totals["test"]}, "shared": {domain_totals["shared"]}}},',
        '  "routes": [',
    ]
    for index, row in enumerate(observed):
        suffix = "," if index + 1 < len(observed) else ""
        lines.append("    " + json.dumps(row, separators=(",", ":"), sort_keys=True) + suffix)
    lines.extend(["  ]", "}"])
    return "\n".join(lines) + "\n"


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--print-reference", action="store_true")
    args = parser.parse_args()
    observed = observe(census_routes())
    if args.print_reference:
        print(reference_document(observed), end="")
        return
    validate_snapshot(observed)
    print(
        "[mirbuilder-fsession-direct-access] ok "
        f"routes={len(observed)} occurrences={sum(int(row['occurrences']) for row in observed)}"
    )


if __name__ == "__main__":
    main()
