#!/usr/bin/env python3
"""Diagnose fixed `.hako` empty exact-EXE footprint without winner claims."""

from __future__ import annotations

import argparse
import os
import re
import shutil
import statistics
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
EMPTY_EVIDENCE_APP = ROOT / "apps/hako-alloc-mimalloc-comparison-empty-exe-proof/main.hako"
EMPTY_NOIO_APP = ROOT / "apps/hako-alloc-mimalloc-comparison-empty-noio-exe-proof/main.hako"
REPEATED_RUNNER = ROOT / "tools/allocator/mimalloc_repeated_measurement_runner.py"
C_RUNNER_SRC = ROOT / "tools/allocator/c_mimalloc_explicit_runner.c"


def run(cmd: list[str], *, stdout: Path | None = None, cwd: Path = ROOT, env: dict[str, str] | None = None) -> str:
    if stdout is None:
        completed = subprocess.run(cmd, cwd=cwd, check=True, text=True, stdout=subprocess.PIPE, env=env)
        return completed.stdout
    with stdout.open("w", encoding="utf-8") as fh:
        subprocess.run(cmd, cwd=cwd, check=True, text=True, stdout=fh, env=env)
    return ""


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def as_int(values: dict[str, str], key: str) -> int:
    text = values.get(key, "0")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{key} must be an integer, got {text!r}") from exc


def ensure_binaries() -> None:
    if not (ROOT / "target/release/hakorune").exists() or not (ROOT / "target/release/ny-llvmc").exists():
        subprocess.run(
            ["cargo", "build", "--release", "--bin", "hakorune", "--bin", "ny-llvmc"],
            cwd=ROOT,
            check=True,
        )


def build_hako_exe(app: Path, tmp_dir: Path, label: str) -> Path:
    ensure_binaries()
    mir_json = tmp_dir / f"{label}.mir.json"
    exe_out = tmp_dir / f"{label}.exe"
    hako_env = os.environ.copy()
    hako_env.setdefault("NYASH_FEATURES", "rune")
    hako_env.setdefault("NYASH_DISABLE_PLUGINS", "1")
    run(
        [
            str(ROOT / "target/release/hakorune"),
            "--backend",
            "mir",
            "--emit-mir-json",
            str(mir_json),
            str(app),
        ],
        stdout=tmp_dir / f"{label}.emit.stdout",
        env=hako_env,
    )
    run(["python3", str(ROOT / "tools/checks/pure_first_route_preflight.py"), str(mir_json)], stdout=tmp_dir / f"{label}.preflight.stdout")
    run(
        [
            "bash",
            str(ROOT / "tools/selfhost/selfhost_build.sh"),
            "--mir-in",
            str(mir_json),
            "--exe",
            str(exe_out),
        ],
        stdout=tmp_dir / f"{label}.build.stdout",
        env=hako_env,
    )
    return exe_out


def run_external_rss(exe: Path, tmp_dir: Path, label: str, sample_count: int, warmup_count: int) -> list[int]:
    samples: list[int] = []
    for idx in range(warmup_count + sample_count):
        time_out = tmp_dir / f"{label}.{idx}.time"
        run_out = tmp_dir / f"{label}.{idx}.out"
        run_err = tmp_dir / f"{label}.{idx}.err"
        with run_out.open("w", encoding="utf-8") as stdout, run_err.open("w", encoding="utf-8") as stderr:
            subprocess.run(
                ["/usr/bin/time", "-f", "%M", "-o", str(time_out), str(exe)],
                cwd=ROOT,
                check=True,
                stdout=stdout,
                stderr=stderr,
            )
        kb_text = time_out.read_text(encoding="utf-8", errors="replace").strip()
        kb = int(kb_text) if kb_text.isdigit() else 0
        if idx >= warmup_count:
            samples.append(kb * 1024)
    return samples


def build_c_runner(tmp_dir: Path) -> Path:
    out = tmp_dir / "c_mimalloc_explicit_runner"
    run(["cc", "-std=c11", "-O2", "-Wall", "-Wextra", str(C_RUNNER_SRC), "-ldl", "-o", str(out)])
    return out


def elf_footprint(path: Path, prefix: str) -> dict[str, int | str]:
    values: dict[str, int | str] = {
        f"{prefix}_file_bytes": path.stat().st_size,
        f"{prefix}_pt_load_file_bytes": 0,
        f"{prefix}_pt_load_mem_bytes": 0,
        f"{prefix}_text_bytes": 0,
        f"{prefix}_rodata_bytes": 0,
        f"{prefix}_data_bytes": 0,
        f"{prefix}_bss_bytes": 0,
        f"{prefix}_needed_lib_count": 0,
        f"{prefix}_needed_lib_names": "none",
    }

    program_headers = run(["readelf", "-W", "-l", str(path)])
    for line in program_headers.splitlines():
        fields = line.split()
        if fields and fields[0] == "LOAD" and len(fields) >= 6:
            values[f"{prefix}_pt_load_file_bytes"] = int(values[f"{prefix}_pt_load_file_bytes"]) + int(fields[4], 16)
            values[f"{prefix}_pt_load_mem_bytes"] = int(values[f"{prefix}_pt_load_mem_bytes"]) + int(fields[5], 16)

    section_headers = run(["readelf", "-W", "-S", str(path)])
    wanted = {
        ".text": f"{prefix}_text_bytes",
        ".rodata": f"{prefix}_rodata_bytes",
        ".data": f"{prefix}_data_bytes",
        ".bss": f"{prefix}_bss_bytes",
    }
    for line in section_headers.splitlines():
        match = re.match(r"\s*\[\s*\d+\]\s+(\S+)\s+\S+\s+\S+\s+\S+\s+([0-9a-fA-F]+)\s+", line)
        if not match:
            continue
        name, size_hex = match.groups()
        key = wanted.get(name)
        if key:
            values[key] = int(size_hex, 16)

    dynamic = run(["readelf", "-W", "-d", str(path)])
    needed = []
    for line in dynamic.splitlines():
        match = re.search(r"Shared library: \[(.+?)\]", line)
        if match:
            needed.append(match.group(1))
    values[f"{prefix}_needed_lib_count"] = len(needed)
    values[f"{prefix}_needed_lib_names"] = ",".join(needed) if needed else "none"
    return values


def find_mimalloc_library(c_library: Path | None, allow_ldconfig_discovery: bool) -> Path:
    if c_library is not None:
        if not c_library.exists():
            raise SystemExit(f"--c-library path does not exist: {c_library}")
        return c_library
    if not allow_ldconfig_discovery:
        raise SystemExit("--c-library PATH or --allow-ldconfig-discovery is required")
    output = subprocess.run(
        ["bash", "-lc", r"ldconfig -p 2>/dev/null | awk '/libmimalloc\.so\.2[[:space:]]/ { print $NF; exit }'"],
        cwd=ROOT,
        check=True,
        text=True,
        stdout=subprocess.PIPE,
    ).stdout.strip()
    if not output:
        raise SystemExit("libmimalloc.so.2 not found")
    path = Path(output)
    if not path.exists():
        raise SystemExit(f"libmimalloc.so.2 path does not exist: {path}")
    return path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", required=True, type=Path)
    parser.add_argument("--sample-count", type=int, default=5)
    parser.add_argument("--warmup-count", type=int, default=1)
    parser.add_argument("--c-library", type=Path)
    parser.add_argument("--allow-ldconfig-discovery", action="store_true")
    args = parser.parse_args()

    if args.sample_count < 1:
        raise SystemExit("--sample-count must be positive")
    if args.warmup_count < 0:
        raise SystemExit("--warmup-count must be non-negative")
    if args.c_library is not None:
        args.allow_ldconfig_discovery = False
    for command in ("readelf", "cc", "/usr/bin/time"):
        if shutil.which(command) is None and not Path(command).exists():
            raise SystemExit(f"missing required command: {command}")

    with tempfile.TemporaryDirectory(prefix="hakorune_empty_footprint.") as tmp:
        tmp_dir = Path(tmp)
        baseline = tmp_dir / "empty-baseline.out"
        run(
            [
                "python3",
                str(REPEATED_RUNNER),
                "--out",
                str(baseline),
                "--sample-count",
                str(args.sample_count),
                "--warmup-count",
                str(args.warmup_count),
                "--workload",
                "representative-empty-v0",
                *(["--c-library", str(args.c_library)] if args.c_library is not None else []),
                *(["--allow-ldconfig-discovery"] if args.c_library is None and args.allow_ldconfig_discovery else []),
            ],
            stdout=tmp_dir / "baseline.stdout",
        )
        baseline_values = read_kv(baseline)
        hako_empty_evidence_rss = as_int(baseline_values, "workload_0_hako_external_rss_median_bytes")
        c_empty_rss = as_int(baseline_values, "workload_0_c_external_rss_median_bytes")

        noio_exe = build_hako_exe(EMPTY_NOIO_APP, tmp_dir, "hako-noio")
        evidence_exe = build_hako_exe(EMPTY_EVIDENCE_APP, tmp_dir, "hako-evidence")
        c_runner = build_c_runner(tmp_dir)

        noio_samples = run_external_rss(noio_exe, tmp_dir, "hako-noio", args.sample_count, args.warmup_count)
        hako_noio_rss = int(statistics.median(noio_samples))

        mimalloc_library = find_mimalloc_library(args.c_library, args.allow_ldconfig_discovery)

        lines: list[str] = [
            "mimalloc_hako_empty_exe_footprint=1",
            "output_contract=mimalloc-comparison-hako-empty-exe-footprint-diagnostic-v0",
            "baseline_workload=representative-empty-v0",
            "diagnostic_workload=representative-empty-noio-v0",
            "measurement_profile=phase295x-repeated-v0",
            f"warmup_count={args.warmup_count}",
            f"sample_count={args.sample_count}",
            "canonical_rss_collector=external-time",
            f"hako_empty_evidence_external_rss_median_bytes={hako_empty_evidence_rss}",
            f"hako_empty_noio_external_rss_median_bytes={hako_noio_rss}",
            f"hako_empty_evidence_minus_noio_rss_bytes={hako_empty_evidence_rss - hako_noio_rss}",
            f"c_empty_external_rss_median_bytes={c_empty_rss}",
        ]

        for values in (
            elf_footprint(evidence_exe, "hako_evidence_exe"),
            elf_footprint(noio_exe, "hako_noio_exe"),
            elf_footprint(c_runner, "c_runner"),
        ):
            for key in sorted(values):
                lines.append(f"{key}={values[key]}")

        lines.extend(
            [
                f"c_mimalloc_library_file_bytes={mimalloc_library.stat().st_size}",
                "static_footprint_evidence=1",
                "static_footprint_is_rss_claim=0",
                "baseline_shrink_action=0",
                "provider_activation=0",
                "host_replacement=0",
                "hook_installed=0",
                "global_allocator_installed=0",
                f"c_library_path={mimalloc_library}",
                "winner_claim=0",
                "summary=ok",
            ]
        )
        args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
        print(args.out.read_text(encoding="utf-8"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
