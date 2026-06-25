#!/usr/bin/env python3
"""Project the minimal MirModule shell transport from live Rust source.

This is a transport-plan slice for `MirModule::new` only. It does not claim
function insertion, globals publication, finalize behavior, generated Hako, or
backend/runtime behavior.
"""

from __future__ import annotations

import argparse
import json
import re
from copy import deepcopy
from pathlib import Path
from typing import Any

from context_fact_extraction import report_or_emit, require


ROOT = Path(__file__).resolve().parents[2]
FIXTURE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "mir-module-minimal-shell-transport-plan-v0.json"
)
TYPES_RS = ROOT / "src/mir/function/types.rs"
IMPL_RS = ROOT / "src/mir/function/module_impl.rs"


def _read(path: Path) -> str:
    return path.read_text()


def _method_body(source: str, signature: str) -> str:
    start = source.find(signature)
    require(start >= 0, f"missing method signature: {signature}")
    brace = source.find("{", start)
    require(brace >= 0, f"missing method body brace: {signature}")
    depth = 0
    for index in range(brace, len(source)):
        char = source[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return source[brace + 1 : index]
    raise AssertionError(f"unterminated method body: {signature}")


def _struct_body(source: str, name: str) -> str:
    marker = f"pub struct {name}"
    start = source.find(marker)
    require(start >= 0, f"missing struct: {name}")
    brace = source.find("{", start)
    require(brace >= 0, f"missing struct body brace: {name}")
    depth = 0
    for index in range(brace, len(source)):
        char = source[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return source[brace + 1 : index]
    raise AssertionError(f"unterminated struct: {name}")


def _field_type(struct_body: str, field: str) -> str:
    pattern = re.compile(rf"pub\s+{re.escape(field)}\s*:\s*(?P<ty>[^,\n]+)")
    match = pattern.search(struct_body)
    require(match is not None, f"missing field: {field}")
    return re.sub(r"\s+", " ", match.group("ty").strip())


def extract_plan() -> dict[str, Any]:
    types = _read(TYPES_RS)
    impl = _read(IMPL_RS)

    module_body = _struct_body(types, "MirModule")
    metadata_body = _struct_body(types, "ModuleMetadata")
    constructor = _method_body(impl, "pub fn new(name: String) -> Self")

    fields = {
        "name": _field_type(module_body, "name"),
        "functions": _field_type(module_body, "functions"),
        "globals": _field_type(module_body, "globals"),
        "metadata": _field_type(module_body, "metadata"),
    }
    require(fields["name"] == "String", "MirModule.name type drift")
    require(fields["functions"] == "BTreeMap<String", "MirModule.functions type drift")
    # The simple extractor stops at the first comma inside generics. Keep an
    # explicit source marker check for the full type to avoid normalizing Rust.
    require("pub functions: BTreeMap<String, MirFunction>," in module_body, "functions map type drift")
    require("pub globals: HashMap<String, ConstValue>," in module_body, "globals map type drift")
    require(fields["metadata"] == "ModuleMetadata", "MirModule.metadata type drift")

    for marker in [
        "name,",
        "functions: BTreeMap::new(),",
        "globals: HashMap::new(),",
        "metadata: ModuleMetadata::default(),",
    ]:
        require(marker in constructor, f"MirModule::new constructor marker drift: {marker}")

    require("#[derive(Debug, Clone, Default)]" in types, "ModuleMetadata default derive drift")
    require("pub source_file: Option<String>," in metadata_body, "metadata source_file type drift")

    return {
        "schema_version": 0,
        "kind": "MirModuleMinimalShellTransportPlanV1",
        "subject": "hakorune_mir::MirModule::new",
        "source_authority": {
            "constructor": "src/mir/function/module_impl.rs::MirModule::new",
            "shape": "src/mir/function/types.rs::MirModule",
        },
        "constructor_signature": {
            "parameter": {"name": "name", "rust_type": "String", "transport": "ModuleNameStringAtom"},
            "return": {"rust_type": "MirModule", "transport": "MirModuleMinimalShell"},
        },
        "shell_fields": [
            {
                "field": "name",
                "rust_type": "String",
                "initializer": "parameter:name",
                "transport": "ModuleNameStringAtom",
            },
            {
                "field": "functions",
                "rust_type": "BTreeMap<String, MirFunction>",
                "initializer": "BTreeMap::new",
                "transport": "EmptyFunctionTable",
            },
            {
                "field": "globals",
                "rust_type": "HashMap<String, ConstValue>",
                "initializer": "HashMap::new",
                "transport": "EmptyGlobalConstTable",
            },
            {
                "field": "metadata",
                "rust_type": "ModuleMetadata",
                "initializer": "ModuleMetadata::default",
                "transport": "ModuleMetadataDefaultShell",
            },
        ],
        "metadata_default_observations": {
            "source_file": None,
            "source_file_transport": "AbsentSourceFile",
            "metadata_mutation_claim": 0,
        },
        "directability": {
            "capability": "MirModuleMinimalShellTransport",
            "decision": "Available",
            "scope": "constructor shell only",
        },
        "non_claims": {
            "source_file_assignment": 0,
            "function_insertion": 0,
            "global_publication": 0,
            "metadata_publication": 0,
            "finalize_module": 0,
            "full_mirbuilder_new": 0,
            "generated_hako_artifact": 0,
            "backend_route_changed": 0,
            "abi_changed": 0,
            "runtime_fallback": 0,
        },
    }


def validate_plan(plan: dict[str, Any]) -> None:
    require(plan["kind"] == "MirModuleMinimalShellTransportPlanV1", "wrong module shell plan kind")
    require(
        plan["directability"]["capability"] == "MirModuleMinimalShellTransport",
        "module shell capability drift",
    )
    fields = {row["field"]: row for row in plan["shell_fields"]}
    require(fields["name"]["initializer"] == "parameter:name", "module name initializer drift")
    require(fields["functions"]["initializer"] == "BTreeMap::new", "functions initializer drift")
    require(fields["globals"]["initializer"] == "HashMap::new", "globals initializer drift")
    require(fields["metadata"]["initializer"] == "ModuleMetadata::default", "metadata initializer drift")
    require(plan["metadata_default_observations"]["source_file"] is None, "source_file default drift")
    for key, value in plan["non_claims"].items():
        require(value == 0, f"non-claim must remain 0: {key}")


def run_drift_probes(plan: dict[str, Any]) -> None:
    probes: list[tuple[str, list[Any], Any]] = [
        ("function table initializer drift", ["shell_fields", 1, "initializer"], "HashMap::new"),
        ("metadata default drift", ["shell_fields", 3, "initializer"], "ModuleMetadata::custom"),
        ("source file claim drift", ["non_claims", "source_file_assignment"], 1),
    ]
    for label, path, value in probes:
        mutated = deepcopy(plan)
        cursor: Any = mutated
        for key in path[:-1]:
            cursor = cursor[key]
        cursor[path[-1]] = value
        try:
            validate_plan(mutated)
        except AssertionError:
            continue
        raise AssertionError(f"drift probe did not fail: {label}")


def build_plan() -> dict[str, Any]:
    plan = extract_plan()
    validate_plan(plan)
    return plan


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--reference", type=Path, default=FIXTURE)
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    parser.add_argument("--drift-probes", action="store_true")
    args = parser.parse_args()

    plan = build_plan()
    if args.drift_probes:
        run_drift_probes(plan)

    return report_or_emit(
        facts=plan,
        reference=args.reference,
        check_reference=args.check_reference,
        emit_json=args.emit_json,
        report=[
            ("output_contract", "rust-lifecycle-mir-module-minimal-shell-transport-v0"),
            ("mir_module_minimal_shell_transport", "green"),
            ("capability", plan["directability"]["capability"]),
            ("source_file_assignment_claim", "0"),
            ("function_insertion_claim", "0"),
            ("generated_hako_artifact", "0"),
            ("backend_behavior_changed", "0"),
            ("runtime_fallback", "0"),
        ],
    )


if __name__ == "__main__":
    raise SystemExit(main())
