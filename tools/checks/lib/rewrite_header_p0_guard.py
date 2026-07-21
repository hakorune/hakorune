#!/usr/bin/env python3
"""ACCESS0-REWRITE-KNOWN-P0 structural/parity guard.

The rewrite projection is lookup-only.  Known, unique-suffix, and equals
routes may consume an explicit header view, while compatibility callers keep
their old module/index path when no view is supplied.  This guard prevents a
second header authority or a silent fallback from entering the disconnected
P0 slice.
"""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
HEADER = ROOT / "src/mir/builder/rewrite/header_lookup.rs"
KNOWN = ROOT / "src/mir/builder/rewrite/known.rs"
SPECIAL = ROOT / "src/mir/builder/rewrite/special.rs"
EMITTER = ROOT / "src/mir/builder/calls/unified_emitter.rs"
CONSULTATION = ROOT / (
    "docs/development/current/main/investigations/"
    "mirbuilder-headerport-i0-production-cutover-consultation-2026-07-21.md"
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def forbid(text: str, fragment: str, label: str) -> None:
    if fragment in text:
        raise AssertionError(f"forbidden {label}: {fragment!r}")


def main() -> int:
    header = HEADER.read_text()
    known = KNOWN.read_text()
    special = SPECIAL.read_text()
    emitter = EMITTER.read_text()
    consultation = CONSULTATION.read_text()

    if len(header.splitlines()) >= 800:
        raise AssertionError("rewrite header projection must remain below 800 lines")
    for fragment in (
        "struct KnownRewriteHeaderViewV1",
        "method_candidates_from_headers",
        "header_view_preserves_signature_arity_policy",
        "header_view_uses_shared_unique_suffix_policy",
        "header_view_missing_symbol_has_no_compatibility_fallback",
        "header_view_known_arity_matrix_keeps_static_and_instance_shapes",
    ):
        require(header, fragment, "P0 header projection/fixture")
    for fragment in ("builder.", "current_module", "module.functions"):
        forbid(header, fragment, f"P0 header projection authority {fragment}")

    for fragment in (
        "try_known_rewrite_with_lookup",
        "try_known_rewrite_to_dst_with_lookup",
        "try_unique_suffix_rewrite_with_lookup",
        "try_unique_suffix_rewrite_to_dst_with_lookup",
        "try_known_or_unique_with_lookup",
        "try_known_or_unique_to_dst_with_lookup",
        "rewrite_call_args_for_signature_with_lookup",
        "lookup\n        .map(|view|",
        "emit_unified_call_with_lookup",
        "annotate_call_result_from_func_name_with_lookup",
        "if !rewrite_enabled()",
        "should_block_primitive_str_rewrite",
    ):
        require(known, fragment, "P0 Known/unique lookup route")
    for fragment in (
        "try_special_equals_to_dst_with_lookup",
        "try_known_rewrite_to_dst_with_lookup",
        "try_unique_suffix_rewrite_to_dst_with_lookup",
    ):
        require(special, fragment, "P0 equals lookup route")
    for fragment in (
        "try_special_equals_to_dst_with_lookup",
        "try_known_or_unique_to_dst_with_lookup",
        "lookup",
    ):
        require(emitter, fragment, "P0 unified emitter forwarding")

    # A supplied view is the explicit source.  Compatibility fallback is
    # permitted only in the `lookup=None` branch of the legacy facade.
    lookup_branch = known.split("fn rewrite_call_args_for_signature_with_lookup", 1)[1]
    lookup_branch = lookup_branch.split("// Static-lowered methods", 1)[0]
    require(lookup_branch, "Some(view)", "P0 supplied-header branch")
    require(lookup_branch, "None =>", "P0 compatibility branch")
    forbid(lookup_branch, "lookup.unwrap_or", "P0 implicit lookup fallback")

    for fragment in (
        "ACCESS0-REWRITE-KNOWN-S0 closeout",
        "ACCESS0-REWRITE-KNOWN-P0",
        "missing headers",
        "static/instance arity",
        "unique\n0/1/>1 candidates",
        "environment gates",
        "primitive guards",
        "error/no-retry",
        "production-consumer guard",
    ):
        require(consultation, fragment, "P0 consultation contract")

    print("[rewrite-header-p0-guard] ok lookup_only=1 routes=3 source_lines=" + str(len(header.splitlines())))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
