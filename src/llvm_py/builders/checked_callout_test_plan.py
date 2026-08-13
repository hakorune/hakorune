"""Closed-mode validation for the neutral CheckedCallOut fixture.

This module is deliberately a test-only observation seam.  It joins the
existing transport parser with the existing symbolic AOT admission loader so
focused tests can prove that a fixture contains the two site-local corridors
expected by W6.  It does not emit LLVM, choose a provider, or become a runtime
plan; production dispatchers must not import it.
"""

from dataclasses import dataclass
from typing import Any, Mapping, Sequence, Tuple

from builders.checked_callout_transport import (
    CheckedCallOutEndView,
    CheckedCallOutFaultView,
    CheckedCallOutNormalResultView,
    CheckedCallOutTerminatorView,
    CheckedCallOutTransportError,
    parse_checked_callout_transport,
)
from builders.dynamic_v2_aot_admission import (
    DynamicV2AotAdmissionError,
    DynamicV2AotAdmissionView,
    load_selected_dynamic_v2_aot_admission,
)


class CheckedCallOutTestPlanError(ValueError):
    """The closed-mode fixture is incomplete or internally inconsistent."""


@dataclass(frozen=True)
class CheckedCallOutTestValidationReport:
    """Borrow-free diagnostics for one complete two-site fixture.

    This report is intentionally not accepted by any lowering or link API.
    It records only facts that the focused test can observe from transport and
    the already-issued admission view.
    """

    site_ids: Tuple[int, int]
    normal_landing_pairs: Tuple[Tuple[int, int], Tuple[int, int]]
    normal_projection_sites: Tuple[int, int]
    end_sites: Tuple[int, ...]
    plan_stamp: Tuple[int, int]


def _admission(function_data: Mapping[str, Any]) -> DynamicV2AotAdmissionView:
    try:
        view = load_selected_dynamic_v2_aot_admission(dict(function_data))
    except DynamicV2AotAdmissionError as error:
        raise CheckedCallOutTestPlanError(str(error)) from error
    if view is None:
        raise CheckedCallOutTestPlanError("fixture requires selected AOT admission")
    return view


def _parse_operations(
    operations: Sequence[Mapping[str, Any]],
) -> Tuple[
    Tuple[CheckedCallOutTerminatorView, ...],
    Tuple[CheckedCallOutNormalResultView, ...],
    Tuple[CheckedCallOutEndView, ...],
    Tuple[CheckedCallOutFaultView, ...],
]:
    if not isinstance(operations, (list, tuple)):
        raise CheckedCallOutTestPlanError("fixture operations must be a sequence")
    parsed = []
    try:
        for operation in operations:
            parsed.append(parse_checked_callout_transport(operation))
    except CheckedCallOutTransportError as error:
        raise CheckedCallOutTestPlanError(str(error)) from error
    return (
        tuple(item for item in parsed if isinstance(item, CheckedCallOutTerminatorView)),
        tuple(item for item in parsed if isinstance(item, CheckedCallOutNormalResultView)),
        tuple(item for item in parsed if isinstance(item, CheckedCallOutEndView)),
        tuple(item for item in parsed if isinstance(item, CheckedCallOutFaultView)),
    )


def _unique_by_site(items: Sequence[Any], label: str) -> dict[int, Any]:
    result = {}
    for item in items:
        if item.site_id in result:
            raise CheckedCallOutTestPlanError(f"duplicate {label} site: {item.site_id}")
        result[item.site_id] = item
    return result


def validate_checked_callout_test_fixture(
    operations: Sequence[Mapping[str, Any]],
    function_data: Mapping[str, Any],
) -> CheckedCallOutTestValidationReport:
    """Validate a complete two-site CheckedCallOut fixture.

    Site ``0`` is the EndAuthorized TextSliceRange corridor and site ``1`` is
    the ImmediateI64 TextFindNeedle corridor, matching the already-admitted
    neutral MIR pair.  The admission view is borrowed only for its existing
    ABI/PlanStamp facts; no selector or block/index lookup is introduced here.
    """

    admission = _admission(function_data)
    terminators, projections, ends, faults = _parse_operations(operations)
    if len(terminators) != 2:
        raise CheckedCallOutTestPlanError("fixture requires exactly two CheckedCallOut terminators")
    if len(projections) != 2:
        raise CheckedCallOutTestPlanError("fixture requires exactly two Normal projections")
    if len(ends) != 1:
        raise CheckedCallOutTestPlanError("fixture requires exactly one End operation")
    if len(faults) != 2:
        raise CheckedCallOutTestPlanError("fixture requires one Fault operation per site")

    terminator_by_site = _unique_by_site(terminators, "terminator")
    projection_by_site = _unique_by_site(projections, "Normal projection")
    end_by_site = _unique_by_site(ends, "End")
    fault_by_site = _unique_by_site(faults, "Fault")
    site_ids = tuple(sorted(terminator_by_site))
    if site_ids != (0, 1):
        raise CheckedCallOutTestPlanError("fixture sites must be the canonical pair (0, 1)")
    if set(projection_by_site) != set(terminator_by_site):
        raise CheckedCallOutTestPlanError("Normal projections must cover both terminators")
    if set(fault_by_site) != set(terminator_by_site):
        raise CheckedCallOutTestPlanError("Fault terminals must cover both terminators")
    end = end_by_site.get(0)
    if end is None or end.lease_slot != 0 or 1 in end_by_site:
        raise CheckedCallOutTestPlanError("only site 0 may consume lease slot 0")
    if {call.role for call in admission.calls} != {"substring", "index_of"}:
        raise CheckedCallOutTestPlanError("admission must cover both TextScan roles")

    landing_pairs = tuple(
        (terminator_by_site[site].normal_landing, terminator_by_site[site].fault_landing)
        for site in site_ids
    )
    return CheckedCallOutTestValidationReport(
        site_ids=site_ids,
        normal_landing_pairs=landing_pairs,
        normal_projection_sites=tuple(sorted(projection_by_site)),
        end_sites=tuple(sorted(end_by_site)),
        plan_stamp=(admission.compiler_domain, admission.invocation_ordinal),
    )


__all__ = [
    "CheckedCallOutTestPlanError",
    "CheckedCallOutTestValidationReport",
    "validate_checked_callout_test_fixture",
]
