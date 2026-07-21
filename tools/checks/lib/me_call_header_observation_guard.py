#!/usr/bin/env python3
"""ACCESS0-MEHEADER-G0 structural guard.

G0 closes the typed source-branded observation seam: one shared `me` policy,
three route adapters, no direct header fallback, and no long-lived observation
state.  This guard must fail if a second policy/dispatcher or an implicit
invocation current-module fallback reappears.
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
        raise AssertionError("G0 source must remain below 800 lines")
    require(builder, "mod me_call_header_observation", "G0 module entry")
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
        require(source, fragment, "G0 vocabulary")
    for fragment in (
        "source_branded_missing_does_not_become_present",
        "first_box_parameter_prepares_instance_receiver",
        "non_box_or_empty_parameters_prepare_static_receiver",
    ):
        require(source, fragment, "G0 fixture")
    forbid(source, "current_module", "G0 implicit module fallback in observation box")
    forbid(source, "collector: &", "G0 stored collector reference")
    for fragment in (
        "prepare_me_lowered_call_v1",
        "descent.observe_me_call_parameters",
        "Port: MethodCallLoweringPortV1",
    ):
        require(handler, fragment, "G0 shared me-policy connection")
    require(descent, "pub(in crate::mir::builder) fn observe_me_call_parameters", "G0 short observation loan")
    require(raw_port, "impl MeCallHeaderObservationPortV1 for RawLegacyChildLoweringPortV1", "G0 legacy adapter")
    require(raw_port, "impl MeCallHeaderObservationPortV1 for RawInvocationChildPortV1", "G0 invocation adapter")
    require(located, "impl MeCallHeaderObservationPortV1 for LocatedLegacyLoweringSessionV1", "G0 located adapter")
    if handler.count("struct MeCallPolicyBox;") != 1:
        raise AssertionError("G0 requires one MeCallPolicyBox definition")
    if handler.count("fn resolve_me_call<Port>") != 1:
        raise AssertionError("G0 requires one shared me policy entry")
    if raw_port.count("impl MeCallHeaderObservationPortV1") != 2:
        raise AssertionError("G0 requires exactly two raw route adapters")
    if located.count("impl MeCallHeaderObservationPortV1") != 1:
        raise AssertionError("G0 requires exactly one located route adapter")
    forbid(handler, "module.functions.get(&fname)", "G0 direct me header reader")
    forbid(handler, "is_instance_method", "G0 duplicate receiver classifier")
    require(descent, "port: &'port mut Port", "G0 short-lived argument capability")
    forbid(descent, "observation: MeCallParameterObservationV1", "G0 persistent observation field")
    invocation_start = raw_port.index(
        "impl MeCallHeaderObservationPortV1 for RawInvocationChildPortV1"
    )
    invocation_end = raw_port.find("impl ", invocation_start + 5)
    invocation_impl = raw_port[invocation_start:] if invocation_end < 0 else raw_port[invocation_start:invocation_end]
    forbid(invocation_impl, "builder.current_module", "G0 invocation module fallback")
    descent_at = handler.index("let arg_values = descent.lower_all(builder)?;")
    instance_arity_at = handler.index("if expected_params != provided_instance")
    static_arity_at = handler.index("if expected_params != provided_static")
    if not descent_at < instance_arity_at or not descent_at < static_arity_at:
        raise AssertionError("G0 arity diagnostics moved before argument descent")
    for fragment in (
        "ACCESS0-MEHEADER-I0",
        "typed-source refinement",
        "MeCallPolicyBox consumers = 1",
    ):
        require(consultation, fragment, "G0 consultation closeout")
    for fragment in (
        "direct method-handler module reads = 0",
        "invocation miss -> current_module fallback = 0",
        "header loan across argument descent = 0",
        "observation persistence/cache = 0",
    ):
        require(consultation, fragment, "G0 law")
    print("[me-call-header-observation-guard] ok me_policy=1 route_adapters=3 source_lines=" + str(len(source.splitlines())))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
