"""Subject execution orchestration for the Hakozuna mixed-ws compare."""

from __future__ import annotations

import subprocess
import sys
from pathlib import Path

from hako_mimalloc_provider_backed_hakmem_ldpreload_bench_pilot import read_kv
from hakozuna_mixed_ws_build_support import run_subject
from replacement_front_smokes import run_replacement_front_cross_thread_smokes


def run_hakozuna_mixed_ws_subjects(
    *,
    args,
    bench: Path,
    root: Path,
    out_dir: Path,
    replacement_front_shim: Path | None,
    mimalloc_library: Path,
) -> tuple[list[tuple[str, Path | None, Path | None, bool]], dict[str, tuple[list[float], list[float], dict[str, int]]], dict[str, dict[str, str]]]:
    provider_shim: Path | None = None
    provider_binary: Path | None = None
    if args.manifest is not None:
        smoke_report = out_dir / "provider-ldpreload-smoke.out"
        smoke_cmd = [
            sys.executable,
            str(Path(__file__).resolve().with_name("provider_package_ldpreload_replacement_smoke.py")),
            "--manifest",
            str(args.manifest.resolve()),
            "--out-dir",
            str(out_dir / "provider-ldpreload-smoke"),
            "--out",
            str(smoke_report),
        ]
        if args.provider_usable_size_mode:
            smoke_cmd.append("--use-provider-usable-size")
        if args.provider_assume_owned_mode:
            smoke_cmd.append("--assume-provider-owned")
        subprocess.run(smoke_cmd, check=True)
        smoke = read_kv(smoke_report)
        provider_shim = Path(smoke["shim_artifact_path"])
        provider_binary = Path(smoke["provider_binary_path"])

    if args.replacement_front_cross_thread_smoke and replacement_front_shim is None:
        raise SystemExit("--replacement-front-cross-thread-smoke requires a replacement front")

    replacement_front_smokes: dict[str, dict[str, str]] = {}
    if args.replacement_front_cross_thread_smoke:
        replacement_front_smokes = run_replacement_front_cross_thread_smokes(
            out_dir=out_dir,
            replacement_front_shim=replacement_front_shim,
        )

    subject_specs: list[tuple[str, Path | None, Path | None, bool]] = [
        ("system_malloc", None, None, False),
        ("c_mimalloc_ldpreload", mimalloc_library, None, False),
    ]
    if provider_shim is not None and provider_binary is not None:
        subject_specs.append(("hakorune_provider_ldpreload", provider_shim, provider_binary, False))
    if replacement_front_shim is not None:
        subject_specs.append(
            ("hakorune_replacement_front_ldpreload", replacement_front_shim, None, True)
        )

    reports: dict[str, tuple[list[float], list[float], dict[str, int]]] = {}
    for subject, ld_preload, provider, replacement_front_mode in subject_specs:
        reports[subject] = run_subject(
            bench=bench,
            root=root,
            out_dir=out_dir,
            subject=subject,
            warmup_count=args.warmup_count,
            sample_count=args.sample_count,
            threads=args.threads,
            iters_per_thread=args.iters_per_thread,
            working_set=args.working_set,
            min_size=args.min_size,
            max_size=args.max_size,
            ld_preload=ld_preload,
            provider_binary=provider,
            provider_usable_size_mode=(
                args.provider_usable_size_mode and subject == "hakorune_provider_ldpreload"
            ),
            provider_assume_owned_mode=(
                args.provider_assume_owned_mode and subject == "hakorune_provider_ldpreload"
            ),
            replacement_front_mode=replacement_front_mode,
        )

    return subject_specs, reports, replacement_front_smokes
