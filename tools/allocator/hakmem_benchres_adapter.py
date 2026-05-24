#!/usr/bin/env python3
"""Adapt hakmem mimalloc-bench benchres.csv into phase-295x-style KV evidence."""

from __future__ import annotations

import argparse
from pathlib import Path


def fail(message: str) -> None:
    raise SystemExit(f"[hakmem-benchres-adapter] {message}")


def parse_elapsed_ms(text: str) -> int:
    value = text.strip()
    if ":" in value:
        minutes, seconds = value.split(":", 1)
        return int(round((int(minutes) * 60.0 + float(seconds)) * 1000.0))
    return int(round(float(value) * 1000.0))


def parse_benchres(path: Path) -> list[dict[str, str]]:
    rows: list[dict[str, str]] = []
    for raw in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = raw.strip()
        if not line or line.startswith("#") or line.startswith("Command "):
            continue
        parts = line.split()
        if len(parts) < 8:
            rows.append({"raw": line, "parse_status": "short"})
            continue
        rows.append(
            {
                "benchmark": parts[0],
                "allocator": normalize_allocator(parts[1]),
                "allocator_raw": parts[1],
                "elapsed": parts[2],
                "elapsed_ms": str(parse_elapsed_ms(parts[2])),
                "peak_rss_bytes": str(int(parts[3]) * 1024),
                "rss_kb": parts[3],
                "user_sec": parts[4],
                "sys_sec": parts[5],
                "major_faults": parts[6],
                "minor_faults": parts[7],
                "parse_status": "ok",
            }
        )
    return rows


def normalize_allocator(name: str) -> str:
    aliases = {
        "mi": "mimalloc",
        "tc": "tcmalloc",
        "sys": "system",
    }
    return aliases.get(name, name)


def emit(rows: list[dict[str, str]], source: Path) -> str:
    parsed = [row for row in rows if row.get("parse_status") == "ok"]
    benchmarks = sorted({row["benchmark"] for row in parsed})
    allocators = sorted({row["allocator"] for row in parsed})
    lines = [
        "output_contract=hakmem-external-benchres-adapter-v0",
        "dataset_role=external-historical-benchmark-corpus",
        f"source_path={source}",
        f"row_count={len(rows)}",
        f"parsed_row_count={len(parsed)}",
        "benchmarks=" + ",".join(benchmarks),
        "allocators=" + ",".join(allocators),
        "elapsed_unit=ms",
        "rss_unit=bytes",
        "winner_claim=0",
        "provider_activation=0",
        "host_replacement=0",
        "hook_installed=0",
        "global_allocator_installed=0",
    ]
    for idx, row in enumerate(parsed[:64]):
        prefix = f"row_{idx}"
        for key in [
            "benchmark",
            "allocator",
            "allocator_raw",
            "elapsed_ms",
            "peak_rss_bytes",
            "user_sec",
            "sys_sec",
            "major_faults",
            "minor_faults",
        ]:
            lines.append(f"{prefix}_{key}={row[key]}")
    lines.append("summary=ok")
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--in", dest="input", type=Path, required=True)
    parser.add_argument("--out", type=Path, default=None)
    args = parser.parse_args()

    source = args.input.resolve()
    if not source.exists():
        fail(f"missing input: {source}")
    rows = parse_benchres(source)
    report = emit(rows, source)
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
