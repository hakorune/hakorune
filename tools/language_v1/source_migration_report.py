#!/usr/bin/env python3
"""Generate registry-keyed Canonical source migration evidence."""

from __future__ import annotations

import argparse
from dataclasses import dataclass
import json
import pathlib
import re
from typing import Callable, Iterable

if __package__:
    from .grammar_contract_registry import load_registry_rows
else:
    from grammar_contract_registry import load_registry_rows


ROOT = pathlib.Path(__file__).resolve().parents[2]
REPORT_SCHEMA = "language-v1-canonical-source-migration-v0"


@dataclass(frozen=True)
class EvidenceToken:
    kind: str
    text: str
    line: int
    column: int


def tokenize_evidence(source: str) -> list[EvidenceToken]:
    tokens: list[EvidenceToken] = []
    i = 0
    line = 1
    column = 1
    while i < len(source):
        ch = source[i]
        if ch in " \t\r\n":
            if ch == "\n":
                line += 1
                column = 1
            else:
                column += 1
            i += 1
            continue
        if source.startswith("//", i) or ch == "#":
            end = source.find("\n", i)
            i = len(source) if end < 0 else end
            continue
        if source.startswith("/*", i):
            end = source.find("*/", i + 2)
            segment = source[i:] if end < 0 else source[i : end + 2]
            line_breaks = segment.count("\n")
            if line_breaks:
                line += line_breaks
                column = len(segment.rsplit("\n", 1)[-1]) + 1
            else:
                column += len(segment)
            i += len(segment)
            continue
        start_line, start_column = line, column
        if ch in {'"', "'"}:
            quote = ch
            i += 1
            column += 1
            value = ""
            while i < len(source):
                current = source[i]
                if current == "\\" and i + 1 < len(source):
                    value += source[i : i + 2]
                    i += 2
                    column += 2
                    continue
                if current == quote:
                    i += 1
                    column += 1
                    break
                value += current
                if current == "\n":
                    line += 1
                    column = 1
                else:
                    column += 1
                i += 1
            tokens.append(EvidenceToken("string", value, start_line, start_column))
            continue
        if ch.isalpha() or ch == "_":
            end = i + 1
            while end < len(source) and (source[end].isalnum() or source[end] == "_"):
                end += 1
            text = source[i:end]
            tokens.append(EvidenceToken("identifier", text, start_line, start_column))
            column += end - i
            i = end
            continue
        if ch.isdigit():
            end = i + 1
            while end < len(source) and source[end].isdigit():
                end += 1
            if end + 1 < len(source) and source[end] == "." and source[end + 1].isdigit():
                end += 1
                while end < len(source) and source[end].isdigit():
                    end += 1
            while end < len(source) and (source[end].isalnum() or source[end] == "_"):
                end += 1
            text = source[i:end]
            tokens.append(EvidenceToken("number", text, start_line, start_column))
            column += end - i
            i = end
            continue
        symbol = next(
            (candidate for candidate in ("%{", "=>", "::", "..") if source.startswith(candidate, i)),
            ch,
        )
        tokens.append(EvidenceToken("symbol", symbol, start_line, start_column))
        i += len(symbol)
        column += len(symbol)
    return tokens


Detector = Callable[[list[EvidenceToken], int], bool]


def _word(word: str) -> Detector:
    return lambda tokens, i: tokens[i].kind == "identifier" and tokens[i].text == word


def _sequence(*parts: tuple[str, str]) -> Detector:
    def matches(tokens: list[EvidenceToken], i: int) -> bool:
        return i + len(parts) <= len(tokens) and all(
            tokens[i + offset].kind == kind and tokens[i + offset].text == text
            for offset, (kind, text) in enumerate(parts)
        )

    return matches


def _typed_integer(tokens: list[EvidenceToken], i: int) -> bool:
    return tokens[i].kind == "number" and bool(re.fullmatch(r"\d+(?:u|i)(?:8|16|32|64|size)", tokens[i].text))


def _legacy_map(tokens: list[EvidenceToken], i: int) -> bool:
    return (
        i + 2 < len(tokens)
        and tokens[i].text == "{"
        and tokens[i + 1].kind == "string"
        and tokens[i + 2].text == ":"
    )


def _box_from_inheritance(tokens: list[EvidenceToken], i: int) -> bool:
    return (
        i + 2 < len(tokens)
        and tokens[i].text == "box"
        and tokens[i + 1].kind == "identifier"
        and tokens[i + 2].text == "from"
    )


def _from_super_call(tokens: list[EvidenceToken], i: int) -> bool:
    return (
        tokens[i].kind == "identifier"
        and tokens[i].text == "from"
        and i + 3 < len(tokens)
        and tokens[i + 1].kind == "identifier"
        and tokens[i + 2].text == "."
        and tokens[i + 3].kind == "identifier"
        and not (
            i >= 2
            and tokens[i - 2].text == "box"
            and tokens[i - 1].kind == "identifier"
        )
    )


def _peek_statement(tokens: list[EvidenceToken], i: int) -> bool:
    return (
        tokens[i].kind == "identifier"
        and tokens[i].text == "peek"
        and i + 1 < len(tokens)
        and tokens[i + 1].text != "("
    )


DETECTORS: dict[str, Detector] = {
    "try_statement": _sequence(("identifier", "try"), ("symbol", "{")),
    "peek": _peek_statement,
    "weak_visibility_field": lambda tokens, i: (
        tokens[i].text in {"public", "private"}
        and i + 1 < len(tokens)
        and tokens[i + 1].text == "weak"
    ),
    "weak_legacy_init_field": _sequence(
        ("identifier", "init"), ("symbol", "{"), ("identifier", "weak")
    ),
    "box_from_inheritance": _box_from_inheritance,
    "from_super_call": _from_super_call,
    "while_loop_condition": _word("while"),
    "typed_integer_suffix": _typed_integer,
    "for_loop": _word("for"),
    "do_while_loop": _word("do"),
    "repeat_loop": _word("repeat"),
    "until_loop": _word("until"),
    "weak_paren_expr": _sequence(("identifier", "weak"), ("symbol", "(")),
    "map_literal_legacy_brace_colon": _legacy_map,
}


def _display_path(path: pathlib.Path) -> str:
    try:
        return str(path.relative_to(ROOT))
    except ValueError:
        return str(path)


def scan_paths(paths: Iterable[pathlib.Path]) -> dict[str, list[dict[str, object]]]:
    rejected_rows = [
        row.row_id
        for row in load_registry_rows()
        if row.profile == "Canonical" and row.status in {"rejected", "reserved"}
    ]
    missing = sorted(set(rejected_rows) - set(DETECTORS))
    if missing:
        raise ValueError(f"unclassified Canonical rejected rows: {missing}")
    findings = {row_id: [] for row_id in rejected_rows}
    for path in sorted(paths):
        tokens = tokenize_evidence(path.read_text(encoding="utf-8"))
        for i, token in enumerate(tokens):
            for row_id in rejected_rows:
                if DETECTORS[row_id](tokens, i):
                    findings[row_id].append(
                        {
                            "path": _display_path(path),
                            "line": token.line,
                            "column": token.column,
                        }
                    )
    return findings


def build_report(root: pathlib.Path) -> dict[str, object]:
    paths = list(root.rglob("*.hako"))
    findings = scan_paths(paths)
    percent_brace_count = 0
    percent_brace_files = set()
    for path in sorted(paths):
        count = sum(token.text == "%{" for token in tokenize_evidence(path.read_text(encoding="utf-8")))
        percent_brace_count += count
        if count:
            percent_brace_files.add(_display_path(path))
    rejected_count = sum(len(rows) for rows in findings.values())
    return {
        "schema": REPORT_SCHEMA,
        "status": "ok" if rejected_count == 0 else "migration_required",
        "source_root": _display_path(root),
        "canonical_percent_brace_map_literals": {
            "occurrence_count": percent_brace_count,
            "file_count": len(percent_brace_files),
        },
        "canonical_rejected_occurrence_count": rejected_count,
        "rejected_rows": [
            {"row_id": row_id, "occurrences": occurrences}
            for row_id, occurrences in findings.items()
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=pathlib.Path, default=ROOT / "lang/src")
    parser.add_argument("--output", type=pathlib.Path)
    parser.add_argument("--allow-migration-required", action="store_true")
    args = parser.parse_args()
    report = build_report(args.root.resolve())
    payload = json.dumps(report, sort_keys=True, separators=(",", ":")) + "\n"
    if args.output:
        args.output.write_text(payload, encoding="utf-8")
    else:
        print(payload, end="")
    if report["status"] != "ok" and not args.allow_migration_required:
        return 2
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
