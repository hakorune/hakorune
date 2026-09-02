#!/usr/bin/env python3
"""Measure MIR compile stages for generated method and loop-bound shapes."""

from __future__ import annotations

import argparse
import hashlib
import json
import math
import os
import pathlib
import re
import subprocess
import sys
import tempfile
import time
from statistics import median
from typing import Any


ROOT = pathlib.Path(__file__).resolve().parents[2]
TIMING_RE = re.compile(
    r"\[mir-compile/timing\] stage=([^ ]+) (?:elapsed_ms|count)=([0-9]+)"
)
OBSERVED_ENV_KEYS = (
    "NYASH_DISABLE_PLUGINS",
    "NYASH_JOINIR_LOWER_GENERIC",
    "NYASH_MIR_COMPILE_TRACE",
    "NYASH_MIR_LOOP_HOIST",
)
SHADOW_STAGE_KEYS = {
    "semantic.route.shadow.dirty_functions",
    "semantic.route.shadow.recomputed_functions",
    "semantic.route.shadow.unchanged_function_recomputes",
    "semantic.route.shadow.family_recomputes",
    "semantic.route.shadow.dependency_edges",
    "semantic.route.shadow.worklist_hash",
    "semantic.route.shadow.parity_mismatches",
}


def shadow_contract_error(stages: dict[str, int]) -> str:
    missing = sorted(SHADOW_STAGE_KEYS.difference(stages))
    if missing:
        return "missing:" + ",".join(missing)
    if stages["semantic.route.shadow.family_recomputes"] <= 0:
        return "empty_family_recomputes"
    if stages["semantic.route.shadow.dependency_edges"] <= 0:
        return "empty_dependency_edges"
    if stages["semantic.route.shadow.worklist_hash"] <= 0:
        return "empty_worklist_hash"
    dirty = stages["semantic.route.shadow.dirty_functions"]
    recomputed = stages["semantic.route.shadow.recomputed_functions"]
    unchanged = stages["semantic.route.shadow.unchanged_function_recomputes"]
    if recomputed < dirty or unchanged != recomputed - dirty:
        return "function_recompute_accounting_mismatch"
    if stages["semantic.route.shadow.parity_mismatches"] != 0:
        return "full_refresh_parity_mismatch"
    return ""


def method_source(method_count: int) -> str:
    methods = "\n".join(
        f"  value_{index}() {{ return {index} }}" for index in range(method_count)
    )
    # The current canonical root path accepts static Main as the owning box.
    # Keep the generated shape otherwise boring so the count is the measured
    # variable rather than an invalid compatibility-root detour.
    return f"static box Main {{\n{methods}\n  main() {{ return 0 }}\n}}\n"


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
static box Main {{
  main() {{ return 0 }}
}}
"""


def sha256_bytes(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def sha256_file(path: pathlib.Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b""):
            digest.update(chunk)
    return digest.hexdigest()


def binary_identity(binary: pathlib.Path, profile: str) -> dict[str, Any]:
    resolved = binary.resolve()
    stat = resolved.stat()
    return {
        "path": str(resolved),
        "profile": profile,
        "sha256": sha256_file(resolved),
        "bytes": stat.st_size,
    }


def source_identity(source: str) -> dict[str, Any]:
    encoded = source.encode("utf-8")
    return {
        "sha256": sha256_bytes(encoded),
        "bytes": len(encoded),
        "lines": len(source.splitlines()),
    }


def percentile_nearest_rank(values: list[int], percentile: float) -> int:
    if not values:
        raise ValueError("percentile requires at least one value")
    rank = max(1, math.ceil(percentile * len(values)))
    return sorted(values)[rank - 1]


def aggregate_stage_runs(stage_runs: list[dict[str, int]]) -> dict[str, int]:
    keys = sorted({key for stages in stage_runs for key in stages})
    return {
        key: int(median([stages[key] for stages in stage_runs if key in stages]))
        for key in keys
        if all(key in stages for stages in stage_runs)
    }


def observed_environment(env: dict[str, str]) -> dict[str, str | None]:
    return {key: env.get(key) for key in OBSERVED_ENV_KEYS}


def environment_identity(env: dict[str, str]) -> dict[str, Any]:
    relevant = sorted(
        (key, value)
        for key, value in env.items()
        if key.startswith(("NYASH_", "HAKO_"))
    )
    encoded = "\n".join(f"{key}={value}" for key, value in relevant).encode("utf-8")
    return {
        "selected": observed_environment(env),
        "relevant_sha256": sha256_bytes(encoded),
        "relevant_keys": [key for key, _ in relevant],
    }


def _run_once(
    binary: pathlib.Path,
    source_path: pathlib.Path,
    timeout_seconds: float,
    env: dict[str, str],
) -> dict[str, Any]:
    started = time.monotonic()
    try:
        completed = subprocess.run(
            [str(binary), "--dump-mir", str(source_path)],
            cwd=ROOT,
            env=env,
            text=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
            timeout=timeout_seconds,
            check=False,
        )
    except subprocess.TimeoutExpired as error:
        stderr = error.stderr or ""
        if isinstance(stderr, bytes):
            stderr = stderr.decode("utf-8", errors="replace")
        return {
            "status": "timeout",
            "elapsed_ms": int((time.monotonic() - started) * 1000),
            "stderr_tail": stderr.splitlines()[-8:],
        }
    return {
        "status": "ok" if completed.returncode == 0 else "error",
        "returncode": completed.returncode,
        "elapsed_ms": int((time.monotonic() - started) * 1000),
        "stages": {
            match.group(1): int(match.group(2))
            for match in TIMING_RE.finditer(completed.stderr)
        },
        "stderr_tail": completed.stderr.splitlines()[-8:],
    }


def run_probe(
    binary: pathlib.Path,
    source: str,
    timeout_seconds: float,
    *,
    warmup_runs: int = 1,
    repeat_runs: int = 5,
    require_shadow_contract: bool = True,
    env_overrides: dict[str, str] | None = None,
) -> dict[str, Any]:
    if warmup_runs < 0:
        raise ValueError("warmup_runs must be non-negative")
    if repeat_runs <= 0:
        raise ValueError("repeat_runs must be positive")

    run_env = os.environ.copy()
    run_env.update(env_overrides or {})
    # The generated static-method probe has no plugin dependency.  Pin the
    # probe to the same no-plugin compiler route used by the existing reader
    # measurement instead of letting a developer's ambient plugin set change
    # the observation.
    run_env["NYASH_DISABLE_PLUGINS"] = "1"
    run_env["NYASH_MIR_COMPILE_TRACE"] = "1"
    with tempfile.TemporaryDirectory(prefix="mir-compile-scaling-") as temp_dir:
        source_path = pathlib.Path(temp_dir) / "probe.hako"
        source_path.write_text(source, encoding="utf-8")
        warmups = [
            _run_once(binary, source_path, timeout_seconds, run_env)
            for _ in range(warmup_runs)
        ]
        failed_warmup = next((run for run in warmups if run["status"] != "ok"), None)
        if failed_warmup is not None:
            return {
                "status": failed_warmup["status"],
                "phase": "warmup",
                "warmup_runs": warmup_runs,
                "repeat_runs": repeat_runs,
                "warmup_results": warmups,
                "retained_results": [],
                "elapsed_ms_runs": [],
                "elapsed_ms_median": None,
                "elapsed_ms_p95": None,
                "stages": {},
                "stage_runs": [],
                "shadow_contract_error": "missing:all"
                if require_shadow_contract
                else "",
                "shadow_contract_checked": require_shadow_contract,
                "environment": environment_identity(run_env),
                **source_identity(source),
            }
        retained = [
            _run_once(binary, source_path, timeout_seconds, run_env)
            for _ in range(repeat_runs)
        ]

    failed_retained = next((run for run in retained if run["status"] != "ok"), None)
    stage_runs = [run.get("stages", {}) for run in retained if run["status"] == "ok"]
    elapsed_runs = [run["elapsed_ms"] for run in retained if run["status"] == "ok"]
    timings = aggregate_stage_runs(stage_runs) if stage_runs else {}
    shadow_error = shadow_contract_error(timings) if timings else "missing:all"
    status = "ok"
    if failed_retained is not None:
        status = failed_retained["status"]
    elif require_shadow_contract and shadow_error:
        status = "error"
    result: dict[str, Any] = {
        "status": status,
        "warmup_runs": warmup_runs,
        "repeat_runs": repeat_runs,
        "warmup_results": warmups,
        "retained_results": retained,
        "elapsed_ms_runs": elapsed_runs,
        "elapsed_ms_median": int(median(elapsed_runs)) if elapsed_runs else None,
        "elapsed_ms_p95": percentile_nearest_rank(elapsed_runs, 0.95)
        if elapsed_runs
        else None,
        "stages": timings,
        "stage_runs": stage_runs,
        "shadow_contract_error": shadow_error if require_shadow_contract else "",
        "shadow_contract_checked": require_shadow_contract,
        "environment": environment_identity(run_env),
        **source_identity(source),
    }
    if failed_retained is not None:
        result["failure_phase"] = "retained"
    return result


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=pathlib.Path, default=ROOT / "target/quick/hakorune")
    parser.add_argument("--profile-label", default="quick")
    parser.add_argument("--method-counts", default="50,100,250")
    parser.add_argument("--include-loop-bounds", action="store_true")
    parser.add_argument("--timeout-sec", type=float, default=120.0)
    parser.add_argument("--warmup-runs", type=int, default=1)
    parser.add_argument("--repeat-runs", type=int, default=5)
    parser.add_argument(
        "--allow-missing-shadow-contract",
        action="store_true",
        help="allow a successful compile without the shadow observation rows",
    )
    args = parser.parse_args()

    binary = args.bin.resolve()
    if not binary.is_file():
        parser.error(f"compiler binary does not exist: {binary}")
    if args.warmup_runs < 0 or args.repeat_runs <= 0:
        parser.error("--warmup-runs must be >= 0 and --repeat-runs must be > 0")

    results = []
    for token in args.method_counts.split(","):
        count = int(token)
        results.append(
            {
                "shape": "static_methods",
                "method_count": count,
                **run_probe(
                    binary,
                    method_source(count),
                    args.timeout_sec,
                    warmup_runs=args.warmup_runs,
                    repeat_runs=args.repeat_runs,
                    require_shadow_contract=not args.allow_missing_shadow_contract,
                ),
            }
        )
    if args.include_loop_bounds:
        for dynamic in (False, True):
            results.append(
                {
                    "shape": "dynamic_loop_bound" if dynamic else "literal_loop_bound",
                    **run_probe(
                        binary,
                        loop_source(dynamic),
                        args.timeout_sec,
                        warmup_runs=args.warmup_runs,
                        repeat_runs=args.repeat_runs,
                        require_shadow_contract=not args.allow_missing_shadow_contract,
                    ),
                }
            )

    print(
        json.dumps(
            {
                "schema": "mir-compile-scaling-v1",
                "runner": {
                    "python": sys.version.split()[0],
                    "binary": binary_identity(binary, args.profile_label),
                    "warmup_runs": args.warmup_runs,
                    "repeat_runs": args.repeat_runs,
                },
                "results": results,
            },
            sort_keys=True,
        )
    )
    return 0 if all(result["status"] == "ok" for result in results) else 2


if __name__ == "__main__":
    raise SystemExit(main())
