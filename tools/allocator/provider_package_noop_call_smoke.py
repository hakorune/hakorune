#!/usr/bin/env python3
"""Call only the provider no-op function after API bind."""

from __future__ import annotations

import argparse
from pathlib import Path

from provider_package_api_bind_smoke import emit as emit_bind
from provider_package_api_bind_smoke import load_api, run


def emit(base_report: str, ping_result: int) -> str:
    replacements = {
        "output_contract=hakorune-provider-package-api-bind-smoke-v0": (
            "output_contract=hakorune-provider-package-noop-call-smoke-v0"
        ),
        "dll_mode=provider-api-bind": "dll_mode=provider-noop-call",
        "provider_call_executed=0": "provider_call_executed=1",
    }
    text = base_report
    for old, new in replacements.items():
        text = text.replace(old, new)
    lines = text.rstrip("\n").splitlines()
    insert_at = lines.index("provider_call_executed=1") + 1
    lines.insert(insert_at, "provider_noop_call_executed=1")
    lines.insert(insert_at + 1, f"provider_noop_call_result={ping_result}")
    return "\n".join(lines) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--manifest", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    manifest_path = args.manifest.resolve()
    fields, descriptor, api_info, binary_path = run(manifest_path)
    api = load_api(binary_path)
    ping_result = int(api.ping())
    base = emit_bind(fields, descriptor, api_info, manifest_path, binary_path)
    report = emit(base, ping_result)
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
