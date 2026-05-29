#!/usr/bin/env python3
"""Report DirectSlotLease feasibility for the current typed-object store."""

from __future__ import annotations

import argparse
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_STORE = ROOT / "crates/nyash_kernel/src/exports/typed_object_store.rs"
DEFAULT_OBJECT = ROOT / "crates/nyash_kernel/src/exports/typed_object.rs"


def require_text(path: Path, text: str) -> None:
    content = path.read_text(encoding="utf-8")
    if text not in content:
        raise SystemExit(f"{path}: missing required text: {text}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--typed-object-store", type=Path, default=DEFAULT_STORE)
    parser.add_argument("--typed-object", type=Path, default=DEFAULT_OBJECT)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    require_text(args.typed_object_store, "RefCell<Vec<TypedSlotObject>>")
    require_text(args.typed_object_store, "Mutex<Vec<TypedSlotObject>>")
    require_text(args.typed_object_store, "objects.push(object)")
    require_text(args.typed_object, "pub(crate) struct TypedSlotObject")
    require_text(args.typed_object, "pub(crate) fields: Vec<TypedSlot>")

    lines = [
        "output_contract=typed-object-direct-slot-lease-feasibility-v0",
        "input_contract=representation-direct-storage-substrate-ssot-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        "current_store_kind=safe_mutex_or_single_thread_refcell_vec",
        "single_thread_exact_backend_exists=1",
        "object_storage_container=Vec<TypedSlotObject>",
        "field_storage_container=Vec<TypedSlot>",
        "object_generation_available=0",
        "object_storage_pinned=0",
        "field_address_stable=0",
        "vec_reallocation_possible=1",
        "borrow_lifetime_representable_in_llvm=0",
        "direct_slot_lease_feasible_without_storage_change=0",
        "raw_runtime_vec_pointer_exposure_allowed=0",
        "required_runtime_storage_change=pinned_typed_object_arena",
        "selected_next=pinned_typed_object_arena_ssot",
        "implementation_open=0",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    text = "\n".join(lines) + "\n"
    if args.out:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(text, encoding="utf-8")
    print(text, end="")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
