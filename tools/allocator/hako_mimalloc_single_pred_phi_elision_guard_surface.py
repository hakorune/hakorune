#!/usr/bin/env python3
"""Define guard surface for single-pred PHI elision."""

from __future__ import annotations

import argparse
from pathlib import Path


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    lines = [
        "output_contract=hako-mimalloc-single-pred-phi-elision-guard-surface-v0",
        "input_contract=hako-mimalloc-single-incoming-phi-copy-elision-owner-selection-v0",
        "selected_owner_file=src/mir/builder/emission/phi.rs",
        "selected_owner_module=crate::mir::builder::emission::phi::materialize_vars_single_pred_at_entry",
        "guard_surface=single_pred_phi_elision",
        "required_before_metric=single_incoming_phi_count",
        "required_before_value=61",
        "required_after_metric=single_incoming_phi_count",
        "required_after_max=15",
        "semantic_guard=current_state_pointer_guard",
        "shape_guard=small_alloc_phi_copy_lowering_probe",
        "perf_guard=object_lifecycle_exact_exe_measurement",
        "implementation_gate=cargo_build_release_hakorune",
        "next_action=implement_guarded_elision",
        "next_row=HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-IMPLEMENTATION-296X-001",
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
