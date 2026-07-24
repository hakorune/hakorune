#!/usr/bin/env python3
"""COVERAGE0-REPAIR-S0 guard for the public StaticHelper0 profile."""

from __future__ import annotations

from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]
STATE = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
TASK = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-post0-public-cutover-coverage0-repair-s0-"
    "execution-task-2026-07-24.md"
)
DESIGN = ROOT / (
    "docs/development/current/main/investigations/"
    "cut0-i0-raw-source0-lower-root-post0-public-cutover-coverage0-design-"
    "question-2026-07-24.md"
)
SOURCES = {
    "coverage": ROOT / "src/mir/compiler/raw_root_helper_coverage.rs",
    "prepare": ROOT / "src/mir/compiler/raw_root_eligibility_prepare.rs",
    "ingress": ROOT / "src/mir/compiler/raw_public_ingress.rs",
    "ingress_tests": ROOT / "src/mir/compiler/raw_public_ingress_p0.rs",
    "eligibility": ROOT / "src/mir/compiler/raw_root_eligibility.rs",
    "children": ROOT / "src/mir/compiler/raw_root_children.rs",
    "coverage_tests": ROOT / "src/mir/compiler/raw_root_helper_coverage.rs",
}


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def main() -> int:
    state = STATE.read_text()
    task = TASK.read_text()
    design = DESIGN.read_text()
    texts = {name: path.read_text() for name, path in SOURCES.items()}

    require(state, "COVERAGE0-REPAIR-prime-r1", "selected repair decision")
    require(design, "Q1 = PLAN0", "PLAN0 authority decision")
    require(design, "Q2 = NarrowV1", "public profile decision")
    if "Status: active implementation" not in task and "Status: closed" not in task:
        raise AssertionError("repair task must be active or closed")
    for fragment in (
        "StaticHelper0",
        "params = []",
        "return_type_name = None",
        "body = []",
        "before physical effects",
        "HelperLinear0 activation = 0",
    ):
        require(task, fragment, f"task contract {fragment}")

    coverage = texts["coverage"]
    prepare = texts["prepare"]
    ingress = texts["ingress"]
    eligibility = texts["eligibility"]
    children = texts["children"]
    require(coverage, "RawStaticHelperCoverageV1", "coverage witness")
    require(coverage, "RawStaticHelper0CoverageErrorV1", "typed coverage error")
    require(coverage, "pub(in crate::mir) fn verify(", "coverage producer")
    require(coverage, "matches_locators", "coverage parity relation")
    require(coverage, "BodyNotEmpty", "empty-body rejection")
    require(eligibility, "verify_public", "public eligibility verifier")
    require(eligibility, "HelperCoverage", "typed eligibility handoff")
    require(eligibility, "into_helper_coverage", "consuming witness handoff")
    require(prepare, "prepare_public_eligibility", "public profile handoff")
    require(ingress, "prepare_public_eligibility", "public profile consumer")
    require(children, "proof.into_helper_coverage()", "CHILDREN0 witness consumer")
    require(children, "planned_locators", "PLAN0 schedule")
    require(children, "witness_locators.as_ref() != planned_locators.as_ref()", "exact locator parity")
    require(children, "let locators = planned_locators", "PLAN0 execution schedule")
    for fixture in (
        "exact_empty_static_helper_is_sealed_once",
        "locator_witness_parity_is_exact_and_ordered",
        "non_empty_helper_is_rejected_before_child_descent",
        "helper_parameters_are_rejected",
        "helper_metadata_is_rejected",
        "helper_uses_attrs_and_contracts_are_rejected",
        "helper_instance_and_override_methods_are_rejected",
    ):
        require(coverage, fixture, f"coverage fixture {fixture}")
    require(children, "app_consumes_the_projected_lexical_helper_schedule", "schedule fixture")
    require(children, "app_with_zero_helpers_keeps_a_typed_zero_child_schedule", "zero-helper fixture")
    require(
        children,
        "plan_and_witness_order_drift_rejects_before_child_descent",
        "plan/witness drift fixture",
    )
    require(children, "non_empty_helper_rejects_before_physical_effects", "pre-physical fixture")
    require(
        texts["ingress_tests"],
        "raw_public_ingress_rejects_nonempty_helper_before_physical_open",
        "public pre-physical fixture",
    )

    if coverage.count("pub(in crate::mir) fn verify(") != 1:
        raise AssertionError("StaticHelper0 coverage producer must be exactly one")
    if "RawStaticHelperCoverageV1::verify" in children:
        raise AssertionError("CHILDREN0 must not re-run helper coverage")
    if "let locators = helper_coverage.into_locators()" in children or "_planned_locators" in children:
        raise AssertionError("CHILDREN0 must execute PLAN0 locators")
    if "body.is_empty()" in children:
        raise AssertionError("CHILDREN0 must not own helper grammar policy")
    if "sorted_method_entries" in children:
        raise AssertionError("CHILDREN0 must not re-derive method order")
    if "HelperLinear0" in "".join(texts.values()):
        raise AssertionError("HelperLinear0 activation must remain zero")
    if any(len(text.splitlines()) >= 800 for text in [task, *texts.values()]):
        raise AssertionError("COVERAGE0 source/task files must remain below 800 lines")

    print(
        "[cut0-i0-root0-raw-source0-lower-root-post0-public-cutover-coverage0-guard] "
        "ok plan_authority=1 witness_parity=1 public_profile=1 pre_physical_reject=1 below_800=1"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
