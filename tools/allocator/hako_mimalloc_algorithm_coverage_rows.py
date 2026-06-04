"""Row refinement helpers for mimalloc algorithm coverage reports."""

from __future__ import annotations

from dataclasses import replace

from hako_mimalloc_algorithm_coverage_support import CoverageRow


def refine_rows(
    rows: list[CoverageRow],
    *,
    product_bins_consumer_enabled: int,
    hotcore_consumer_enabled: int,
    hotcore_next_bridge: str,
) -> list[CoverageRow]:
    refined_rows: list[CoverageRow] = []
    for row in rows:
        if row.area == "size_class_policy" and product_bins_consumer_enabled:
            row = replace(
                row,
                replacement_front=1,
                status="split_model_and_fixed_front",
                evidence="size_class_box.hako + benchmark replacement-front size-class bridge",
                next_bridge="measure current size-class bridge or connect product pages",
            )
        if row.area == "object_lifecycle_hot_core" and hotcore_consumer_enabled:
            row = replace(
                row,
                replacement_front=1,
                status="split_model_and_fixed_front",
                evidence=(
                    "object_lifecycle_hot_core_box.hako + benchmark HotCore/PageModel front"
                ),
                next_bridge=hotcore_next_bridge,
            )
        refined_rows.append(row)
    return refined_rows
