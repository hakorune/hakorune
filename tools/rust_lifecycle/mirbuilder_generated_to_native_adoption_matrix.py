#!/usr/bin/env python3
"""Report the generated-to-native adoption matrix for MirBuilder family slices."""

from __future__ import annotations

import argparse
import json
import subprocess
import sys
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
ROUTE_MANIFEST = ROOT / "lang/generated/rust_derived/hakorune_mir_builder/family_routes.json"


def _load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def _run_guard(script: str) -> None:
    command = script if script.startswith("bash ") else f"bash {script}"
    result = subprocess.run(command, cwd=ROOT, capture_output=True, text=True, shell=True, check=False)
    if result.returncode != 0:
        raise SystemExit(result.stdout + result.stderr)


def _route_for(artifact_manifest: str) -> dict[str, Any]:
    routes = _load_json(ROUTE_MANIFEST)
    for route in routes["routes"]:
        if route["artifact_manifest"] == artifact_manifest:
            return route
    raise SystemExit(f"missing route for artifact: {artifact_manifest}")


def _require_native_source(path: Path) -> None:
    if not path.exists():
        raise SystemExit(f"missing native source: {path}")


def _report_row(
    prefix: str,
    *,
    generated_route: dict[str, Any],
    generated_route_guard: str,
    native_source: Path,
    native_guard: str,
) -> None:
    _require_native_source(native_source)
    _run_guard(generated_route_guard)
    _run_guard(native_guard)
    print(f"{prefix}.generated_route={generated_route['route']}")
    print(f"{prefix}.route_state={generated_route['state']}")
    print(f"{prefix}.selected_on_mainline={int(bool(generated_route.get('selected_on_mainline', False)))}")
    print(f"{prefix}.native_source=green")
    print(f"{prefix}.native_behavior_exe_guard=green")
    print(f"{prefix}.source_selfhost_claim=0")


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true", help="kept for symmetry; no alternate mode")
    args = parser.parse_args()
    if args.check:
        pass

    binding_route = _route_for("lang/generated/rust_derived/hakorune_mir_builder/binding_context.artifact.json")
    simple_map_route = _route_for("lang/generated/rust_derived/hakorune_mir_builder/variable_context_simple_map.artifact.json")
    snapshot_restore_route = _route_for("lang/generated/rust_derived/hakorune_mir_builder/variable_context_snapshot_restore.artifact.json")
    carrier_snapshot_route = _route_for("lang/generated/rust_derived/hakorune_mir_builder/variable_context_carrier_snapshot.artifact.json")
    explicit_carrier_route = _route_for("lang/generated/rust_derived/hakorune_mir_builder/variable_context_explicit_carrier_snapshot.artifact.json")
    prepared_state_route_selection = _load_json(
        ROOT / "lang/generated/rust_derived/hakorune_mir_builder/mirbuilder_next_value_id_prepared_state_kernel.route.json"
    )
    prepared_state_route = {
        "route": prepared_state_route_selection["profiles"]["selfhost_mainline"]["route"],
        "state": prepared_state_route_selection["artifact"]["state"],
        "selected_on_mainline": True,
    }

    _report_row(
        "binding_context",
        generated_route=binding_route,
        generated_route_guard="bash tools/checks/rust_lifecycle_binding_context_derived_route_selection_guard.sh",
        native_source=ROOT / "apps/lib/hakorune_mir_builder/binding_context.hako",
        native_guard="bash tools/checks/rust_mirbuilder_binding_context_native_guard.sh",
    )
    _report_row(
        "variable_context_simple_map",
        generated_route=simple_map_route,
        generated_route_guard=simple_map_route["guard_command"],
        native_source=ROOT / "apps/lib/hakorune_mir_builder/variable_context.hako",
        native_guard="bash tools/checks/rust_mirbuilder_variable_context_native_simple_map_guard.sh",
    )
    _report_row(
        "variable_context_snapshot_restore",
        generated_route=snapshot_restore_route,
        generated_route_guard=snapshot_restore_route["guard_command"],
        native_source=ROOT / "apps/lib/hakorune_mir_builder/variable_context.hako",
        native_guard="bash tools/checks/rust_mirbuilder_variable_context_native_snapshot_restore_guard.sh",
    )
    _report_row(
        "carrier_info_snapshot_apis",
        generated_route=carrier_snapshot_route,
        generated_route_guard=carrier_snapshot_route["guard_command"],
        native_source=ROOT / "apps/lib/hakorune_mir_builder/carrier_info.hako",
        native_guard="bash tools/checks/rust_mirbuilder_carrier_info_native_snapshot_guard.sh",
    )
    _report_row(
        "allocation_policy_prepared_state_next_value_id",
        generated_route=prepared_state_route,
        generated_route_guard="bash tools/checks/rust_lifecycle_mirbuilder_allocation_policy_mainline_pilot_guard.sh",
        native_source=ROOT / "lang/src/compiler/lib/next_value_id_prepared_state_kernel.hako",
        native_guard="bash tools/checks/rust_lifecycle_mirbuilder_allocation_policy_hako_adoption_decision_recheck_002_guard.sh",
    )
    _run_guard(explicit_carrier_route["guard_command"])
    print(f"carrier_info_snapshot_apis.explicit_generated_route={explicit_carrier_route['route']}")
    print(f"carrier_info_snapshot_apis.explicit_route_state={explicit_carrier_route['state']}")
    print(f"carrier_info_snapshot_apis.explicit_selected_on_mainline={int(bool(explicit_carrier_route['selected_on_mainline']))}")
    print(f"carrier_info_snapshot_apis.explicit_source_selfhost_claim=0")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
