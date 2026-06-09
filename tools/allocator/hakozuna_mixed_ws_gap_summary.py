"""Summarize Hakozuna mixed-ws compare reports into a short gap ladder."""

from __future__ import annotations

import argparse
from pathlib import Path

from hakozuna_mixed_ws_gap_summary_report import emit_summary

# provider_declared_route / provider_execution_route are emitted by emit_summary.
# provider_registration_v1_present=1 / provider_registration_hot_path_uses=provider_ops_only
# / provider_registration_type_abi_hot_path_lookup_count=0 are lifted by emit_summary.
# replacement_front_subject_present= / same_benchmark_binary=1 / min_sample_seconds_required=
# / measurement_quality= / measurement_too_short / replacement_front_ordinary_app_route_candidate=
# / replacement_front_product_gate= / replacement_front_product_activation_contract_v0=
# / replacement_front_product_activation_blockers= / replacement_front_rollback_optout_plan_v0=
# / replacement_front_rollback_optout_env= / replacement_front_product_preflight_report_v0=
# / replacement_front_product_preflight_activation_ready= / replacement_front_product_preflight_missing=
# / replacement_front_product_smoke_pack_v0 / replacement_front_malloc_family_smoke_ok
# / replacement_front_bypasses_type_abi=1 / replacement_front_bypasses_provider_dispatch=
# type_abi_route_descriptor_present=1 / type_abi_hot_path_lookup_count=


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("compare_report", type=Path)
    parser.add_argument("--out", type=Path, required=True)
    args = parser.parse_args()

    report = emit_summary(args.compare_report)
    args.out.write_text(report, encoding="utf-8")
    print(report, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
