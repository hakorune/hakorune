#!/usr/bin/env python3
"""Private caller-zero S6C producer -> inspect V1 ingress."""

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

from inspect_scope_identity import (
    build_identity_contract,
    require_unique_asm_symbol,
    require_unique_llvm_function,
    require_unique_mir_function,
    validate_identity_contract,
)
from inspect_provenance_model import build_provenance


PRODUCER_CONTRACT = "hako-inspect-s6c-producer-v1"
TEST_NAME = (
    "mir::builder::resolved_lowering::common_v2_s6c_cursor_cfg_tests::"
    "pinned_text_real_candidate_json_preserves_carrier_lineage"
)


def _sha256(path: Path) -> str:
    if not path.is_file():
        raise SystemExit(f"S6C ingress artifact missing: {path}")
    return hashlib.sha256(path.read_bytes()).hexdigest()


def _load_object(path: Path) -> dict[str, Any]:
    if not path.is_file():
        raise SystemExit(f"S6C ingress JSON missing: {path}")
    value = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(value, dict):
        raise SystemExit(f"S6C ingress JSON root must be an object: {path}")
    return value


def validate_producer_dir(producer: Path) -> tuple[dict[str, Any], dict[str, Any]]:
    manifest = _load_object(producer / "producer.json")
    if manifest.get("output_contract") != PRODUCER_CONTRACT:
        raise SystemExit("S6C producer contract mismatch")
    if manifest.get("source_kind") != "source_backed_fixture":
        raise SystemExit("S6C producer source kind mismatch")
    if manifest.get("source_path") != "apps/tests/scan_with_init_typed_ok_min.hako":
        raise SystemExit("S6C producer source path mismatch")
    if manifest.get("source_file") != "source.full.hako":
        raise SystemExit("S6C producer source filename mismatch")
    if manifest.get("mir_json_file") != "real.json":
        raise SystemExit("S6C producer MIR filename mismatch")
    source = producer / "source.full.hako"
    mir_path = producer / "real.json"
    if manifest.get("source_sha256") != _sha256(source):
        raise SystemExit("S6C producer source digest mismatch")
    if manifest.get("mir_json_sha256") != _sha256(mir_path):
        raise SystemExit("S6C producer MIR digest mismatch")
    mir = _load_object(mir_path)
    mir_function = manifest.get("mir_function")
    if not isinstance(mir_function, str) or not mir_function:
        raise SystemExit("S6C producer MIR selector missing")
    require_unique_mir_function(mir, mir_function)
    return manifest, mir


def _write_json_atomic(path: Path, value: dict[str, Any]) -> None:
    tmp = path.with_name(path.name + ".tmp")
    tmp.write_text(json.dumps(value, indent=2, sort_keys=True) + "\n", encoding="utf-8")
    os.replace(tmp, path)


def seal_ingress(
    *,
    producer: Path,
    final_llvm: Path,
    object_file: Path,
    disassembly: Path,
    provenance_raw: Path,
    llvm_function: str,
    asm_symbol: str,
    out: Path,
) -> dict[str, Any]:
    out.mkdir(parents=True, exist_ok=True)
    identity_path = out / "identity.json"
    identity_path.unlink(missing_ok=True)
    manifest, _mir = validate_producer_dir(producer)
    llvm_text = final_llvm.read_text(encoding="utf-8", errors="replace")
    asm_text = disassembly.read_text(encoding="utf-8", errors="replace")
    require_unique_llvm_function(llvm_text, llvm_function)
    require_unique_asm_symbol(asm_text, asm_symbol)
    if not object_file.is_file() or object_file.stat().st_size == 0:
        raise SystemExit("S6C ingress object is missing or empty")
    for source, name in (
        (producer / "source.full.hako", "source.full.hako"),
        (producer / "real.json", "mir.raw.json"),
        (final_llvm, "llvm.ir"),
        (object_file, "object.bin"),
        (disassembly, "asm.s"),
    ):
        shutil.copyfile(source, out / name)
    provenance = build_provenance(
        raw_path=provenance_raw,
        mir_path=out / "mir.raw.json",
        llvm_path=out / "llvm.ir",
        mir_function=str(manifest["mir_function"]),
        llvm_function=llvm_function,
    )
    _write_json_atomic(out / "lowering.provenance.json", provenance)
    identity = build_identity_contract(
        out_dir=out,
        source_file=out / "source.full.hako",
        selector={
            "kind": "function",
            "region_id": "s6c_real_candidate",
            "start_line": 1,
            "end_line": len((out / "source.full.hako").read_text(encoding="utf-8").splitlines()),
        },
        artifact_names=[
            "source.full.hako",
            "mir.raw.json",
            "llvm.ir",
            "object.bin",
            "asm.s",
            "lowering.provenance.json",
        ],
        mappings={
            "source_to_mir": "exact",
            "mir_to_llvm": "issuer_exact",
            "llvm_to_asm": "symbol",
        },
        mir_function=str(manifest["mir_function"]),
        llvm_function=llvm_function,
        asm_symbol=asm_symbol,
    )
    _write_json_atomic(identity_path, identity)
    validate_identity_contract(out, identity)
    return identity


def _run(command: list[str], *, cwd: Path, env: dict[str, str] | None = None) -> None:
    result = subprocess.run(command, cwd=cwd, env=env, text=True, capture_output=True)
    if result.returncode != 0:
        raise SystemExit(
            f"S6C ingress command failed ({result.returncode}): {' '.join(command)}\n"
            + result.stdout
            + result.stderr
        )


def run_ingress(args: argparse.Namespace) -> int:
    root = Path(args.repo_root).resolve()
    driver = Path(args.structural_driver).resolve()
    if not driver.is_file():
        raise SystemExit(f"S6C structural driver missing: {driver}")
    with tempfile.TemporaryDirectory(prefix="hako_inspect_s6c.") as raw_tmp:
        tmp = Path(raw_tmp)
        producer = tmp / "producer"
        env = os.environ.copy()
        env["HAKO_INSPECT_S6C_PRODUCER_DIR"] = str(producer)
        env.setdefault("CARGO_BUILD_JOBS", "4")
        _run(
            [
                "cargo",
                "test",
                "--manifest-path",
                str(root / "Cargo.toml"),
                "--profile",
                "quick",
                "--lib",
                "-q",
                TEST_NAME,
                "--",
                "--exact",
            ],
            cwd=root,
            env=env,
        )
        final_llvm = tmp / "final.ll"
        object_file = tmp / "final.o"
        disassembly = tmp / "objdump.txt"
        provenance_raw = tmp / "lowering.provenance.tsv"
        _run(
            [
                str(driver), str(producer / "real.json"), str(object_file),
                str(final_llvm), str(provenance_raw),
            ],
            cwd=root,
        )
        objdump = subprocess.run(
            ["objdump", "-d", "--demangle", str(object_file)],
            cwd=root,
            text=True,
            capture_output=True,
        )
        if objdump.returncode != 0:
            raise SystemExit("S6C object disassembly failed\n" + objdump.stderr)
        disassembly.write_text(objdump.stdout, encoding="utf-8")
        identity = seal_ingress(
            producer=producer,
            final_llvm=final_llvm,
            object_file=object_file,
            disassembly=disassembly,
            provenance_raw=provenance_raw,
            llvm_function=args.llvm_function,
            asm_symbol=args.asm_symbol,
            out=Path(args.out),
        )
    print(identity["candidate_seal"])
    return 0


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, required=True)
    parser.add_argument("--structural-driver", type=Path, required=True)
    parser.add_argument("--llvm-function", required=True)
    parser.add_argument("--asm-symbol", required=True)
    parser.add_argument("--out", type=Path, required=True)
    return run_ingress(parser.parse_args(argv))


if __name__ == "__main__":
    raise SystemExit(main())
