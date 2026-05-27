#!/usr/bin/env python3
"""Select the MIR-builder owner for single-incoming phi/copy elision."""

from __future__ import annotations

import argparse
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
PHI_EMISSION = ROOT / "src/mir/builder/emission/phi.rs"
PHI_HELPERS = ROOT / "src/mir/utils/phi_helpers.rs"
LOCAL_SSA = ROOT / "src/mir/builder/ssa/local.rs"
NEXT_ROW = "HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-GUARD-SURFACE-296X-001"


def require_contains(path: Path, needle: str) -> None:
    text = path.read_text(encoding="utf-8", errors="replace")
    if needle not in text:
        raise SystemExit(f"{path}: missing {needle!r}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    require_contains(PHI_EMISSION, "materialize_vars_single_pred_at_entry")
    require_contains(PHI_EMISSION, "insert_phi_single")
    require_contains(PHI_HELPERS, "pub fn insert_phi_single")
    require_contains(LOCAL_SSA, "pub fn ensure")

    lines = [
        "output_contract=hako-mimalloc-single-incoming-phi-copy-elision-owner-selection-v0",
        "input_contract=hako-mimalloc-small-alloc-phi-copy-lowering-probe-v0",
        f"selected_owner_file={PHI_EMISSION.relative_to(ROOT).as_posix()}",
        "selected_owner_module=crate::mir::builder::emission::phi::materialize_vars_single_pred_at_entry",
        f"supporting_phi_helper_file={PHI_HELPERS.relative_to(ROOT).as_posix()}",
        "supporting_phi_helper=MirBuilder::insert_phi_single",
        f"supporting_copy_owner_file={LOCAL_SSA.relative_to(ROOT).as_posix()}",
        "supporting_copy_owner=crate::mir::builder::ssa::local::ensure",
        "candidate_change_kind=mirbuilder_elision",
        "next_action=probe_owner",
        "next_diagnostic=single_pred_phi_elision_guard_surface",
        f"next_row={NEXT_ROW}",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
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
