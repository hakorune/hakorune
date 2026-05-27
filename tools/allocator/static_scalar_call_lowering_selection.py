#!/usr/bin/env python3
"""Select the guarded call-lowering route for verified static-scalar facts."""

from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    lines = [
        "output_contract=static-scalar-call-lowering-selection-v0",
        "input_contract=static-scalar-method-fact-inference-v0",
        "lowering_route=handle_static_method_call_zero_arg_before_emit_unified_call",
        "guard_surface=object_lifecycle_reason_static_receiver_zero_arg",
        "required_fact=verified_static_scalar_method_fact",
        "arg_policy=zero_args_only",
        "generic_cse=0",
        "whole_box_pure=0",
        "fallback_on_missing_fact=keep_call",
        "selected_next=static_scalar_call_lowering_implementation",
        "summary=ok",
    ]
    text = "\n".join(lines) + "\n"
    if args.out is None:
        print(text, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
