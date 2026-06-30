#!/usr/bin/env python3
"""Resolve CallLowering unified-call mode gate projection policy."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
FEATURE_POLICY = FIXTURES / "mirbuilder-call-lowering-feature-predicates-projection-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-call-lowering-unified-call-mode-gate-projection-policy-v0.json"
SUBCLUSTER_ID = "UnifiedCallModeGate"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def source_markers(source_text: str, config_text: str) -> list[str]:
    candidates = [
        "builder_unified_call_mode",
        "default ON during development; explicit opt-out supported",
        '"0"',
        '"false"',
        '"off"',
        "NYASH_MIR_UNIFIED_CALL",
        "env_string(\"NYASH_MIR_UNIFIED_CALL\")",
    ]
    haystack = source_text + "\n" + config_text
    return [marker for marker in candidates if marker in haystack]


def build_policy() -> dict[str, Any]:
    feature_policy = read_json(FEATURE_POLICY)
    surfaces = [
        surface for surface in feature_policy["source_surfaces"]
        if surface["feature_subcluster_id"] == SUBCLUSTER_ID
    ]
    if len(surfaces) != 1 or surfaces[0]["symbol"] != "is_unified_call_enabled":
        raise SystemExit(f"unexpected UnifiedCallModeGate surfaces: {surfaces}")

    surface = surfaces[0]
    source_text = read_source(surface["source_path"])
    config_path = "src/config/env/builder_flags.rs"
    config_text = read_source(config_path)

    return {
        "schema_version": 0,
        "kind": "MirBuilderCallLoweringUnifiedCallModeGateProjectionPolicyV1",
        "token": "MIRBUILDER-CALL-LOWERING-UNIFIED-CALL-MODE-GATE-PROJECTION-POLICY-001",
        "input_state": {
            "feature_predicates_policy": rel(FEATURE_POLICY),
            "selected_feature_subcluster_id": SUBCLUSTER_ID,
            "source_count": len(surfaces),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.config_gate",
            "borrow_axis": "NoBorrow",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "source_surface": {
            "source_id": surface["source_id"],
            "symbol": surface["symbol"],
            "source_path": surface["source_path"],
            "params": surface["params"],
            "return_type": surface["return_type"],
            "config_path": config_path,
            "config_accessor": "builder_unified_call_mode",
            "env_var": "NYASH_MIR_UNIFIED_CALL",
            "source_markers": source_markers(source_text, config_text),
        },
        "selected_policy": {
            "policy": "KeepParentConfigGate",
            "owner_edge": "mirbuilder::call_lowering_unified_call_mode_gate",
            "config_authority": "src/config/env/builder_flags.rs::builder_unified_call_mode",
            "projection_surface_selected": False,
            "hako_config_gate_selected": False,
            "new_env_flag_selected": False,
            "reason_token": "UnifiedCallModeGateIsConfigAuthorityOwned",
        },
        "decision": {
            "kind": "KeepParentOwner",
            "selected_next_card": "MIRBUILDER-CALL-LOWERING-PURE-METHOD-CATALOG-POLICY-001",
            "reason_token": "ConfigGateDoesNotOpenStandaloneProjectionOwner",
        },
        "claims": {
            "manual_family_selection": 0,
            "projection_surface_selected": 0,
            "hako_config_gate_selected": 0,
            "new_env_flag": 0,
            "runtime_or_projection_policy_by_name": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "native_seed_materialization": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in policy fixture.")
    args = parser.parse_args()

    output = stable_json(build_policy())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-call-lowering-unified-call-mode-gate-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
