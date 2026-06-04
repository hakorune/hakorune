#!/usr/bin/env python3
"""Unit tests for mimalloc perf attribution owner selection."""

from __future__ import annotations

import sys
import unittest
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))
import hako_mimalloc_perf_attribution as attribution


class BackendStoreShapeSelectionTest(unittest.TestCase):
    def test_direct_array_dominant_mixed_store_shape_selects_direct_array_owner(self) -> None:
        selected, next_bridge = attribution._select_backend_store_shape(
            ["local_free", "used", "alloc_count", "local_free_count"],
            [],
            "direct_array_owner",
        )

        self.assertEqual(selected, "direct_array_dominant_mixed_store_shape")
        self.assertEqual(next_bridge, "classify_directarray_owner_instruction_shape")

    def test_primitive_dominant_mixed_directarray_shape_stays_on_state_elision(self) -> None:
        selected, next_bridge = attribution._select_backend_store_shape(
            ["local_free", "used"],
            [],
            "primitive_hot_state",
        )

        self.assertEqual(selected, "primitive_dominant_directarray_mixed_store_shape")
        self.assertEqual(next_bridge, "classify_backend_store_shape_for_state_write_elision")


class DirectArrayOwnerInstructionShapeTest(unittest.TestCase):
    def test_refcount_like_direct_array_owner_instruction_is_not_data_path(self) -> None:
        instruction = attribution.AnnotatedInstruction(
            percent=20.59,
            address="41328d",
            asm="incq   0x70(%r13)",
        )

        selected, next_bridge = attribution._select_directarray_owner_instruction_shape(
            instruction,
            {0x70: "local_free"},
        )

        self.assertEqual(selected, "directarray_owner_handle_field_refcount_like")
        self.assertEqual(
            next_bridge,
            "classify_handle_field_materialization_or_owner_handle_loads",
        )


if __name__ == "__main__":
    unittest.main()
