#!/usr/bin/env python3
"""Format remote-free production facade evidence into a stable presentation contract."""

from __future__ import annotations

import argparse
from pathlib import Path


EVIDENCE_CONTRACT = "mimalloc-comparison-remote-free-production-facade-evidence-v0"
OUTPUT_CONTRACT = "mimalloc-comparison-remote-free-production-facade-presentation-v0"
PROOF_BUNDLE = (
    "worker_tls_cache+remote_free_policy+ptr_remote_free_list+"
    "remote_abandoned_owner_policy+remote_free_page_integration+"
    "threadsafe_abi+native_stress"
)


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
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


def as_int(values: dict[str, str], key: str, label: str) -> int:
    text = values.get(key, "0")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be an integer, got {text!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--report", required=True, type=Path)
    parser.add_argument("--out", required=True, type=Path)
    args = parser.parse_args()

    report = read_kv(args.report)
    label = "remote-free production facade presentation"
    require(report, "output_contract", EVIDENCE_CONTRACT, label)
    require(report, "proof_bundle", PROOF_BUNDLE, label)
    require(report, "summary", "ok", label)
    require(report, "provider_active", "0", label)
    require(report, "replacement_active", "0", label)
    require(report, "winner_claim", "0", label)
    require(report, "native_multi_worker_stress", "1", label)
    require(report, "thread_safe_abi", "1", label)

    worker_count = as_int(report, "worker_count", label)
    iterations_per_worker = as_int(report, "iterations_per_worker", label)
    expected_remote_free_count = as_int(report, "expected_remote_free_count", label)
    observed_remote_free_count = as_int(report, "observed_remote_free_count", label)
    drained_nodes = as_int(report, "drained_nodes", label)
    payload_sum_nonzero = as_int(report, "payload_sum_nonzero", label)

    if worker_count <= 0:
        raise SystemExit(f"{label}: worker_count must be positive")
    if iterations_per_worker <= 0:
        raise SystemExit(f"{label}: iterations_per_worker must be positive")
    if expected_remote_free_count <= 0:
        raise SystemExit(f"{label}: expected_remote_free_count must be positive")
    if observed_remote_free_count != expected_remote_free_count:
        raise SystemExit(f"{label}: observed_remote_free_count mismatch")
    if drained_nodes != expected_remote_free_count:
        raise SystemExit(f"{label}: drained_nodes mismatch")
    if payload_sum_nonzero != 1:
        raise SystemExit(f"{label}: payload_sum_nonzero must be 1")

    lines = [
        "mimalloc_remote_free_production_facade_presentation=1",
        f"output_contract={OUTPUT_CONTRACT}",
        f"input_contract={EVIDENCE_CONTRACT}",
        "presentation_only=1",
        f"proof_bundle={report['proof_bundle']}",
        f"worker_id={report['worker_id']}",
        f"tls_cache_slot={report['tls_cache_slot']}",
        f"atomic_route={report['atomic_route']}",
        f"remote_pending={report['remote_pending']}",
        f"abandoned_owner={report['abandoned_owner']}",
        f"page_ownership={report['page_ownership']}",
        f"thread_safe_abi={report['thread_safe_abi']}",
        f"native_multi_worker_stress={report['native_multi_worker_stress']}",
        f"worker_count={worker_count}",
        f"iterations_per_worker={iterations_per_worker}",
        f"expected_remote_free_count={expected_remote_free_count}",
        f"observed_remote_free_count={observed_remote_free_count}",
        f"drained_nodes={drained_nodes}",
        f"payload_sum_nonzero={payload_sum_nonzero}",
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
