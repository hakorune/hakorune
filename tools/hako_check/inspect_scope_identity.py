"""Pure V1 identity seal model for hako_check inspect bundles."""

from __future__ import annotations

import hashlib
import json
import re
from pathlib import Path
from typing import Any


IDENTITY_CONTRACT = "hako-inspect-bundle-identity-v1"


def sha256_file(path: Path) -> str:
    if not path.is_file():
        raise SystemExit(f"identity artifact missing: {path}")
    return hashlib.sha256(path.read_bytes()).hexdigest()


def require_unique_mir_function(mir: dict[str, Any], name: str) -> None:
    functions = mir.get("functions")
    if not isinstance(functions, list):
        raise SystemExit("identity MIR functions must be an array")
    matches = [row for row in functions if isinstance(row, dict) and row.get("name") == name]
    if len(matches) != 1:
        raise SystemExit(
            f"identity MIR function must be unique: name={name!r} matches={len(matches)}"
        )


def llvm_defined_functions(text: str) -> list[str]:
    pattern = re.compile(
        r'^\s*define\b[^@]*@(?:"((?:[^"\\]|\\.)+)"|([-A-Za-z$._0-9]+))\s*\(',
        re.MULTILINE,
    )
    return [quoted or plain for quoted, plain in pattern.findall(text)]


def require_unique_llvm_function(text: str, name: str) -> None:
    matches = [item for item in llvm_defined_functions(text) if item == name]
    if len(matches) != 1:
        raise SystemExit(
            f"identity LLVM function must be unique: name={name!r} matches={len(matches)}"
        )


def objdump_symbols(text: str) -> list[tuple[str, int]]:
    pattern = re.compile(r"^\s*[0-9a-fA-F]+\s+<([^>]+)>:\s*$")
    rows: list[tuple[str, int]] = []
    for line_no, line in enumerate(text.splitlines(), start=1):
        match = pattern.match(line)
        if match:
            rows.append((match.group(1), line_no))
    return rows


def require_unique_asm_symbol(text: str, name: str) -> int:
    matches = [line_no for symbol, line_no in objdump_symbols(text) if symbol == name]
    if len(matches) != 1:
        raise SystemExit(
            f"identity assembly symbol must be unique: name={name!r} matches={len(matches)}"
        )
    return matches[0]


def build_identity_contract(
    *,
    out_dir: Path,
    source_file: Path,
    selector: dict[str, Any],
    artifact_names: list[str],
    mappings: dict[str, str],
    mir_function: str,
    llvm_function: str,
    asm_symbol: str,
) -> dict[str, Any]:
    artifacts = {
        name: {
            "file": name,
            "sha256": sha256_file(out_dir / name),
        }
        for name in sorted(artifact_names)
    }
    shape_ready = (
        bool(mir_function and llvm_function and asm_symbol)
        and mappings == {
            "source_to_mir": "exact",
            "mir_to_llvm": "block",
            "llvm_to_asm": "symbol",
        }
        and all(name in artifacts for name in ("llvm.ir", "executable.bin", "asm.s"))
    )
    payload: dict[str, Any] = {
        "output_contract": IDENTITY_CONTRACT,
        "tool_surface": "hako_check_inspect_scope",
        "observation_only": True,
        "artifact_lineage_only": True,
        "cross_layer_correspondence": "unclaimed",
        "keeper_selection": False,
        "measurement_authority": False,
        "source_file": str(source_file),
        "selector": selector,
        "artifacts": artifacts,
        "selectors": {
            "mir_function": mir_function,
            "llvm_function": llvm_function,
            "asm_symbol": asm_symbol,
        },
        "mappings": mappings,
        "shape_ready": shape_ready,
        "summary": "ok",
    }
    seal_payload = {
        "output_contract": IDENTITY_CONTRACT,
        "artifacts": artifacts,
        "selectors": payload["selectors"],
        "mappings": mappings,
    }
    canonical = json.dumps(seal_payload, sort_keys=True, separators=(",", ":")).encode("utf-8")
    payload["candidate_seal"] = "sha256:" + hashlib.sha256(canonical).hexdigest()
    return payload


def validate_identity_contract(out_dir: Path, identity: dict[str, Any]) -> None:
    if identity.get("output_contract") != IDENTITY_CONTRACT:
        raise SystemExit("identity contract version mismatch")
    candidate_seal = identity.get("candidate_seal")
    canonical = json.dumps(
        {
            "output_contract": IDENTITY_CONTRACT,
            "artifacts": identity.get("artifacts"),
            "selectors": identity.get("selectors"),
            "mappings": identity.get("mappings"),
        },
        sort_keys=True,
        separators=(",", ":"),
    ).encode("utf-8")
    expected_seal = "sha256:" + hashlib.sha256(canonical).hexdigest()
    if candidate_seal != expected_seal:
        raise SystemExit("identity candidate seal mismatch")
    artifacts = identity.get("artifacts")
    if not isinstance(artifacts, dict) or not artifacts:
        raise SystemExit("identity artifact table missing")
    for name, row in artifacts.items():
        if not isinstance(name, str) or not isinstance(row, dict):
            raise SystemExit("identity artifact row malformed")
        if row.get("file") != name:
            raise SystemExit(f"identity artifact filename drift: {name}")
        if row.get("sha256") != sha256_file(out_dir / name):
            raise SystemExit(f"identity artifact digest mismatch: {name}")
    selectors = identity.get("selectors")
    if not isinstance(selectors, dict):
        raise SystemExit("identity selector table missing")
    mir_function = selectors.get("mir_function")
    llvm_function = selectors.get("llvm_function")
    asm_symbol = selectors.get("asm_symbol")
    if mir_function:
        mir = json.loads((out_dir / "mir.raw.json").read_text(encoding="utf-8"))
        if not isinstance(mir, dict):
            raise SystemExit("identity MIR root must be an object")
        require_unique_mir_function(mir, str(mir_function))
    if llvm_function:
        require_unique_llvm_function(
            (out_dir / "llvm.ir").read_text(encoding="utf-8", errors="replace"),
            str(llvm_function),
        )
    if asm_symbol:
        require_unique_asm_symbol(
            (out_dir / "asm.s").read_text(encoding="utf-8", errors="replace"),
            str(asm_symbol),
        )
