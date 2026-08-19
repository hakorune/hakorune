"""Effect boundary for the observation-only inspect shape command."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
from pathlib import Path
from typing import Any

from inspect_scope_identity import require_unique_asm_symbol, validate_identity_contract
from inspect_shape_model import COUNT_KEYS, asm_shape, build_shape_report


def _load_object(path: Path) -> dict[str, Any]:
    if not path.is_file():
        raise SystemExit(f"shape JSON artifact missing: {path}")
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise SystemExit(f"shape JSON root must be an object: {path}")
    return value


def _sha256(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _write_atomic(path: Path, text: str) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    tmp = path.with_name(path.name + ".tmp")
    tmp.write_text(text, encoding="utf-8")
    os.replace(tmp, path)


def _display(value: int | None) -> str:
    return "-" if value is None else str(value)


def _report_kv(report: dict[str, Any]) -> str:
    rows = [
        f"output_contract={report['output_contract']}",
        f"candidate_seal={report['candidate_seal']}",
        "observation_only=1",
        "cross_layer_correspondence=unclaimed",
        "keeper_selection=0",
        "measurement_authority=0",
    ]
    for layer, counts in report["layers"].items():
        for key in COUNT_KEYS:
            rows.append(f"{layer}_{key}={_display(counts[key])}")
    rows.append("summary=ok")
    return "\n".join(rows) + "\n"


def _summary(report: dict[str, Any]) -> str:
    header = "| layer | " + " | ".join(COUNT_KEYS) + " |"
    divider = "|---|" + "---:|" * len(COUNT_KEYS)
    rows = ["# Lowering Shape", "", header, divider]
    for layer, counts in report["layers"].items():
        rows.append(
            f"| {layer} | "
            + " | ".join(_display(counts[key]) for key in COUNT_KEYS)
            + " |"
        )
    rows.extend(
        [
            "",
            "- cross-layer correspondence: unclaimed",
            "- keeper selection: 0",
            "- measurement authority: 0",
        ]
    )
    return "\n".join(rows) + "\n"


def run_shape(args: argparse.Namespace) -> int:
    bundle = Path(args.bundle)
    identity = _load_object(bundle / "identity.json")
    validate_identity_contract(bundle, identity)
    external_c = None
    if bool(args.c_asm) != bool(args.c_symbol):
        raise SystemExit("shape external C requires both --c-asm and --c-symbol")
    if args.c_asm:
        c_path = Path(args.c_asm)
        c_text = c_path.read_text(encoding="utf-8", errors="replace")
        require_unique_asm_symbol(c_text, args.c_symbol)
        external_c = {
            "kind": "external_c_assembly",
            "sha256": _sha256(c_path),
            "symbol": args.c_symbol,
            "authority": "external_reference_only",
            "shape": asm_shape(c_text, args.c_symbol),
        }
    report = build_shape_report(
        identity=identity,
        mir=_load_object(bundle / "mir.raw.json"),
        llvm_text=(bundle / "llvm.ir").read_text(encoding="utf-8", errors="replace"),
        asm_text=(bundle / "asm.s").read_text(encoding="utf-8", errors="replace"),
        external_c=external_c,
    )
    out = Path(args.out) if args.out else bundle / "shape"
    _write_atomic(out / "shape.json", json.dumps(report, indent=2, sort_keys=True) + "\n")
    _write_atomic(out / "report.kv", _report_kv(report))
    _write_atomic(out / "summary.md", _summary(report))
    print(_report_kv(report), end="")
    return 0
