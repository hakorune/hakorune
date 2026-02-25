#!/usr/bin/env python3
"""Entry-mode stability aggregation helpers for phase21.5."""

from __future__ import annotations

import json
import pathlib
import statistics
from typing import Any


def parse_entry_mode_log(path: pathlib.Path) -> dict[str, Any] | None:
    text = path.read_text()
    for raw in text.splitlines():
        line = raw.strip()
        if not line or not line.startswith("{"):
            continue
        try:
            payload = json.loads(line)
        except json.JSONDecodeError:
            continue
        if not isinstance(payload, dict):
            continue
        # When --json-lines is used, keep only summary line.
        if payload.get("kind") == "sample":
            continue
        source_total = int(payload.get("source_total_ms", 0) or 0)
        prebuilt_total = int(payload.get("mir_shape_prebuilt_total_ms", 0) or 0)
        delta_abs = int(payload.get("delta_ms_abs", 0) or 0)
        source_cases = payload.get("source_cases_ms", {})
        prebuilt_cases = payload.get("mir_shape_prebuilt_cases_ms", {})
        case_delta = payload.get("case_delta_ms", {})
        hotspot = payload.get("hotspot_case_delta", {})
        if (
            source_total > 0
            and prebuilt_total > 0
            and isinstance(source_cases, dict)
            and isinstance(prebuilt_cases, dict)
            and source_cases
            and prebuilt_cases
            and isinstance(case_delta, dict)
            and case_delta
            and isinstance(hotspot, dict)
        ):
            return payload
    return None


def collect_entry_mode_aggregate(out_dir: pathlib.Path) -> dict[str, Any]:
    source_totals: list[int] = []
    prebuilt_totals: list[int] = []
    delta_abs_vals: list[int] = []
    source_by_name: dict[str, list[int]] = {}
    prebuilt_by_name: dict[str, list[int]] = {}
    delta_by_name: dict[str, list[int]] = {}
    thresholds: list[int] = []

    for entry_log in sorted(out_dir.glob("apps_entry_mode.round*.json")):
        payload = parse_entry_mode_log(entry_log)
        if payload is None:
            continue
        source_total = int(payload.get("source_total_ms", 0) or 0)
        prebuilt_total = int(payload.get("mir_shape_prebuilt_total_ms", 0) or 0)
        delta_abs = int(payload.get("delta_ms_abs", 0) or 0)
        if source_total > 0:
            source_totals.append(source_total)
        if prebuilt_total > 0:
            prebuilt_totals.append(prebuilt_total)
        if delta_abs >= 0:
            delta_abs_vals.append(delta_abs)
        threshold = int(payload.get("significance_ms_threshold", 0) or 0)
        if threshold >= 0:
            thresholds.append(threshold)

        source_cases = payload.get("source_cases_ms", {})
        prebuilt_cases = payload.get("mir_shape_prebuilt_cases_ms", {})
        case_delta = payload.get("case_delta_ms", {})

        if isinstance(source_cases, dict):
            for name, value in source_cases.items():
                iv = int(value)
                if iv > 0:
                    source_by_name.setdefault(str(name), []).append(iv)
        if isinstance(prebuilt_cases, dict):
            for name, value in prebuilt_cases.items():
                iv = int(value)
                if iv > 0:
                    prebuilt_by_name.setdefault(str(name), []).append(iv)
        if isinstance(case_delta, dict):
            for name, value in case_delta.items():
                iv = int(value)
                delta_by_name.setdefault(str(name), []).append(iv)

    return {
        "source_totals": source_totals,
        "prebuilt_totals": prebuilt_totals,
        "delta_abs_vals": delta_abs_vals,
        "thresholds": thresholds,
        "source_by_name": source_by_name,
        "prebuilt_by_name": prebuilt_by_name,
        "delta_by_name": delta_by_name,
    }


def build_entry_mode_baseline_fields(aggregate: dict[str, Any]) -> dict[str, Any]:
    source_totals = aggregate["source_totals"]
    prebuilt_totals = aggregate["prebuilt_totals"]
    delta_abs_vals = aggregate["delta_abs_vals"]
    thresholds = aggregate["thresholds"]
    source_by_name = aggregate["source_by_name"]
    prebuilt_by_name = aggregate["prebuilt_by_name"]
    delta_by_name = aggregate["delta_by_name"]

    source_total_ms = int(statistics.median(sorted(source_totals))) if source_totals else 0
    prebuilt_total_ms = int(statistics.median(sorted(prebuilt_totals))) if prebuilt_totals else 0
    delta_abs_ms = int(statistics.median(sorted(delta_abs_vals))) if delta_abs_vals else 0
    significance_ms_threshold = int(statistics.median(sorted(thresholds))) if thresholds else 0
    source_per_app_ms = {
        app_name: int(statistics.median(sorted(vals)))
        for app_name, vals in source_by_name.items()
        if vals
    }
    prebuilt_per_app_ms = {
        app_name: int(statistics.median(sorted(vals)))
        for app_name, vals in prebuilt_by_name.items()
        if vals
    }
    case_delta_ms = {
        app_name: int(statistics.median(sorted(vals)))
        for app_name, vals in delta_by_name.items()
        if vals
    }
    hotspot_case = ""
    hotspot_delta_abs_ms = 0
    for app_name in sorted(case_delta_ms.keys()):
        abs_delta = abs(int(case_delta_ms[app_name]))
        if abs_delta > hotspot_delta_abs_ms:
            hotspot_delta_abs_ms = abs_delta
            hotspot_case = app_name

    return {
        "apps_entry_mode_source_total_ms": source_total_ms,
        "apps_entry_mode_prebuilt_total_ms": prebuilt_total_ms,
        "apps_entry_mode_delta_abs_ms": delta_abs_ms,
        "apps_entry_mode_significance_ms_threshold": significance_ms_threshold,
        "apps_entry_mode_source_per_app_ms": source_per_app_ms,
        "apps_entry_mode_prebuilt_per_app_ms": prebuilt_per_app_ms,
        "apps_entry_mode_case_delta_ms": case_delta_ms,
        "apps_entry_mode_hotspot_case": hotspot_case,
        "apps_entry_mode_hotspot_delta_abs_ms": hotspot_delta_abs_ms,
    }
