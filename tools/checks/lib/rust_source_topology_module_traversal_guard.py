#!/usr/bin/env python3
"""Freeze MODULE0's disconnected explicit-module traversal boundary."""

from __future__ import annotations

import json
import sys
from pathlib import Path


TAG = "rust-source-topology-module-traversal"
TOOL = "tools/checks/rust_source_topology"
MODULE_DIR = f"{TOOL}/src/project/modules"
FILES = {
    "model": f"{MODULE_DIR}/model.rs",
    "error": f"{MODULE_DIR}/error.rs",
    "declarations": f"{MODULE_DIR}/declarations.rs",
    "cfg_gate": f"{MODULE_DIR}/cfg_gate.rs",
    "path_resolution": f"{MODULE_DIR}/path_resolution.rs",
    "traversal": f"{MODULE_DIR}/traversal.rs",
    "module": f"{MODULE_DIR}/mod.rs",
    "project": f"{TOOL}/src/project/mod.rs",
    "cfg_eval": f"{TOOL}/src/project/cfg_eval.rs",
    "rustc_cfg": f"{TOOL}/src/project/rustc_cfg.rs",
    "cli": f"{TOOL}/src/main.rs",
    "test": f"{TOOL}/tests/module_topology.rs",
}
PROFILE_FIXTURE = f"{TOOL}/tests/fixtures/module0_workspace/profiles_v1.json"
SELF = "tools/checks/lib/rust_source_topology_module_traversal_guard.py"


def fail(message: str) -> None:
    raise SystemExit(f"[{TAG}] ERROR: {message}")


def read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        fail(f"missing {relative}")
    return path.read_text(encoding="utf-8")


def require_count(source: str, needle: str, expected: int, label: str) -> None:
    actual = source.count(needle)
    if actual != expected:
        fail(f"{label}: expected={expected} actual={actual}")


def require_absent(source: str, needle: str, label: str) -> None:
    if needle in source:
        fail(f"{label}: forbidden token present: {needle}")


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    sources = {name: read(root, path) for name, path in FILES.items()}
    module_sources = "\n".join(
        sources[name]
        for name in (
            "model",
            "error",
            "declarations",
            "cfg_gate",
            "path_resolution",
            "traversal",
            "module",
        )
    )

    require_count(
        sources["traversal"],
        "pub fn collect_declared_module_topology_v1(",
        1,
        "module traversal owner",
    )
    require_count(
        sources["model"],
        "pub struct DeclaredModuleTopologyV1",
        1,
        "durable topology product",
    )
    require_count(
        sources["declarations"],
        "pub(super) fn parse_module_source_v1(",
        1,
        "module declaration parser",
    )
    require_count(
        sources["cfg_gate"],
        "pub(super) fn sealed_cfg_environment_v1(",
        1,
        "sealed cfg environment",
    )
    require_count(
        sources["path_resolution"],
        "pub(super) fn resolve_external_module_v1(",
        1,
        "external module resolver",
    )
    require_count(
        sources["traversal"],
        "extract_single_file_source(",
        1,
        "S0a source observation connection",
    )
    require_count(
        sources["cfg_eval"],
        "target_predicates_sealed",
        1,
        "sealed target predicate branch",
    )
    require_count(
        sources["rustc_cfg"],
        "pub(crate) fn cfg_key_values(",
        1,
        "rustc cfg key/value bridge",
    )

    for forbidden in (
        "FINALIZE0",
        "MirBuilder",
        "from_profile_input",
        "profile_expected_root_features",
        "HashSet<PathBuf>",
        "include expansion",
        "semantic def-path",
    ):
        require_absent(module_sources, forbidden, "MODULE0 authority boundary")
    require_absent(sources["model"], "PathBuf", "durable topology absolute-path state")
    require_absent(sources["model"], "#[serde(skip)]", "hidden durable execution state")
    for forbidden in (
        "collect_declared_module_topology_v1",
        "DeclaredModuleTopologyV1",
    ):
        require_absent(sources["cli"], forbidden, "project CLI before S0b-G0")

    require_count(sources["test"], "#[test]", 7, "MODULE0 focused test inventory")
    for tag in (
        "OrdinaryModuleMissing",
        "OrdinaryModuleAmbiguous",
        "UnknownCfg",
        "CanonicalCycle",
        "SourceOutsideWorkspace",
        "ModuleInBlock",
    ):
        if tag not in sources["test"]:
            fail(f"missing focused rejection fixture: {tag}")

    profile_document = json.loads(read(root, PROFILE_FIXTURE))
    profiles = profile_document.get("profiles", [])
    if len(profiles) != 6:
        fail(f"MODULE0 fixture profile count: expected=6 actual={len(profiles)}")

    guarded = [*FILES.values(), PROFILE_FIXTURE, SELF]
    oversized = [
        relative
        for relative in guarded
        if len(read(root, relative).splitlines()) >= 800
    ]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        f"[{TAG}] ok traversal=1 declaration_parser=1 cfg_owner=1 "
        "path_owner=1 cli_consumers=0 tests=7 profiles=6"
    )


if __name__ == "__main__":
    main()
