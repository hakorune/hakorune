#!/usr/bin/env python3
"""Join before/after source and measurement reports for one keeper."""

from __future__ import annotations

import argparse
from pathlib import Path


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def as_int(values: dict[str, str], key: str, default: int = 0) -> int:
    text = values.get(key)
    if text is None or text == "":
        return default
    try:
        return int(text)
    except ValueError:
        return default


def effect_from_delta(before_ms: int, after_ms: int, source_delta_ready: bool) -> str:
    if before_ms <= 0 or after_ms <= 0:
        return "inconclusive"
    if after_ms < before_ms:
        return "accepted"
    if after_ms == before_ms and source_delta_ready:
        return "no_effect"
    if after_ms > before_ms and source_delta_ready:
        return "regressed"
    return "inconclusive"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--keeper-id", required=True)
    parser.add_argument("--before-source", type=Path, required=True)
    parser.add_argument("--after-source", type=Path, required=True)
    parser.add_argument("--before-measurement", type=Path, required=True)
    parser.add_argument("--after-measurement", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    before_source = read_kv(args.before_source)
    after_source = read_kv(args.after_source)
    before_measurement = read_kv(args.before_measurement)
    after_measurement = read_kv(args.after_measurement)

    source_surface_delta_ready = int(
        before_source.get("output_contract", "").startswith("hako-check-perf-surface")
        and after_source.get("output_contract", "").startswith("hako-check-perf-surface")
    )
    measurement_delta_ready = int(
        before_measurement.get("summary") == "ok" and after_measurement.get("summary") == "ok"
    )

    before_loop_array_get = as_int(before_source, "loop_array_get_count")
    after_loop_array_get = as_int(after_source, "loop_array_get_count")
    before_loop_field_get = as_int(before_source, "loop_field_get_count")
    after_loop_field_get = as_int(after_source, "loop_field_get_count")
    before_elapsed = as_int(before_measurement, "after_hako_elapsed_median_ms")
    after_elapsed = as_int(after_measurement, "after_hako_elapsed_median_ms")
    keeper_effect = effect_from_delta(before_elapsed, after_elapsed, source_surface_delta_ready == 1)

    lines = [
        "output_contract=hako-mimalloc-keeper-before-after-diff-v0",
        f"keeper_id={args.keeper_id}",
        f"source_surface_delta_ready={source_surface_delta_ready}",
        f"measurement_delta_ready={measurement_delta_ready}",
        f"before_source_contract={before_source.get('output_contract', '')}",
        f"after_source_contract={after_source.get('output_contract', '')}",
        f"before_measurement_contract={before_measurement.get('output_contract', '')}",
        f"after_measurement_contract={after_measurement.get('output_contract', '')}",
        f"before_loop_array_get_count={before_loop_array_get}",
        f"after_loop_array_get_count={after_loop_array_get}",
        f"delta_loop_array_get_count={after_loop_array_get - before_loop_array_get}",
        f"before_loop_field_get_count={before_loop_field_get}",
        f"after_loop_field_get_count={after_loop_field_get}",
        f"delta_loop_field_get_count={after_loop_field_get - before_loop_field_get}",
        f"before_hako_elapsed_median_ms={before_elapsed}",
        f"after_hako_elapsed_median_ms={after_elapsed}",
        f"delta_hako_elapsed_median_ms={after_elapsed - before_elapsed}",
        f"keeper_effect={keeper_effect}",
        "winner_claim=0",
        "summary=ok",
    ]
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
