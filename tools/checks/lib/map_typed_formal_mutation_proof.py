#!/usr/bin/env python3
"""Prove explicit MapBox-formal transport without classifying TYPE0."""

from __future__ import annotations

import json
import os
from pathlib import Path
import re
import subprocess
import sys
from typing import Any

PROOF_ID = "map-typed-formal-mutation-proof"
APP_DIR = Path("apps/map-typed-formal-mutation-proof")
APP = APP_DIR / "main.hako"
COMMAND = APP_DIR / "storage_command.hako"
ARTIFACT_DIR = Path("target/checks/map-typed-formal-mutation-proof")
MODULE_NAME = "apps.proofs.map_typed_formal_mutation_storage_command"
MODULE_PATH = "apps/map-typed-formal-mutation-proof/storage_command.hako"
PUT = "TypedMapMutationCommandV1.put_proven/3"
CONTAINS = "TypedMapMutationCommandV1.contains/3"
BIRTH = "TypedMapFieldOwnerV1.birth/0"
FIELD_PUT = "TypedMapFieldOwnerV1.put/2"
LATE_FIELD_PUT = "TypedMapFieldOwnerV1.put_after_validation/3"
CASES = (
    "local_direct",
    "local_helper",
    "field_direct",
    "field_helper",
    "late_field_direct",
    "late_field_helper",
    "repeated",
    "instance_isolation",
    "two_file_transport",
    "negative_then_fresh",
)


class ProofFailure(RuntimeError):
    pass


def run(
    argv: list[str], root: Path, env: dict[str, str] | None = None
) -> subprocess.CompletedProcess[str]:
    merged = os.environ.copy()
    if env:
        merged.update(env)
    completed = subprocess.run(
        argv, cwd=root, env=merged, text=True, capture_output=True, check=False
    )
    if completed.returncode != 0:
        raise ProofFailure(
            f"command failed rc={completed.returncode}: {' '.join(argv)}\n"
            f"stdout:\n{completed.stdout}\nstderr:\n{completed.stderr}"
        )
    return completed


def build_bins(root: Path) -> dict[str, Path]:
    command = [
        "cargo",
        "build",
        "-q",
        "--features",
        "vm-reference",
        "--bin",
        "hakorune",
    ]
    run(command, root)
    run(command[:2] + ["--release"] + command[2:], root)
    return {
        "debug": root / "target/debug/hakorune",
        "release": root / "target/release/hakorune",
    }


def verify_source(root: Path) -> dict[str, Any]:
    main = (root / APP).read_text(encoding="utf-8")
    command = (root / COMMAND).read_text(encoding="utf-8")
    manifest = (root / "hako.toml").read_text(encoding="utf-8")

    expected_using = (
        f'using "{MODULE_NAME}" as TypedMapMutationCommandV1'
    )
    if main.splitlines()[0].strip() != expected_using:
        raise ProofFailure("main must use the exact named two-file module")
    expected_mapping = f'"{MODULE_NAME}" = "{MODULE_PATH}"'
    if manifest.count(expected_mapping) != 1:
        raise ProofFailure("hako.toml must own one exact TYPE0 module mapping")

    typed_formals = re.findall(
        r"(?m)^\s*(put_proven|contains)\(storage:\s*MapBox,", command
    )
    if typed_formals != ["put_proven", "contains"]:
        raise ProofFailure("typed storage formals must be exact and command-owned")
    if "storage.set(" not in command or ".set(" in main:
        raise ProofFailure("only the imported command may mutate MapBox storage")

    combined = main + "\n" + command
    forbidden = {
        "HMI dependency": r"\bHMI\b|\bHmi",
        "MapBox return": r"(?m)^\s*return\s+(?:me\.)?storage\s*$",
        "mutator result binding": r"=\s*TypedMapMutationCommandV1\.put_proven",
        "ownership spelling": r"\b(?:share|move|clone)\b",
    }
    for label, pattern in forbidden.items():
        if re.search(pattern, combined):
            raise ProofFailure(f"source contains forbidden {label}")
    if len(re.findall(r"\bme\.storage\s*=", main)) != 1:
        raise ProofFailure("storage field must be assigned exactly once in birth")
    return {
        "typed_storage_formals": typed_formals,
        "module_name": MODULE_NAME,
        "module_path": MODULE_PATH,
    }


def parse_runtime(text: str) -> dict[str, int]:
    lines = [line.strip() for line in text.splitlines() if line.strip()]
    if PROOF_ID not in lines:
        raise ProofFailure("runtime output is missing proof id")
    result: dict[str, int] = {}
    for case in CASES:
        prefix = f"case.{case}="
        rows = [line for line in lines if line.startswith(prefix)]
        if len(rows) != 1 or rows[0][len(prefix) :] not in ("0", "1"):
            raise ProofFailure(f"runtime output must contain one boolean {prefix} row")
        result[case] = int(rows[0][len(prefix) :])
    if "selection=UNCLASSIFIED-S0" not in lines or "summary=observed" not in lines:
        raise ProofFailure("runtime fixture classified before TYPE0-V0")
    if any(value != 1 for value in result.values()):
        raise ProofFailure(f"runtime matrix is not fully green: {result}")
    return result


def run_mode(root: Path, mode: str, binary: Path) -> tuple[dict[str, int], Path]:
    env = {"NYASH_FEATURES": "rune", "NYASH_DISABLE_PLUGINS": "1"}
    runtime = run([str(binary), "--backend", "vm", str(APP)], root, env)
    runtime_path = root / ARTIFACT_DIR / f"{mode}.runtime.txt"
    runtime_path.write_text(runtime.stdout, encoding="utf-8")
    mir_path = root / ARTIFACT_DIR / f"{mode}.mir.json"
    emitted = run(
        [str(binary), "--emit-mir-json", str(mir_path), str(APP)], root, env
    )
    (root / ARTIFACT_DIR / f"{mode}.emit.txt").write_text(
        emitted.stdout + emitted.stderr, encoding="utf-8"
    )
    return parse_runtime(runtime.stdout), mir_path


def instructions(function: dict[str, Any]):
    for block in function.get("blocks", []):
        yield from block.get("instructions", [])


def type_label(value: Any) -> str:
    if value is None:
        return "Unknown"
    if isinstance(value, str):
        return value
    if isinstance(value, dict):
        if value.get("box_type"):
            return f"handle:{value['box_type']}"
        if value.get("kind") == "string":
            return "String"
        if value.get("kind"):
            return str(value["kind"])
    return f"other:{value!r}"


def analyze_function(function: dict[str, Any]) -> dict[str, Any]:
    metadata = function.get("metadata", {})
    value_types = metadata.get("value_types", {})
    params = list(function.get("params", []))
    param_index = {value: index for index, value in enumerate(params)}
    definitions: dict[int, dict[str, Any]] = {}
    op_counts: dict[str, int] = {}
    for instruction in instructions(function):
        op = str(instruction.get("op"))
        op_counts[op] = op_counts.get(op, 0) + 1
        if isinstance(instruction.get("dst"), int):
            definitions[instruction["dst"]] = instruction

    def root_of(value: Any, seen: frozenset[int] = frozenset()) -> str:
        if not isinstance(value, int):
            return "missing"
        if value in seen:
            return "cycle"
        if value in param_index:
            return f"param:{param_index[value]}"
        instruction = definitions.get(value)
        if instruction is None:
            return "undefined"
        nested = seen | {value}
        op = instruction.get("op")
        if op == "copy":
            return root_of(instruction.get("src"), nested)
        if op == "phi":
            roots = sorted(
                {root_of(row[0], nested) for row in instruction.get("incoming", [])}
            )
            return roots[0] if len(roots) == 1 else f"phi({','.join(roots)})"
        if op == "newbox":
            return f"newbox:{instruction.get('type')}"
        if op == "field_get":
            return (
                f"field:{instruction.get('field')}"
                f"<{root_of(instruction.get('box'), nested)}>"
            )
        if op == "mir_call":
            callee = instruction.get("mir_call", {}).get("callee", {})
            return f"call:{callee.get('name')}"
        return str(op)

    calls: list[dict[str, Any]] = []
    field_gets: list[dict[str, Any]] = []
    releases: list[str] = []
    returns: list[str] = []
    for instruction in instructions(function):
        op = instruction.get("op")
        if op == "mir_call":
            payload = instruction.get("mir_call", {})
            callee = payload.get("callee", {})
            args = list(payload.get("args", []))
            calls.append(
                {
                    "type": callee.get("type"),
                    "name": callee.get("name"),
                    "box_name": callee.get("box_name"),
                    "certainty": callee.get("certainty"),
                    "receiver_root": root_of(callee.get("receiver")),
                    "arg_roots": [root_of(value) for value in args],
                    "arg_types": [
                        type_label(value_types.get(str(value))) for value in args
                    ],
                }
            )
        elif op == "field_get":
            field_gets.append(
                {
                    "field": instruction.get("field"),
                    "root": root_of(instruction.get("dst")),
                    "declared_type": type_label(instruction.get("declared_type")),
                    "result_type": type_label(
                        value_types.get(str(instruction.get("dst")))
                    ),
                }
            )
        elif op == "release_strong":
            releases.extend(root_of(value) for value in instruction.get("values", []))
        elif op == "ret":
            returns.append(
                type_label(value_types.get(str(instruction.get("value"))))
            )
    return {
        "name": function.get("name"),
        "declared_params": metadata.get("declared_param_decls", []),
        "params": [type_label(value_types.get(str(value))) for value in params],
        "calls": calls,
        "field_gets": field_gets,
        "generic_routes": metadata.get("generic_method_routes", []),
        "release_roots": sorted(releases),
        "returns": returns,
        "op_counts": {
            op: op_counts.get(op, 0)
            for op in ("field_set", "copy_owned", "destroy_owned", "release_strong")
        },
    }


def exact_calls(
    evidence: dict[str, Any],
    *,
    name: str,
    call_type: str | None = None,
    box_name: str | None = None,
) -> list[dict[str, Any]]:
    return [
        row
        for row in evidence["calls"]
        if row["name"] == name
        and (call_type is None or row["type"] == call_type)
        and (box_name is None or row["box_name"] == box_name)
    ]


def verify_typed_helper(
    evidence: dict[str, Any], expected_methods: tuple[str, ...]
) -> dict[str, Any]:
    declared = evidence["declared_params"]
    if not declared or declared[0] != {
        "name": "storage",
        "declared_type_name": "MapBox",
        "implicit_receiver": False,
    }:
        raise ProofFailure(f"{evidence['name']} lost exact storage ParamDecl")
    if not evidence["params"] or evidence["params"][0] != "handle:MapBox":
        raise ProofFailure(f"{evidence['name']} lost handle:MapBox parameter ValueId")
    for method in expected_methods:
        rows = exact_calls(
            evidence, name=method, call_type="Method", box_name="MapBox"
        )
        if len(rows) != 1:
            raise ProofFailure(f"{evidence['name']} must have one MapBox.{method}")
        if rows[0]["certainty"] != "Known" or rows[0]["receiver_root"] != "param:0":
            raise ProofFailure(f"{evidence['name']} lost MapBox/Known param:0 route")
        routes = [
            route
            for route in evidence["generic_routes"]
            if route.get("method") == method
        ]
        if len(routes) != 1 or routes[0].get("box_name") != "MapBox":
            raise ProofFailure(f"{evidence['name']} metadata route drift for {method}")
        if routes[0].get("receiver_origin_box") != "MapBox":
            raise ProofFailure(f"{evidence['name']} receiver origin drift for {method}")
    if any(kind == "handle:MapBox" for kind in evidence["returns"]):
        raise ProofFailure(f"{evidence['name']} returns a MapBox")
    if any(evidence["op_counts"][op] for op in ("field_set", "copy_owned", "destroy_owned")):
        raise ProofFailure(f"{evidence['name']} emitted forbidden ownership/field op")
    return {
        "declared_param0": declared[0],
        "parameter0_type": evidence["params"][0],
        "methods": [
            {
                "method": method,
                "box_name": exact_calls(
                    evidence, name=method, call_type="Method", box_name="MapBox"
                )[0]["box_name"],
                "certainty": exact_calls(
                    evidence, name=method, call_type="Method", box_name="MapBox"
                )[0]["certainty"],
                "receiver_root": exact_calls(
                    evidence, name=method, call_type="Method", box_name="MapBox"
                )[0]["receiver_root"],
            }
            for method in expected_methods
        ],
        "returns": evidence["returns"],
    }


def verify_global_put(evidence: dict[str, Any], expected_root: str) -> dict[str, Any]:
    rows = exact_calls(evidence, name=PUT, call_type="Global")
    if len(rows) != 1:
        raise ProofFailure(f"{evidence['name']} must call typed put exactly once")
    row = rows[0]
    if row["arg_roots"][0] != expected_root:
        raise ProofFailure(f"{evidence['name']} storage root drift")
    if row["arg_types"][0] not in ("handle:MapBox", "Unknown"):
        raise ProofFailure(f"{evidence['name']} storage argument type is invalid")
    return row


def normalize_mir(path: Path) -> dict[str, Any]:
    document = json.loads(path.read_text(encoding="utf-8"))
    functions = {
        function.get("name"): analyze_function(function)
        for function in document.get("functions", [])
    }
    required = {PUT, CONTAINS, BIRTH, FIELD_PUT, LATE_FIELD_PUT}
    missing = sorted(required - functions.keys())
    if missing:
        raise ProofFailure(f"MIR is missing required functions: {missing}")

    put = verify_typed_helper(functions[PUT], ("set",))
    contains = verify_typed_helper(functions[CONTAINS], ("has", "get"))
    if functions[PUT]["returns"] != ["void"]:
        raise ProofFailure("put helper must return no-value only")

    local_callers = (
        "Main.case_local_direct/0",
        "Main.case_local_helper/0",
        "Main.case_two_file_transport/0",
    )
    local_puts = {
        name: verify_global_put(functions[name], "newbox:MapBox")
        for name in local_callers
    }
    field_put = verify_global_put(functions[FIELD_PUT], "field:storage<param:0>")
    late_put = verify_global_put(
        functions[LATE_FIELD_PUT], "field:storage<param:0>"
    )
    late_fields = [
        row
        for row in functions[LATE_FIELD_PUT]["field_gets"]
        if row["field"] == "storage"
    ]
    if len(late_fields) != 1:
        raise ProofFailure("late-field mutator must contain one storage field_get")

    birth = functions[BIRTH]["op_counts"]["field_set"]
    if birth != 1:
        raise ProofFailure("birth must publish storage exactly once")
    for name, evidence in functions.items():
        if name != BIRTH and evidence["op_counts"]["field_set"]:
            raise ProofFailure(f"{name} reassigns a field after birth")

    totals = {
        op: sum(row["op_counts"][op] for row in functions.values())
        for op in ("copy_owned", "destroy_owned", "release_strong")
    }
    if totals["copy_owned"] or totals["destroy_owned"]:
        raise ProofFailure("TYPE0 emitted CopyOwned or DestroyOwned")
    forbidden_releases = [
        root
        for name, row in functions.items()
        for root in row["release_roots"]
        if (name in (PUT, CONTAINS) and root == "param:0")
        or root.startswith("field:storage<")
        or root.startswith("call:TypedMapMutationCommandV1.")
    ]
    if forbidden_releases:
        raise ProofFailure(f"forbidden selected release roots: {forbidden_releases}")

    return {
        "helpers": {"put": put, "contains": contains},
        "call_sites": {
            "local_puts": local_puts,
            "field_put": field_put,
            "late_field_put": late_put,
            "late_field_get": late_fields[0],
        },
        "birth_storage_field_sets": birth,
        "totals": totals,
    }


def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    source = verify_source(root)
    (root / ARTIFACT_DIR).mkdir(parents=True, exist_ok=True)
    runtime_by_mode: dict[str, dict[str, int]] = {}
    mir_by_mode: dict[str, dict[str, Any]] = {}
    for mode, binary in build_bins(root).items():
        runtime, mir_path = run_mode(root, mode, binary)
        runtime_by_mode[mode] = runtime
        mir_by_mode[mode] = normalize_mir(mir_path)
    if runtime_by_mode["debug"] != runtime_by_mode["release"]:
        raise ProofFailure("debug/release runtime matrix drift")
    if mir_by_mode["debug"] != mir_by_mode["release"]:
        raise ProofFailure("debug/release normalized MIR evidence drift")

    report = {
        "schema_version": 1,
        "proof_id": PROOF_ID,
        "source": source,
        "runtime": runtime_by_mode["debug"],
        "mir": mir_by_mode["debug"],
        "selection": "UNCLASSIFIED-M0",
    }
    report_path = root / ARTIFACT_DIR / "report.json"
    report_path.write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print(PROOF_ID)
    for case in CASES:
        print(f"case.{case}={report['runtime'][case]}")
    print(
        "mir.put.parameter0="
        f"{report['mir']['helpers']['put']['parameter0_type']}"
    )
    print(
        "mir.put.route="
        f"{report['mir']['helpers']['put']['methods'][0]['box_name']}/"
        f"{report['mir']['helpers']['put']['methods'][0]['certainty']}"
    )
    print(
        "mir.late_field_get.type="
        f"{report['mir']['call_sites']['late_field_get']['result_type']}"
    )
    print(f"mir.copy_owned={report['mir']['totals']['copy_owned']}")
    print(f"mir.destroy_owned={report['mir']['totals']['destroy_owned']}")
    print(f"mir.release_strong={report['mir']['totals']['release_strong']}")
    print("selection=UNCLASSIFIED-M0")
    print(f"report={report_path.relative_to(root)}")
    print("summary=observed")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ProofFailure as error:
        print(f"[{PROOF_ID}] ERROR: {error}", file=sys.stderr)
        raise SystemExit(1)
