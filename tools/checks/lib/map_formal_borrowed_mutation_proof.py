#!/usr/bin/env python3
"""Prove ordinary MapBox-formal mutation visibility without classifying V0."""

from __future__ import annotations

import json
import os
from pathlib import Path
import re
import subprocess
import sys
from typing import Any

PROOF_ID = "map-formal-borrowed-mutation-proof"
APP = Path("apps/map-formal-borrowed-mutation-proof/main.hako")
ARTIFACT_DIR = Path("target/checks/map-formal-borrowed-mutation-proof")
CASES = (
    "local_direct_baseline",
    "field_direct_baseline",
    "local_formal_literal_direct",
    "local_formal_dynamic_direct",
    "local_formal_dynamic_helper",
    "field_formal_literal_direct",
    "field_formal_dynamic_direct",
    "field_formal_dynamic_helper",
    "repeated_mutation",
    "instance_isolation",
)
HELPERS = {
    "put": "MapFormalMutationCommandV1.map_formal_mutation_v1_put/3",
    "has": "MapFormalMutationCommandV1.map_formal_mutation_v1_has/2",
    "load": "MapFormalMutationCommandV1.map_formal_mutation_v1_load_present/2",
}
LOCAL_CALLERS = (
    "Main.map_formal_mutation_v1_case_local_formal_literal_direct/0",
    "Main.map_formal_mutation_v1_case_local_formal_dynamic_direct/0",
    "Main.map_formal_mutation_v1_case_local_formal_dynamic_helper/0",
)
FIELD_MUTATOR = "MapFormalFieldOwnerV1.map_formal_mutation_v1_command_put_id/2"
FIELD_HELPER_OBSERVER = (
    "MapFormalFieldOwnerV1.map_formal_mutation_v1_helper_contains_id/2"
)
FIELD_DIRECT_OBSERVER = (
    "MapFormalFieldOwnerV1.map_formal_mutation_v1_direct_contains_id/2"
)
BIRTH = "MapFormalFieldOwnerV1.birth/0"

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
    common = ["cargo", "build", "-q", "--features", "vm-reference", "--bin", "hakorune"]
    run(common, root)
    run(common[:2] + ["--release"] + common[2:], root)
    return {
        "debug": root / "target/debug/hakorune",
        "release": root / "target/release/hakorune",
    }

def verify_source(root: Path) -> None:
    text = (root / APP).read_text(encoding="utf-8")
    forbidden = {
        "application import": r"(?m)^\s*(?:import|using)\b",
        "MapBox return": r"(?m)^\s*return\s+(?:me\.)?storage\s*$",
        "mutator result assignment": r"=\s*MapFormalMutationCommandV1"
        r"\.map_formal_mutation_v1_put\s*\(",
        "ownership spelling": r"\b(?:share|move|clone)\b",
        "result ledger": r"\b(?:ArrayBox|ResultBox)\b",
    }
    for label, pattern in forbidden.items():
        if re.search(pattern, text):
            raise ProofFailure(f"source contains forbidden {label}")
    if len(re.findall(r"\bme\.storage\s*=", text)) != 1:
        raise ProofFailure("storage field must be assigned exactly once in birth")

def parse_runtime(text: str) -> dict[str, int]:
    lines = [line.strip() for line in text.splitlines() if line.strip()]
    if PROOF_ID not in lines:
        raise ProofFailure("runtime output is missing proof id")
    result: dict[str, int] = {}
    for case in CASES:
        prefix = f"case.{case}="
        matches = [line for line in lines if line.startswith(prefix)]
        if len(matches) != 1 or matches[0][len(prefix) :] not in ("0", "1"):
            raise ProofFailure(f"runtime output must contain one boolean {prefix} row")
        result[case] = int(matches[0][len(prefix) :])
    if "selection=UNCLASSIFIED-S0" not in lines or "summary=observed" not in lines:
        raise ProofFailure("runtime output classified before V0 or lost summary")
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
        for instruction in block.get("instructions", []):
            yield instruction

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
    value_types = function.get("metadata", {}).get("value_types", {})
    params = list(function.get("params", []))
    param_index = {value_id: index for index, value_id in enumerate(params)}
    definitions: dict[int, dict[str, Any]] = {}
    op_counts: dict[str, int] = {}
    for instruction in instructions(function):
        op = str(instruction.get("op"))
        op_counts[op] = op_counts.get(op, 0) + 1
        if isinstance(instruction.get("dst"), int):
            definitions[instruction["dst"]] = instruction

    def root_of(value_id: Any, seen: frozenset[int] = frozenset()) -> str:
        if not isinstance(value_id, int):
            return "missing"
        if value_id in seen:
            return "cycle"
        if value_id in param_index:
            return f"param:{param_index[value_id]}"
        instruction = definitions.get(value_id)
        if instruction is None:
            return "undefined"
        op = instruction.get("op")
        nested = seen | {value_id}
        if op == "copy":
            return root_of(instruction.get("src"), nested)
        if op == "phi":
            roots = sorted({root_of(row[0], nested) for row in instruction["incoming"]})
            return roots[0] if len(roots) == 1 else f"phi({','.join(roots)})"
        if op == "newbox":
            return f"newbox:{instruction.get('type')}"
        if op == "field_get":
            return (
                f"field:{instruction.get('field')}"
                f"<{root_of(instruction.get('box'), nested)}>"
            )
        if op == "binop":
            return (
                f"binop:{instruction.get('operation')}"
                f"<{root_of(instruction.get('lhs'), nested)},"
                f"{root_of(instruction.get('rhs'), nested)}>"
            )
        if op == "const":
            return f"const:{type_label(value_types.get(str(value_id)))}"
        if op == "mir_call":
            callee = instruction.get("mir_call", {}).get("callee", {})
            return f"call:{callee.get('name')}"
        return str(op)

    method_calls: list[dict[str, Any]] = []
    global_calls: list[dict[str, Any]] = []
    releases: list[str] = []
    return_kinds: list[str] = []
    for instruction in instructions(function):
        op = instruction.get("op")
        if op == "mir_call":
            payload = instruction.get("mir_call", {})
            callee = payload.get("callee", {})
            args = list(payload.get("args", []))
            row = {
                "name": callee.get("name"),
                "arg_roots": [root_of(value) for value in args],
                "arg_types": [type_label(value_types.get(str(value))) for value in args],
                "result_type": type_label(
                    value_types.get(str(instruction.get("dst")))
                ),
            }
            if callee.get("type") == "Global":
                global_calls.append(row)
            else:
                row.update(
                    {
                        "box_name": callee.get("box_name"),
                        "certainty": callee.get("certainty"),
                        "receiver_root": root_of(callee.get("receiver")),
                    }
                )
                method_calls.append(row)
        elif op == "release_strong":
            releases.extend(root_of(value) for value in instruction.get("values", []))
        elif op == "ret":
            value = instruction.get("value")
            return_kinds.append(type_label(value_types.get(str(value))))
    return {
        "name": function.get("name"),
        "params": [type_label(value_types.get(str(value))) for value in params],
        "op_counts": {
            name: op_counts.get(name, 0)
            for name in (
                "field_get",
                "field_set",
                "copy_owned",
                "destroy_owned",
                "release_strong",
            )
        },
        "method_calls": method_calls,
        "global_calls": global_calls,
        "release_roots": sorted(releases),
        "return_kinds": return_kinds,
    }

def exact_calls(
    evidence: dict[str, Any], *, name: str, box: str | None = None
) -> list[dict[str, Any]]:
    rows = (
        evidence["global_calls"] if box is None else evidence["method_calls"]
    )
    return [
        row
        for row in rows
        if row["name"] == name and (box is None or row["box_name"] == box)
    ]

def verify_helper(evidence: dict[str, Any], methods: tuple[str, ...]) -> None:
    counts = evidence["op_counts"]
    if any(counts[name] != 0 for name in ("field_set", "copy_owned", "destroy_owned", "release_strong")):
        raise ProofFailure(f"{evidence['name']} emitted forbidden ownership/field operation")
    if evidence["params"][0] != "handle:MapBox":
        raise ProofFailure(f"{evidence['name']} lost MapBox formal metadata")
    expected_args = {"set": ["param:1", "param:2"], "has": ["param:1"], "get": ["param:1"]}
    for method in methods:
        rows = exact_calls(evidence, name=method, box="RuntimeDataBox")
        if len(rows) != 1:
            raise ProofFailure(f"{evidence['name']} must have one RuntimeDataBox.{method}")
        row = rows[0]
        if row["certainty"] != "Union" or row["receiver_root"] != "param:0":
            raise ProofFailure(f"{evidence['name']} helper receiver contract drift")
        if row["arg_roots"] != expected_args[method]:
            raise ProofFailure(f"{evidence['name']} helper argument order drift")
    if any(kind == "handle:MapBox" for kind in evidence["return_kinds"]):
        raise ProofFailure(f"{evidence['name']} returns raw MapBox")

def verify_global_storage_call(
    evidence: dict[str, Any], name: str, expected_root: str, arity: int
) -> dict[str, Any]:
    rows = exact_calls(evidence, name=name)
    if len(rows) != 1:
        raise ProofFailure(f"{evidence['name']} must call {name} exactly once")
    row = rows[0]
    if len(row["arg_roots"]) != arity:
        raise ProofFailure(f"{evidence['name']} argument cardinality drift for {name}")
    if row["arg_roots"][0] != expected_root:
        raise ProofFailure(f"{evidence['name']} storage root drift for {name}")
    if row["arg_types"][0] != "handle:MapBox":
        raise ProofFailure(f"{evidence['name']} storage argument type drift for {name}")
    return row


def normalize_mir(path: Path) -> dict[str, Any]:
    document = json.loads(path.read_text(encoding="utf-8"))
    functions = {
        function.get("name"): analyze_function(function)
        for function in document.get("functions", [])
    }
    required = {
        BIRTH,
        FIELD_MUTATOR,
        FIELD_HELPER_OBSERVER,
        FIELD_DIRECT_OBSERVER,
        *HELPERS.values(),
        *LOCAL_CALLERS,
    }
    missing = sorted(required - functions.keys())
    if missing:
        raise ProofFailure(f"MIR is missing required functions: {missing}")

    verify_helper(functions[HELPERS["put"]], ("set",))
    verify_helper(functions[HELPERS["has"]], ("has",))
    verify_helper(functions[HELPERS["load"]], ("has", "get"))
    if functions[HELPERS["put"]]["return_kinds"] != ["void"]:
        raise ProofFailure("put helper must return no-value only")

    local_rows = [
        verify_global_storage_call(functions[name], HELPERS["put"], "newbox:MapBox", 3)
        for name in LOCAL_CALLERS
    ]
    local_helper = functions[LOCAL_CALLERS[-1]]
    local_has = verify_global_storage_call(
        local_helper, HELPERS["has"], "newbox:MapBox", 2
    )
    local_load = verify_global_storage_call(
        local_helper, HELPERS["load"], "newbox:MapBox", 2
    )
    if local_rows[-1]["arg_roots"][0] != local_has["arg_roots"][0] or local_has[
        "arg_roots"
    ][0] != local_load["arg_roots"][0]:
        raise ProofFailure("local helper observation changed storage root")
    field_put = verify_global_storage_call(
        functions[FIELD_MUTATOR],
        HELPERS["put"],
        "field:storage<param:0>",
        3,
    )
    field_has = verify_global_storage_call(
        functions[FIELD_HELPER_OBSERVER],
        HELPERS["has"],
        "field:storage<param:0>",
        2,
    )
    field_load = verify_global_storage_call(
        functions[FIELD_HELPER_OBSERVER],
        HELPERS["load"],
        "field:storage<param:0>",
        2,
    )
    if field_has["arg_roots"][0] != field_load["arg_roots"][0]:
        raise ProofFailure("field helper observation changed storage root")

    direct_observers = [functions[LOCAL_CALLERS[0]], functions[LOCAL_CALLERS[1]], functions[FIELD_DIRECT_OBSERVER]]
    direct_known = 0
    for evidence in direct_observers:
        for method in ("has", "get"):
            rows = exact_calls(evidence, name=method, box="MapBox")
            if not rows or any(row["certainty"] != "Known" for row in rows):
                raise ProofFailure(f"{evidence['name']} lost direct MapBox.{method}/Known")
            direct_known += len(rows)

    birth = functions[BIRTH]["op_counts"]
    if birth["field_set"] != 1:
        raise ProofFailure("birth must publish storage exactly once")
    for name, evidence in functions.items():
        if name != BIRTH and evidence["op_counts"]["field_set"] != 0:
            raise ProofFailure(f"{name} reassigns a field after birth")

    totals = {
        op: sum(row["op_counts"][op] for row in functions.values())
        for op in ("copy_owned", "destroy_owned", "release_strong")
    }
    if totals["copy_owned"] != 0 or totals["destroy_owned"] != 0:
        raise ProofFailure("DELTA0 emitted CopyOwned or DestroyOwned")
    forbidden_release_roots = [
        root
        for name, row in functions.items()
        for root in row["release_roots"]
        if (name in HELPERS.values() and root == "param:0")
        or root.startswith("field:storage<")
        or root.startswith("call:MapFormalMutationCommandV1.")
    ]
    if forbidden_release_roots:
        raise ProofFailure(
            f"forbidden storage/helper release roots: {forbidden_release_roots}"
        )

    return {
        "helper": {
            key: {
                "params": functions[name]["params"],
                "method_calls": functions[name]["method_calls"],
                "return_kinds": functions[name]["return_kinds"],
                "op_counts": functions[name]["op_counts"],
            }
            for key, name in HELPERS.items()
        },
        "call_sites": {
            "local_puts": local_rows,
            "field_put": field_put,
            "field_has": field_has,
            "field_load": field_load,
        },
        "direct_mapbox_known_observations": direct_known,
        "birth_storage_field_sets": birth["field_set"],
        "totals": totals,
    }

def classify(cases: dict[str, int]) -> str:
    if not cases["local_direct_baseline"] or not cases["field_direct_baseline"]:
        return "STOP-BASELINE0"
    if not cases["local_formal_literal_direct"]:
        return "STATIC-FORMAL-MUTATION0"
    if not cases["local_formal_dynamic_direct"] or not cases["local_formal_dynamic_helper"]:
        return "STATIC-FORMAL-KEY-OR-OBSERVE0"
    if not cases["field_formal_literal_direct"]:
        return "FIELD-STATIC-FORMAL-MUTATION0"
    if not cases["field_formal_dynamic_direct"]:
        return "FIELD-STATIC-DYNAMIC0"
    if not cases["field_formal_dynamic_helper"]:
        return "FIELD-STATIC-OBSERVATION0"
    if not cases["repeated_mutation"]:
        return "STATIC-FORMAL-REPEAT0"
    if not cases["instance_isolation"]:
        return "STATIC-FORMAL-ISOLATION0"
    return "A-PRIME-AUTHORIZED"

def main() -> int:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    verify_source(root)
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

    selection = classify(runtime_by_mode["debug"])
    report = {
        "schema_version": 1,
        "proof_id": PROOF_ID,
        "runtime": runtime_by_mode["debug"],
        "mir": mir_by_mode["debug"],
        "selection": selection,
    }
    report_path = root / ARTIFACT_DIR / "report.json"
    report_path.write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print(PROOF_ID)
    for case in CASES:
        print(f"case.{case}={report['runtime'][case]}")
    print(f"mir.direct_mapbox_known={report['mir']['direct_mapbox_known_observations']}")
    print(f"mir.copy_owned={report['mir']['totals']['copy_owned']}")
    print(f"mir.destroy_owned={report['mir']['totals']['destroy_owned']}")
    print(f"mir.release_strong={report['mir']['totals']['release_strong']}")
    print(f"selection={selection}")
    print(f"report={report_path.relative_to(root)}")
    print("summary=observed")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ProofFailure as error:
        print(f"[map-formal-borrowed-mutation-proof] ERROR: {error}", file=sys.stderr)
        raise SystemExit(1)
