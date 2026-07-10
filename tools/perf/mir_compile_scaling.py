#!/usr/bin/env python3
"""Measure MIR compile stages for generated method and loop-bound shapes."""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import re
import subprocess
import tempfile
import time


ROOT = pathlib.Path(__file__).resolve().parents[2]
TIMING_RE = re.compile(
    r"\[mir-compile/timing\] stage=([^ ]+) (?:elapsed_ms|count)=([0-9]+)"
)


def method_source(method_count: int) -> str:
    methods = "\n".join(
        f"  value_{index}() {{ return {index} }}" for index in range(method_count)
    )
    return f"static box ScalingProbe {{\n{methods}\n}}\n"


def loop_source(dynamic: bool) -> str:
    bound = "source.length() + 1" if dynamic else "200000"
    return f"""static box LoopBoundProbe {{
  scan(source) {{
    local i = 0
    local max = {bound}
    loop(i < max) {{ i = i + 1 }}
    return i
  }}
}}
"""


def run_probe(binary: pathlib.Path, source: str, timeout_seconds: float) -> dict:
    with tempfile.TemporaryDirectory(prefix="mir-compile-scaling-") as temp_dir:
        source_path = pathlib.Path(temp_dir) / "probe.hako"
        source_path.write_text(source, encoding="utf-8")
        started = time.monotonic()
        try:
            completed = subprocess.run(
                [str(binary), "--dump-mir", str(source_path)],
                cwd=ROOT,
                env=os.environ | {"NYASH_MIR_COMPILE_TRACE": "1"},
                text=True,
                stdout=subprocess.DEVNULL,
                stderr=subprocess.PIPE,
                timeout=timeout_seconds,
                check=False,
            )
        except subprocess.TimeoutExpired:
            return {
                "status": "timeout",
                "elapsed_ms": int((time.monotonic() - started) * 1000),
            }
    timings = {
        match.group(1): int(match.group(2))
        for match in TIMING_RE.finditer(completed.stderr)
    }
    return {
        "status": "ok" if completed.returncode == 0 else "error",
        "elapsed_ms": int((time.monotonic() - started) * 1000),
        "stages": timings,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=pathlib.Path, default=ROOT / "target/debug/hakorune")
    parser.add_argument("--method-counts", default="50,100,250")
    parser.add_argument("--include-loop-bounds", action="store_true")
    parser.add_argument("--timeout-sec", type=float, default=120.0)
    args = parser.parse_args()

    results = []
    for token in args.method_counts.split(","):
        count = int(token)
        results.append(
            {
                "shape": "static_methods",
                "method_count": count,
                **run_probe(args.bin, method_source(count), args.timeout_sec),
            }
        )
    if args.include_loop_bounds:
        for dynamic in (False, True):
            results.append(
                {
                    "shape": "dynamic_loop_bound" if dynamic else "literal_loop_bound",
                    **run_probe(args.bin, loop_source(dynamic), args.timeout_sec),
                }
            )

    print(
        json.dumps(
            {"schema": "mir-compile-scaling-v0", "results": results}, sort_keys=True
        )
    )
    return 0 if all(result["status"] == "ok" for result in results) else 2


if __name__ == "__main__":
    raise SystemExit(main())
