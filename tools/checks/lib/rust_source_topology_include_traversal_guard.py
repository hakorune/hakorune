#!/usr/bin/env python3
"""Freeze INCLUDE0's disconnected literal include occurrence boundary."""

from __future__ import annotations

import sys
from pathlib import Path


TAG = "rust-source-topology-include-traversal"
TOOL = "tools/checks/rust_source_topology"
MODULE_DIR = f"{TOOL}/src/project/modules"
FILES = {
    "model": f"{MODULE_DIR}/model.rs",
    "error": f"{MODULE_DIR}/error.rs",
    "include_scope": f"{MODULE_DIR}/include_scope.rs",
    "include_scope_candidate": f"{MODULE_DIR}/include_scope_candidate.rs",
    "declarations": f"{MODULE_DIR}/declarations.rs",
    "path_resolution": f"{MODULE_DIR}/path_resolution.rs",
    "traversal": f"{MODULE_DIR}/traversal.rs",
    "module": f"{MODULE_DIR}/mod.rs",
    "project": f"{TOOL}/src/project/mod.rs",
    "cli": f"{TOOL}/src/main.rs",
    "test": f"{TOOL}/tests/include_topology.rs",
    "module_test": f"{TOOL}/tests/module_topology.rs",
}
FIXTURE = f"{TOOL}/tests/fixtures/module0_workspace/roots/must_remain_opaque.rs"
SELF = "tools/checks/lib/rust_source_topology_include_traversal_guard.py"


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
    implementation = "\n".join(
        sources[name]
        for name in (
            "model",
            "error",
            "include_scope",
            "declarations",
            "path_resolution",
            "traversal",
            "module",
        )
    )

    require_count(
        sources["model"],
        "pub struct DeclaredIncludeEdgeV1",
        1,
        "durable include edge product",
    )
    require_count(
        sources["declarations"],
        "pub(super) enum ModulePositionItemV1",
        1,
        "ordered module/include declaration owner",
    )
    require_count(
        sources["declarations"],
        "pub(super) fn parse_included_module_source_v1(",
        1,
        "included item-list parser",
    )
    require_count(
        sources["declarations"],
        "pub(super) fn parse_included_direct_items_v1(",
        1,
        "included raw-item parser facade",
    )
    require_count(
        sources["declarations"],
        "fn parse_source_file_v1(",
        1,
        "sole included/source syntax parser owner",
    )
    require_count(
        sources["path_resolution"],
        "pub(super) fn resolve_include_source_v1(",
        1,
        "include path resolver",
    )
    require_count(
        sources["traversal"],
        "fn add_include_source(",
        1,
        "include occurrence traversal owner",
    )
    require_count(
        sources["include_scope"],
        "pub(super) struct IncludeScopeLanesV1",
        1,
        "two-lane include scope vocabulary owner",
    )
    require_count(
        sources["include_scope"],
        "pub(super) enum ModuleLocalIncludeNameLaneV1",
        1,
        "module-local scope lane vocabulary",
    )
    require_count(
        sources["include_scope"],
        "pub(super) enum TextualIncludeMacroLaneV1",
        1,
        "textual scope lane vocabulary",
    )
    require_count(
        sources["include_scope"],
        "pub(super) fn child_module_entry(&self) -> Self",
        1,
        "child module scope boundary",
    )
    require_count(
        sources["traversal"],
        "format!(\"include:{}\"",
        1,
        "include occurrence ID owner",
    )
    for token in (
        "parent_include_edge_id",
        "owning_module_instance_id",
        "child_source_observation_id",
    ):
        if token not in sources["model"]:
            fail(f"missing include occurrence identity field: {token}")

    for forbidden in (
        "FINALIZE0",
        "MirBuilder",
        "HashSet<PathBuf>",
        "semantic def-path",
        "concat!(\"OUT_DIR\"",
    ):
        require_absent(implementation, forbidden, "INCLUDE0 authority boundary")
    require_absent(sources["model"], "PathBuf", "durable include absolute-path state")
    require_absent(
        sources["model"], "#[serde(skip)]", "hidden include execution state"
    )
    require_absent(
        sources["cli"],
        "DeclaredIncludeEdgeV1",
        "project CLI before S0b-G0",
    )
    disconnected_scope_consumers = "\n".join(
        sources[name]
        for name in (
            "model",
            "error",
            "declarations",
            "path_resolution",
            "traversal",
            "module",
        )
    )
    require_absent(
        disconnected_scope_consumers,
        "IncludeScopeLanesV1",
        "INCLUDE-SCOPE0-S0 production consumers",
    )

    require_count(
        sources["module"],
        "#[cfg(test)]\nmod include_scope_candidate;",
        1,
        "INCLUDE-SCOPE0-P0 test-only module registration",
    )
    require_count(
        sources["include_scope_candidate"],
        "fn observe_file_content_scope_v1(",
        1,
        "root content scope observer",
    )
    require_count(
        sources["include_scope_candidate"],
        "fn observe_same_module_include_v1(",
        1,
        "same-module include scope observer",
    )
    for forbidden in (
        "syn::parse_file",
        "decide_cfg_attribute_stream_v1",
        "include_macro_ambiguity",
        "to_token_stream",
        "line_starts_v1",
        "item_range_v1",
        "collect_direct_module_position_items_v1",
    ):
        require_absent(
            sources["include_scope_candidate"],
            forbidden,
            "INCLUDE-SCOPE0-P0 shared-authority boundary",
        )
    for fixture in (
        "excluded_glob_has_no_scope_effect_but_active_and_unknown_globs_do",
        "parent_import_does_not_poison_inline_or_external_child_entry",
        "textual_macro_visibility_is_source_ordered_and_inherited_by_children",
        "included_source_scope_returns_to_following_sibling",
        "excluded_content_performs_zero_scope_scans",
    ):
        if fixture not in sources["include_scope_candidate"]:
            fail(f"missing INCLUDE-SCOPE0-P0 fixture: {fixture}")
    require_count(
        sources["include_scope_candidate"],
        "#[test]",
        5,
        "INCLUDE-SCOPE0-P0 focused test inventory",
    )

    require_count(sources["test"], "#[test]", 7, "INCLUDE0 focused test inventory")
    for tag in (
        "CanonicalCycle",
        "UnknownCfg",
        "NonLiteralInclude",
        "UnsupportedIncludeContext",
        "IncludeMacroIdentityUnresolved",
        "UnsupportedIncludedPreamble",
        "SourceOutsideWorkspace",
    ):
        if tag not in sources["test"]:
            fail(f"missing focused include fixture: {tag}")

    guarded = [*FILES.values(), FIXTURE, SELF]
    oversized = [
        relative for relative in guarded if len(read(root, relative).splitlines()) >= 800
    ]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        f"[{TAG}] ok edge_owner=1 ordered_items=1 path_owner=1 "
        "traversal_owner=1 cli_consumers=0 scope_consumers=0 "
        "scope_proof_consumers=0 include_tests=7 scope_proof_tests=5"
    )


if __name__ == "__main__":
    main()
