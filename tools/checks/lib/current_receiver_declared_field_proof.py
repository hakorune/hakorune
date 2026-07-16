#!/usr/bin/env python3
"""Classify current-receiver declared-field provenance without changing MIR."""

from __future__ import annotations

import json
import os
from pathlib import Path
import re
import subprocess
import sys
from typing import Any

PROOF_ID = "current-receiver-declared-field-proof"
APP = Path("apps/current-receiver-declared-field-proof/main.hako")
ARTIFACT_DIR = Path("target/checks/current-receiver-declared-field-proof")
OWNER = "DeclaredFieldOwnerV1"
UNTYPED_OWNER = "UntypedFieldOwnerV1"
DIRECT = f"{OWNER}.declfield_probe_v1_direct_array/1"
FALLTHROUGH = f"{OWNER}.declfield_probe_v1_after_validation/2"
NESTED = f"{OWNER}.declfield_probe_v1_after_nested_validation/2"
ALIAS = f"{OWNER}.declfield_probe_v1_through_receiver_alias/1"
MAP = f"{OWNER}.declfield_probe_v1_map_roundtrip/3"
TYPED_CONTROL = "TypedArrayFieldControlV1.push_and_length/2"
UNTYPED_CONTROL = f"{UNTYPED_OWNER}.declfield_probe_v1_untyped_field/1"
PARAM_CONTROL = (
    "ExplicitOwnerParamControlV1.declfield_probe_v1_push_and_length/2"
)
CASES = ("A1", "A2", "A3", "A4", "A5", "A6", "A7", "M1", "C1", "N1", "N2")


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
    base = ["cargo", "build", "-q", "--features", "vm-reference", "--bin", "hakorune"]
    run(base, root)
    run(base[:2] + ["--release"] + base[2:], root)
    return {
        "debug": root / "target/debug/hakorune",
        "release": root / "target/release/hakorune",
    }


def verify_source(root: Path) -> dict[str, Any]:
    source = (root / APP).read_text(encoding="utf-8")
    forbidden = {
        "HMI name": r"\bHMI\b|\bHmi",
        "import": r"(?m)^\s*(?:using|import)\b",
        "ownership spelling": r"\b(?:share|move|clone)\b",
    }
    for label, pattern in forbidden.items():
        if re.search(pattern, source):
            raise ProofFailure(f"source contains forbidden {label}")
    if len(re.findall(r"\bme\.items\s*=", source)) != 2:
        raise ProofFailure("typed and untyped items must each be born exactly once")
    if len(re.findall(r"\bme\.map\s*=", source)) != 1:
        raise ProofFailure("typed map must be born exactly once")
    if "items: ArrayBox" not in source or "map: MapBox" not in source:
        raise ProofFailure("typed owner lost explicit field declarations")
    if "push_and_length(items: ArrayBox," not in source:
        raise ProofFailure("typed ArrayBox comparison helper is missing")
    if "push_and_length(owner: DeclaredFieldOwnerV1," not in source:
        raise ProofFailure("ordinary typed owner parameter control is missing")
    return {
        "cases": list(CASES),
        "typed_fields": {"items": "ArrayBox", "map": "MapBox"},
        "selector": "--case",
    }


def parse_runtime(text: str, case_id: str) -> int:
    lines = [line.strip() for line in text.splitlines() if line.strip()]
    required = {
        PROOF_ID,
        f"case={case_id}",
        "selection=UNCLASSIFIED-S0",
        "summary=observed",
    }
    if not required.issubset(lines):
        raise ProofFailure(f"{case_id} runtime output lost stable observation rows")
    observed = [line for line in lines if line.startswith("observed=")]
    if len(observed) != 1 or observed[0] not in ("observed=0", "observed=1"):
        raise ProofFailure(f"{case_id} runtime output must contain one observed bit")
    return int(observed[0].split("=", 1)[1])


def run_mode(root: Path, mode: str, binary: Path) -> tuple[dict[str, int], Path]:
    env = {"NYASH_FEATURES": "rune", "NYASH_DISABLE_PLUGINS": "1"}
    runtime: dict[str, int] = {}
    for case_id in CASES:
        completed = run(
            [
                str(binary),
                "--backend",
                "vm",
                str(APP),
                "--",
                "--case",
                case_id,
            ],
            root,
            env,
        )
        runtime[case_id] = parse_runtime(completed.stdout, case_id)
        (root / ARTIFACT_DIR / f"{mode}.{case_id}.runtime.txt").write_text(
            completed.stdout, encoding="utf-8"
        )
    mir_path = root / ARTIFACT_DIR / f"{mode}.mir.json"
    emitted = run(
        [str(binary), "--emit-mir-json", str(mir_path), str(APP)], root, env
    )
    (root / ARTIFACT_DIR / f"{mode}.emit.txt").write_text(
        emitted.stdout + emitted.stderr, encoding="utf-8"
    )
    return runtime, mir_path


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
        if value.get("kind"):
            return str(value["kind"])
    return f"other:{value!r}"


def analyze_function(function: dict[str, Any]) -> dict[str, Any]:
    metadata = function.get("metadata", {})
    params = list(function.get("params", []))
    declared = list(metadata.get("declared_param_decls", []))
    value_types = metadata.get("value_types", {})
    param_index = {value: index for index, value in enumerate(params)}
    definitions = {
        row["dst"]: row
        for row in instructions(function)
        if isinstance(row.get("dst"), int)
    }

    def root_of(value: Any, seen: frozenset[int] = frozenset()) -> str:
        if not isinstance(value, int):
            return "missing"
        if value in seen:
            return "cycle"
        if value in param_index:
            index = param_index[value]
            is_receiver = (
                index == 0
                and bool(declared)
                and declared[0].get("implicit_receiver") is True
            )
            return "current_receiver" if is_receiver else f"foreign_parameter:{index}"
        row = definitions.get(value)
        if row is None:
            return "missing"
        nested = seen | {value}
        op = str(row.get("op"))
        if op == "copy":
            inner = root_of(row.get("src"), nested)
            if inner == "current_receiver":
                return "Copy(current_receiver)"
            if inner.startswith("Copy(") or inner.startswith("Copy*("):
                tail = inner[inner.find("(") + 1 : -1]
                return f"Copy*({tail})"
            return f"Copy({inner})"
        if op == "phi":
            roots = sorted(
                {root_of(pair[0], nested) for pair in row.get("incoming", [])}
            )
            return f"Phi({','.join(roots)})"
        if op == "select":
            return "Select"
        if op == "copy_owned":
            return "CopyOwned"
        if op == "field_get":
            return f"FieldGet({row.get('field')})"
        if op == "newbox":
            return f"NewBox({row.get('type')})"
        if op == "mir_call":
            return "Call"
        return op

    field_gets: list[dict[str, Any]] = []
    method_rows: set[tuple[str, str, str]] = set()
    selected_releases: list[str] = []
    op_counts = {"copy_owned": 0, "destroy_owned": 0, "release_strong": 0}
    for row in instructions(function):
        op = str(row.get("op"))
        if op in op_counts:
            op_counts[op] += 1
        if op == "field_get":
            box = row.get("box")
            dst = row.get("dst")
            field_gets.append(
                {
                    "field": row.get("field"),
                    "base_root": root_of(box),
                    "base_type": type_label(value_types.get(str(box))),
                    "declared_type": type_label(row.get("declared_type")),
                    "result_type": type_label(value_types.get(str(dst))),
                }
            )
        elif op == "mir_call":
            callee = row.get("mir_call", {}).get("callee", {})
            if callee.get("type") == "Method":
                method_rows.add(
                    (
                        str(callee.get("name")),
                        str(callee.get("box_name")),
                        str(callee.get("certainty")),
                    )
                )
        elif op == "release_strong":
            for value in row.get("values", []):
                root = root_of(value)
                if "current_receiver" in root or root.startswith("FieldGet("):
                    selected_releases.append(root)
    for row in metadata.get("generic_method_routes", []):
        method_rows.add(
            (
                str(row.get("method")),
                str(row.get("box_name")),
                "Known"
                if row.get("receiver_origin_box") == row.get("box_name")
                else "Union",
            )
        )
    return {
        "declared_params": declared,
        "param_types": [type_label(value_types.get(str(value))) for value in params],
        "field_gets": field_gets,
        "methods": [
            {"method": method, "box": box, "certainty": certainty}
            for method, box, certainty in sorted(method_rows)
        ],
        "op_counts": op_counts,
        "selected_releases": sorted(selected_releases),
    }


def normalize_registry(document: dict[str, Any]) -> dict[str, dict[str, str]]:
    registry: dict[str, dict[str, str]] = {}
    for owner in document.get("user_box_decls", []):
        fields = {
            str(row.get("name")): (
                str(row.get("declared_type"))
                if row.get("declared_type") is not None
                else "Unknown"
            )
            for row in owner.get("field_decls", [])
        }
        registry[str(owner.get("name"))] = fields
    return registry


def normalize_mir(path: Path) -> dict[str, Any]:
    document = json.loads(path.read_text(encoding="utf-8"))
    all_functions = {
        str(function.get("name")): function
        for function in document.get("functions", [])
    }
    required = {
        DIRECT,
        FALLTHROUGH,
        NESTED,
        ALIAS,
        MAP,
        TYPED_CONTROL,
        UNTYPED_CONTROL,
        PARAM_CONTROL,
    }
    missing = sorted(required - all_functions.keys())
    if missing:
        raise ProofFailure(f"MIR is missing required functions: {missing}")
    functions = {
        name: analyze_function(all_functions[name]) for name in sorted(required)
    }
    totals = {
        op: sum(row["op_counts"][op] for row in functions.values())
        for op in ("copy_owned", "destroy_owned")
    }
    selected_releases = sorted(
        root
        for row in functions.values()
        for root in row["selected_releases"]
    )
    return {
        "registry": normalize_registry(document),
        "functions": functions,
        "totals": totals,
        "selected_releases": selected_releases,
    }


def field_rows(mir: dict[str, Any], function: str, field: str) -> list[dict[str, Any]]:
    return [
        row
        for row in mir["functions"][function]["field_gets"]
        if row["field"] == field
    ]


def has_method(
    mir: dict[str, Any], function: str, method: str, box: str, certainty: str
) -> bool:
    return {
        "method": method,
        "box": box,
        "certainty": certainty,
    } in mir["functions"][function]["methods"]


def classify(runtime: dict[str, int], mir: dict[str, Any]) -> str:
    direct_fields = field_rows(mir, DIRECT, "items")
    direct_ok = (
        runtime["A1"] == 1
        and len(direct_fields) == 2
        and all(row["declared_type"] == "handle:ArrayBox" for row in direct_fields)
        and any(row["result_type"] == "handle:ArrayBox" for row in direct_fields)
        and has_method(mir, DIRECT, "push", "ArrayBox", "Known")
        and has_method(mir, DIRECT, "length", "ArrayBox", "Known")
    )
    if not direct_ok or any(value != 1 for value in runtime.values()):
        return "BASELINE-RUNTIME-BROKEN"

    selected = mir["functions"][FALLTHROUGH]
    receiver_ok = (
        bool(selected["declared_params"])
        and selected["declared_params"][0]
        == {
            "name": "me",
            "declared_type_name": None,
            "implicit_receiver": True,
        }
        and bool(selected["param_types"])
        and selected["param_types"][0] == f"handle:{OWNER}"
    )
    if not receiver_ok:
        return "CURRENT-RECEIVER-IDENTITY-MISSING"

    fields = mir["registry"].get(OWNER, {})
    if fields.get("items") != "ArrayBox" or fields.get("map") != "MapBox":
        return "DECLARED-FIELD-REGISTRY-MISSING"

    selected_fields = field_rows(mir, FALLTHROUGH, "items")
    if not selected_fields:
        return "CURRENT-RECEIVER-IDENTITY-MISSING"
    if any(row["base_type"] != f"handle:{OWNER}" for row in selected_fields):
        return "BASE-TYPE-MISMATCH"
    roots = [row["base_root"] for row in selected_fields]
    if any("Phi(" in root for root in roots):
        return "PHI-ROOT-DESIGN-REQUIRED"
    if all(
        root == "current_receiver"
        or root.startswith("Copy(current_receiver)")
        or root.startswith("Copy*(current_receiver)")
        for root in roots
    ):
        return "COPY-ROOT-DECLFIELD-AUTHORIZED"
    return "CURRENT-RECEIVER-IDENTITY-MISSING"


def verify_controls(mir: dict[str, Any]) -> None:
    typed = mir["functions"][TYPED_CONTROL]
    if typed["declared_params"][0].get("declared_type_name") != "ArrayBox":
        raise ProofFailure("typed helper lost its ArrayBox formal")
    for method in ("push", "length"):
        if not has_method(mir, TYPED_CONTROL, method, "ArrayBox", "Known"):
            raise ProofFailure(f"typed helper lost ArrayBox/Known {method}")
    for method in ("set", "has", "get"):
        if not has_method(mir, MAP, method, "MapBox", "Known"):
            raise ProofFailure(f"MapBox regression lost Known {method}")

    untyped_fields = field_rows(mir, UNTYPED_CONTROL, "items")
    if not untyped_fields or any(
        row["declared_type"] != "Unknown" for row in untyped_fields
    ):
        raise ProofFailure("untyped field control gained a declared field type")
    param_fields = field_rows(mir, PARAM_CONTROL, "items")
    if not param_fields or any(
        "foreign_parameter:0" not in row["base_root"] for row in param_fields
    ):
        raise ProofFailure("ordinary typed parameter control became current receiver")

    late_fields = field_rows(mir, FALLTHROUGH, "items")
    if any(row["declared_type"] != "Unknown" for row in late_fields):
        raise ProofFailure("fallthrough late field unexpectedly gained declared type")
    if any(row["result_type"] != "Unknown" for row in late_fields):
        raise ProofFailure("fallthrough late field unexpectedly gained result type")
    for method in ("push", "length"):
        if not has_method(mir, FALLTHROUGH, method, "RuntimeDataBox", "Union"):
            raise ProofFailure(f"fallthrough control lost RuntimeDataBox/Union {method}")
    if mir["totals"]["copy_owned"] or mir["totals"]["destroy_owned"]:
        raise ProofFailure("DECLFIELD0 emitted CopyOwned or DestroyOwned")
    if mir["selected_releases"]:
        raise ProofFailure("DECLFIELD0 emitted selected-route ReleaseStrong")


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
        verify_controls(mir_by_mode[mode])
    if runtime_by_mode["debug"] != runtime_by_mode["release"]:
        raise ProofFailure("debug/release runtime matrix drift")
    if mir_by_mode["debug"] != mir_by_mode["release"]:
        raise ProofFailure("debug/release normalized MIR drift")

    runtime = runtime_by_mode["debug"]
    mir = mir_by_mode["debug"]
    selection = classify(runtime, mir)
    report = {
        "schema_version": 1,
        "proof_id": PROOF_ID,
        "source": source,
        "runtime": runtime,
        "mir": mir,
        "selection": selection,
    }
    report_path = root / ARTIFACT_DIR / "report.json"
    report_path.write_text(
        json.dumps(report, indent=2, sort_keys=True) + "\n", encoding="utf-8"
    )
    print(PROOF_ID)
    for case_id in CASES:
        print(f"case.{case_id}={runtime[case_id]}")
    selected_fields = field_rows(mir, FALLTHROUGH, "items")
    print(f"mir.selected.base_root={selected_fields[0]['base_root']}")
    print(f"mir.selected.base_type={selected_fields[0]['base_type']}")
    print(f"mir.selected.declared_type={selected_fields[0]['declared_type']}")
    print(f"mir.selected.result_type={selected_fields[0]['result_type']}")
    print(f"mir.copy_owned={mir['totals']['copy_owned']}")
    print(f"mir.destroy_owned={mir['totals']['destroy_owned']}")
    print(f"mir.selected_release_strong={len(mir['selected_releases'])}")
    print(f"selection={selection}")
    print(f"report={report_path.relative_to(root)}")
    print("summary=observed")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except ProofFailure as error:
        print(f"[{PROOF_ID}] ERROR: {error}", file=sys.stderr)
        raise SystemExit(1)
