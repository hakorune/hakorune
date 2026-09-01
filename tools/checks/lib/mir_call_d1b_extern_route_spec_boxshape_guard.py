"""Fail-closed guard for the extern route-spec lookup BoxShape split."""

from __future__ import annotations

from pathlib import Path
import hashlib
import re
import subprocess


ROW = "MIR-BUILDER-EXTERN-ROUTE-SPEC-CATALOG-LOOKUP-BOXSHAPE-S0"
KEY = "mir_builder_extern_route_spec_catalog_lookup_boxshape_s0_2026_09_01"
CARD_REL = Path(
    "docs/development/current/main/investigations/"
    "mir-call-core-r6-d1b-method-none-manifest-2026-08-25.toml"
)
STATE_REL = Path("docs/development/current/main/CURRENT_STATE.toml")
PARENT_REL = Path("src/mir/extern_call_route_plan/route_spec.rs")
CHILD_REL = Path("src/mir/extern_call_route_plan/route_spec/catalog_lookup.rs")
MODULE_REL = Path("src/mir/extern_call_route_plan.rs")

EXPECTED_VARIANTS = 47
EXPECTED_DIRECT_ROWS = 36
EXPECTED_MACRO_ROWS = 11
EXPECTED_VARIANT_SHA = "f1136d316ea75221f2e7d2d0056e07db25eab3484725692b519c79637a7b4a65"
EXPECTED_CATALOG_SHA = "2c931c48c8f9c183828fb787afe1a51075b589598030e54572106bbd44656503"


def _fail(api, message: str) -> None:
    api.fail(f"extern route-spec BoxShape: {message}")


def _read(root: Path, rel: Path, api) -> str:
    path = root / rel
    if not path.is_file():
        _fail(api, f"missing owner {rel}")
    return path.read_text(encoding="utf-8")


def _sha256(value: str) -> str:
    return hashlib.sha256(value.encode("utf-8")).hexdigest()


def _changed_paths(root: Path, base: str, api) -> set[str]:
    result = subprocess.run(
        ["git", "diff", "--name-only", f"{base}..HEAD"],
        cwd=root,
        capture_output=True,
        text=True,
        check=False,
    )
    if result.returncode != 0:
        _fail(api, f"cannot inspect implementation diff from {base}: {result.stderr.strip()}")
    return {line for line in result.stdout.splitlines() if line.strip()}


def check_extern_route_spec_boxshape_s0(state: dict, card: dict, root: Path, api) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        _fail(api, "row requires fast or closeout work_mode")
    if state.get("current_execution_row") != ROW:
        _fail(api, "row is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        _fail(api, "row must clear current_design_stop")
    if state.get("next_execution_card") != ROW:
        _fail(api, "next_execution_card drifted")
    if state.get("next_execution_card_path") != str(CARD_REL):
        _fail(api, "next_execution_card_path drifted")

    row = card.get(KEY)
    if not isinstance(row, dict) or row.get("task_id") != ROW:
        _fail(api, f"manifest entry {KEY} is missing or has a wrong task_id")
    status = row.get("status")
    if status not in {"fast_open", "landed"}:
        _fail(api, f"status is not fast_open/landed: {status!r}")
    if row.get("implementation_permission") is not (status == "fast_open"):
        _fail(api, "status and implementation_permission disagree")

    parent = _read(root, PARENT_REL, api)
    child = _read(root, CHILD_REL, api)
    module = _read(root, MODULE_REL, api)
    for rel, text in ((PARENT_REL, parent), (CHILD_REL, child)):
        if len(text.splitlines()) >= 760:
            _fail(api, f"{rel} reached the 760-line boundary")
        if "include!(" in text:
            _fail(api, f"{rel} uses forbidden include!")

    enum_match = re.search(
        r"pub enum ExternCallRouteKind \{(?P<body>.*?)\n\}", parent, re.S
    )
    if not enum_match:
        _fail(api, "parent enum owner is missing")
    variants = re.findall(
        r"^\s{4}([A-Za-z][A-Za-z0-9_]*),?\s*$", enum_match.group("body"), re.M
    )
    if len(variants) != EXPECTED_VARIANTS:
        _fail(api, f"variant count drifted: {len(variants)} != {EXPECTED_VARIANTS}")
    if _sha256("\n".join(variants)) != EXPECTED_VARIANT_SHA:
        _fail(api, "variant order/name fingerprint drifted")

    catalog_match = re.search(
        r"static EXTERN_CALL_ROUTE_SPECS.*?(?=pub fn extern_call_route_specs)",
        parent,
        re.S,
    )
    if not catalog_match:
        _fail(api, "parent catalog owner is missing")
    catalog = catalog_match.group(0)
    direct_rows = len(re.findall(r"ExternCallRouteSpec\s*\{", catalog))
    macro_rows = len(re.findall(r"strict_json_tree_route!\s*\(", catalog))
    if (direct_rows, macro_rows) != (EXPECTED_DIRECT_ROWS, EXPECTED_MACRO_ROWS):
        _fail(api, f"catalog row census drifted: direct={direct_rows}, macro={macro_rows}")
    if _sha256(re.sub(r"\s+", "", catalog)) != EXPECTED_CATALOG_SHA:
        _fail(api, "catalog fingerprint drifted")

    for token in (
        "mod catalog_lookup;",
        "pub enum ExternCallRouteKind",
        "pub struct ExternCallRouteSpec",
        "static EXTERN_CALL_ROUTE_SPECS",
        "pub fn extern_call_route_specs",
    ):
        if parent.count(token) != 1:
            _fail(api, f"parent must own exactly one {token!r}")
    if parent.count("pub use catalog_lookup::{") != 1:
        _fail(api, "parent lookup re-export is missing")
    if "impl ExternCallRouteKind {" in parent:
        _fail(api, "kind lookup impl remains in parent")
    for name in (
        "normalize_extern_symbol",
        "classify_extern_call_route",
        "is_hostbridge_extern_invoke_symbol",
    ):
        if re.search(rf"pub fn {name}\s*\(", parent):
            _fail(api, f"lookup function remains in parent: {name}")

    if child.count("impl ExternCallRouteKind {") != 1:
        _fail(api, "child must own exactly one kind lookup impl")
    for name in (
        "normalize_extern_symbol",
        "classify_extern_call_route",
        "is_hostbridge_extern_invoke_symbol",
    ):
        if len(re.findall(rf"pub fn {name}\s*\(", child)) != 1:
            _fail(api, f"child must own exactly one lookup function: {name}")
    if "use crate::mir::core_method_op::{LoweringPlanEmitKind, LoweringPlanTier};" not in child:
        _fail(api, "child lowering-plan import is missing")
    if "use super::{ExternCallRouteKind, ExternCallRouteSpec, EXTERN_CALL_ROUTE_SPECS};" not in child:
        _fail(api, "child parent-owner import is missing")
    for token in ("pub enum ExternCallRouteKind", "pub struct ExternCallRouteSpec", "static EXTERN_CALL_ROUTE_SPECS"):
        if token in child:
            _fail(api, f"child illegally owns catalog token {token!r}")
    if "pub use route_spec::{" not in module or "ExternCallRouteKind" not in module:
        _fail(api, "outer route-plan re-export drifted")

    if status == "landed":
        base = row.get("base_commit")
        if not isinstance(base, str) or not base.strip():
            _fail(api, "landed row lacks base_commit")
        allowed = row.get("allowed_files")
        if not isinstance(allowed, list) or not all(isinstance(item, str) for item in allowed):
            _fail(api, "landed row allowed_files is malformed")
        changed = _changed_paths(root, base, api)
        if not changed <= set(allowed):
            _fail(api, f"changed paths escaped allowed boundary: {sorted(changed - set(allowed))}")
