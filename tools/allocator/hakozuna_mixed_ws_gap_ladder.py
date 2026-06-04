#!/usr/bin/env python3
"""Run and summarize Hakozuna mixed-ws allocator gap evidence.

This is a narrow orchestration layer over hakozuna_mixed_ws_ldpreload_compare.py.
The underlying compare report remains the detailed evidence; this tool emits a
small front-door summary for deciding why provider-backed replacement is cold
against C mimalloc.
"""

from __future__ import annotations

import argparse
import subprocess
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
COMPARE_TOOL = ROOT / "tools" / "allocator" / "hakozuna_mixed_ws_ldpreload_compare.py"


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


def as_int(values: dict[str, str], key: str, default: int = 0) -> int:
    text = values.get(key, str(default))
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{key} must be an integer, got {text!r}") from exc


def ratio(value: float, baseline: float) -> str:
    if baseline <= 0.0:
        return "nan"
    return f"{value / baseline:.3f}"


def slower_percent(value: float, baseline: float) -> str:
    if value <= 0.0:
        return "nan"
    return f"{(baseline / value - 1.0) * 100.0:.1f}"


def subject_rows(values: dict[str, str]) -> dict[str, dict[str, str]]:
    rows: dict[str, dict[str, str]] = {}
    subject_count = as_int(values, "subject_count")
    for index in range(subject_count):
        prefix = f"subject_{index}_"
        subject_id = values.get(f"{prefix}id")
        if not subject_id:
            raise SystemExit(f"missing {prefix}id")
        row: dict[str, str] = {}
        for key, value in values.items():
            if key.startswith(prefix):
                row[key.removeprefix(prefix)] = value
        rows[subject_id] = row
    return rows


def run_compare(args: argparse.Namespace, compare_report: Path) -> None:
    cmd = [
        sys.executable,
        str(COMPARE_TOOL),
        "--out",
        str(compare_report),
        "--out-dir",
        str(args.out_dir / "compare-artifacts"),
        "--sample-count",
        str(args.sample_count),
        "--warmup-count",
        str(args.warmup_count),
        "--min-sample-seconds",
        str(args.min_sample_seconds),
        "--threads",
        str(args.threads),
        "--iters-per-thread",
        str(args.iters_per_thread),
        "--working-set",
        str(args.working_set),
        "--min-size",
        str(args.min_size),
        "--max-size",
        str(args.max_size),
    ]
    if args.allow_ldconfig_discovery:
        cmd.append("--allow-ldconfig-discovery")
    if args.hakozuna_root is not None:
        cmd.extend(["--hakozuna-root", str(args.hakozuna_root)])
    if args.mimalloc_library is not None:
        cmd.extend(["--mimalloc-library", str(args.mimalloc_library)])
    if args.manifest is not None:
        cmd.extend(["--manifest", str(args.manifest)])
    if args.provider_usable_size_mode:
        cmd.append("--provider-usable-size-mode")
    if args.provider_assume_owned_mode:
        cmd.append("--provider-assume-owned-mode")
    if args.replacement_front_native_slot_mode:
        cmd.append("--replacement-front-native-slot-mode")
    if args.replacement_front_lock_mode:
        cmd.append("--replacement-front-lock-mode")
    if args.replacement_front_thread_local_mode:
        cmd.append("--replacement-front-thread-local-mode")
    if args.replacement_front_cross_thread_smoke:
        cmd.append("--replacement-front-cross-thread-smoke")
    if args.replacement_front_skip_hot_counters:
        cmd.append("--replacement-front-skip-hot-counters")
    if args.replacement_front_tls_counter_mode:
        cmd.append("--replacement-front-tls-counter-mode")
    if args.replacement_front_slot_size is not None:
        cmd.extend(["--replacement-front-slot-size", str(args.replacement_front_slot_size)])
    if args.replacement_front_match_workload_realloc_size:
        cmd.append("--replacement-front-match-workload-realloc-size")
    if args.replacement_front_match_hako_size_class:
        cmd.append("--replacement-front-match-hako-size-class")
    subprocess.run(cmd, cwd=ROOT, check=True)


def append_if_present(lines: list[str], values: dict[str, str], key: str) -> None:
    value = values.get(key)
    if value is not None:
        lines.append(f"{key}={value}")


def emit_summary(compare_report: Path, out: Path) -> None:
    values = read_kv(compare_report)
    require(values, "output_contract", "hakozuna-mixed-ws-ldpreload-compare-v0", "compare")
    require(values, "winner_claim", "0", "compare")
    rows = subject_rows(values)

    glibc = rows.get("system_malloc")
    mimalloc = rows.get("c_mimalloc_ldpreload")
    provider = rows.get("hakorune_provider_ldpreload")
    replacement_front = rows.get("hakorune_replacement_front_ldpreload")
    if glibc is None:
        raise SystemExit("compare report missing system_malloc subject")
    if mimalloc is None:
        raise SystemExit("compare report missing c_mimalloc_ldpreload subject")

    glibc_median = float(glibc["throughput_median_ops_per_sec"])
    mimalloc_median = float(mimalloc["throughput_median_ops_per_sec"])

    lines = [
        "output_contract=hakozuna-mixed-ws-gap-ladder-v0",
        "input_contract=hakozuna-mixed-ws-ldpreload-compare-v0",
        f"compare_report={compare_report}",
        f"type_abi_route_descriptor_present={values.get('type_abi_route_descriptor_present', '0')}",
        f"type_abi_descriptor_plane={values.get('type_abi_descriptor_plane', 'unknown')}",
        f"type_abi_hot_path_lookup_count={values.get('type_abi_hot_path_lookup_count', 'unknown')}",
        f"benchmark_threads={values.get('benchmark_threads', 'unknown')}",
        f"benchmark_iters_per_thread={values.get('benchmark_iters_per_thread', 'unknown')}",
        f"benchmark_working_set={values.get('benchmark_working_set', 'unknown')}",
        f"sample_count={values.get('sample_count', 'unknown')}",
        f"warmup_count={values.get('warmup_count', 'unknown')}",
        "min_sample_seconds_required="
        f"{values.get('min_sample_seconds_required', 'unknown')}",
        "min_observed_sample_seconds="
        f"{values.get('min_observed_sample_seconds', 'unknown')}",
        "median_observed_sample_seconds="
        f"{values.get('median_observed_sample_seconds', 'unknown')}",
        f"measurement_quality={values.get('measurement_quality', 'unknown')}",
        f"glibc_median_ops_per_sec={glibc_median:.3f}",
        f"system_mimalloc_median_ops_per_sec={mimalloc_median:.3f}",
        f"glibc_vs_mimalloc_ratio={ratio(glibc_median, mimalloc_median)}",
        "provider_subject_present=" + ("1" if provider is not None else "0"),
        "replacement_front_subject_present="
        + ("1" if replacement_front is not None else "0"),
        f"provider_usable_size_mode={values.get('provider_usable_size_mode', '0')}",
        f"provider_assume_owned_mode={values.get('provider_assume_owned_mode', '0')}",
        f"same_benchmark_binary=1",
        f"same_workload=1",
        f"same_threads=1",
        f"same_iters_per_thread=1",
        f"same_working_set=1",
        f"same_sample_count=1",
    ]

    for key in (
        "replacement_front_product_smoke_pack_v0",
        "replacement_front_product_smoke_pack_non_activating",
        "replacement_front_malloc_family_smoke_ok",
        "replacement_front_malloc_family_null_free_smoke_ok",
        "replacement_front_malloc_family_host_passthrough_count",
        "replacement_front_cross_thread_free_smoke_ok",
        "replacement_front_abandoned_owner_smoke_ok",
        "replacement_front_cross_thread_realloc_smoke_ok",
        "replacement_front_cross_thread_free_policy",
        "replacement_front_cross_thread_realloc_policy",
        "replacement_front_cross_thread_free_remote_free_push_count",
        "replacement_front_cross_thread_free_remote_free_drain_count",
        "replacement_front_abandoned_owner_abandoned_arena_count",
        "replacement_front_abandoned_owner_abandoned_remote_free_count",
        "replacement_front_cross_thread_realloc_unsupported_count",
        "replacement_front_cross_thread_realloc_host_passthrough_count",
        "provider_registration_v1_present",
        "provider_registration_descriptor_plane",
        "provider_registration_ops_plane",
        "provider_registration_descriptor_ops_pairing",
        "provider_registration_hot_path_uses",
        "provider_registration_type_abi_hot_path_lookup_count",
        "provider_ops_version",
        "provider_kind",
        "provider_claim_ops_enabled",
        "provider_ldpreload_declared_package_origin",
        "provider_ldpreload_declared_route",
        "provider_ldpreload_execution_route",
        "provider_ldpreload_measurement_route",
        "provider_ldpreload_provider_allocator_kind",
        "provider_ldpreload_alloc_free_route",
        "provider_ldpreload_uses_host_malloc",
        "provider_ldpreload_uses_hako_object_lifecycle",
        "provider_ldpreload_object_lifecycle_entrypoint_usage",
        "provider_ldpreload_hako_hot_path_claim",
        "provider_ldpreload_hako_object_lifecycle_hot_path",
        "provider_ldpreload_hako_object_lifecycle_metadata_only",
        "provider_manifest_hako_semantic_provider_codegen",
        "provider_manifest_hako_provider_object_lifecycle_entrypoint_verified",
        "provider_manifest_hako_provider_alloc_free_route",
        "provider_manifest_provider_allocator_kind",
        "provider_manifest_provider_abi_claim_ops_v1",
        "provider_manifest_provider_free_claim_enabled",
        "provider_manifest_provider_realloc_claim_enabled",
        "provider_manifest_provider_usable_size_claim_enabled",
        "provider_manifest_compat_alloc_free_owns_still_supported",
        "provider_manifest_compat_owns_free_mainline",
        "provider_manifest_host_allocator_vtable_init",
        "provider_manifest_hako_provider_alloc_free_uses_host_malloc",
        "provider_manifest_hako_provider_alloc_free_uses_hako_object_lifecycle",
        "provider_manifest_hako_provider_object_lifecycle_entrypoint_usage",
    ):
        append_if_present(lines, values, key)

    if provider is not None:
        provider_median = float(provider["throughput_median_ops_per_sec"])
        provider_declared_route = provider.get("declared_route", "unknown")
        provider_execution_route = provider.get("execution_route", "unknown")
        provider_front_class = provider.get("benchmark_front_class", "unknown")
        provider_hako_hot_path_claim = provider.get("hako_hot_path_claim", "0")
        provider_registration_present = values.get("provider_registration_v1_present", "0")
        provider_registration_hot_path_uses = values.get(
            "provider_registration_hot_path_uses", "unknown"
        )
        provider_registration_type_lookup_count = values.get(
            "provider_registration_type_abi_hot_path_lookup_count", "unknown"
        )
        provider_ops_version = values.get("provider_ops_version", "unknown")
        provider_kind = values.get("provider_kind", "unknown")
        provider_claim_ops_enabled = values.get("provider_claim_ops_enabled", "0")
        provider_ops = as_int(provider, "shim_provider_operation_count_total")
        init_fallback = as_int(provider, "shim_init_real_fallback_count_total")
        init_fallback_in_provider = as_int(
            provider, "shim_init_fallback_in_provider_call_count_total"
        )
        host_passthrough = as_int(provider, "shim_host_passthrough_count_total")
        runtime_fallback = as_int(provider, "shim_runtime_real_fallback_count_total")
        pointer_overflow = as_int(provider, "shim_pointer_table_overflow_total")
        free_claim = as_int(provider, "shim_provider_free_claim_count_total")
        free_not_owned = as_int(provider, "shim_provider_free_not_owned_count_total")
        free_claim_bound = as_int(provider, "shim_provider_free_claim_bound_total")
        realloc_claim = as_int(provider, "shim_provider_realloc_claim_count_total")
        realloc_not_owned = as_int(provider, "shim_provider_realloc_not_owned_count_total")
        realloc_failed = as_int(provider, "shim_provider_realloc_failed_count_total")
        realloc_claim_bound = as_int(provider, "shim_provider_realloc_claim_bound_total")
        usable_size_claim = as_int(provider, "shim_provider_usable_size_claim_count_total")
        usable_size_not_owned = as_int(
            provider, "shim_provider_usable_size_not_owned_count_total"
        )
        usable_size_claim_bound = as_int(
            provider, "shim_provider_usable_size_claim_bound_total"
        )
        host_allocator_init_bound = as_int(provider, "shim_host_allocator_init_bound_total")
        host_allocator_init_result = as_int(provider, "shim_host_allocator_init_result_total")
        host_allocator_vtable_init = as_int(
            provider, "shim_host_allocator_vtable_init_count_total"
        )
        host_allocator_usable_size_bound = as_int(
            provider, "shim_host_allocator_usable_size_bound_total"
        )
        claim_mainline = as_int(provider, "shim_claim_mainline_mode_enabled_total")
        tracking_insert = as_int(provider, "shim_track_probe_total_total")
        tracking_lookup = as_int(provider, "shim_find_probe_total_total")
        next_owner = provider.get("next_owner_family", "unknown")
        lines.extend(
            [
                f"provider_median_ops_per_sec={provider_median:.3f}",
                f"provider_declared_route={provider_declared_route}",
                f"provider_execution_route={provider_execution_route}",
                f"provider_benchmark_front_class={provider_front_class}",
                f"provider_hako_hot_path_claim={provider_hako_hot_path_claim}",
                f"provider_registration_v1_present={provider_registration_present}",
                f"provider_registration_hot_path_uses={provider_registration_hot_path_uses}",
                "provider_registration_type_abi_hot_path_lookup_count="
                f"{provider_registration_type_lookup_count}",
                f"provider_ops_version={provider_ops_version}",
                f"provider_kind={provider_kind}",
                f"provider_claim_ops_enabled={provider_claim_ops_enabled}",
                f"provider_vs_mimalloc_ratio={ratio(provider_median, mimalloc_median)}",
                "provider_slower_than_mimalloc_percent="
                f"{slower_percent(provider_median, mimalloc_median)}",
                f"provider_vs_glibc_ratio={ratio(provider_median, glibc_median)}",
                f"provider_operation_count_total={provider_ops}",
                f"provider_init_real_fallback_count_total={init_fallback}",
                f"provider_init_fallback_in_provider_call_count_total={init_fallback_in_provider}",
                f"provider_host_passthrough_count_total={host_passthrough}",
                f"provider_runtime_real_fallback_count_total={runtime_fallback}",
                f"provider_pointer_table_overflow_total={pointer_overflow}",
                f"provider_free_claim_count_total={free_claim}",
                f"provider_free_not_owned_count_total={free_not_owned}",
                f"provider_free_claim_bound_total={free_claim_bound}",
                f"provider_realloc_claim_count_total={realloc_claim}",
                f"provider_realloc_not_owned_count_total={realloc_not_owned}",
                f"provider_realloc_failed_count_total={realloc_failed}",
                f"provider_realloc_claim_bound_total={realloc_claim_bound}",
                f"provider_usable_size_claim_count_total={usable_size_claim}",
                f"provider_usable_size_not_owned_count_total={usable_size_not_owned}",
                f"provider_usable_size_claim_bound_total={usable_size_claim_bound}",
                f"provider_host_allocator_init_bound_total={host_allocator_init_bound}",
                f"provider_host_allocator_init_result_total={host_allocator_init_result}",
                f"provider_host_allocator_vtable_init_count_total={host_allocator_vtable_init}",
                f"provider_host_allocator_usable_size_bound_total={host_allocator_usable_size_bound}",
                f"provider_claim_mainline_mode_enabled_total={claim_mainline}",
                f"shim_tracking_insert_probe_total={tracking_insert}",
                f"shim_tracking_lookup_probe_total={tracking_lookup}",
                "shim_provider_owned_truth=0",
                "shim_owns_precheck_hot_path=0",
                "provider_init_real_fallback_per_provider_operation="
                f"{ratio(float(init_fallback), float(provider_ops)) if provider_ops > 0 else 'nan'}",
                "provider_host_passthrough_per_provider_operation="
                f"{ratio(float(host_passthrough), float(provider_ops)) if provider_ops > 0 else 'nan'}",
                f"provider_next_owner_family={next_owner}",
            ]
        )

    if replacement_front is not None:
        replacement_median = float(replacement_front["throughput_median_ops_per_sec"])
        replacement_front_class = replacement_front.get("benchmark_front_class", "unknown")
        replacement_execution_route = replacement_front.get("execution_route", "unknown")
        replacement_direct_core = replacement_front.get("direct_core_call", "0")
        replacement_provider_dispatch = replacement_front.get("provider_table_dispatch", "unknown")
        replacement_provider_api_required = replacement_front.get(
            "provider_api_hot_path_required", "unknown"
        )
        replacement_tracking_hot_path = replacement_front.get("tracking_hot_path", "unknown")
        replacement_benchmark_only = replacement_front.get("benchmark_only", "unknown")
        replacement_ordinary_app_route = replacement_front.get(
            "replacement_front_ordinary_app_route_candidate", "unknown"
        )
        replacement_product_gate = replacement_front.get(
            "replacement_front_product_gate", "unknown"
        )
        replacement_product_activation_ready = replacement_front.get(
            "replacement_front_product_activation_ready", "unknown"
        )
        replacement_product_activation_contract = replacement_front.get(
            "replacement_front_product_activation_contract_v0", "unknown"
        )
        replacement_product_activation_blockers = replacement_front.get(
            "replacement_front_product_activation_blockers", "unknown"
        )
        lines.extend(
            [
                f"replacement_front_median_ops_per_sec={replacement_median:.3f}",
                f"replacement_front_execution_route={replacement_execution_route}",
                "replacement_front_ordinary_app_route_candidate="
                f"{replacement_ordinary_app_route}",
                f"replacement_front_product_gate={replacement_product_gate}",
                "replacement_front_product_activation_ready="
                f"{replacement_product_activation_ready}",
                "replacement_front_product_activation_contract_v0="
                f"{replacement_product_activation_contract}",
                "replacement_front_product_activation_requires_quality_ok="
                f"{replacement_front.get('replacement_front_product_activation_requires_quality_ok', 'unknown')}",
                "replacement_front_product_activation_requires_provider_dispatch_bypass="
                f"{replacement_front.get('replacement_front_product_activation_requires_provider_dispatch_bypass', 'unknown')}",
                "replacement_front_product_activation_requires_type_abi_hot_lookup_zero="
                f"{replacement_front.get('replacement_front_product_activation_requires_type_abi_hot_lookup_zero', 'unknown')}",
                "replacement_front_product_activation_requires_cross_thread_policy="
                f"{replacement_front.get('replacement_front_product_activation_requires_cross_thread_policy', 'unknown')}",
                "replacement_front_product_activation_requires_remote_abandoned_counters="
                f"{replacement_front.get('replacement_front_product_activation_requires_remote_abandoned_counters', 'unknown')}",
                "replacement_front_product_activation_requires_rollback_optout_plan="
                f"{replacement_front.get('replacement_front_product_activation_requires_rollback_optout_plan', 'unknown')}",
                "replacement_front_product_activation_blockers="
                f"{replacement_product_activation_blockers}",
                f"replacement_front_benchmark_front_class={replacement_front_class}",
                f"replacement_front_vs_mimalloc_ratio={ratio(replacement_median, mimalloc_median)}",
                "replacement_front_slower_than_mimalloc_percent="
                f"{slower_percent(replacement_median, mimalloc_median)}",
                f"replacement_front_vs_glibc_ratio={ratio(replacement_median, glibc_median)}",
                f"replacement_front_bypasses_type_abi=1",
                f"replacement_front_bypasses_provider_dispatch={1 if replacement_provider_dispatch == '0' else 0}",
                f"replacement_front_provider_table_dispatch={replacement_provider_dispatch}",
                f"replacement_front_provider_api_hot_path_required={replacement_provider_api_required}",
                f"replacement_front_tracking_hot_path={replacement_tracking_hot_path}",
                f"replacement_front_direct_core_call={replacement_direct_core}",
                f"replacement_front_benchmark_only={replacement_benchmark_only}",
                "replacement_front_product_claim=0",
            ]
        )

    lines.extend(
        [
            "provider_activation=0",
            "production_replacement_active=0",
            "hook_installed=0",
            "global_allocator_product_claim=0",
            "winner_claim=0",
            "summary="
            f"{'ok' if values.get('measurement_quality', 'ok') == 'ok' else 'measurement_too_short'}",
        ]
    )
    out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print(out.read_text(encoding="utf-8"), end="")
    if values.get("measurement_quality", "ok") != "ok":
        raise SystemExit(
            "measurement quality is too short for keeper comparison; "
            f"min_observed_sample_seconds={values.get('min_observed_sample_seconds', 'unknown')}, "
            f"required={values.get('min_sample_seconds_required', 'unknown')}"
        )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path)
    parser.add_argument("--hakozuna-root", type=Path)
    parser.add_argument("--mimalloc-library", type=Path)
    parser.add_argument("--allow-ldconfig-discovery", action="store_true")
    parser.add_argument("--manifest", type=Path)
    parser.add_argument(
        "--provider-usable-size-mode",
        action="store_true",
        help="measurement-only: bypass provider shim tracking through private usable_size symbol",
    )
    parser.add_argument(
        "--provider-assume-owned-mode",
        action="store_true",
        help="measurement-only: with usable-size mode, skip provider owns checks before free/realloc",
    )
    parser.add_argument("--sample-count", type=int, default=5)
    parser.add_argument("--warmup-count", type=int, default=1)
    parser.add_argument(
        "--min-sample-seconds",
        type=float,
        default=0.0,
        help=(
            "require every sampled bench run to last at least this many seconds; "
            "0 preserves legacy smoke-sized probes"
        ),
    )
    parser.add_argument("--threads", type=int, default=4)
    parser.add_argument("--iters-per-thread", type=int, default=1000)
    parser.add_argument("--working-set", type=int, default=128)
    parser.add_argument("--min-size", type=int, default=16)
    parser.add_argument("--max-size", type=int, default=1024)
    parser.add_argument("--replacement-front-native-slot-mode", action="store_true")
    parser.add_argument("--replacement-front-lock-mode", action="store_true")
    parser.add_argument("--replacement-front-thread-local-mode", action="store_true")
    parser.add_argument("--replacement-front-cross-thread-smoke", action="store_true")
    parser.add_argument("--replacement-front-skip-hot-counters", action="store_true")
    parser.add_argument("--replacement-front-tls-counter-mode", action="store_true")
    parser.add_argument("--replacement-front-slot-size", type=int)
    parser.add_argument("--replacement-front-match-workload-realloc-size", action="store_true")
    parser.add_argument("--replacement-front-match-hako-size-class", action="store_true")
    args = parser.parse_args()
    if args.min_sample_seconds < 0.0:
        raise SystemExit("--min-sample-seconds must be non-negative")

    if args.out_dir is None:
        args.out_dir = Path(f"{args.out}.artifacts.d")
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out_dir.mkdir(parents=True, exist_ok=True)

    compare_report = args.out_dir / "compare.out"
    run_compare(args, compare_report)
    emit_summary(compare_report, args.out)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
