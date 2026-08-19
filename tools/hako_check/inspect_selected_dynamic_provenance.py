#!/usr/bin/env python3
"""Real-source selected Dynamic MIR -> lowered-LLVM provenance ingress."""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import shutil
import subprocess
import tempfile
from pathlib import Path
from typing import Any

from inspect_provenance_model import build_provenance
from inspect_scope_identity import (
    build_identity_contract,
    require_unique_mir_function,
    validate_identity_contract,
)


PRODUCER_CONTRACT = "hako-inspect-selected-dynamic-producer-v1"
FUNCTION = "ParserScanLoopBox.skip_while/4"
TEST_NAME = (
    "mir::builder::resolved_lowering::selected_dynamic_physical_emitter::tests::"
    "combined_corridor_emits_typed_prerequisites_and_callouts_in_unpublished_session"
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


def seal_product(
    *, producer: Path, lowered: Path, raw: Path, out: Path,
) -> dict[str, Any]:
    manifest = validate_producer(producer)
    if out.exists():
        raise SystemExit("selected Dynamic provenance output already exists")
    out.parent.mkdir(parents=True, exist_ok=True)
    staging = Path(tempfile.mkdtemp(prefix=f".{out.name}.", dir=out.parent))
    try:
        for source, name in (
            (producer / "source.full.hako", "source.full.hako"),
            (producer / "real.json", "mir.raw.json"),
            (lowered, "llvm.lowered-pre-opt.ir"),
            (raw, "lowering.origins.tsv"),
        ):
            shutil.copyfile(source, staging / name)
        if manifest.get("source_sha256") != _sha256(staging / "source.full.hako"):
            raise SystemExit("selected Dynamic staged source digest mismatch")
        if manifest.get("mir_json_sha256") != _sha256(staging / "mir.raw.json"):
            raise SystemExit("selected Dynamic staged MIR digest mismatch")
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
            artifact_names=[
                "source.full.hako", "mir.raw.json", "llvm.lowered-pre-opt.ir",
                "lowering.origins.tsv", "lowering.provenance.json",
            ],
            mappings={
                "source_to_mir": "exact",
                "mir_to_llvm": "issuer_exact_lowered_pre_opt",
                "lowered_llvm_to_final_llvm": "unavailable",
                "llvm_to_asm": "unavailable",
            },
            mir_function=FUNCTION,
            llvm_function="",
            asm_symbol="",
        )
        identity["producer_contract"] = manifest["output_contract"]
        _write_atomic(
            staging / "identity.json",
            json.dumps(identity, indent=2, sort_keys=True) + "\n",
        )
        validate_identity_contract(staging, identity)
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
        ])
        _write_atomic(staging / "summary.md", summary)
        os.replace(staging, out)
        return identity
    finally:
        if staging.exists():
            shutil.rmtree(staging)


def _run(command: list[str], *, cwd: Path, env: dict[str, str] | None = None) -> None:
    result = subprocess.run(command, cwd=cwd, env=env, text=True, capture_output=True)
    if result.returncode:
        raise SystemExit("selected Dynamic provenance command failed\n" + result.stderr)


def run(args: argparse.Namespace) -> int:
    root = args.repo_root.resolve()
    with tempfile.TemporaryDirectory(prefix="hako_selected_dynamic_provenance.") as raw_tmp:
        temporary = Path(raw_tmp)
        producer = temporary / "producer"
        env = os.environ.copy()
        env["HAKO_INSPECT_SELECTED_DYNAMIC_PRODUCER_DIR"] = str(producer)
        env.setdefault("CARGO_BUILD_JOBS", "4")
        _run([
            "cargo", "test", "--manifest-path", str(root / "Cargo.toml"),
            "--profile", "quick", "--lib", "-q", TEST_NAME, "--", "--exact",
        ], cwd=root, env=env)
        _run([
            str(args.driver.resolve()), str(producer / "real.json"),
            str(temporary / "real.o"), str(temporary / "lowered.ll"),
            str(temporary / "origins.tsv"),
        ], cwd=root)
        identity = seal_product(
            producer=producer, lowered=temporary / "lowered.ll",
            raw=temporary / "origins.tsv", out=args.out.resolve(),
        )
    print(identity["candidate_seal"])
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, required=True)
    parser.add_argument("--driver", type=Path, required=True)
    parser.add_argument("--out", type=Path, required=True)
    return run(parser.parse_args(argv))


if __name__ == "__main__":
    raise SystemExit(main())
