#!/usr/bin/env python3
"""ACCESS0-MEHEADER-S0 structural guard.

The S0 product is disconnected vocabulary: one typed source-branded
observation, one pure receiver/arity prepare step, and zero production
consumers.  This guard must fail if the observation starts owning a Builder,
collector, metadata, or an implicit current-module fallback.
"""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
SOURCE = ROOT / "src/mir/builder/me_call_header_observation.rs"
BUILDER = ROOT / "src/mir/builder.rs"
HANDLER = ROOT / "src/mir/builder/method_call_handlers.rs"
CONSULTATION = ROOT / (
    "docs/development/current/main/investigations/"
    "mirbuilder-headerport-access0-me-handler-consultation-2026-07-21.md"
)


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def forbid(text: str, fragment: str, label: str) -> None:
    if fragment in text:
        raise AssertionError(f"forbidden {label}: {fragment!r}")


def main() -> int:
    source = SOURCE.read_text()
    builder = BUILDER.read_text()
    handler = HANDLER.read_text()
    consultation = CONSULTATION.read_text()

    if len(source.splitlines()) >= 800:
        raise AssertionError("S0 source must remain below 800 lines")
    require(builder, "mod me_call_header_observation", "S0 module entry")
    for fragment in (
        "MeCallHeaderSourceV1",
        "MeCallParameterObservationV1",
        "MeCallHeaderObservationPortV1",
        "MethodCallLoweringPortV1",
        "PreparedMeLoweredCallV1",
        "prepare_me_lowered_call_v1",
        "InvocationCollector",
        "ModuleCompatibility",
    ):
        require(source, fragment, "S0 vocabulary")
    for fragment in (
        "source_branded_missing_does_not_become_present",
        "first_box_parameter_prepares_instance_receiver",
        "non_box_or_empty_parameters_prepare_static_receiver",
    ):
        require(source, fragment, "S0 fixture")
    forbid(source, "current_module", "S0 implicit module fallback")
    forbid(source, "collector: &", "S0 stored collector reference")
    if source.count("impl MeCallHeaderObservationPortV1") != 0:
        raise AssertionError("S0 production observation consumers must be zero")
    if "MeCallParameterObservationV1" in handler:
        raise AssertionError("MeCallPolicyBox must remain disconnected through S0")
    for fragment in (
        "ACCESS0-MEHEADER-S0",
        "typed-source refinement",
        "MeCallPolicyBox consumers = 0",
    ):
        require(consultation, fragment, "S0 consultation closeout")
    print("[me-call-header-observation-guard] ok consumers=0 source_lines=" + str(len(source.splitlines())))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
