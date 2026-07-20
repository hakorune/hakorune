#!/usr/bin/env python3
"""Freeze CFGSTREAM0 and CONTENTCFG0 topology decision boundaries."""

from __future__ import annotations

import sys
from pathlib import Path


TAG = "rust-source-topology-cfg-stream"
TOOL = "tools/checks/rust_source_topology"
FILES = {
    "cfg_eval": f"{TOOL}/src/project/cfg_eval.rs",
    "cfg_stream": f"{TOOL}/src/project/cfg_stream.rs",
    "model": f"{TOOL}/src/project/model.rs",
    "cfg_gate": f"{TOOL}/src/project/modules/cfg_gate.rs",
    "content_gate": f"{TOOL}/src/project/modules/content_gate.rs",
    "content_draft": f"{TOOL}/src/project/modules/content_draft.rs",
    "content_candidate": f"{TOOL}/src/project/modules/content_candidate.rs",
    "declarations": f"{TOOL}/src/project/modules/declarations.rs",
    "module_model": f"{TOOL}/src/project/modules/model.rs",
    "traversal": f"{TOOL}/src/project/modules/traversal.rs",
    "content_issuance": f"{TOOL}/src/project/modules/content_issuance.rs",
    "stream_test": f"{TOOL}/tests/cfg_stream.rs",
    "module_test": f"{TOOL}/tests/module_topology.rs",
    "include_test": f"{TOOL}/tests/include_topology.rs",
    "profile_test": f"{TOOL}/tests/profile_cfg.rs",
}
SELF = "tools/checks/lib/rust_source_topology_cfg_stream_guard.py"


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
            "cfg_eval",
            "cfg_stream",
            "cfg_gate",
            "content_gate",
            "content_draft",
            "declarations",
            "module_model",
            "traversal",
            "content_issuance",
        )
    )

    require_count(
        sources["cfg_stream"],
        "pub fn decide_cfg_attribute_stream_v1(",
        1,
        "ordered cfg stream decision owner",
    )
    require_count(
        sources["cfg_eval"],
        "pub(super) fn decide_cfg_predicate_syntax_v1(",
        1,
        "one-row predicate evaluator",
    )
    require_count(
        sources["cfg_gate"],
        "pub(super) fn decide_module_cfg_stream_v1(",
        1,
        "module stream facade",
    )
    require_count(
        sources["cfg_gate"],
        "decide_cfg_attribute_stream_v1(rows, environment)",
        1,
        "module stream facade consumer",
    )
    require_count(
        sources["traversal"],
        "decide_module_cfg_stream_v1(&declaration.outer_topology_rows",
        2,
        "module/include declaration consumers",
    )
    require_count(
        sources["model"],
        "pub struct CfgAttributeActivePathEffectV1",
        1,
        "active path effect product",
    )
    require_count(
        sources["module_model"],
        "pub cfg_decision: CfgAttributeStreamDecisionV1",
        2,
        "stream decision edge publication",
    )
    require_count(
        sources["declarations"],
        "pub outer_topology_rows: Box<[CfgAttributeStreamInputRowV1]>",
        2,
        "exact declaration stream inputs",
    )

    for forbidden in (
        "decide_cfg_rows_v1",
        "CfgDecisionV1",
        "CfgRowDecisionV1",
        "outer_cfg_syntax",
        "collect_cfg_attr_paths",
        "validate_cfg_attr_contents",
    ):
        require_absent(implementation, forbidden, "retired eager cfg authority")
    for forbidden in (
        "decide_cfg_predicate_syntax_v1",
        "cfg_expr::",
        ".eval(",
    ):
        require_absent(sources["cfg_gate"], forbidden, "module cfg consumer predicate evaluation")

    if "included_stream_exports_exact_direct_and_nested_path_effects" not in sources["stream_test"]:
        fail("missing active path effect fixture")
    if "inactive_nonliteral" not in sources["module_test"]:
        fail("missing inactive cfg_attr path fixture")
    if "excluded_nested" not in sources["include_test"]:
        fail("missing excluded include cfg_attr fixture")

    require_count(
        sources["content_gate"],
        "pub struct DeclaredModuleContentGateV1",
        1,
        "content gate vocabulary owner",
    )
    require_count(
        sources["content_draft"],
        "pub(super) fn classify_module_content_draft_v1(",
        1,
        "content gate classifier owner",
    )
    require_count(
        sources["content_draft"],
        "CfgDecisionStateV1::Unknown => Err(ModuleContentDraftErrorV1::UnknownCfg",
        1,
        "unknown content gate typed stop",
    )
    require_count(
        sources["content_issuance"],
        "impl ModuleTraversalV1 {",
        1,
        "post-outer-cfg content issuance owner",
    )
    require_count(
        sources["content_issuance"],
        "pub(super) fn add_inline_module(",
        1,
        "inline content issuance consumer",
    )
    require_count(
        sources["content_issuance"],
        "pub(super) fn add_external_module(",
        1,
        "external content issuance consumer",
    )
    require_count(
        sources["traversal"],
        "self.classify_file_content(ModuleContentCandidateIdV1::Root",
        1,
        "root content issuance consumer",
    )
    require_count(
        sources["traversal"],
        "root_content_gate: None,",
        1,
        "empty traversal root-gate state",
    )
    require_count(
        sources["traversal"],
        "\n                    content_gate: None,",
        1,
        "outer-excluded edge no-gate publication",
    )
    require_count(
        sources["content_issuance"],
        "content_gate: Some(content_gate),",
        4,
        "outer-included content-gate publications",
    )
    require_count(
        sources["content_issuance"],
        "ModuleContentCandidateIdV1::ModuleEdge {",
        2,
        "external and inline edge candidate issuers",
    )
    require_count(
        sources["content_issuance"],
        "ClassifiedModuleContentDraftV1::Excluded { gate } => (gate, None),",
        2,
        "excluded content has no direct declaration product",
    )
    require_count(
        sources["content_issuance"],
        "let Some(parsed) = parsed else {",
        2,
        "excluded content stops before child issuance",
    )
    require_count(
        sources["traversal"],
        "ModuleContentCandidateIdV1::ModuleEdge {",
        1,
        "read-only edge candidate identity verifier",
    )
    require_count(
        sources["module_model"],
        "pub root_content_gate: DeclaredModuleContentGateV1",
        1,
        "root gate topology publication",
    )
    require_count(
        sources["module_model"],
        "pub content_gate: Option<DeclaredModuleContentGateV1>",
        1,
        "edge gate topology publication",
    )
    for source_name, forbidden, label in (
        (
            "traversal",
            "resolve_external_module_v1",
            "traversal must not resolve post-gate external content",
        ),
        (
            "content_issuance",
            "decide_module_cfg_stream_v1",
            "content issuer must not re-evaluate outer cfg",
        ),
        (
            "content_issuance",
            "decide_cfg_attribute_stream_v1",
            "content issuer must not re-evaluate inner cfg",
        ),
        (
            "content_issuance",
            "content_gate: None",
            "included issuer must not publish an absent content gate",
        ),
        (
            "traversal",
            "inline_items",
            "retired eager inline declaration surface",
        ),
    ):
        require_absent(sources[source_name], forbidden, label)

    if "#[cfg(test)]\nmod content_candidate;" not in read(
        root, f"{TOOL}/src/project/modules/mod.rs"
    ):
        fail("content candidate observer escaped cfg(test)")
    for fixture in (
        "content_gate_controls_root_external_and_inline_instance_issue",
        "content_gate_unknown_and_outer_exclusion_have_distinct_typed_boundaries",
        "reachable_inner_cfg_and_unknown_attribute_stop_explicitly",
        "excluded_content_is_parsed_but_never_exposes_direct_items",
    ):
        if fixture not in sources["module_test"] + sources["content_draft"]:
            fail(f"missing content-gate boundary fixture: {fixture}")

    guarded = [*FILES.values(), SELF]
    oversized = [
        relative for relative in guarded if len(read(root, relative).splitlines()) >= 800
    ]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")

    print(
        f"[{TAG}] ok stream_owner=1 predicate_owner=1 declaration_consumers=2 "
        "eager_owners=0 effect_product=1 content_gate_owners=1 "
        "content_issuance_owner=1 excluded_edge_gate=0 unknown_success=0"
    )


if __name__ == "__main__":
    main()
