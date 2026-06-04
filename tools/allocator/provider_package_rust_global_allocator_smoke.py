#!/usr/bin/env python3
"""Smoke a generated Rust #[global_allocator] backed by provider API."""

from __future__ import annotations

import argparse
import os
import subprocess
from pathlib import Path

from provider_package_api_bind_smoke import run
from provider_package_load_only_smoke import sha256_file
from provider_package_rust_global_allocator_smoke_source import RUST_SOURCE


def parse_stdout(text: str) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in text.splitlines():
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    manifest_path = args.manifest.resolve()
    fields, _descriptor, _api, binary_path = run(manifest_path)

    out_dir = args.out_dir.resolve()
    out_dir.mkdir(parents=True, exist_ok=True)
    rust_source = out_dir / "hako_provider_global_allocator_smoke.rs"
    rust_binary = out_dir / "hako_provider_global_allocator_smoke"
    stdout_path = out_dir / "smoke.stdout"
    stderr_path = out_dir / "smoke.stderr"
    rust_source.write_text(RUST_SOURCE.lstrip(), encoding="utf-8")
    subprocess.run(
        ["rustc", "--edition=2021", "-O", str(rust_source), "-o", str(rust_binary)],
        check=True,
    )

    env = os.environ.copy()
    env["HAKORUNE_PROVIDER_LIBRARY"] = str(binary_path)
    if fields.get("host_allocator_vtable_init") == "1":
        env["HAKORUNE_PROVIDER_HOST_ALLOCATOR_REQUIRED"] = "1"
    completed = subprocess.run(
        [str(rust_binary)],
        env=env,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    stdout_path.write_text(completed.stdout, encoding="utf-8")
    stderr_path.write_text(completed.stderr, encoding="utf-8")
    values = parse_stdout(completed.stdout)
    bind_success = int(values.get("rust_provider_bind_success", "0"))
    alloc_count = int(values.get("rust_provider_alloc_count", "0"))
    free_count = int(values.get("rust_provider_free_count", "0"))
    runtime_fallback = int(values.get("rust_runtime_fallback_count", "0"))
    overflow = int(values.get("rust_pointer_table_overflow", "0"))
    summary = "ok" if (
        completed.returncode == 0
        and bind_success == 1
        and alloc_count > 0
        and free_count > 0
        and runtime_fallback == 0
        and overflow == 0
    ) else "failed"

    lines = [
        "output_contract=hako-mimalloc-provider-backed-rust-global-allocator-smoke-v0",
        "input_contract=hakorune-provider-runtime-load-stage-7b-v0",
        "dll_mode=provider-backed-rust-global-allocator-pilot",
        f"manifest={manifest_path}",
        f"provider_binary_path={binary_path}",
        f"provider_binary_sha256={sha256_file(binary_path)}",
        f"rust_source_path={rust_source}",
        f"rust_binary_path={rust_binary}",
        f"rust_stdout_path={stdout_path}",
        f"rust_stderr_path={stderr_path}",
        f"rust_exit_code={completed.returncode}",
        "provider_api_bound=1",
        "provider_call_executed=1",
        "allocator_entrypoint_called=1",
        "replacement_active=0",
        "global_allocator=1",
        "global_allocator_scope=generated-rust-smoke-process-only",
        "global_allocator_product_claim=0",
        "hook_installed=0",
        "winner_claim=0",
    ]
    for key in sorted(values):
        lines.append(f"{key}={values[key]}")
    lines.append(f"summary={summary}")
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0 if summary == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
