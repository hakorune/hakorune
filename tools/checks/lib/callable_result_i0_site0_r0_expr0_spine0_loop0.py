#!/usr/bin/env python3
"""Structural guard for LOOP0-S0a call-source schema migration."""

from __future__ import annotations

from pathlib import Path


CONSTRUCTOR_PATHS = (
    "src/mir/builder/control_flow/plan/normalizer/common.rs",
    "src/mir/builder/control_flow/plan/normalizer/helpers_value.rs",
    "src/mir/builder/control_flow/plan/normalizer/cond_lowering_prelude.rs",
    "src/mir/builder/control_flow/plan/normalizer/loop_body_lowering.rs",
    "src/mir/builder/control_flow/plan/parts/stmt.rs",
    "src/mir/builder/control_flow/plan/features/generic_loop_body/v0.rs",
    "src/mir/builder/control_flow/plan/features/generic_loop_body/v1.rs",
    "src/mir/builder/control_flow/plan/features/loop_cond_bc_util.rs",
    "src/mir/builder/control_flow/plan/features/loop_cond_co_stmt.rs",
    "src/mir/builder/control_flow/plan/features/loop_cond_continue_with_return_body_helpers.rs",
    "src/mir/builder/control_flow/plan/features/loop_cond_return_in_body_pipeline.rs",
    "src/mir/builder/control_flow/plan/features/loop_true_break_continue_pipeline.rs",
)


def _read(root: Path, relative: str) -> str:
    path = root / relative
    if not path.is_file():
        raise RuntimeError(f"LOOP0-S0a missing {relative}")
    return path.read_text(encoding="utf-8")


def check_loop0_s0a(root: Path) -> str:
    source_path = "src/mir/builder/control_flow/plan/call_source.rs"
    effect_path = "src/mir/builder/control_flow/plan/effect.rs"
    remapper_path = (
        "src/mir/builder/control_flow/plan/normalizer/"
        "cond_lowering_freshen/remapper.rs"
    )
    source = _read(root, source_path)
    effect = _read(root, effect_path)
    remapper = _read(root, remapper_path)

    if source.count("enum CoreCallSourceV1") != 1:
        raise RuntimeError("LOOP0-S0a requires one call-source vocabulary owner")
    if source.count("fn visit_core_call_sources_v1") != 1:
        raise RuntimeError("LOOP0-S0a requires one exhaustive call-source visitor")
    if effect.count("source: CoreCallSourceV1,") != 4:
        raise RuntimeError("LOOP0-S0a requires source fields on all four call variants")

    plan_root = root / "src/mir/builder/control_flow/plan"
    production_plan_sources = []
    for path in sorted(plan_root.rglob("*.rs")):
        text = path.read_text(encoding="utf-8").split("#[cfg(test)]", maxsplit=1)[0]
        production_plan_sources.append(text)
    production_plan_text = "\n".join(production_plan_sources)
    unlocated = production_plan_text.count("source: CoreCallSourceV1::Unlocated")
    if unlocated != 34:
        raise RuntimeError(
            f"LOOP0-S0a raw constructor migration drift: expected=34 actual={unlocated}"
        )
    if "CoreCallSourceV1::LocatedMethodCall(" in production_plan_text:
        raise RuntimeError("LOOP0-S0a production located call-source producers must remain zero")

    source_production = source.split("#[cfg(test)]", maxsplit=1)[0]
    if source_production.count("LocatedMethodCall") != 1:
        raise RuntimeError(
            "LOOP0-S0a production call-source module must only define the located variant"
        )
    remapper_production = remapper.split("#[cfg(test)]", maxsplit=1)[0]
    if "LocatedMethodCall" in remapper_production:
        raise RuntimeError("LOOP0-S0a remapper must preserve provenance opaquely")
    if remapper_production.count("source: _,") != 4:
        raise RuntimeError("LOOP0-S0a remapper must cover all four sources without mutation")

    all_mir_production = []
    for path in sorted((root / "src/mir").rglob("*.rs")):
        all_mir_production.append(
            path.read_text(encoding="utf-8").split("#[cfg(test)]", maxsplit=1)[0]
        )
    all_mir_production_text = "\n".join(all_mir_production)
    for forbidden in (
        "VerifiedLocatedCoreLoopPlanV1",
        "VerifiedCallableResultLoopClaimScheduleV1",
        "ClaimedCallableResultLoopBatchV1",
        "claim_loop_batch",
    ):
        if forbidden in all_mir_production_text:
            raise RuntimeError(f"LOOP0-S0a premature S0b/ledger authority: {forbidden}")
    if "callable_result_representation" in production_plan_text:
        raise RuntimeError("LOOP0-S0a plan tree must not import callable-result authority")

    builder_root = _read(root, "src/mir/builder.rs").split("#[cfg(test)]", maxsplit=1)[0]
    for forbidden_field in (
        "VerifiedCallableResultCallerLedgerV1",
        "ClaimedCallableResultActivationSiteV1",
        "VerifiedLocatedCoreLoopPlanV1",
        "VerifiedCallableResultLoopClaimScheduleV1",
        "ClaimedCallableResultLoopBatchV1",
    ):
        if forbidden_field in builder_root:
            raise RuntimeError(f"LOOP0-S0a MirBuilder authority leak: {forbidden_field}")

    touched = (source_path, effect_path, remapper_path, *CONSTRUCTOR_PATHS, __file__)
    oversized = []
    for path in touched:
        relative = str(path) if isinstance(path, str) else str(Path(path).relative_to(root))
        if len(_read(root, relative).splitlines()) >= 800:
            oversized.append(relative)
    if oversized:
        raise RuntimeError(f"LOOP0-S0a source/check files reached 800 lines: {oversized}")

    return "loop0_s0a_sources=4 raw_constructors=34 located_producers=0"
