#!/usr/bin/env python3
"""Check optional ownership tuple syntax on unchanged source-issued ABI inputs.

This exercises the external V2 schema boundary only; it grants no V4 borrowed
receiver, Birth commit/discharge, or runtime ownership permission.
"""
import copy
import ctypes
import json
from pathlib import Path
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
assert len(sys.argv) == 3, "pass the exact shim library and source capture directory"
shim = ctypes.CDLL(str(Path(sys.argv[1]).resolve()))
validate = shim.hako_llvmc_validate_published_lifecycle_physical_v2
validate.argtypes = [ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]
validate.restype = ctypes.c_int
libc = ctypes.CDLL(None)
libc.free.argtypes = [ctypes.c_void_p]
captures = Path(sys.argv[2])

with tempfile.TemporaryDirectory(prefix="hako owned layout syntax ") as directory:
    path = Path(directory) / "wire.json"

    def check(data, reason=None):
        path.write_text(json.dumps(data))
        error = ctypes.c_void_p()
        rc = validate(str(path).encode(), ctypes.byref(error))
        message = ctypes.string_at(error).decode() if error.value else ""
        if error.value:
            libc.free(error)
        if reason is None:
            assert rc == 0, message
        else:
            assert rc != 0 and reason in message, (reason, rc, message)

    for optimize in ("false", "true"):
        for shape in ("null-only", "constructed"):
            data = json.loads((captures / f"hako-owned-layout-{shape}-opt{optimize}.json").read_text())
            check(data)
            legacy = copy.deepcopy(data)
            for layout in legacy["layouts"]:
                layout.pop("owned_object_residences")
            check(legacy)
            if shape == "null-only":
                continue
            parent = next(row for row in data["layouts"] if row["owned_object_residences"])
            assert len(parent["owned_object_residences"]) == 1
            for mutation in ("duplicate", "out-of-range", "self-child", "unknown-child",
                             "array-overlap", "extra-key", "invalid-ordinal"):
                changed = copy.deepcopy(data)
                layout = next(row for row in changed["layouts"] if row["object_id"] == parent["object_id"])
                row = layout["owned_object_residences"][0]
                if mutation == "duplicate":
                    layout["owned_object_residences"].append(copy.deepcopy(row))
                elif mutation == "out-of-range":
                    row["field_ordinal"] = layout["field_count"]
                elif mutation == "self-child":
                    row["child_object_id"] = layout["object_id"]
                elif mutation == "unknown-child":
                    row["child_object_id"] = max(item["object_id"] for item in changed["layouts"]) + 1
                elif mutation == "array-overlap":
                    layout["owned_residences"] = [row["field_ordinal"]]
                elif mutation == "extra-key":
                    row["permission"] = "owned"
                else:
                    row["field_ordinal"] = "0"
                check(changed, "abi-layout")
                print(optimize, mutation, "rejected at external layout schema")
    print("4 original inputs and 4 legacy shapes accepted; 14 malformed tuple inputs rejected")
