#!/usr/bin/env python3
"""Classify MirBuilder migration artifacts by real compiler reachability."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-artifact-reachability-classification-inventory-v0.json"

TOKEN = "MIRBUILDER-ARTIFACT-REACHABILITY-CLASSIFICATION-INVENTORY-001"
NEXT_CARD = "MIRBUILDER-CURRENT-ACTIVE-RUST-LIFECYCLE-GUARD-RESOLVER-001"

COMPARE_BRIDGES = [
    ROOT / "src/mir/builder/compare_branch_emission_bridge.rs",
    ROOT / "src/mir/builder/compare_localssa_finalize_compare_bridge.rs",
    ROOT / "src/mir/builder/compare_mir_compare_emission_bridge.rs",
    ROOT / "src/mir/builder/compare_rhs_symbolref_contract.rs",
    ROOT / "src/mir/builder/compare_rhs_symbolref_lookup_bridge.rs",
    ROOT / "src/mir/builder/compare_rhs_valueid_resolution_bridge.rs",
]

LIVE_FASTPATH_EXAMPLES = [
    ROOT / "src/mir/generic_method_route_plan/write_routes.rs",
    ROOT / "src/mir/generic_method_route_plan/collection_read_routes.rs",
    ROOT / "src/mir/generic_method_route_plan/string_routes.rs",
    ROOT / "src/mir/route_value_type_publication.rs",
    ROOT / "src/mir/builder/control_flow/joinir/route_entry/runtime_adjacent_shadow_guard.rs",
    ROOT / "src/mir/global_call_route_plan/same_module_static_helper_contract.rs",
]

COMPILER_ENTRY = ROOT / "lang/src/compiler/entry/compiler.hako"
COMPILER_ROOT = ROOT / "lang/src/compiler"
COMPILER_LIB = COMPILER_ROOT / "lib"
BUILDER_RS = ROOT / "src/mir/builder.rs"
COMPARISON_RS = ROOT / "src/mir/builder/ops/comparison.rs"
DEV_GATE = ROOT / "tools/checks/dev_gate.sh"
DEV_GATE_QUICK = ROOT / "tools/checks/lib/dev_gate_quick_steps.sh"
GUARD_ROWS = ROOT / "tools/checks/guard_rows.toml"
PROOF_APPS = ROOT / "tools/checks/proof_apps.toml"
WORKFLOWS = ROOT / ".github/workflows"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def line_count(path: Path) -> int:
    if not path.exists():
        return 0
    return len(read(path).splitlines())


def text_count(path: Path, needle: str) -> int:
    if not path.exists():
        return 0
    return read(path).count(needle)


def tree_text_count(root: Path, pattern: str, needle: str) -> int:
    if not root.exists():
        return 0
    return sum(text_count(path, needle) for path in root.rglob(pattern) if path.is_file())


def rust_lifecycle_guard_counts() -> dict[str, int]:
    scripts = list((ROOT / "tools/checks").glob("rust_lifecycle*.sh"))
    return {
        "script_count": len(scripts),
        "guard_count": len([p for p in scripts if p.name.endswith("_guard.sh")]),
        "gate_count": len([p for p in scripts if p.name.endswith("_gate.sh")]),
        "parity_gate_count": len([p for p in scripts if p.name.endswith("_parity_gate.sh")]),
    }


def hako_module_to_path(module: str) -> Path | None:
    prefix = "lang.compiler."
    if not module.startswith(prefix):
        return None
    rel_module = module[len(prefix) :].replace(".", "/")
    path = COMPILER_ROOT / f"{rel_module}.hako"
    return path if path.exists() else None


def reachable_hako_from_compiler() -> set[Path]:
    seen: set[Path] = set()
    stack = [COMPILER_ENTRY]
    using_re = re.compile(r"^\s*using\s+([A-Za-z0-9_.]+)", re.MULTILINE)
    while stack:
        path = stack.pop()
        if path in seen or not path.exists():
            continue
        seen.add(path)
        for module in using_re.findall(read(path)):
            next_path = hako_module_to_path(module)
            if next_path is not None and next_path not in seen:
                stack.append(next_path)
    return seen


def compare_bridge_rows() -> list[dict[str, Any]]:
    builder = read(BUILDER_RS)
    comparison = read(COMPARISON_RS)
    rows = []
    for path in COMPARE_BRIDGES:
        module = path.stem
        exists = path.exists()
        rows.append(
            {
                "path": rel(path),
                "exists": exists,
                "line_count": line_count(path),
                "builder_mod_declared": f"mod {module};" in builder,
                "production_fastpath_reference_count": comparison.count(module),
                "classification": "deleted_proof_bridge"
                if not exists
                else "proof_only_rust_bridge",
            }
        )
    return rows


def hako_mirror_inventory() -> dict[str, Any]:
    lib_files = sorted(COMPILER_LIB.glob("*.hako"))
    reached = reachable_hako_from_compiler()
    reached_lib = sorted(path for path in reached if COMPILER_LIB in path.parents)
    guard_refs = {
        rel(path): tree_text_count(ROOT / "tools/checks", "*.sh", rel(path))
        + tree_text_count(ROOT / "tools/checks", "*.sh", path.stem)
        for path in lib_files
    }
    guard_referenced = [path for path in lib_files if guard_refs.get(rel(path), 0) > 0]
    mirror_marked = [
        path
        for path in lib_files
        if "classifier_policy_mirror_only" in read(path)
        or "policy mirror" in read(path)
        or "Rust-oracle" in read(path)
    ]
    return {
        "compiler_entry": rel(COMPILER_ENTRY),
        "compiler_reachable_hako_count": len(reached),
        "compiler_reachable_lib_count": len(reached_lib),
        "compiler_reachable_lib_paths": [rel(path) for path in reached_lib],
        "lib_hako_count": len(lib_files),
        "guard_referenced_lib_hako_count": len(guard_referenced),
        "mirror_marked_lib_hako_count": len(mirror_marked),
        "classification": "shadow_mirror_library",
    }


def live_fastpath_rows() -> list[dict[str, Any]]:
    rows = []
    for path in LIVE_FASTPATH_EXAMPLES:
        rows.append(
            {
                "path": rel(path),
                "exists": path.exists(),
                "line_count": line_count(path) if path.exists() else 0,
                "classification": "live_fastpath_or_route_plan_owner",
            }
        )
    return rows


def build_fixture() -> dict[str, Any]:
    compare_rows = compare_bridge_rows()
    hako_inventory = hako_mirror_inventory()
    guard_counts = rust_lifecycle_guard_counts()
    source_files = {
        rel(path): sha256_file(path)
        for path in [
            BUILDER_RS,
            COMPARISON_RS,
            DEV_GATE,
            DEV_GATE_QUICK,
            GUARD_ROWS,
            PROOF_APPS,
            *COMPARE_BRIDGES,
            *LIVE_FASTPATH_EXAMPLES,
            COMPILER_ENTRY,
        ]
        if path.exists()
    }
    deleted_compare_count = len([row for row in compare_rows if not row["exists"]])
    live_compare_count = len([row for row in compare_rows if row["exists"]])

    return {
        "schema_version": 0,
        "kind": "MirBuilderArtifactReachabilityClassificationInventoryV1",
        "token": TOKEN,
        "source_files": source_files,
        "artifact_classes": {
            "live_fastpath": live_fastpath_rows(),
            "proof_only_rust_bridge": compare_rows,
            "shadow_mirror_library": hako_inventory,
            "unreached_guard_ecosystem": {
                **guard_counts,
                "dev_gate_rust_lifecycle_refs": text_count(DEV_GATE, "rust_lifecycle"),
                "dev_gate_quick_rust_lifecycle_refs": text_count(DEV_GATE_QUICK, "rust_lifecycle"),
                "guard_rows_rust_lifecycle_refs": text_count(GUARD_ROWS, "rust_lifecycle"),
                "proof_apps_rust_lifecycle_refs": text_count(PROOF_APPS, "rust_lifecycle"),
                "workflow_rust_lifecycle_refs": tree_text_count(WORKFLOWS, "*.yml", "rust_lifecycle")
                + tree_text_count(WORKFLOWS, "*.yaml", "rust_lifecycle"),
                "classification": "unreached_historical_guard_set",
            },
        },
        "summary": {
            "artifact_reachability_classification_inventory": 1,
            "live_fastpath_owner_examples_count": len(LIVE_FASTPATH_EXAMPLES),
            "compare_proof_bridge_file_count": len(compare_rows),
            "compare_proof_bridge_deleted_file_count": deleted_compare_count,
            "compare_proof_bridge_live_file_count": live_compare_count,
            "compare_proof_bridge_total_lines": sum(row["line_count"] for row in compare_rows),
            "compare_proof_bridge_production_connected": 0,
            "hako_lib_compiler_reachable_count": hako_inventory["compiler_reachable_lib_count"],
            "hako_mirror_library_fastpath_connected": 0,
            "rust_lifecycle_guard_script_count": guard_counts["script_count"],
            "run_all_rust_lifecycle_guards_by_default": 0,
            "active_guard_resolver_required": 1,
            "source_selfhost_claim": 0,
        },
        "decision": {
            "kind": "SelectActiveGuardResolverBeforeShadowConsume",
            "reason_token": "ReachabilityMixedClosedWorldArtifacts",
            "selected_next_card": NEXT_CARD,
        },
        "claims": {
            "inventory_only": 1,
            "compare_bridge_deleted": 0,
            "compare_bridge_production_connected": 0,
            "hako_runtime_route_authority": 0,
            "hako_backend_lowering_authority": 0,
            "all_rust_lifecycle_guards_in_ci": 0,
            "all_rust_lifecycle_guards_in_dev_gate": 0,
            "rust_fastpath_rewired": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "source_selfhost_claim": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-artifact-reachability-classification-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
