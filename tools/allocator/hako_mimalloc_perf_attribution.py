#!/usr/bin/env python3
"""Summarize whether a mimalloc perf artifact can select the next owner.

This tool intentionally separates two attribution levels:

* symbol attribution from `perf report`
* instruction attribution from `perf annotate`

The current AOT direct-exact app often collapses hot samples into `ny_main`.
That is useful evidence, but it is not enough to claim a DirectArray or
PageModel-specific perf delta. The report below makes that boundary explicit.
"""

from __future__ import annotations

import argparse
import re
from dataclasses import dataclass
from pathlib import Path


PERCENT_RE = r"(?P<pct>[0-9]+(?:\.[0-9]+)?)%"
PERF_REPORT_RE = re.compile(
    rf"^\s*{PERCENT_RE}\s+\S+\s+\S+\s+\[\.\]\s+(?P<symbol>.+?)\s*$"
)
ANNOTATE_RE = re.compile(
    r"^\s*(?P<pct>[0-9]+(?:\.[0-9]+)?)\s*:\s+"
    r"(?P<addr>[0-9a-fA-F]+):\s+"
    r"(?P<asm>.+?)\s*$"
)


@dataclass(frozen=True)
class PerfSymbol:
    percent: float
    symbol: str


@dataclass(frozen=True)
class AnnotatedInstruction:
    percent: float
    address: str
    asm: str

    @property
    def mnemonic(self) -> str:
        return self.asm.strip().split(None, 1)[0] if self.asm.strip() else ""


def _read(path: Path | None) -> str:
    if path is None:
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def parse_perf_symbols(text: str) -> list[PerfSymbol]:
    symbols: list[PerfSymbol] = []
    for line in text.splitlines():
        match = PERF_REPORT_RE.match(line)
        if not match:
            continue
        symbols.append(
            PerfSymbol(
                percent=float(match.group("pct")),
                symbol=match.group("symbol").strip(),
            )
        )
    return symbols


def parse_annotated_instructions(text: str) -> list[AnnotatedInstruction]:
    instructions: list[AnnotatedInstruction] = []
    for line in text.splitlines():
        match = ANNOTATE_RE.match(line)
        if not match:
            continue
        instructions.append(
            AnnotatedInstruction(
                percent=float(match.group("pct")),
                address=match.group("addr").lower(),
                asm=match.group("asm").strip(),
            )
        )
    return instructions


def _sum_matching(
    instructions: list[AnnotatedInstruction],
    predicate,
) -> float:
    return sum(ins.percent for ins in instructions if predicate(ins))


def _is_branch(ins: AnnotatedInstruction) -> bool:
    mnemonic = ins.mnemonic
    return mnemonic.startswith("j") or mnemonic in {"loop", "ret"}


def _is_call(ins: AnnotatedInstruction) -> bool:
    return ins.mnemonic.startswith("call")


def _is_memory(ins: AnnotatedInstruction) -> bool:
    return "(" in ins.asm or "[" in ins.asm


def _is_store_like(ins: AnnotatedInstruction) -> bool:
    mnemonic = ins.mnemonic
    asm = ins.asm
    if mnemonic in {"inc", "incq", "incl", "dec", "decq", "decl"}:
        return _is_memory(ins)
    if "," not in asm:
        return False
    dest = asm.rsplit(",", 1)[-1]
    return "(" in dest or "[" in dest


def _is_arithmetic_or_compare(ins: AnnotatedInstruction) -> bool:
    mnemonic = ins.mnemonic
    return mnemonic.startswith(
        (
            "add",
            "sub",
            "inc",
            "dec",
            "cmp",
            "test",
            "and",
            "or",
            "xor",
            "cmov",
        )
    )


def _kv_bool(value: bool) -> str:
    return "1" if value else "0"


def emit_report(args: argparse.Namespace) -> str:
    perf_symbols = parse_perf_symbols(_read(args.perf_report))
    annotated = parse_annotated_instructions(_read(args.perf_annotate))

    top_symbol = perf_symbols[0] if perf_symbols else PerfSymbol(0.0, "")
    top_target_is_symbol = bool(args.symbol and top_symbol.symbol == args.symbol)
    symbol_collapse = top_target_is_symbol and top_symbol.percent >= args.collapse_threshold

    direct_array_symbol_pct = sum(
        symbol.percent
        for symbol in perf_symbols
        if "DirectArray" in symbol.symbol or "direct_array" in symbol.symbol
    )
    page_model_symbol_pct = sum(
        symbol.percent
        for symbol in perf_symbols
        if "HakoAllocPageModel" in symbol.symbol or "PageModel" in symbol.symbol
    )

    nonzero = [ins for ins in annotated if ins.percent > 0.0]
    top_instruction = max(nonzero, key=lambda ins: ins.percent, default=None)
    total_local = sum(ins.percent for ins in nonzero)

    symbol_attribution_available = (
        direct_array_symbol_pct > 0.0 or page_model_symbol_pct > 0.0
    )
    instruction_attribution_available = bool(nonzero)
    perf_delta_ready = bool(symbol_attribution_available)
    blocker = "none"
    next_bridge = "owner_delta_measurement"
    if not perf_delta_ready:
        if symbol_collapse and instruction_attribution_available:
            blocker = "ny_main_symbol_collapse"
            next_bridge = "asm_instruction_classifier_or_in_process_perf_mode"
        elif not instruction_attribution_available:
            blocker = "missing_perf_annotate_samples"
            next_bridge = "rerun_perf_with_higher_repeat_or_symbol"
        else:
            blocker = "missing_directarray_or_pagemodel_symbol_attribution"
            next_bridge = "asm_instruction_classifier_or_symbol_split"

    lines = [
        "output_contract=hako-mimalloc-perf-attribution-v0",
        f"perf_report={args.perf_report or ''}",
        f"perf_annotate={args.perf_annotate or ''}",
        f"target_symbol={args.symbol}",
        f"top_symbol={top_symbol.symbol}",
        f"top_symbol_percent={top_symbol.percent:.2f}",
        f"top_symbol_is_target={_kv_bool(top_target_is_symbol)}",
        f"symbol_collapse_detected={_kv_bool(symbol_collapse)}",
        f"symbol_attribution_available={_kv_bool(symbol_attribution_available)}",
        f"direct_array_symbol_percent={direct_array_symbol_pct:.2f}",
        f"page_model_symbol_percent={page_model_symbol_pct:.2f}",
        f"instruction_attribution_available={_kv_bool(instruction_attribution_available)}",
        f"annotate_nonzero_instruction_count={len(nonzero)}",
        f"annotate_total_local_percent={total_local:.2f}",
        f"annotate_branch_percent={_sum_matching(nonzero, _is_branch):.2f}",
        f"annotate_call_percent={_sum_matching(nonzero, _is_call):.2f}",
        f"annotate_memory_percent={_sum_matching(nonzero, _is_memory):.2f}",
        f"annotate_store_like_percent={_sum_matching(nonzero, _is_store_like):.2f}",
        f"annotate_arithmetic_compare_percent={_sum_matching(nonzero, _is_arithmetic_or_compare):.2f}",
        f"page_model_hot_array_perf_delta_measurement_plan_v0=1",
        f"page_model_hot_array_perf_delta_ready={_kv_bool(perf_delta_ready)}",
        f"page_model_hot_array_perf_delta_blocker={blocker}",
        f"page_model_hot_array_perf_delta_next_bridge={next_bridge}",
    ]
    if top_instruction is not None:
        lines.extend(
            [
                f"top_instruction_percent={top_instruction.percent:.2f}",
                f"top_instruction_address={top_instruction.address}",
                f"top_instruction_mnemonic={top_instruction.mnemonic}",
                f"top_instruction_asm={top_instruction.asm}",
            ]
        )
    else:
        lines.extend(
            [
                "top_instruction_percent=0.00",
                "top_instruction_address=",
                "top_instruction_mnemonic=",
                "top_instruction_asm=",
            ]
        )
    lines.append("summary=ok")
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--perf-report", type=Path)
    parser.add_argument("--perf-annotate", type=Path)
    parser.add_argument("--symbol", default="ny_main")
    parser.add_argument("--collapse-threshold", type=float, default=90.0)
    args = parser.parse_args()
    print(emit_report(args), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
