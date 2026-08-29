#!/usr/bin/env python3
"""HDR0-P0 authority handoff guard.

This guard checks that HeaderPort remains an explicit annotation/publication
capability after the caller-zero Resolved/tail-recovery facade is removed.
"""

from __future__ import annotations

from pathlib import Path


def _require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def _forbid(text: str, fragment: str, label: str) -> None:
    if fragment in text:
        raise AssertionError(f"forbidden {label}: {fragment!r}")


def verify_authority_erasure(root: Path) -> None:
    source = {
        "build": (root / "src/mir/builder/calls/build.rs").read_text(),
        "static": (root / "src/mir/builder/calls/static_resolution.rs").read_text(),
        "materializer": (root / "src/mir/builder/calls/materializer.rs").read_text(),
        "emitter": (root / "src/mir/builder/calls/unified_emitter.rs").read_text(),
        "receipt": (root / "src/mir/builder/calls/unified_emitter/post_success.rs").read_text(),
    }

    _require(source["build"], "RawFunctionHeaderLookupPortV1", "call header capability")
    for retired in (
        "build_resolved_function_call",
        "try_unique_static_method_recovery",
        "try_tail_based_resolver",
    ):
        _forbid(source["build"], retired, "retired Resolved header consumer")
        _forbid(source["static"], retired, "retired static recovery owner")

    _forbid(
        source["materializer"],
        "GlobalPresenceAuthorityV1",
        "retired global-presence authority",
    )
    _forbid(
        source["materializer"],
        "try_global_additional_resolvers",
        "retired additional Global resolver",
    )
    _require(
        source["materializer"],
        "materialize_receiver_in_callee",
        "receiver-only materializer owner",
    )
    _require(
        source["emitter"],
        "lookup: Option<&dyn FunctionSignatureLookupV1>",
        "explicit emitter header capability",
    )

    _require(source["receipt"], "lookup: Option<&'lookup dyn FunctionSignatureLookupV1>", "receipt lookup")
    _require(source["receipt"], "if let Some(lookup) = self.lookup", "receipt authority branch")
    _require(
        source["receipt"],
        "annotate_call_result_from_func_name_with_lookup",
        "post-success lookup annotation",
    )

    for path in (
        root / "tools/checks/lib/headerport_authority_erasure_guard.py",
        root / "tools/checks/lib/headerport_candidate0_guard.py",
        root / "src/mir/builder/calls/build.rs",
        root / "src/mir/builder/calls/static_resolution.rs",
        root / "src/mir/builder/calls/materializer.rs",
        root / "src/mir/builder/calls/unified_emitter.rs",
        root / "src/mir/builder/calls/unified_emitter/post_success.rs",
    ):
        if len(path.read_text().splitlines()) >= 800:
            raise AssertionError(f"800-line boundary exceeded: {path.relative_to(root)}")


if __name__ == "__main__":
    verify_authority_erasure(Path(__file__).resolve().parents[3])
    print("[headerport-authority-erasure-guard] ok")
