#!/usr/bin/env python3
"""Validate the finite force-hv1 design-stop sidecar and its owner pointer."""

from __future__ import annotations

import json
import hashlib
import re
import sys
import tomllib
from pathlib import Path

from force_hv1_leaf_census import derive_inventory, derive_summary


TAG = "force-hv1-caller-manifest-guard"


def fail(message: str) -> None:
    raise SystemExit(f"[{TAG}] result_class=current-change failure status=fail: {message}")


if len(sys.argv) != 6:
    fail("usage: force_hv1_caller_manifest_guard.py CARD MANIFEST STATE WORKSTREAM PHASE")

card_path, manifest_path, state_path, workstream_path = map(Path, sys.argv[1:5])
requested_phase = sys.argv[5]
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

if requested_phase == "force_hv1_guard_current_lifecycle":
    row = "FORCE-HV1-GUARD-CURRENT-LIFECYCLE-I0"
    lifecycle_open = state.get("current_execution_row") == row
    if lifecycle_open:
        if state.get("work_mode") != "fast":
            fail("current lifecycle I0 requires CURRENT_STATE work_mode=fast")
        if state.get("next_execution_card") != row:
            fail("current lifecycle I0 next execution card drifted")
        if card.get("status") != "force_hv1_guard_current_lifecycle_i0_fast_open":
            fail("active card is not marked current lifecycle I0 fast-open")
        if "FORCE-HV1-GUARD-CURRENT-LIFECYCLE-I0 fast-open" not in str(
            card.get("permission_scope", "")
        ):
            fail("current lifecycle I0 permission scope is not narrow and explicit")
        if state.get("current_design_stop"):
            fail("current lifecycle I0 fast row must not retain a design stop")
    else:
        if state.get("work_mode") != "design_stop":
            fail("current lifecycle I0 closeout requires design_stop")
        if state.get("current_execution_row") != "MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-D0-FORCE-HV1-FATE":
            fail("current lifecycle I0 closeout must return to the force-hv1 fate row")
        if not str(state.get("next_execution_card", "")).startswith("none"):
            fail("current lifecycle I0 closeout requires no execution card")
        if card.get("status") != "direct_historical_delete_r0_landed":
            fail("current lifecycle I0 closeout requires the landed R0a card")
        if "current lifecycle I0 is landed" not in workstream:
            fail("current lifecycle I0 closeout receipt is missing")
    if fate.get("direct_historical_delete_r0_status") != "landed":
        fail("current lifecycle I0 requires the landed R0a inventory")

    retired = manifest.get("retired_inventory")
    if not isinstance(retired, dict) or retired.get("status") != "r0a_landed":
        fail("current lifecycle I0 requires the landed retired inventory")
    records = retired.get("records")
    if not isinstance(records, list) or len(records) != 30:
        fail("current lifecycle I0 requires exactly 30 retired records")
    retired_paths = {record.get("path") for record in records if isinstance(record, dict)}
    if len(retired_paths) != 30 or None in retired_paths:
        fail("current lifecycle I0 retired inventory is incomplete or duplicated")
    if any((root / path).exists() for path in retired_paths):
        fail("current lifecycle I0 retired path reappeared")

    leaf_paths = manifest.get("leaf_paths")
    observations = manifest.get("observations")
    if not isinstance(leaf_paths, list) or len(leaf_paths) != 86:
        fail("current lifecycle I0 requires the active 86-leaf inventory")
    if not isinstance(observations, dict) or set(observations) != set(leaf_paths):
        fail("current lifecycle I0 active observations are incomplete")
    try:
        derived = derive_inventory(root, leaf_paths)
    except ValueError as error:
        fail(str(error))
    for item in derived:
        if observations.get(item["path"]) != item:
            fail(f"current lifecycle I0 body-derived observation drifted: {item['path']}")
    summary = derive_summary(derived)
    expected_summary = {
        "lexical_leaves": 86,
        "lexical_sites": 90,
        "route_class": {
            "DirectForceSealed": 3,
            "HelperForceConditional": 44,
            "ExplicitCoreResidualSealed": 35,
            "DynamicArtifactOpen": 4,
        },
        "route_sites": {
            "DirectForceSealed": 3,
            "HelperForceConditional": 45,
            "ExplicitCoreResidualSealed": 35,
            "DynamicArtifactOpen": 7,
        },
    }
    if summary != expected_summary or manifest.get("observed_counts") != summary:
        fail("current lifecycle I0 body-derived summary drifted")
    if retired.get("original_active_leaves") != 116 or retired.get("retired_leaves") != 30:
        fail("current lifecycle I0 retirement arithmetic drifted")
    if retired.get("active_leaves_after_cutover") != 86 or retired.get("active_sites_after_cutover") != 90:
        fail("current lifecycle I0 active arithmetic drifted")
    if retired_paths & set(leaf_paths):
        fail("current lifecycle I0 active and retired inventories overlap")

    expected_direct = {
        "tools/smokes/v2/profiles/integration/core/phase2050/flow_phi2_select_by_pred_rc99_primary_canary_vm.sh",
        "tools/smokes/v2/profiles/integration/core/phase2051/selfhost_v1_primary_rc42_canary_vm.sh",
        "tools/smokes/v2/profiles/integration/core/phase2051/selfhost_v1_provider_primary_rc42_canary_vm.sh",
    }
    direct_paths = {
        item["path"]
        for item in derived
        if item["derived"]["route_class"] == "DirectForceSealed"
    }
    if direct_paths != expected_direct:
        fail("current lifecycle I0 direct exception set drifted")
    groups = manifest.get("groups")
    if not isinstance(groups, list):
        fail("current lifecycle I0 manifest groups are missing")
    active_group_paths = [
        path for group in groups if isinstance(group, dict) for path in group.get("paths", [])
    ]
    if len(active_group_paths) != 86 or set(active_group_paths) != set(leaf_paths):
        fail("current lifecycle I0 groups do not cover exactly the active inventory")
    family_counts: dict[str, int] = {}
    for group in groups:
        if not isinstance(group, dict):
            fail("current lifecycle I0 manifest group is invalid")
        family = group.get("caller_family")
        family_counts[family] = family_counts.get(family, 0) + len(group.get("paths", []))
    if family_counts != {"direct": 3, "textual_helper": 74, "wrapper_only": 9}:
        fail("current lifecycle I0 caller-family summary drifted")
    if "SMOKE-OWNER-PACK-ZERO-MATCH-D0" not in workstream:
        fail("current lifecycle I0 workstream is missing the zero-match design row")
    if "FORCE-HV1-STAGE1-AOT-BOUNDARY-D0" not in workstream:
        fail("current lifecycle I0 workstream is missing the Stage1 boundary row")
    print(
        f"[{TAG}] result_class=current-change failure status=pass "
        "phase=force_hv1_guard_current_lifecycle_i0 active_leaves=86 "
        f"active_sites=90 retired=30 direct=3/3 implementation={'fast_open' if lifecycle_open else 'closed'}"
    )
    raise SystemExit(0)

if state.get("current_execution_row") == "FORCE-HV1-DIRECT-HISTORICAL-DELETE-R0":
    row = "FORCE-HV1-DIRECT-HISTORICAL-DELETE-R0"
    if state.get("work_mode") != "fast":
        fail("direct HistoricalDelete R0a requires CURRENT_STATE work_mode=fast")
    if state.get("next_execution_card") != row:
        fail("direct HistoricalDelete R0a next execution card drifted")
    if card.get("status") != "direct_historical_delete_r0_fast_open":
        fail("active card is not marked direct HistoricalDelete R0a fast-open")
    if fate.get("direct_historical_delete_r0_status") != "fast_open":
        fail("direct HistoricalDelete R0a scoped status is not fast_open")
    if fate.get("direct_historical_delete_r0_implementation_permission") is not True:
        fail("direct HistoricalDelete R0a scoped permission is not set")
    review = manifest.get("fate_review")
    if not isinstance(review, dict) or review.get("task_id") != row:
        fail("direct HistoricalDelete R0a fate review is missing")
    if review.get("status") != "design_accepted_candidate":
        fail("direct HistoricalDelete R0a fate review status drifted")
    if review.get("implementation_permission") is not False:
        fail("fate review must not expose implementation permission")
    family = review.get("reviewed_family")
    if not isinstance(family, dict) or family.get("path_count") != 30:
        fail("direct HistoricalDelete R0a reviewed family must contain 30 paths")
    groups = manifest.get("groups")
    if not isinstance(groups, list):
        fail("manifest groups are missing for direct HistoricalDelete R0a")
    matches = [
        group
        for group in groups
        if isinstance(group, dict)
        and group.get("caller_family") == "direct"
        and group.get("fate") == "HistoricalDelete"
    ]
    if len(matches) != 1:
        fail("direct HistoricalDelete R0a source group is not unique")
    group_paths = matches[0].get("paths")
    exceptions = set(review.get("exception_paths") or [])
    expected_exceptions = {
        "tools/smokes/v2/profiles/integration/core/phase2050/flow_phi2_select_by_pred_rc99_primary_canary_vm.sh",
        "tools/smokes/v2/profiles/integration/core/phase2051/selfhost_v1_provider_primary_rc42_canary_vm.sh",
    }
    if exceptions != expected_exceptions or not exceptions.issubset(set(group_paths or [])):
        fail("direct HistoricalDelete R0a exception paths drifted")
    candidate_paths = [path for path in group_paths if path not in exceptions]
    if len(candidate_paths) != 30:
        fail("direct HistoricalDelete R0a candidate set is not 30 paths")
    observations = manifest.get("observations")
    leaf_paths = manifest.get("leaf_paths")
    if not isinstance(observations, dict) or not isinstance(leaf_paths, list):
        fail("body-derived v1 inventory is missing for direct HistoricalDelete R0a")
    if set(observations) != set(leaf_paths):
        fail("body-derived observations do not cover the active leaf inventory")
    for path in candidate_paths + sorted(exceptions):
        if not (root / path).is_file():
            fail(f"direct HistoricalDelete R0a source leaf is missing: {path}")
    records = sorted((path, observations[path].get("body_sha256")) for path in candidate_paths)
    if any(not isinstance(digest, str) for _, digest in records):
        fail("direct HistoricalDelete R0a body digest is missing")
    digest = hashlib.sha256("".join(f"{path}\0{body}\n" for path, body in records).encode()).hexdigest()
    if family.get("body_digest_sha256") != digest:
        fail("direct HistoricalDelete R0a body digest drifted")
    projection_owners = review.get("projection_owners")
    if not isinstance(projection_owners, list) or len(projection_owners) != 7:
        fail("direct HistoricalDelete R0a projection owner inventory drifted")
    missing_projection = [path for path in projection_owners if not (root / path).is_file()
    ]
    if missing_projection:
        fail("direct HistoricalDelete R0a projection owner missing: " + ", ".join(missing_projection))
    try:
        derived = derive_inventory(root, leaf_paths)
    except ValueError as error:
        fail(str(error))
    for item in derived:
        if observations.get(item["path"]) != item:
            fail(f"body-derived observation drifted: {item['path']}")
    if manifest.get("observed_counts") != {
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
        fail("direct HistoricalDelete R0a must start from the landed 116/120 observation")
    print(
        f"[{TAG}] result_class=current-change failure status=pass "
        "phase=force_hv1_direct_historical_delete_r0 candidate=30 "
        "exceptions=2 active_leaves=116 active_sites=120 implementation=fast_open"
    )
    raise SystemExit(0)

if fate.get("direct_historical_delete_r0_status") == "guard_closeout_fast_open":
    row = "FORCE-HV1-R0A-GUARD-CLOSEOUT-I0"
    if state.get("work_mode") != "fast" or state.get("current_execution_row") != row:
        fail("R0a guard closeout requires its scoped fast pointer")
    if state.get("next_execution_card") != row:
        fail("R0a guard closeout next execution card drifted")
    if card.get("status") != "force_hv1_r0a_guard_closeout_fast_open":
        fail("active card is not marked R0a guard closeout fast-open")
    if fate.get("direct_historical_delete_r0_implementation_permission") is not True:
        fail("R0a guard closeout scoped permission is not set")
    retired = manifest.get("retired_inventory")
    if not isinstance(retired, dict) or retired.get("status") != "r0a_landed":
        fail("R0a guard closeout requires the landed retired inventory")
    if retired.get("retired_leaves") != 30 or retired.get("active_leaves_after_cutover") != 86:
        fail("R0a guard closeout arithmetic drifted")
    if len(retired.get("records") or []) != 30:
        fail("R0a guard closeout requires all 30 retired records")
    leaf_paths = manifest.get("leaf_paths")
    observations = manifest.get("observations")
    if not isinstance(leaf_paths, list) or len(leaf_paths) != 86:
        fail("R0a guard closeout requires the active 86-leaf inventory")
    if not isinstance(observations, dict) or set(observations) != set(leaf_paths):
        fail("R0a guard closeout active observations are incomplete")
    print(
        f"[{TAG}] result_class=current-change failure status=pass "
        "phase=force_hv1_r0a_guard_closeout_i0 active_leaves=86 "
        "retired=30 implementation=fast_open"
    )
    raise SystemExit(0)

if fate.get("direct_historical_delete_r0_status") == "landed":
    row = "FORCE-HV1-DIRECT-HISTORICAL-DELETE-R0"
    if state.get("work_mode") != "design_stop":
        fail("landed direct HistoricalDelete R0a must return CURRENT_STATE to design_stop")
    if state.get("current_execution_row") != "MIR-CALL-INGRESS-SCHEMA-SELECTOR-WPRE-D0-FORCE-HV1-FATE":
        fail("landed direct HistoricalDelete R0a current row must return to the force-hv1 fate design stop")
    if card.get("status") != "direct_historical_delete_r0_landed":
        fail("active card is not marked direct HistoricalDelete R0a landed")
    if fate.get("direct_historical_delete_r0_implementation_permission") is not False:
        fail("landed direct HistoricalDelete R0a scoped permission must be closed")
    review = manifest.get("fate_review")
    if not isinstance(review, dict) or review.get("task_id") != row or review.get("status") != "r0a_landed":
        fail("landed direct HistoricalDelete R0a fate review is missing")
    if review.get("implementation_permission") is not False:
        fail("landed fate review must not expose implementation permission")
    family = review.get("reviewed_family")
    if not isinstance(family, dict) or family.get("path_count") != 30:
        fail("landed direct HistoricalDelete R0a reviewed family must contain 30 paths")
    retired = manifest.get("retired_inventory")
    if not isinstance(retired, dict) or retired.get("status") != "r0a_landed":
        fail("retired R0a inventory is missing")
    if retired.get("task_id") != row:
        fail("retired R0a inventory task id drifted")
    if retired.get("original_active_leaves") != 116 or retired.get("retired_leaves") != 30:
        fail("retired R0a inventory arithmetic drifted")
    if retired.get("active_leaves_after_cutover") != 86 or retired.get("active_sites_after_cutover") != 90:
        fail("retired R0a active arithmetic drifted")
    records = retired.get("records")
    if not isinstance(records, list) or len(records) != 30:
        fail("retired R0a inventory must retain exactly 30 records")
    retired_paths = []
    for record in records:
        if not isinstance(record, dict) or set(("path", "body_sha256", "owner", "disposition")) - set(record):
            fail("retired R0a record is incomplete")
        if record.get("owner") != "force-hv1-retirement" or record.get("disposition") != "HistoricalDelete":
            fail("retired R0a record ownership drifted")
        retired_paths.append(record["path"])
        if (root / record["path"]).exists():
            fail(f"retired R0a source leaf still exists: {record['path']}")
    if len(set(retired_paths)) != 30:
        fail("retired R0a inventory contains duplicate paths")
    if retired.get("body_digest_sha256") != family.get("body_digest_sha256"):
        fail("retired R0a inventory digest does not match the reviewed family")
    observations = manifest.get("observations")
    leaf_paths = manifest.get("leaf_paths")
    if not isinstance(observations, dict) or not isinstance(leaf_paths, list):
        fail("post-R0a body-derived inventory is missing")
    if len(leaf_paths) != 86 or len(observations) != 86 or set(observations) != set(leaf_paths):
        fail("post-R0a active inventory must contain exactly 86 leaves")
    if set(retired_paths) & set(leaf_paths):
        fail("retired R0a paths remain in the active inventory")
    expected_exceptions = {
        "tools/smokes/v2/profiles/integration/core/phase2050/flow_phi2_select_by_pred_rc99_primary_canary_vm.sh",
        "tools/smokes/v2/profiles/integration/core/phase2051/selfhost_v1_provider_primary_rc42_canary_vm.sh",
    }
    if not expected_exceptions.issubset(set(leaf_paths)):
        fail("R0b exception leaves are not retained in the active inventory")
    for item in derive_inventory(root, leaf_paths):
        if observations.get(item["path"]) != item:
            fail(f"post-R0a body-derived observation drifted: {item['path']}")
    expected_observed = {
        "lexical_leaves": 86,
        "lexical_sites": 90,
        "route_class": {
            "DirectForceSealed": 3,
            "HelperForceConditional": 44,
            "ExplicitCoreResidualSealed": 35,
            "DynamicArtifactOpen": 4,
        },
        "route_sites": {
            "DirectForceSealed": 3,
            "HelperForceConditional": 45,
            "ExplicitCoreResidualSealed": 35,
            "DynamicArtifactOpen": 7,
        },
    }
    if manifest.get("observed_counts") != expected_observed:
        fail("post-R0a observed counts are not the reviewed 86/90 matrix")
    groups = manifest.get("groups")
    if not isinstance(groups, list):
        fail("post-R0a manifest groups are missing")
    active_group_paths = [path for group in groups for path in group.get("paths", []) if isinstance(group, dict)]
    if len(active_group_paths) != 86 or len(set(active_group_paths)) != 86 or set(active_group_paths) != set(leaf_paths):
        fail("post-R0a groups do not cover exactly the active 86 leaves")
    family_counts: dict[str, int] = {}
    for group in groups:
        if not isinstance(group, dict):
            fail("post-R0a manifest group is invalid")
        family_name = group.get("caller_family")
        family_counts[family_name] = family_counts.get(family_name, 0) + len(group.get("paths", []))
    if family_counts != {"direct": 3, "textual_helper": 74, "wrapper_only": 9}:
        fail("post-R0a caller-family counts drifted")
    direct_history = [
        group for group in groups
        if group.get("caller_family") == "direct" and group.get("fate") == "HistoricalDelete"
    ]
    if len(direct_history) != 1 or set(direct_history[0].get("paths", [])) != expected_exceptions:
        fail("post-R0a direct HistoricalDelete group must retain only the two exceptions")
    # R0a edits only phase2047-2050 projections; deleted leaf names must not survive there.
    for projection in (
        "tools/smokes/v2/profiles/integration/core/phase2047/run_all.sh",
        "tools/smokes/v2/profiles/integration/core/phase2048/run_all.sh",
        "tools/smokes/v2/profiles/integration/core/phase2049/run_all.sh",
        "tools/smokes/v2/profiles/integration/core/phase2050/run_all.sh",
    ):
        content = (root / projection).read_text(encoding="utf-8")
        for retired_path in retired_paths:
            if Path(retired_path).name in content:
                fail(f"retired R0a leaf remains in projection {projection}: {retired_path}")
    print(
        f"[{TAG}] result_class=current-change failure status=pass "
        "phase=force_hv1_direct_historical_delete_r0_post "
        "active_leaves=86 active_sites=90 retired=30 exceptions=2 implementation=closed"
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
