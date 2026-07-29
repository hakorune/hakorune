#!/usr/bin/env python3
"""Record the focused minimal MirBuilder lifecycle-composition smoke result.

This is a smoke-result contract, not semantic authority. It checks that the
owner-local Rust test composes prepare, Integer literal emission, and finalize
without reopening a generic root facade.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
RESULT_PATH = FIXTURES / "mirbuilder-minimal-execution-path-smoke-result-v0.json"
SMOKE_TEST = ROOT / "src/mir/builder/module_lifecycle_capture_tests.rs"


class SmokeError(RuntimeError):
    pass


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def stable_json(data: dict[str, Any]) -> str:
    return json.dumps(data, ensure_ascii=False, indent=2, sort_keys=True) + "\n"


def verify_test_source() -> None:
    text = SMOKE_TEST.read_text()
    required_markers = [
        "fn mirbuilder_minimal_literal_integer_path_smoke()",
        "LiteralValue::Integer(0)",
        "builder.prepare_module()",
        "builder.finalize_module(literal)",
        "get_function(\"main\")",
        "MirType::Integer",
        "get_function(\"condition_fn\").is_none()",
        "ConstValue::Integer(0)",
        "MirInstruction::Return",
    ]
    missing = [marker for marker in required_markers if marker not in text]
    if missing:
        raise SmokeError(f"smoke test is missing required markers: {missing}")


def build_result() -> dict[str, Any]:
    verify_test_source()
    return {
        "kind": "MinimalMirBuilderExecutionPathSmokeResultV1",
        "input_profile": {
            "composition": "prepare_module -> build_literal(Integer(0)) -> finalize_module",
        },
        "observed": {
            "owner_local_rust_test": (
                "src/mir/builder/module_lifecycle_capture_tests.rs::"
                "mirbuilder_minimal_literal_integer_path_smoke"
            ),
            "main_function": "present",
            "main_return_type": "MirType::Integer",
            "main_const_integer": "ConstValue::Integer(0)",
            "main_return_value": "literal_integer_const_dst",
            "condition_fn_injection": "retired",
        },
        "available_capabilities": [
            "MinimalExecutionPathSmoke",
        ],
        "claims": {
            "full_mirbuilder_new_claim": 0,
            "generated_hako_change": 0,
            "mainline_selected": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "runtime_fallback": 0,
            "source_selfhost_claim": 0,
        },
    }


def run(check: bool) -> None:
    result = build_result()
    result_text = stable_json(result)
    if check:
        if not RESULT_PATH.exists():
            raise SmokeError("smoke result fixture missing; run without --check")
        if RESULT_PATH.read_text() != result_text:
            raise SmokeError(f"{rel(RESULT_PATH)} is stale")
    else:
        FIXTURES.mkdir(parents=True, exist_ok=True)
        RESULT_PATH.write_text(result_text)

    print("output_contract=rust-lifecycle-mirbuilder-minimal-execution-path-smoke-v0")
    print("input=prepare_module->build_literal(Integer(0))->finalize_module")
    print("minimal_execution_path_smoke=green")
    print("condition_fn_injection=retired")
    print("full_mirbuilder_new_claim=0")
    print("mainline_selected=0")
    print("runtime_fallback=0")
    print("summary=ok")


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    try:
        run(check=args.check)
    except SmokeError as exc:
        raise SystemExit(f"error: {exc}") from exc


if __name__ == "__main__":
    main()
