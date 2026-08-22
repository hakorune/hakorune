#!/usr/bin/env python3
"""Derive an external-C ASM comparison from one sealed Hako footprint bundle."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import tempfile
from pathlib import Path
from typing import Any

from inspect_origin_footprint import build_origin_footprint
from inspect_scope_identity import sha256_file, validate_identity_contract
from inspect_selected_dynamic_provenance import (
    SELECTED_DYNAMIC_PAYLOADS,
    validate_product_inventory,
)
from inspect_shape_model import asm_shape


COMPARISON_CONTRACT = "hako-origin-footprint-c-reference-v0"


def _load_object(path: Path) -> dict[str, Any]:
    try:
        value = json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        raise SystemExit(f"C-reference JSON invalid: {path}") from error
    if not isinstance(value, dict):
        raise SystemExit(f"C-reference JSON must be an object: {path}")
    return value


def _validate_hako_bundle(bundle: Path) -> tuple[dict[str, Any], dict[str, Any]]:
    if not bundle.is_dir():
        raise SystemExit("C-reference Hako bundle must be a directory")
    identity = _load_object(bundle / "identity.json")
    validate_identity_contract(bundle, identity)
    if set(identity.get("artifacts", {})) != set(SELECTED_DYNAMIC_PAYLOADS):
        raise SystemExit("C-reference Hako payload inventory mismatch")
    validate_product_inventory(bundle, identity)
    provenance = _load_object(bundle / "lowering.provenance.json")
    footprint = _load_object(bundle / "origin-footprint.json")
    selectors = identity.get("selectors")
    if not isinstance(selectors, dict) or not isinstance(selectors.get("asm_symbol"), str):
        raise SystemExit("C-reference Hako ASM selector missing")
    rebuilt = build_origin_footprint(
        provenance=provenance,
        llvm_text=(bundle / "llvm.lowered-pre-opt.ir").read_text(
            encoding="utf-8", errors="replace"
        ),
        asm_text=(bundle / "asm.s").read_text(encoding="utf-8", errors="replace"),
        asm_symbol=selectors["asm_symbol"],
    )
    if footprint != rebuilt:
        raise SystemExit("C-reference Hako footprint contract mismatch")
    return identity, footprint


def _summary(hako_symbol: str, hako: dict[str, Any], c_symbol: str,
             external_c: dict[str, Any]) -> str:
    rows = [
        "# External C ASM reference", "",
        "- correspondence: unavailable",
        "- external reference only: 1",
        "- observation only: 1",
        "- keeper selection: 0",
        "- measurement authority: 0", "",
        "| column | symbol | instructions | branches | calls | returns |",
        "|---|---|---:|---:|---:|---:|",
    ]
    for column, symbol, shape in (
        ("Hako", hako_symbol, hako), ("external C", c_symbol, external_c)
    ):
        rows.append(
            f"| {column} | `{symbol}` | {shape['instructions']} | "
            f"{shape['branches']} | {shape['calls']} | {shape['returns']} |"
        )
    return "\n".join(rows) + "\n"


def derive_report(*, bundle: Path, c_asm: Path, c_symbol: str, out: Path) -> dict[str, Any]:
    identity, footprint = _validate_hako_bundle(bundle)
    if not c_asm.is_file():
        raise SystemExit("C-reference assembly must be a regular file")
    if not c_symbol:
        raise SystemExit("C-reference symbol must be explicit")
    c_text = c_asm.read_text(encoding="utf-8", errors="replace")
    c_shape = asm_shape(c_text, c_symbol)
    if out.exists():
        raise SystemExit("C-reference output already exists")
    out.parent.mkdir(parents=True, exist_ok=True)
    staging = Path(tempfile.mkdtemp(prefix=f".{out.name}.", dir=out.parent))
    try:
        hako_symbol = footprint["asm"]["symbol"]
        summary = _summary(hako_symbol, footprint["asm"]["shape"], c_symbol, c_shape)
        summary_path = staging / "summary.md"
        summary_path.write_text(summary, encoding="utf-8")
        report = {
            "output_contract": COMPARISON_CONTRACT,
            "hako_candidate_seal": identity["candidate_seal"],
            "hako_origin_footprint_sha256": sha256_file(bundle / "origin-footprint.json"),
            "external_reference": {
                "kind": "external_c_assembly",
                "sha256": sha256_file(c_asm),
                "symbol": c_symbol,
                "authority": "external_reference_only",
            },
            "columns": {
                "hako_asm": {"symbol": hako_symbol, "shape": footprint["asm"]["shape"]},
                "c_asm": {"symbol": c_symbol, "shape": c_shape},
            },
            "correspondence": "unavailable",
            "observation_only": True,
            "keeper_selection": False,
            "measurement_authority": False,
            "summary_file": {"file": "summary.md", "sha256": sha256_file(summary_path)},
            "summary": "ok",
        }
        (staging / "comparison.json").write_text(
            json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
        )
        if {path.name for path in staging.iterdir()} != {"comparison.json", "summary.md"}:
            raise SystemExit("C-reference report inventory mismatch")
        os.replace(staging, out)
        return report
    finally:
        if staging.exists():
            shutil.rmtree(staging)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bundle", type=Path, required=True)
    parser.add_argument("--c-asm", type=Path, required=True)
    parser.add_argument("--c-symbol", required=True)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()
    derive_report(
        bundle=args.bundle.resolve(), c_asm=args.c_asm.resolve(),
        c_symbol=args.c_symbol, out=args.out.resolve(),
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
