#!/usr/bin/env python3
"""Validate the finite force-hv1 design-stop sidecar and its owner pointer."""

from __future__ import annotations

import json
import re
import sys
import tomllib
from pathlib import Path

from force_hv1_leaf_census import derive_inventory


TAG = "force-hv1-caller-manifest-guard"


def fail(message: str) -> None:
    raise SystemExit(f"[{TAG}] result_class=current-change failure status=fail: {message}")


if len(sys.argv) != 5:
    fail("usage: force_hv1_caller_manifest_guard.py CARD MANIFEST STATE WORKSTREAM")

card_path, manifest_path, state_path, workstream_path = map(Path, sys.argv[1:])
for path in (card_path, manifest_path, state_path, workstream_path):
    if not path.is_file():
        fail(f"required owner missing: {path}")

with card_path.open("rb") as stream:
    card = tomllib.load(stream)
with state_path.open("rb") as stream:
    state = tomllib.load(stream)
manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
workstream = workstream_path.read_text(encoding="utf-8")

root = card_path.parents[5]
relative_card = card_path.relative_to(root).as_posix()
relative_manifest = manifest_path.relative_to(root).as_posix()
fate = card.get("force_hv1_fate_d0")
if not isinstance(fate, dict):
    fail("active card force_hv1_fate_d0 section is missing")
if card.get("implementation_permission") is not False:
    fail("force-hv1 fate implementation permission must remain false")
if fate.get("manifest_path") != relative_manifest:
    fail("active card manifest path drifted")
if fate.get("guard_path") != "tools/checks/lib/force_hv1_caller_manifest_guard.py":
    fail("active card manifest guard path drifted")
if manifest.get("Exception") != "machine-consumed stable artifact":
    fail("manifest exception marker missing")
if manifest.get("ParentCurrentCard") != relative_card:
    fail("manifest parent card drifted")
if not re.fullmatch(r"[0-9a-f]{40}", str(manifest.get("source_commit", ""))):
    fail("manifest source_commit is not a full commit id")

if state.get("current_execution_row") == "FORCE-HV1-CENSUS-PER-LEAF-SCHEMA-S0":
    if state.get("work_mode") != "fast":
        fail("census S0 requires CURRENT_STATE work_mode=fast")
    if fate.get("census_s0_status") != "fast_open":
        fail("census S0 status is not fast_open")
    if fate.get("census_s0_implementation_permission") is not True:
        fail("census S0 scoped permission is not set")
    if manifest.get("schema") != "force-hv1-caller-disposition-manifest-v1":
        fail("census S0 requires the v1 manifest")
    leaf_paths = manifest.get("leaf_paths")
    observations = manifest.get("observations")
    if not isinstance(leaf_paths, list) or not all(isinstance(item, str) for item in leaf_paths):
        fail("v1 leaf_paths inventory is missing")
    if not isinstance(observations, dict):
        fail("v1 observations are missing")
    try:
        derived = derive_inventory(root, leaf_paths)
    except ValueError as error:
        fail(str(error))
    if set(observations) != set(leaf_paths):
        fail("v1 observations do not cover exactly the leaf inventory")
    for item in derived:
        if observations.get(item["path"]) != item:
            fail(f"body-derived observation drifted: {item['path']}")
    observed_counts = manifest.get("observed_counts")
    if observed_counts != {
        "lexical_leaves": 116,
        "lexical_sites": 120,
        "route_class": {
            "DirectForceSealed": 33,
            "HelperForceConditional": 44,
            "ExplicitCoreResidualSealed": 35,
            "DynamicArtifactOpen": 4,
        },
        "route_sites": {
            "DirectForceSealed": 33,
            "HelperForceConditional": 45,
            "ExplicitCoreResidualSealed": 35,
            "DynamicArtifactOpen": 7,
        },
    }:
        fail("v1 observed counts are not the reviewed body-derived matrix")
    print(
        f"[{TAG}] result_class=current-change failure status=pass "
        "phase=force_hv1_census_s0 leaves=116 sites=120 "
        "direct=33/33 conditional=44/45 explicit_core=35/35 dynamic=4/7 "
        "fate=unmodified implementation=fast_open"
    )
    raise SystemExit(0)

if state.get("work_mode") != "design_stop":
    fail("force-hv1 fate must remain design_stop")
if state.get("current_execution_row") != "MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-D0-FORCE-HV1-FATE":
    fail("force-hv1 fate current row drifted")

expected = {
    "lexical_leaves": 116,
    "force_reachers": 78,
    "force_invocations": 80,
    "residual_consumers": 38,
    "migration_blockers": 9,
    "unclassified": 0,
}
counts = manifest.get("expected_counts")
if not isinstance(counts, dict):
    fail("expected_counts is missing")
for key, value in expected.items():
    if counts.get(key) != value:
        fail(f"expected count drifted: {key}")
if counts.get("caller_family") != {"direct": 33, "textual_helper": 74, "wrapper_only": 9}:
    fail("caller-family count drifted")
if counts.get("fate") != {
    "HistoricalDelete": 69,
    "ProductAot": 8,
    "Stage1ProofAot": 1,
    "SemanticReference": 0,
    "ParkedSealedResidual": 38,
}:
    fail("fate count drifted")

groups = manifest.get("groups")
if not isinstance(groups, list) or not groups:
    fail("manifest groups are missing")
paths: list[str] = []
family_counts: dict[str, int] = {}
fate_counts: dict[str, int] = {}
force_reachers = force_invocations = residual = 0
for group in groups:
    required = {"caller_family", "invocation_count", "reachability", "residual_kind", "fate", "fate_contract", "paths"}
    if not required.issubset(group):
        fail("a manifest group is missing required fields")
    group_paths = group["paths"]
    if not isinstance(group_paths, list) or not group_paths or not all(isinstance(item, str) for item in group_paths):
        fail("a manifest group has invalid paths")
    if group["invocation_count"] not in {1, 2}:
        fail("manifest invocation count is outside the bounded census")
    if group["reachability"] not in {"force_reacher", "nonforce_residual"}:
        fail("manifest reachability enum drifted")
    if group["fate_contract"] != group["fate"]:
        fail("manifest fate contract is not explicit")
    paths.extend(group_paths)
    family_counts[group["caller_family"]] = family_counts.get(group["caller_family"], 0) + len(group_paths)
    fate_counts[group["fate"]] = fate_counts.get(group["fate"], 0) + len(group_paths)
    if group["reachability"] == "force_reacher":
        force_reachers += len(group_paths)
        force_invocations += len(group_paths) * group["invocation_count"]
    else:
        residual += len(group_paths)

if len(paths) != 116 or len(set(paths)) != 116:
    fail("manifest paths are not exactly 116 unique leaves")
missing = [item for item in paths if not (root / item).is_file()]
if missing:
    fail("manifest names missing files: " + ", ".join(missing[:3]))
if family_counts != {"direct": 33, "textual_helper": 74, "wrapper_only": 9}:
    fail("derived caller-family census drifted")
if (force_reachers, force_invocations, residual) != (78, 80, 38):
    fail("derived reachability census drifted")
if fate_counts != {"HistoricalDelete": 69, "Stage1ProofAot": 1, "ProductAot": 8, "ParkedSealedResidual": 38}:
    fail("derived fate census drifted")

owner_paths = {item.get("path") for item in manifest.get("owner_inventory", []) if isinstance(item, dict)}
expected_owner_paths = {
    "src/main.rs",
    "src/runner/hv1_inline.rs",
    "src/runner/mod.rs",
    "src/lib.rs",
    "tools/smokes/v2/lib/test_runner.sh",
    "tools/smokes/v2/lib/test_runner_builder_helpers.sh",
    "tools/smokes/v2/lib/test_runner_stdout_core_helpers.sh",
    "tools/smokes/v2/profiles/integration/core/phase2170/run_all.sh",
}
if owner_paths != expected_owner_paths:
    fail("force-hv1 owner inventory drifted")
aggregate = manifest.get("aggregate_rows")
if not isinstance(aggregate, list) or len(aggregate) != 1 or aggregate[0].get("path") not in expected_owner_paths:
    fail("phase2170 aggregate owner is missing")
for marker in ("116 lexical leaves", "78 force reachers / 80 invocations", "non-force residual consumers", "migration blockers=9"):
    if marker not in workstream:
        fail(f"workstream is missing corrected force-hv1 marker: {marker}")
print(f"[{TAG}] result_class=current-change failure status=pass leaves=116 force_reachers=78 invocations=80 residual=38 blockers=9 implementation=forbidden")
