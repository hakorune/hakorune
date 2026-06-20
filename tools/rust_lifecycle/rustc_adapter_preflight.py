#!/usr/bin/env python3
"""Diagnostic-only rustc adapter entry preflight.

This tool checks whether a future external rustc semantic adapter can be
introduced from the current workspace. It does not invoke rustc internals,
parse raw pretty dumps, generate RustLifecycleFacts, choose Hako policy, emit
.hako, or touch backend behavior.
"""

from __future__ import annotations

import shutil
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
SYN_ADAPTER = ROOT / "apps/rust-subset-to-hako/tools/syn_adapter/Cargo.toml"
HARNESS_DESIGN = (
    ROOT
    / "docs/development/current/main/design/"
    / "rustc-semir-binding-context-adapter-harness-design.md"
)
REFERENCE_FACTS = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "binding-context-adapter-facts-v0.json"
)


def require(condition: bool, message: str) -> None:
    if not condition:
        raise AssertionError(message)


def main() -> None:
    cargo = shutil.which("cargo")
    rustc = shutil.which("rustc")

    require(cargo is not None, "cargo not found")
    require(rustc is not None, "rustc not found")
    require(SYN_ADAPTER.exists(), f"missing syn adapter manifest: {SYN_ADAPTER}")
    require(HARNESS_DESIGN.exists(), f"missing harness design: {HARNESS_DESIGN}")
    require(REFERENCE_FACTS.exists(), f"missing reference facts: {REFERENCE_FACTS}")

    design = HARNESS_DESIGN.read_text()
    require("RustLifecycleFacts-v0 JSON only" in design, "facts-only contract")
    require("do_not_choose_OrderedMapBox_in_adapter=1" in design, "policy stop line")
    require("raw pretty MIR text" in design, "raw dump stop line")

    print("output_contract=rustc-semir-binding-context-toolchain-preflight-v0")
    print("toolchain_preflight_green=1")
    print("adapter_entry_identified=1")
    print("subject=BindingContext")
    print("cargo_available=1")
    print("rustc_available=1")
    print("raw_rustc_dump_as_schema=0")
    print("lifecycle_facts_generated=0")
    print("hako_plan_emitted=0")
    print("hako_source_emitted=0")
    print("backend_behavior_changed=0")
    print("summary=ok")


if __name__ == "__main__":
    main()
