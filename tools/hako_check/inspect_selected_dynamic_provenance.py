#!/usr/bin/env python3
"""Real-source selected Dynamic MIR -> lowered-LLVM provenance ingress."""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import tempfile
from pathlib import Path
from typing import Any

from inspect_provenance_model import build_provenance
from inspect_origin_footprint import (
    build_origin_footprint, render_origin_footprint_markdown,
)
from inspect_scope_identity import (
    build_identity_contract,
    require_unique_mir_function,
    validate_identity_contract,
)


PRODUCER_CONTRACT = "hako-inspect-selected-dynamic-producer-v1"
FUNCTION = "ParserScanLoopBox.skip_while/4"
SELECTED_DYNAMIC_PAYLOADS = (
    "producer.json", "source.full.hako", "mir.raw.json",
    "llvm.lowered-pre-opt.ir", "lowering.origins.tsv",
    "lowering.provenance.json", "object.bin", "asm.s",
    "origin-footprint.json", "summary.md",
)


def _sha256(path: Path) -> str:
    if not path.is_file():
        raise SystemExit(f"selected Dynamic provenance artifact missing: {path}")
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _load(path: Path) -> dict[str, Any]:
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise SystemExit(f"selected Dynamic provenance JSON must be an object: {path}")
    return value


def _write_atomic(path: Path, text: str) -> None:
    temporary = path.with_name(path.name + ".tmp")
    temporary.write_text(text, encoding="utf-8")
    os.replace(temporary, path)


def validate_producer(producer: Path) -> dict[str, Any]:
    manifest = _load(producer / "producer.json")
    expected = {
        "output_contract": PRODUCER_CONTRACT,
        "source_kind": "source_backed_fixture",
        "launch_kind": "route_admission_scaffold_non_authority",
        "source_path": "lang/src/compiler/parser/scan/parser_scan_loop_box.hako",
        "source_file": "source.full.hako",
        "mir_json_file": "real.json",
        "mir_function": FUNCTION,
    }
    for key, value in expected.items():
        if manifest.get(key) != value:
            raise SystemExit(f"selected Dynamic producer {key} mismatch")
    source = producer / "source.full.hako"
    mir_path = producer / "real.json"
    if manifest.get("source_sha256") != _sha256(source):
        raise SystemExit("selected Dynamic producer source digest mismatch")
    if manifest.get("mir_json_sha256") != _sha256(mir_path):
        raise SystemExit("selected Dynamic producer MIR digest mismatch")
    require_unique_mir_function(_load(mir_path), FUNCTION)
    return manifest


def validate_product_inventory(staging: Path, identity: dict[str, Any]) -> None:
    artifacts = identity.get("artifacts")
    if not isinstance(artifacts, dict):
        raise SystemExit("selected Dynamic identity artifact table missing")
    expected = set(artifacts) | {"identity.json"}
    actual = {path.name for path in staging.iterdir() if path.is_file()}
    if actual != expected or any(path.is_dir() for path in staging.iterdir()):
        raise SystemExit("selected Dynamic published inventory mismatch")


def seal_product(
    *, producer: Path, lowered: Path, raw: Path, object_path: Path,
    asm_path: Path, out: Path,
) -> dict[str, Any]:
    manifest = validate_producer(producer)
    if out.exists():
        raise SystemExit("selected Dynamic provenance output already exists")
    out.parent.mkdir(parents=True, exist_ok=True)
    staging = Path(tempfile.mkdtemp(prefix=f".{out.name}.", dir=out.parent))
    try:
        for source, name in (
            (producer / "producer.json", "producer.json"),
            (producer / "source.full.hako", "source.full.hako"),
            (producer / "real.json", "mir.raw.json"),
            (lowered, "llvm.lowered-pre-opt.ir"),
            (raw, "lowering.origins.tsv"),
            (object_path, "object.bin"),
            (asm_path, "asm.s"),
        ):
            shutil.copyfile(source, staging / name)
        if manifest.get("source_sha256") != _sha256(staging / "source.full.hako"):
            raise SystemExit("selected Dynamic staged source digest mismatch")
        if manifest.get("mir_json_sha256") != _sha256(staging / "mir.raw.json"):
            raise SystemExit("selected Dynamic staged MIR digest mismatch")
        if _load(staging / "producer.json") != manifest:
            raise SystemExit("selected Dynamic staged producer manifest mismatch")
        provenance = build_provenance(
            raw_path=staging / "lowering.origins.tsv",
            mir_path=staging / "mir.raw.json",
            llvm_path=staging / "llvm.lowered-pre-opt.ir",
            mir_function=FUNCTION,
            llvm_function=FUNCTION,
            issuer="selected_dynamic_c1_lowerer",
            llvm_boundary="lowered_pre_opt",
        )
        _write_atomic(
            staging / "lowering.provenance.json",
            json.dumps(provenance, indent=2, sort_keys=True) + "\n",
        )
        footprint = build_origin_footprint(
            provenance=provenance,
            llvm_text=(staging / "llvm.lowered-pre-opt.ir").read_text(
                encoding="utf-8", errors="replace"
            ),
            asm_text=(staging / "asm.s").read_text(
                encoding="utf-8", errors="replace"
            ),
            asm_symbol=FUNCTION,
        )
        _write_atomic(
            staging / "origin-footprint.json",
            json.dumps(footprint, indent=2, sort_keys=True) + "\n",
        )
        coverage = provenance["coverage"]
        summary = "\n".join([
            "# Selected Dynamic Lowering Provenance", "",
            "- MIR → lowered LLVM: issuer_exact",
            "- lowered LLVM → final LLVM: unavailable",
            "- LLVM → ASM: unavailable",
            "- observation only: 1", "- keeper selection: 0",
            "- measurement authority: 0", "",
            "| layer | blocks | edges |", "|---|---:|---:|",
            f"| MIR | {coverage['mir_blocks']} | {coverage['mir_edges']} |",
            f"| lowered LLVM | {coverage['llvm_blocks']} | {coverage['llvm_edges']} |",
            "",
        ]) + "\n" + render_origin_footprint_markdown(footprint)
        _write_atomic(staging / "summary.md", summary)
        identity = build_identity_contract(
            out_dir=staging,
            source_file=Path("source.full.hako"),
            selector={
                "kind": "function",
                "region_id": "selected_dynamic_real_candidate",
                "start_line": 1,
                "end_line": len(
                    (staging / "source.full.hako").read_text().splitlines()
                ),
            },
            artifact_names=list(SELECTED_DYNAMIC_PAYLOADS),
            mappings={
                "source_to_mir": "exact",
                "mir_to_llvm": "issuer_exact_lowered_pre_opt",
                "lowered_llvm_to_final_llvm": "unavailable",
                "llvm_to_asm": "unavailable",
            },
            mir_function=FUNCTION,
            llvm_function="",
            asm_symbol=FUNCTION,
        )
        identity["producer_contract"] = manifest["output_contract"]
        _write_atomic(
            staging / "identity.json",
            json.dumps(identity, indent=2, sort_keys=True) + "\n",
        )
        validate_identity_contract(staging, identity)
        validate_product_inventory(staging, identity)
        os.replace(staging, out)
        return identity
    finally:
        if staging.exists():
            shutil.rmtree(staging)
