"""Report assembly for mimalloc algorithm coverage."""

from __future__ import annotations

from pathlib import Path

from hako_mimalloc_algorithm_coverage_context import build_coverage_report_context
from hako_mimalloc_algorithm_coverage_model_fields import build_model_report_fields
from hako_mimalloc_algorithm_coverage_perf_fields import build_perf_report_fields
from hako_mimalloc_algorithm_coverage_summary_fields import build_summary_report_fields
from hako_mimalloc_algorithm_coverage_support import CoverageRow


def report_dict(
    rows: list[CoverageRow],
    *,
    benchmark_report: Path | None = None,
    fastpath_report: Path | None = None,
    state_report: Path | None = None,
    perf_attribution_report: Path | None = None,
    accumulator_report: Path | None = None,
) -> dict[str, object]:
    context = build_coverage_report_context(
        rows,
        benchmark_report=benchmark_report,
        fastpath_report=fastpath_report,
        state_report=state_report,
        perf_attribution_report=perf_attribution_report,
        accumulator_report=accumulator_report,
    )
    return {
        **build_summary_report_fields(context=context),
        **build_perf_report_fields(context["measurement_state"], context["perf_attribution"]),
        **build_model_report_fields(context=context),
    }
