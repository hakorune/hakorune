#!/usr/bin/env python3
"""ACCESS0-MEHEADER-I0 structural guard.

I0 connects the typed source-branded observation to the one shared `me` policy
and three compatibility/raw route adapters.  This guard must fail if the
observation owns a Builder, collector, metadata, or an implicit current-module
fallback.
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
    raw_port = (ROOT / "src/mir/builder/recursive_child_lowering.rs").read_text()
    located = (ROOT / "src/mir/builder/located_legacy_lowering.rs").read_text()
    descent = (ROOT / "src/mir/builder/calls/method_call_descent.rs").read_text()
    consultation = CONSULTATION.read_text()

    if len(source.splitlines()) >= 800:
        raise AssertionError("I0 source must remain below 800 lines")
    require(builder, "mod me_call_header_observation", "I0 module entry")
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
        require(source, fragment, "I0 vocabulary")
    for fragment in (
        "source_branded_missing_does_not_become_present",
        "first_box_parameter_prepares_instance_receiver",
        "non_box_or_empty_parameters_prepare_static_receiver",
    ):
        require(source, fragment, "I0 fixture")
    forbid(source, "current_module", "I0 implicit module fallback in observation box")
    forbid(source, "collector: &", "I0 stored collector reference")
    for fragment in (
        "prepare_me_lowered_call_v1",
        "descent.observe_me_call_parameters",
        "Port: MethodCallLoweringPortV1",
    ):
        require(handler, fragment, "I0 shared me-policy connection")
    require(descent, "pub(in crate::mir::builder) fn observe_me_call_parameters", "I0 short observation loan")
    require(raw_port, "impl MeCallHeaderObservationPortV1 for RawLegacyChildLoweringPortV1", "I0 legacy adapter")
    require(raw_port, "impl MeCallHeaderObservationPortV1 for RawInvocationChildPortV1", "I0 invocation adapter")
    require(located, "impl MeCallHeaderObservationPortV1 for LocatedLegacyLoweringSessionV1", "I0 located adapter")
    forbid(handler, "module.functions.get(&fname)", "I0 direct me header reader")
    descent_at = handler.index("let arg_values = descent.lower_all(builder)?;")
    instance_arity_at = handler.index("if expected_params != provided_instance")
    static_arity_at = handler.index("if expected_params != provided_static")
    if not descent_at < instance_arity_at or not descent_at < static_arity_at:
        raise AssertionError("P0 arity diagnostics moved before argument descent")
    for fragment in (
        "ACCESS0-MEHEADER-I0",
        "typed-source refinement",
        "MeCallPolicyBox consumers = 1",
    ):
        require(consultation, fragment, "I0 consultation closeout")
    print("[me-call-header-observation-guard] ok me_policy=1 source_lines=" + str(len(source.splitlines())))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
