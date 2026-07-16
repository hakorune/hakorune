#!/usr/bin/env python3
"""Observe the generic MapFieldOwner proof without selecting a compiler fix."""

from __future__ import annotations

import json
import os
from pathlib import Path
import subprocess
import sys
from typing import Any


PROOF_ID = "map-field-owner-boxshape-proof"
APP = Path("apps/map-field-owner-boxshape-proof/main.hako")
ARTIFACT_DIR = Path("target/checks/map-field-owner-boxshape-proof")
CASE_NAMES = (
    "local_map",
    "field_literal",
    "field_formal_concat",
    "field_formal_key",
    "same_method_direct",
    "same_method_self",
    "control_merge_one",
    "control_merge_two",
    "receiver_alias",
    "instance_isolation",
)
FUNCTION_CASES = {
    "local_map": (
        "Main.map_field_probe_v1_case_local_map/0",
    ),
    "field_literal": (
        "MapFieldOwnerProbeV1.map_field_probe_v1_put_literal/1",
        "MapFieldOwnerProbeV1.map_field_probe_v1_has_literal/0",
        "MapFieldOwnerProbeV1.map_field_probe_v1_load_literal_present/0",
    ),
    "field_formal_concat": (
        "MapFieldOwnerProbeV1.map_field_probe_v1_put_id/2",
        "MapFieldOwnerProbeV1.map_field_probe_v1_has_id/1",
        "MapFieldOwnerProbeV1.map_field_probe_v1_load_id_present/1",
    ),
    "field_formal_key": (
        "MapFieldOwnerProbeV1.map_field_probe_v1_put_key/2",
        "MapFieldOwnerProbeV1.map_field_probe_v1_has_key/1",
        "MapFieldOwnerProbeV1.map_field_probe_v1_load_key_present/1",
    ),
    "same_method_direct": (
        "MapFieldOwnerProbeV1.map_field_probe_v1_put_then_has_direct/2",
    ),
    "same_method_self": (
        "MapFieldOwnerProbeV1.map_field_probe_v1_put_then_has_self/2",
        "MapFieldOwnerProbeV1.map_field_probe_v1_contains_id_internal/2",
    ),
    "control_merge_one": (
        "MapFieldOwnerProbeV1.map_field_probe_v1_put_after_fallthrough_one/4",
    ),
    "control_merge_two": (
        "MapFieldOwnerProbeV1.map_field_probe_v1_put_after_fallthrough_two/5",
    ),
    "receiver_alias": (
        "MapFieldOwnerProbeV1.map_field_probe_v1_alias_put/2",
    ),
    "instance_isolation": (
        "Main.map_field_probe_v1_case_instance_isolation/0",
    ),
}


class ProofFailure(RuntimeError):
    pass


def run(
    argv: list[str],
    *,
    root: Path,
    env: dict[str, str] | None = None,
) -> subprocess.CompletedProcess[str]:
    merged_env = os.environ.copy()
    if env:
        merged_env.update(env)
    completed = subprocess.run(
        argv,
        cwd=root,
        env=merged_env,
        text=True,
        capture_output=True,
        check=False,
    )
    if completed.returncode != 0:
        rendered = " ".join(argv)
        raise ProofFailure(
            f"command failed rc={completed.returncode}: {rendered}\n"
            f"stdout:\n{completed.stdout}\nstderr:\n{completed.stderr}"
        )
    return completed


def build_bins(root: Path) -> dict[str, Path]:
    run(
        ["cargo", "build", "-q", "--features", "vm-reference", "--bin", "hakorune"],
        root=root,
    )
    run(
        [
            "cargo",
            "build",
            "-q",
            "--release",
            "--features",
            "vm-reference",
            "--bin",
            "hakorune",
        ],
        root=root,
    )
    return {
        "debug": root / "target/debug/hakorune",
        "release": root / "target/release/hakorune",
    }


def parse_runtime_output(text: str) -> dict[str, int]:
    lines = [line.strip() for line in text.splitlines() if line.strip()]
    if PROOF_ID not in lines:
        raise ProofFailure("runtime output is missing proof id")
    cases: dict[str, int] = {}
    for name in CASE_NAMES:
        prefix = f"case.{name}="
        matches = [line for line in lines if line.startswith(prefix)]
        if len(matches) != 1:
            raise ProofFailure(f"runtime output must contain one {prefix} row")
        value_text = matches[0][len(prefix) :]
        if value_text not in ("0", "1"):
            raise ProofFailure(f"{prefix} must be 0 or 1, got {value_text!r}")
        cases[name] = int(value_text)
    if "selection=UNCLASSIFIED-S0" not in lines:
        raise ProofFailure("S0 runtime output changed selection before V0")
    if "summary=observed" not in lines:
        raise ProofFailure("runtime output is missing summary=observed")
    return cases


def run_mode(root: Path, mode: str, binary: Path) -> tuple[dict[str, int], Path]:
    env = {
        "NYASH_FEATURES": "rune",
        "NYASH_DISABLE_PLUGINS": "1",
    }
    runtime = run(
        [str(binary), "--backend", "vm", str(APP)],
        root=root,
        env=env,
    )
    runtime_path = root / ARTIFACT_DIR / f"{mode}.runtime.txt"
    runtime_path.write_text(runtime.stdout, encoding="utf-8")
    cases = parse_runtime_output(runtime.stdout)

    mir_path = root / ARTIFACT_DIR / f"{mode}.mir.json"
    emit = run(
        [str(binary), "--emit-mir-json", str(mir_path), str(APP)],
        root=root,
        env=env,
    )
    (root / ARTIFACT_DIR / f"{mode}.emit.txt").write_text(
        emit.stdout + emit.stderr,
        encoding="utf-8",
    )
    return cases, mir_path


def iter_instructions(function: dict[str, Any]):
    for block in function.get("blocks", []):
        block_id = block.get("id")
        for index, instruction in enumerate(block.get("instructions", [])):
            yield block_id, index, instruction


def type_label(value_type: Any) -> str:
    if value_type is None:
        return "Unknown"
    if isinstance(value_type, str):
        return value_type
    if isinstance(value_type, dict):
        kind = value_type.get("kind")
        box_type = value_type.get("box_type")
        if kind == "string" or box_type == "StringBox":
            return "String"
        if box_type:
            return f"handle:{box_type}"
        if kind:
            return str(kind)
    return f"other:{value_type!r}"


def function_evidence(function: dict[str, Any]) -> dict[str, Any]:
    value_types = function.get("metadata", {}).get("value_types", {})
    definitions: dict[int, tuple[str, Any]] = {}
    calls: list[dict[str, Any]] = []
    counts = {
        "field_get": 0,
        "field_set": 0,
        "phi": 0,
        "copy_owned": 0,
        "destroy_owned": 0,
        "release_strong": 0,
    }

    for block_id, index, instruction in iter_instructions(function):
        op = instruction.get("op")
        if op in counts:
            counts[op] += 1
        dst = instruction.get("dst")
        if isinstance(dst, int):
            definitions[dst] = (op, instruction)
        if op != "mir_call":
            continue
        payload = instruction.get("mir_call", {})
        callee = payload.get("callee", {})
        name = callee.get("name")
        if name not in ("set", "has", "get"):
            continue
        args = payload.get("args", [])
        if name == "set" and len(args) >= 2:
            key_value = args[-2]
        elif args:
            key_value = args[-1]
        else:
            key_value = None
        calls.append(
            {
                "site": f"{block_id}:{index}",
                "method": name,
                "box_name": callee.get("box_name"),
                "certainty": callee.get("certainty"),
                "receiver_value": callee.get("receiver"),
                "key_value": key_value,
                "key_type": type_label(value_types.get(str(key_value))),
            }
        )

    def root_of(value_id: Any, seen: frozenset[int] = frozenset()) -> str:
        if not isinstance(value_id, int):
            return "missing"
        if value_id in seen:
            return "cycle"
        definition = definitions.get(value_id)
        if definition is None:
            return f"param:{value_id}"
        op, instruction = definition
        next_seen = seen | {value_id}
        if op == "copy":
            return root_of(instruction.get("src"), next_seen)
        if op == "phi":
            roots = sorted(
                {
                    root_of(incoming[0], next_seen)
                    for incoming in instruction.get("incoming", [])
                }
            )
            if len(roots) == 1:
                return roots[0]
            return "phi(" + ",".join(roots) + ")"
        if op == "field_get":
            field = instruction.get("field")
            return f"field:{field}<{root_of(instruction.get('box'), next_seen)}>"
        if op == "newbox":
            return f"newbox:{instruction.get('type')}"
        if op == "binop":
            operation = instruction.get("operation")
            lhs = root_of(instruction.get("lhs"), next_seen)
            rhs = root_of(instruction.get("rhs"), next_seen)
            return f"binop:{operation}<{lhs},{rhs}>"
        if op == "const":
            return f"const:{type_label(value_types.get(str(value_id)))}"
        return str(op)

    for call in calls:
        call["receiver_root"] = root_of(call.pop("receiver_value"))
        call["key_root"] = root_of(call.pop("key_value"))

    receiver_phis: list[dict[str, Any]] = []
    for value_id, (op, instruction) in sorted(definitions.items()):
        if op != "phi":
            continue
        value_type = type_label(value_types.get(str(value_id)))
        if value_type != "handle:MapFieldOwnerProbeV1":
            continue
        receiver_phis.append(
            {
                "root": root_of(value_id),
                "input_roots": sorted(
                    root_of(item[0]) for item in instruction.get("incoming", [])
                ),
                "input_count": len(instruction.get("incoming", [])),
            }
        )

    return {
        "function": function.get("name"),
        "counts": counts,
        "map_calls": calls,
        "receiver_phis": receiver_phis,
    }


def normalize_mir(path: Path) -> dict[str, Any]:
    document = json.loads(path.read_text(encoding="utf-8"))
    functions = {
        function.get("name"): function for function in document.get("functions", [])
    }
    required = {"MapFieldOwnerProbeV1.birth/0", *sum(FUNCTION_CASES.values(), ())}
    missing = sorted(required - functions.keys())
    if missing:
        raise ProofFailure(f"MIR is missing required functions: {missing}")

    birth = function_evidence(functions["MapFieldOwnerProbeV1.birth/0"])
    birth_instructions = [
        instruction
        for _, _, instruction in iter_instructions(
            functions["MapFieldOwnerProbeV1.birth/0"]
        )
    ]
    map_newboxes = [
        instruction
        for instruction in birth_instructions
        if instruction.get("op") == "newbox" and instruction.get("type") == "MapBox"
    ]
    storage_sets = [
        instruction
        for instruction in birth_instructions
        if instruction.get("op") == "field_set"
        and instruction.get("field") == "storage"
    ]
    if len(map_newboxes) != 1 or len(storage_sets) != 1:
        raise ProofFailure("birth must publish exactly one MapBox into storage")

    cases: dict[str, list[dict[str, Any]]] = {}
    totals = {
        "copy_owned": 0,
        "destroy_owned": 0,
        "release_strong": 0,
        "mapbox_known_calls": 0,
        "runtime_union_calls": 0,
    }
    for case_name, function_names in FUNCTION_CASES.items():
        rows = [function_evidence(functions[name]) for name in function_names]
        cases[case_name] = rows
        for row in rows:
            counts = row["counts"]
            totals["copy_owned"] += counts["copy_owned"]
            totals["destroy_owned"] += counts["destroy_owned"]
            totals["release_strong"] += counts["release_strong"]
            for call in row["map_calls"]:
                if (
                    call["box_name"] == "MapBox"
                    and call["certainty"] == "Known"
                ):
                    totals["mapbox_known_calls"] += 1
                if (
                    call["box_name"] == "RuntimeDataBox"
                    and call["certainty"] == "Union"
                ):
                    totals["runtime_union_calls"] += 1

    if totals["copy_owned"] != 0 or totals["destroy_owned"] != 0:
        raise ProofFailure("STOP0 must not emit CopyOwned or DestroyOwned")

    return {
        "birth": birth,
        "cases": cases,
        "totals": totals,
    }


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    app_path = root / APP
    if not app_path.is_file():
        raise ProofFailure(f"missing proof app: {APP}")
    (root / ARTIFACT_DIR).mkdir(parents=True, exist_ok=True)

    bins = build_bins(root)
    runtime_by_mode: dict[str, dict[str, int]] = {}
    mir_by_mode: dict[str, dict[str, Any]] = {}
    for mode in ("debug", "release"):
        cases, mir_path = run_mode(root, mode, bins[mode])
        runtime_by_mode[mode] = cases
        mir_by_mode[mode] = normalize_mir(mir_path)

    if runtime_by_mode["debug"] != runtime_by_mode["release"]:
        raise ProofFailure("debug/release runtime matrix drift")
    if mir_by_mode["debug"] != mir_by_mode["release"]:
        raise ProofFailure("debug/release normalized MIR evidence drift")

    report = {
        "schema_version": 1,
        "proof_id": PROOF_ID,
        "runtime": runtime_by_mode["debug"],
        "mir": mir_by_mode["debug"],
        "selection": "UNCLASSIFIED-M0",
    }
    report_path = root / ARTIFACT_DIR / "report.json"
    report_path.write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n",
        encoding="utf-8",
    )

    totals = report["mir"]["totals"]
    print(PROOF_ID)
    for name in CASE_NAMES:
        print(f"case.{name}={report['runtime'][name]}")
    print(f"mir.mapbox_known_calls={totals['mapbox_known_calls']}")
    print(f"mir.runtime_union_calls={totals['runtime_union_calls']}")
    print(f"mir.copy_owned={totals['copy_owned']}")
    print(f"mir.destroy_owned={totals['destroy_owned']}")
    print(f"mir.release_strong={totals['release_strong']}")
    print("selection=UNCLASSIFIED-M0")
    print(f"report={report_path.relative_to(root)}")
    print("summary=observed")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ProofFailure as error:
        print(f"[map-field-owner-boxshape-proof] ERROR: {error}", file=sys.stderr)
        raise SystemExit(1)
