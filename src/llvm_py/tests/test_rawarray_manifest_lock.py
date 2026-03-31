#!/usr/bin/env python3
import tomllib
import unittest
from pathlib import Path

from src.llvm_py.instructions.mir_call.runtime_data_dispatch import (
    select_array_collection_call_spec,
)


ROOT = Path(__file__).resolve().parents[3]
ABI_MANIFEST = ROOT / "docs/development/current/main/design/abi-export-manifest-v0.toml"
ABI_DEFAULTS = ROOT / "lang/src/vm/boxes/generated/abi_adapter_registry_defaults.hako"
RUNTIME_DECL_DEFAULTS = (
    ROOT / "lang/src/shared/backend/ll_emit/generated/runtime_decl_defaults.hako"
)


class _DummyResolver:
    def __init__(self, value_types=None, integerish_ids=None):
        self.value_types = value_types or {}
        self.integerish_ids = set(integerish_ids or [])


def _load_manifest_rows():
    data = tomllib.loads(ABI_MANIFEST.read_text())
    return data["rows"]


def _find_row(rows, box_type, method):
    for row in rows:
        if row.get("box_type") == box_type and row.get("method") == method:
            return row
    raise AssertionError(f"missing manifest row: {box_type}.{method}")


class TestRawarrayManifestLock(unittest.TestCase):
    def test_arraybox_manifest_rows_use_slot_symbols(self):
        rows = _load_manifest_rows()
        expected = {
            "push": "nyash.array.slot_append_hh",
            "len": "nyash.array.slot_len_h",
            "length": "nyash.array.slot_len_h",
            "size": "nyash.array.slot_len_h",
            "get": "nyash.array.slot_load_hi",
            "set": "nyash.array.slot_store_hih",
        }
        for method, symbol in expected.items():
            row = _find_row(rows, "ArrayBox", method)
            self.assertEqual(row["symbol"], symbol)

    def test_array_selector_matches_manifest_for_daily_routes(self):
        rows = _load_manifest_rows()
        manifest_get = _find_row(rows, "ArrayBox", "get")["symbol"]
        manifest_set = _find_row(rows, "ArrayBox", "set")["symbol"]
        manifest_push = _find_row(rows, "ArrayBox", "push")["symbol"]

        resolver_i64 = _DummyResolver(value_types={2: "i64", 3: "i64"}, integerish_ids={2, 3})
        resolver_i64_key_handle_value = _DummyResolver(
            value_types={2: "i64", 3: {"kind": "handle", "box_type": "StringBox"}},
            integerish_ids={2},
        )

        self.assertEqual(
            select_array_collection_call_spec(method="get", resolver=resolver_i64, arg_vids=[2])[0],
            manifest_get,
        )
        self.assertEqual(
            select_array_collection_call_spec(
                method="set", resolver=resolver_i64_key_handle_value, arg_vids=[2, 3]
            )[0],
            manifest_set,
        )
        self.assertEqual(
            select_array_collection_call_spec(
                method="push", resolver=resolver_i64_key_handle_value, arg_vids=[2]
            )[0],
            manifest_push,
        )

    def test_array_i64_i64_set_selector_uses_slot_store_hii(self):
        resolver = _DummyResolver(value_types={2: "i64", 3: "i64"}, integerish_ids={2, 3})
        spec = select_array_collection_call_spec(method="set", resolver=resolver, arg_vids=[2, 3])
        self.assertEqual(spec[0], "nyash.array.slot_store_hii")

    def test_generated_abi_defaults_keep_slot_symbols(self):
        text = ABI_DEFAULTS.read_text()
        self.assertIn('"ArrayBox", "push", "nyash.array.slot_append_hh"', text)
        self.assertIn('"ArrayBox", "len", "nyash.array.slot_len_h"', text)
        self.assertIn('"ArrayBox", "length", "nyash.array.slot_len_h"', text)
        self.assertIn('"ArrayBox", "size", "nyash.array.slot_len_h"', text)
        self.assertIn('"ArrayBox", "get", "nyash.array.slot_load_hi"', text)
        self.assertIn('"ArrayBox", "set", "nyash.array.slot_store_hih"', text)
        self.assertNotIn('"ArrayBox", "get", "nyash.array.get_h"', text)
        self.assertNotIn('"ArrayBox", "set", "nyash.array.set_h"', text)
        self.assertNotIn('"ArrayBox", "push", "nyash.array.push_h"', text)
        self.assertNotIn('"ArrayBox", "len", "nyash.array.len_h"', text)

    def test_runtime_decl_defaults_declare_canonical_slot_symbols(self):
        text = RUNTIME_DECL_DEFAULTS.read_text()
        for symbol in (
            "nyash.array.slot_load_hi",
            "nyash.array.slot_len_h",
            "nyash.array.slot_append_hh",
            "nyash.array.slot_store_hih",
            "nyash.array.slot_store_hii",
            "nyash.runtime_data.get_hh",
            "nyash.runtime_data.set_hhh",
            "nyash.runtime_data.has_hh",
        ):
            self.assertIn(f'"{symbol}"', text)


if __name__ == "__main__":
    unittest.main()
