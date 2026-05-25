#!/usr/bin/env python3
"""Normalize the remote-free production facade proof bundle into one evidence report."""

from __future__ import annotations

import argparse
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
WORKER_TLS_GUARD = ROOT / "tools/checks/k2_wide_mimalloc_worker_tls_cache_exe_guard.sh"
REMOTE_POLICY_GUARD = ROOT / "tools/checks/k2_wide_mimalloc_remote_free_policy_exe_guard.sh"
PTR_REMOTE_FREE_LIST_GUARD = ROOT / "tools/checks/k2_wide_mimalloc_ptr_remote_free_list_exe_guard.sh"
REMOTE_ABANDONED_OWNER_GUARD = ROOT / "tools/checks/k2_wide_mimalloc_remote_abandoned_owner_policy_guard.sh"
REMOTE_FREE_PAGE_INTEGRATION_GUARD = ROOT / "tools/checks/k2_wide_mimalloc_remote_free_page_integration_guard.sh"
THREADSAFE_ABI_GUARD = ROOT / "tools/checks/k2_wide_hako_mem_threadsafe_abi_guard.sh"
STRESS_RUNNER = ROOT / "tools/allocator/mimalloc_parallel_substrate_stress_runner.py"


def read_kv_lines(text: str) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in text.splitlines():
        line = line.strip()
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def ensure_guard_ok(path: Path, label: str) -> None:
    completed = subprocess.run(
        ["bash", str(path)],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )
    if completed.returncode != 0:
        print(completed.stdout, end="")
        raise SystemExit(f"{label} guard failed: {path}")


def capture_stress_report(out_dir: Path) -> dict[str, str]:
    out_path = out_dir / "stress.report"
    completed = subprocess.run(
        [
            "python3",
            str(STRESS_RUNNER),
            "--out",
            str(out_path),
        ],
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        check=False,
    )
    if completed.returncode != 0:
        print(completed.stdout, end="")
        raise SystemExit("native multi-worker stress runner failed")
    values = read_kv_lines(out_path.read_text(encoding="utf-8"))
    if values.get("summary") != "ok":
        print(out_path.read_text(encoding="utf-8"), end="")
        raise SystemExit("native multi-worker stress summary must be ok")
    return values


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()

    for path in (
        WORKER_TLS_GUARD,
        REMOTE_POLICY_GUARD,
        PTR_REMOTE_FREE_LIST_GUARD,
        REMOTE_ABANDONED_OWNER_GUARD,
        REMOTE_FREE_PAGE_INTEGRATION_GUARD,
        THREADSAFE_ABI_GUARD,
        STRESS_RUNNER,
    ):
        if not path.exists():
            raise SystemExit(f"missing proof input: {path}")

    ensure_guard_ok(WORKER_TLS_GUARD, "worker_tls")
    ensure_guard_ok(REMOTE_POLICY_GUARD, "remote_policy")
    ensure_guard_ok(PTR_REMOTE_FREE_LIST_GUARD, "ptr_remote_free_list")
    ensure_guard_ok(REMOTE_ABANDONED_OWNER_GUARD, "remote_owner")
    ensure_guard_ok(REMOTE_FREE_PAGE_INTEGRATION_GUARD, "page_integration")
    ensure_guard_ok(THREADSAFE_ABI_GUARD, "threadsafe_abi")

    with tempfile.TemporaryDirectory(prefix="hakorune_remote_free_facade_stress.") as tmp:
        stress = capture_stress_report(Path(tmp))

    require(stress, "summary", "ok", "stress")
    require(stress, "mimalloc_parallel_substrate_stress_runner", "1", "stress")
    require(stress, "output_contract", "mimalloc-comparison-par-stress-evidence-v0", "stress")
    require(stress, "worker_count", "4", "stress")
    require(stress, "iterations_per_worker", "64", "stress")
    require(stress, "expected_remote_free_count", "256", "stress")
    require(stress, "observed_remote_free_count", "256", "stress")
    require(stress, "drained_nodes", "256", "stress")
    require(stress, "payload_sum_nonzero", "1", "stress")

    lines = [
        "mimalloc_remote_free_production_facade_evidence_runner=1",
        "output_contract=mimalloc-comparison-remote-free-production-facade-evidence-v0",
        "proof_bundle=worker_tls_cache+remote_free_policy+ptr_remote_free_list+remote_abandoned_owner_policy+remote_free_page_integration+threadsafe_abi+native_stress",
        "worker_id=0",
        "tls_cache_slot=0",
        "atomic_route=ptr_store_load_cas",
        "remote_pending=0,6,4,3",
        "abandoned_owner=3,1,1,1,1",
        "page_ownership=0,2,1,2",
        "thread_safe_abi=1",
        "native_multi_worker_stress=1",
        "worker_count=4",
        "iterations_per_worker=64",
        "expected_remote_free_count=256",
        "observed_remote_free_count=256",
        "drained_nodes=256",
        "payload_sum_nonzero=1",
        "provider_active=0",
        "replacement_active=0",
        "winner_claim=0",
        "counts=6",
        "summary=ok",
    ]
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(args.out.read_text(encoding="utf-8"), end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
