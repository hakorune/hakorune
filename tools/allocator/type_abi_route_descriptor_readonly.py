#!/usr/bin/env python3
"""Read Type ABI route descriptor fields from an existing report.

This adapter is intentionally read-only. It validates the descriptor/control
plane fields that allocator reports already emitted, then reprints the route
identity evidence without calling Provider ABI operations or replacement-front
entrypoints.
"""

from __future__ import annotations

import argparse
from pathlib import Path
from typing import Iterable


def read_kv_report(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line_no, raw_line in enumerate(
        path.read_text(encoding="utf-8").splitlines(), 1
    ):
        line = raw_line.strip()
        if not line or line.startswith("#"):
            continue
        if "=" not in line:
            raise SystemExit(f"{path}:{line_no}: expected key=value line")
        key, value = line.split("=", 1)
        values[key.strip()] = value.strip()
    return values


def require_value(values: dict[str, str], key: str, expected: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"expected {key}={expected}, got {actual!r}")


def pick(values: dict[str, str], keys: Iterable[str], default: str = "unknown") -> str:
    for key in keys:
        value = values.get(key)
        if value is not None and value != "":
            return value
    return default


def build_descriptor(values: dict[str, str], source_report: Path) -> list[str]:
    require_value(values, "type_abi_route_descriptor_present", "1")
    require_value(values, "type_abi_descriptor_plane", "route_descriptor_control_plane")
    require_value(values, "type_abi_hot_path_lookup_count", "0")

    provider_declared_route = pick(
        values,
        ("provider_declared_route", "provider_ldpreload_declared_route"),
    )
    provider_execution_route = pick(
        values,
        (
            "provider_execution_route",
            "provider_ldpreload_execution_route",
            "provider_ldpreload_measurement_route",
        ),
    )
    provider_hako_hot_path_claim = pick(
        values,
        (
            "provider_hako_hot_path_claim",
            "provider_ldpreload_hako_hot_path_claim",
        ),
        default="0",
    )
    provider_declared_package_origin = pick(
        values,
        ("provider_ldpreload_declared_package_origin",),
    )
    provider_benchmark_front_class = pick(
        values,
        ("provider_benchmark_front_class", "provider_ldpreload_benchmark_front_class"),
    )
    provider_kind = pick(
        values,
        ("provider_kind", "provider_ldpreload_kind"),
    )
    host_allocator_vtable_init = pick(
        values,
        ("host_allocator_vtable_init", "provider_host_allocator_vtable_init_count_total"),
    )
    replacement_front_execution_route = pick(
        values,
        ("replacement_front_execution_route",),
    )
    replacement_front_ordinary_app_route_candidate = pick(
        values,
        ("replacement_front_ordinary_app_route_candidate",),
    )
    replacement_front_product_gate = pick(
        values,
        ("replacement_front_product_gate",),
    )
    replacement_front_product_activation_ready = pick(
        values,
        ("replacement_front_product_activation_ready",),
        default="0",
    )
    replacement_front_product_claim = pick(
        values,
        ("replacement_front_product_claim",),
        default="0",
    )
    replacement_front_product_activation_contract = pick(
        values,
        ("replacement_front_product_activation_contract_v0",),
        default="0",
    )
    replacement_front_product_activation_blockers = pick(
        values,
        ("replacement_front_product_activation_blockers",),
    )

    if (
        replacement_front_execution_route == "replacement_front_benchmark"
        and replacement_front_product_claim == "1"
    ):
        raise SystemExit(
            "benchmark replacement-front route must not claim product replacement"
        )

    if (
        provider_execution_route == "provider_host_adapter_ldpreload"
        and provider_hako_hot_path_claim == "1"
    ):
        raise SystemExit(
            "host_backed_adapter provider route must not claim .hako hot path"
        )

    return [
        "output_contract=type-abi-route-descriptor-readonly-v0",
        f"input_contract={values.get('output_contract', 'unknown')}",
        f"source_report={source_report}",
        "readonly_descriptor_consumption=1",
        "python_introspection_adapter=1",
        "hako_check_core_change=0",
        "provider_abi_execution_change=0",
        "replacement_front_hot_path_change=0",
        "allocator_behavior_change=0",
        "type_abi_route_descriptor_present=1",
        "type_abi_descriptor_plane=route_descriptor_control_plane",
        "type_abi_hot_path_lookup_count=0",
        f"provider_declared_package_origin={provider_declared_package_origin}",
        f"provider_declared_route={provider_declared_route}",
        f"provider_execution_route={provider_execution_route}",
        f"provider_benchmark_front_class={provider_benchmark_front_class}",
        f"provider_hako_hot_path_claim={provider_hako_hot_path_claim}",
        f"provider_kind={provider_kind}",
        f"host_allocator_vtable_init={host_allocator_vtable_init}",
        f"replacement_front_execution_route={replacement_front_execution_route}",
        "replacement_front_ordinary_app_route_candidate="
        f"{replacement_front_ordinary_app_route_candidate}",
        f"replacement_front_product_gate={replacement_front_product_gate}",
        "replacement_front_product_activation_ready="
        f"{replacement_front_product_activation_ready}",
        f"replacement_front_product_claim={replacement_front_product_claim}",
        "replacement_front_product_activation_contract_v0="
        f"{replacement_front_product_activation_contract}",
        "replacement_front_product_activation_blockers="
        f"{replacement_front_product_activation_blockers}",
        "summary=ok",
    ]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--report",
        type=Path,
        required=True,
        help="key=value report to read",
    )
    parser.add_argument("--out", type=Path, help="optional output path")
    args = parser.parse_args()

    values = read_kv_report(args.report)
    lines = build_descriptor(values, args.report)
    text = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    else:
        print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
