#!/usr/bin/env python3
"""HEADERPORT0 Candidate0-S0 disconnected ownership guard.

The candidate owns one shell/collector state, lends the Builder only to an
active lowering closure, and exposes only typed abort/discard outcomes.  This
guard prevents the vocabulary from becoming a production capture/commit path
before Candidate0-P0 is complete.
"""

from __future__ import annotations

import pathlib


ROOT = pathlib.Path(__file__).resolve().parents[3]
CANDIDATE = ROOT / "src/mir/builder/module_lowering_invocation_candidate.rs"
CANDIDATE_P0 = ROOT / "src/mir/builder/module_lowering_invocation_candidate_p0.rs"
MAIN_EXPANSION = ROOT / "src/mir/builder/main_expansion.rs"
BUILDER_MOD = ROOT / "src/mir/builder.rs"
CARD = ROOT / (
    "docs/development/current/main/investigations/"
    "mirbuilder-headerport-i0-production-cutover-consultation-2026-07-21.md"
)
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def forbid(text: str, fragment: str, label: str) -> None:
    if fragment in text:
        raise AssertionError(f"forbidden {label}: {fragment!r}")


def main() -> int:
    candidate = CANDIDATE.read_text()
    candidate_p0 = CANDIDATE_P0.read_text()
    main_expansion = MAIN_EXPANSION.read_text()
    builder_mod = BUILDER_MOD.read_text()
    card = CARD.read_text()
    state = STATE.read_text()

    if len(candidate.splitlines()) >= 800:
        raise AssertionError("Candidate0 source must remain below 800 lines")
    if len(candidate_p0.splitlines()) >= 800:
        raise AssertionError("Candidate0 P0 source must remain below 800 lines")
    if len(main_expansion.splitlines()) >= 800:
        raise AssertionError("MAINROLE0-S0 source must remain below 800 lines")

    for fragment in (
        "VerifiedMainExpansionV1",
        "VerifiedMainRootBodyV1",
        "VerifiedMainStaticChildV1",
        "callable_main_compat",
        "MainExpansionErrorV1",
        "app_shape_ignores_non_main_top_level_statements",
        "script_shape_without_static_main_stays_out_of_this_product",
        "child_and_root_static_contracts_are_checked_before_builder_effects",
        "duplicate_main_boxes_are_rejected_without_order_dependence",
    ):
        require(main_expansion, fragment, "MAINROLE0-S0/P0 source product/fixtures")

    for path in (ROOT / "src/mir/builder").rglob("*.rs"):
        if path in (MAIN_EXPANSION, BUILDER_MOD) or path.name.endswith("_tests.rs"):
            continue
        if "VerifiedMainExpansionV1" in path.read_text():
            raise AssertionError(
                f"MAINROLE0-S0 production consumer exists: {path.relative_to(ROOT)}"
            )

    for fragment in (
        "ModuleLoweringInvocationCandidateV1",
        "InvocationCandidateFailureStageV1",
        "InvocationCandidateAbortProofV1",
        "with_active_lowering",
        "boundary_unchanged",
        "InvocationCandidatePublicationV1::Unchanged",
        "InvocationCandidateRetryV1::Forbidden",
        "pub(in crate::mir::builder) fn abort",
        "pub(in crate::mir::builder) fn discard",
        "candidate_owns_shell_and_collector_until_abort",
        "builder_borrow_is_scoped_to_active_lowering_only",
    ):
        require(candidate, fragment, "Candidate0-S0 vocabulary/fixture")
    for fragment in (
        "InvocationCandidateRouteProofBuilderV1",
        "InvocationCandidateRouteProofV1",
        "UnexpectedRoute",
        "candidate_abort_proof_co_seals_all_nine_route_rows",
        "duplicate_route_is_rejected_before_seal",
        "InvocationRouteMatrixV1::rows()",
        "InvocationCandidatePublicationV1::Unchanged",
        "InvocationCandidateRetryV1::Forbidden",
    ):
        require(candidate_p0, fragment, "Candidate0-P0 route co-seal/fixture")

    # The candidate may mention MirBuilder only as a short-lived method
    # parameter.  It must not store a Builder or expose a module map.
    struct = candidate.split("pub(in crate::mir::builder) struct ModuleLoweringInvocationCandidateV1", 1)[1]
    struct = struct.split("impl ModuleLoweringInvocationCandidateV1", 1)[0]
    forbid(struct, "MirBuilder", "candidate-stored Builder")
    forbid(candidate, "self.current_module", "candidate ambient module authority")
    forbid(candidate, "builder.current_module", "candidate ambient module authority")
    forbid(candidate, "ModuleLoweringPortV1", "candidate-owned collector port")
    forbid(candidate, "fn retry(", "candidate retry implementation")

    require(builder_mod, "mod module_lowering_invocation_candidate;", "Candidate0 module registration")
    other_builder_files = []
    for path in (ROOT / "src/mir/builder").rglob("*.rs"):
        if path in (CANDIDATE, CANDIDATE_P0, BUILDER_MOD):
            continue
        if "ModuleLoweringInvocationCandidateV1" in path.read_text():
            other_builder_files.append(str(path.relative_to(ROOT)))
    if other_builder_files:
        raise AssertionError(
            "Candidate0 production/test consumer exists outside the disconnected owner: "
            + ", ".join(other_builder_files)
        )
    for symbol in (
        "InvocationCandidateRouteProofBuilderV1",
        "InvocationCandidateRouteProofV1",
    ):
        for path in (ROOT / "src/mir/builder").rglob("*.rs"):
            if path in (CANDIDATE_P0, BUILDER_MOD) or path.name.endswith("_tests.rs"):
                continue
            if symbol in path.read_text():
                raise AssertionError(
                    f"Candidate0-P0 production consumer exists: {path.relative_to(ROOT)}"
                )

    for fragment in (
        "HEADERPORT0-REENTRANT-TERM0-I0-CANDIDATE0-S0 (closed)",
        "HEADERPORT0-REENTRANT-TERM0-I0-CANDIDATE0-P0 (closed)",
        "M-root-prime decision lock and task order",
        "HEADERPORT0-I0-MAINROLE0-S0/P0 (closed)",
        "HEADERPORT0-I0-BODYDRAIN0-S0/P0\n  next code-facing row",
        "one disconnected invocation-owned shell/collector candidate",
        "typed abort/no-publication/no-retry proof",
        "production capture/commit remains forbidden",
        "`CUT0` remains forbidden",
    ):
        require(card, fragment, "Candidate0 task boundary")
    require(
        state,
        "HEADERPORT0-I0-BODYDRAIN0-S0 is next",
        "current Candidate0/MainROLE0 pointer",
    )

    print(
        "[headerport-candidate0-guard] ok disconnected=1 "
        f"source_lines={len(candidate.splitlines())} production_consumers=0"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
