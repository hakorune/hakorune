#!/usr/bin/env python3
"""Structural guard for LOOP0-I0b's one stack-scoped emission port."""

from __future__ import annotations

from pathlib import Path

from callable_result_i0_site0_r0_expr0_spine0_loop0 import _production, _read


PORT = "src/mir/builder/control_flow/plan/lowerer/emission_port.rs"
CORE = "src/mir/builder/control_flow/plan/lowerer/core.rs"
EFFECT = "src/mir/builder/control_flow/plan/lowerer/effect_emission.rs"
LOCATED = "src/mir/builder/control_flow/plan/located_loop.rs"


def _require(text: str, needle: str, expected: int, label: str) -> None:
    actual = text.count(needle)
    if actual != expected:
        raise RuntimeError(f"LOOP0-I0b {label}: expected={expected} actual={actual}")


def check_loop0_i0b(root: Path) -> str:
    port = _read(root, PORT)
    port_production = _production(port)
    core = _read(root, CORE)
    effect = _read(root, EFFECT)
    located = _read(root, LOCATED)

    _require(port_production, "enum CorePlanEffectEmissionPortV1", 1, "emission authority")
    # One constructor plus one dispatch arm and one consuming finish arm.
    _require(port_production, "Self::Raw", 3, "raw construction, dispatch, and finish")
    # The public constructor spells the fully-qualified enum name; the two
    # `Self` occurrences are dispatch and consuming finish.
    _require(port_production, "Self::Claimed", 2, "claimed dispatch and finish")
    _require(port_production, "fn emit_selected_exact_i64", 1, "selected terminal")
    _require(port_production, "target.mir_symbol_projection()", 4, "canonical target projection")
    _require(port_production, "CoreCallSourceV1::LocatedMethodCall", 1, "exact site identity")
    _require(core, "fn lower_with_emission_port", 1, "port-aware core entry")
    _require(core, "CorePlanEffectEmissionPortV1::raw()", 1, "raw facade")
    _require(effect, "fn emit_raw_effect", 1, "raw leaf primitive")
    if "fn emit_effect(" in effect:
        raise RuntimeError("LOOP0-I0b legacy direct effect-emission entry remains")

    _require(located, "struct ClaimedLocatedCoreLoopExecutionV1", 1, "claimed bundle")
    _require(located, "fn into_claimed_execution", 1, "consuming claim handoff")
    _require(located, ".claim_loop_batch(self.schedule)", 1, "atomic schedule claim")
    _require(located, "CorePlanEffectEmissionPortV1::claimed", 1, "bundle port creation")
    _require(located, "port.finish()?", 1, "exact claim completion")

    for forbidden in ("derive(Clone", "Arc<", "Rc<", "static mut", "thread_local!"):
        if forbidden in port or forbidden in located:
            raise RuntimeError(f"LOOP0-I0b stack-scoped product owns forbidden state: {forbidden}")

    terminal = port_production[port_production.index("fn emit_selected_exact_i64") :]
    if "func" in terminal:
        raise RuntimeError("LOOP0-I0b selected terminal reads raw GlobalCall spelling")

    touched = (PORT, CORE, EFFECT, LOCATED)
    oversized = [relative for relative in touched if len(_read(root, relative).splitlines()) >= 800]
    if oversized:
        raise RuntimeError(f"LOOP0-I0b source/check files reached 800 lines: {oversized}")

    return "i0b=1 emission_port=1 raw_facade=1 claimed_bundle=1 selected_terminal=1"
