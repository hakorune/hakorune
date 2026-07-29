#!/usr/bin/env python3
"""HDR0-P0 authority handoff guard.

This guard checks the explicit HeaderPort call path without enabling any
production module capture. It keeps resolve, emission, and post-success
annotation on one lookup-aware path while retaining the named legacy facade.
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
    _require(source["build"], "port.with_function_headers(|lookup|", "short header loan")
    _require(
        source["build"],
        "try_tail_based_resolver_with_headers(&name, &arg_values, headers)",
        "explicit tail route",
    )

    unique = source["static"].split("pub(super) fn try_unique_static_method_recovery", 1)[1]
    unique = unique.split("/// Try the dev-only tail resolver", 1)[0]
    _require(unique, "emit_unified_call_with_lookup", "unique recovery lookup-aware emission")
    tail = source["static"].split(
        "pub(in crate::mir::builder) fn try_tail_based_resolver_with_headers", 1
    )[1]
    _require(tail, "method_candidates_from_headers", "deterministic explicit tail projection")
    _require(tail, "emit_unified_call_with_lookup", "explicit tail lookup-aware emission")
    _forbid(tail, "emit_legacy_call", "explicit tail legacy emission")

    _require(
        source["materializer"],
        "enum GlobalPresenceAuthorityV1",
        "exclusive materializer authority",
    )
    _require(
        source["materializer"],
        "try_global_additional_resolvers_with_authority",
        "authority-owned materializer entry",
    )
    _forbid(
        source["materializer"],
        "fn try_global_additional_resolvers(",
        "retired global-presence facade",
    )
    _forbid(
        source["materializer"],
        "current_module.functions.contains_key",
        "retired direct module-presence observation",
    )
    _require(
        source["materializer"],
        "annotate_call_result_from_func_name_with_lookup",
        "lookup-aware materializer annotation",
    )
    _require(
        source["emitter"],
        "GlobalPresenceAuthorityV1::InvocationHeader",
        "explicit materializer authority wiring",
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
