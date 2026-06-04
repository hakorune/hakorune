"""Shared helpers for mimalloc perf attribution reports."""

from __future__ import annotations

import json
import re
from dataclasses import dataclass
from pathlib import Path

from allocator_field_buckets import bucket_for_field, fields_from_hint


PERCENT_RE = r"(?P<pct>[0-9]+(?:\.[0-9]+)?)%"
PERF_REPORT_RE = re.compile(
    rf"^\s*{PERCENT_RE}\s+\S+\s+\S+\s+\[\.\]\s+(?P<symbol>.+?)\s*$"
)
ANNOTATE_RE = re.compile(
    r"^\s*(?P<pct>[0-9]+(?:\.[0-9]+)?)\s*:\s+"
    r"(?P<addr>[0-9a-fA-F]+):\s+"
    r"(?P<asm>.+?)\s*$"
)
OBJDUMP_RE = re.compile(
    r"^\s*(?P<addr>[0-9a-fA-F]+):\s+"
    r"(?P<bytes>(?:[0-9a-fA-F]{2}\s+)+)\s*"
    r"(?P<asm>.+?)\s*$"
)
MEMORY_OPERAND_RE = re.compile(
    r"(?P<offset>-?(?:0x[0-9a-fA-F]+|\d+))?\((?P<body>[^)]*)\)"
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


@dataclass(frozen=True)
class ObjdumpInstruction:
    address: str
    asm: str

    @property
    def mnemonic(self) -> str:
        return self.asm.strip().split(None, 1)[0] if self.asm.strip() else ""


def read_text(path: Path | None) -> str:
    if path is None:
        return ""
    return path.read_text(encoding="utf-8", errors="replace")


def _read(path: Path | None) -> str:
    return read_text(path)


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


def parse_objdump_instructions(text: str) -> list[ObjdumpInstruction]:
    instructions: list[ObjdumpInstruction] = []
    for line in text.splitlines():
        match = OBJDUMP_RE.match(line)
        if not match:
            continue
        instructions.append(
            ObjdumpInstruction(
                address=match.group("addr").lower(),
                asm=match.group("asm").strip(),
            )
        )
    return instructions


def parse_layout_field_offsets(
    text: str,
    box_name: str,
    *,
    base_offset: int,
    field_stride: int,
) -> dict[int, str]:
    if not text or not box_name:
        return {}
    try:
        payload = json.loads(text)
    except json.JSONDecodeError:
        return {}
    decls = payload.get("user_box_decls") if isinstance(payload, dict) else None
    if not isinstance(decls, list):
        return {}
    for decl in decls:
        if not isinstance(decl, dict):
            continue
        if decl.get("name") != box_name and decl.get("box_name") != box_name:
            continue
        fields = decl.get("field_decls")
        if not isinstance(fields, list):
            return {}
        offsets: dict[int, str] = {}
        for field in fields:
            if not isinstance(field, dict):
                continue
            name = field.get("name")
            index = field.get("field_index")
            if not isinstance(name, str) or not isinstance(index, int):
                continue
            offsets[base_offset + index * field_stride] = name
        return offsets
    return {}


def _parse_int_literal(value: str) -> int | None:
    try:
        return int(value, 0)
    except ValueError:
        return None


def _field_hints_for_asm(asm: str, field_offsets: dict[int, str]) -> str:
    if not field_offsets:
        return "none"
    hints: list[str] = []
    seen: set[int] = set()
    for match in MEMORY_OPERAND_RE.finditer(asm):
        # PageModel field hints only apply to simple object-slot operands such
        # as `0xa0(%rax)`. Scaled operands like `0x20(%rdx,%rsi,8)` are
        # DirectArray element accesses and must not be re-labeled as box fields.
        if "," in match.group("body"):
            continue
        raw = match.group("offset") or "0"
        offset = _parse_int_literal(raw)
        if offset is None or offset in seen:
            continue
        seen.add(offset)
        name = field_offsets.get(offset)
        if name is None:
            continue
        hints.append(f"0x{offset:x}:{name}")
    return ",".join(hints) or "none"


def _field_names_for_asm(asm: str, field_offsets: dict[int, str]) -> list[str]:
    return fields_from_hint(_field_hints_for_asm(asm, field_offsets))


def _append_unique(target: list[str], values: list[str]) -> None:
    for value in values:
        if value not in target:
            target.append(value)


def _context_fields_for_address(
    objdump: list[ObjdumpInstruction],
    address: str,
    radius: int,
    field_offsets: dict[int, str],
) -> list[str]:
    if not objdump or not address:
        return []
    index_by_address = {ins.address: idx for idx, ins in enumerate(objdump)}
    idx = index_by_address.get(address.lower())
    if idx is None:
        return []
    start = max(0, idx - radius)
    end = min(len(objdump), idx + radius + 1)
    fields: list[str] = []
    for ins in objdump[start:end]:
        _append_unique(fields, _field_names_for_asm(ins.asm, field_offsets))
    return fields


def _count_bucket(fields: list[str], bucket: str) -> int:
    return sum(1 for field in fields if bucket_for_field(field) == bucket)


def _count_public_or_proof(fields: list[str]) -> int:
    return sum(
        1
        for field in fields
        if "public_semantics" in bucket_for_field(field)
        or "proof_evidence" in bucket_for_field(field)
    )
def _context_window(
    objdump: list[ObjdumpInstruction],
    address: str,
    radius: int,
) -> list[ObjdumpInstruction]:
    if not objdump or not address:
        return []
    index_by_address = {ins.address: idx for idx, ins in enumerate(objdump)}
    idx = index_by_address.get(address.lower())
    if idx is None:
        return []
    start = max(0, idx - radius)
    end = min(len(objdump), idx + radius + 1)
    return objdump[start:end]


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
    if mnemonic.startswith(("cmp", "test")):
        return False
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


def _instruction_category(ins: AnnotatedInstruction) -> str:
    if _is_call(ins):
        return "call"
    if _is_branch(ins):
        return "branch"
    if _is_store_like(ins):
        return "store_like"
    if _is_memory(ins):
        return "memory"
    if _is_arithmetic_or_compare(ins):
        return "arithmetic_compare"
    return "other"


def _instruction_category_from_asm(asm: str) -> str:
    return _instruction_category(AnnotatedInstruction(0.0, "", asm))


def _sanitize_context(value: str) -> str:
    return value.replace("|", "/").replace("=", "~")


def _context_for_address(
    objdump: list[ObjdumpInstruction],
    address: str,
    radius: int,
    field_offsets: dict[int, str],
) -> tuple[str, str, str]:
    if not objdump or not address:
        return "", "0", "none"
    index_by_address = {ins.address: idx for idx, ins in enumerate(objdump)}
    idx = index_by_address.get(address.lower())
    if idx is None:
        return "", "0", "not_found"
    start = max(0, idx - radius)
    end = min(len(objdump), idx + radius + 1)
    window = objdump[start:end]
    encoded = "|".join(
        f"{ins.address}:{_instruction_category_from_asm(ins.asm)}:"
        f"{_field_hints_for_asm(ins.asm, field_offsets)}:"
        f"{_sanitize_context(ins.asm)}"
        for ins in window
    )
    categories = ",".join(sorted({_instruction_category_from_asm(ins.asm) for ins in window}))
    return encoded, str(len(window)), categories or "none"


def _kv_bool(value: bool) -> str:
    return "1" if value else "0"


I64_SIGNED_MAX = (1 << 63) - 1
