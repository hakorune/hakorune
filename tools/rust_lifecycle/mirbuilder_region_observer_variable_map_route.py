#!/usr/bin/env python3
"""Verify RegionObserver variable_map read-fold route selection."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

from context_fact_extraction import require
from extract_region_observer_variable_map_facts import extract_facts
from mirbuilder_ordered_read_fold_converter import compile_ordered_read_fold


ROOT = Path(__file__).resolve().parents[2]
FIXTURE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/region-observer-variable-map-route-v0.json"


PLAN = {
    "borrow_use_id": "RegionObserver::classify_slots_from_variable_map",
    "source": "builder.variable_ctx.variable_map",
    "destination": "slots",
    "source_order_proof": "Denied",
}


def _deny_reason(exc: BaseException) -> tuple[str, str]:
    match = re.search(r"Deny\(([^)]+)\)(?:: detail=([A-Za-z0-9_]+))?", str(exc))
    if not match:
        raise SystemExit(f"unable to read deny reason from: {exc!r}")
    return match.group(1), match.group(2) or ""


def route_report() -> dict[str, Any]:
    facts = extract_facts()
    try:
        compile_ordered_read_fold(facts, PLAN)
    except ValueError as exc:
        reason, detail = _deny_reason(exc)
    else:
        raise SystemExit("expected SourceOrdered read-fold to deny")

    return {
        "schema_version": 0,
        "kind": "MirBuilderRegionObserverVariableMapRoute",
        "subject": "mir::region::observer::classify_slots_from_variable_map",
        "source": facts["source"],
        "route": "Deny",
        "deny": {
            "reason": reason,
            "detail": detail,
        },
        "decision": [
            "do not generate RegionObserver variable_map read-fold artifact",
            "do not substitute insertion order for Rust BTreeMap<String> order",
            "do not add RegionObserver key-name special cases",
        ],
        "stop_line": [
            "source_ordered_read_fold_claim=0",
            "runtime_fallback=0",
            "generated_hako=0",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    args = parser.parse_args()

    report = route_report()
    if args.check_reference:
        require(report == json.loads(FIXTURE.read_text()), "RegionObserver variable-map route fixture differs")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=region-observer-variable-map-route-v0")
    print(f"route={report['route']}")
    print(f"deny_reason={report['deny']['reason']}")
    print(f"deny_detail={report['deny']['detail']}")
    print("generated_hako=0")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
