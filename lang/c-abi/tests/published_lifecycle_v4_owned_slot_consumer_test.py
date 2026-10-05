#!/usr/bin/env python3
"""Reject forged Birth ownership paths after accepting unchanged source captures."""
import copy
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
TESTS = ROOT / "lang/c-abi/tests"
ENV = dict(os.environ, HAKO_NYRT_PLUGIN_HOST="off")


def checked(args):
    result = subprocess.run([str(arg) for arg in args], text=True,
                            capture_output=True, env=ENV)
    assert result.returncode == 0, (args, result.stdout, result.stderr)
    return result


def operation(block):
    return block["terminator"]["instruction"].get("operation", {})


def select(data, kind):
    for fn in data["functions"]:
        if fn["role"] == "birth_unit":
            for block in fn["blocks"]:
                if (operation(block).get("kind") == kind and
                        operation(block).get("base") == fn["receiver"]):
                    return fn, block
    raise AssertionError(kind)


def bypass(block):
    target = block["terminator"]["instruction"]["normal"]
    block["terminator"]["instruction"] = {"op": "jump", "target": target, "args": None}
    block["edges"] = [{"target": target, "args": None}]


assert len(sys.argv) == 3, "pass unchanged provider captures for both optimization settings"
with tempfile.TemporaryDirectory(prefix="hako Birth slots ") as directory:
    work = Path(directory)
    driver, obj = work / "driver", work / "out.o"
    checked(["cc", TESTS / "published_lifecycle_v4_driver.c",
             "-L" + str(ROOT / "target/release"), "-lhako_llvmc_ffi",
             "-Wl,-rpath," + str(ROOT / "target/release"), "-o", driver])
    for argument in sys.argv[1:]:
        path = Path(argument).resolve()
        original = json.loads(path.read_text())
        checked([driver, path, obj])
        data = copy.deepcopy(original)
        fn, _ = select(data, "field_residence_release")
        # Remove acquisition and store together: there is no live allocation to
        # catch the bypass. The required owned-slot inventory must reject it.
        maximum = max(row["instruction"].get("dst", 0)
                      for block in fn["blocks"] for row in block["instructions"])
        fn["blocks"] = [{"id": fn["entry"], "edges": [], "instructions": [
            {"index": 0, "instruction": {"op": "fault_frame_enter", "mode": "borrowed",
                                          "dst": maximum + 1}},
            {"index": 1, "instruction": {"op": "const_unit", "dst": maximum + 2}}],
            "terminator": {"index": 2, "instruction": {"op": "return", "value": maximum + 2}}}]
        wire = work / "uninitialized-birth.json"
        wire.write_text(json.dumps(data))
        obj.unlink(missing_ok=True)
        result = subprocess.run([str(driver), str(wire), str(obj)], text=True,
                                capture_output=True, env=ENV)
        assert result.returncode != 0 and not obj.exists(), result.stderr
        print(path.name, "reject", "all required initialization bypass")
        for kind in ("field_residence_release", "object_field_release"):
            for label in ("missing-cleanup", "wrong-slot", "wrong-receiver", "double-release"):
                data = copy.deepcopy(original)
                fn, block = select(data, kind)
                op = operation(block)
                if label == "missing-cleanup":
                    bypass(block)
                elif label == "wrong-slot":
                    op["field_ordinal"] = (op["field_ordinal"] + 1) % (
                        next(row["field_count"] for row in data["layouts"]
                             if row["object_id"] == op["object_id"]))
                elif label == "wrong-receiver":
                    op["base"] = next(row["instruction"]["dst"]
                        for candidate in fn["blocks"] for row in candidate["instructions"]
                        if row["instruction"]["op"] == "invoke_normal_result")
                else:
                    duplicate = copy.deepcopy(block)
                    duplicate["id"] = max(row["id"] for row in fn["blocks"]) + 1
                    duplicate["instructions"] = []
                    duplicate["terminator"]["index"] = 0
                    operation(duplicate)["site"] += 1000000
                    block["terminator"]["instruction"]["normal"] = duplicate["id"]
                    block["edges"] = [{"target": duplicate["id"], "args": None},
                                      {"target": block["terminator"]["instruction"]["fault"], "args": None}]
                    fn["blocks"].append(duplicate)
                wire = work / "bad.json"
                wire.write_text(json.dumps(data))
                obj.unlink(missing_ok=True)
                result = subprocess.run([str(driver), str(wire), str(obj)], text=True,
                                        capture_output=True, env=ENV)
                assert result.returncode != 0 and not obj.exists(), (
                    path, kind, label, result.stdout, result.stderr)
                print(path.name, "reject", kind, label)
        for kind in ("field_set", "object_field_set"):
            data = copy.deepcopy(original)
            # For Array field_set select a declared owned slot, not scalar marker.
            fn, block = next((fn, block) for fn in data["functions"]
                if fn["role"] == "birth_unit" for block in fn["blocks"]
                if operation(block).get("kind") == kind and (
                    kind == "object_field_set" or operation(block)["field_ordinal"] in
                    next(row["owned_residences"] for row in data["layouts"]
                         if row["object_id"] == operation(block)["object_id"])))
            bypass(block)
            wire = work / "bypassed-store.json"
            wire.write_text(json.dumps(data))
            obj.unlink(missing_ok=True)
            result = subprocess.run([str(driver), str(wire), str(obj)], text=True,
                                    capture_output=True, env=ENV)
            assert result.returncode != 0 and not obj.exists(), (kind, result.stderr)
            print(path.name, "reject", kind, "Normal-store-bypass")
            data = copy.deepcopy(original)
            fn, block = select(data, kind)
            duplicate = copy.deepcopy(block)
            duplicate["id"] = max(row["id"] for row in fn["blocks"]) + 1
            duplicate["instructions"] = []
            duplicate["terminator"]["index"] = 0
            operation(duplicate)["site"] += 1000000
            block["terminator"]["instruction"]["normal"] = duplicate["id"]
            block["edges"][0]["target"] = duplicate["id"]
            fn["blocks"].append(duplicate)
            wire.write_text(json.dumps(data))
            obj.unlink(missing_ok=True)
            result = subprocess.run([str(driver), str(wire), str(obj)], text=True,
                                    capture_output=True, env=ENV)
            assert result.returncode != 0 and not obj.exists(), (kind, result.stderr)
            print(path.name, "reject", kind, "duplicate-store")

        data = copy.deepcopy(original)
        fn, block = next((fn, block) for fn in data["functions"]
            if fn["role"] == "birth_unit" for block in fn["blocks"]
            if operation(block).get("kind") == "object_field_release"
            and operation(block).get("base") == fn["receiver"]
            and operation(block).get("field_ordinal") == 1)
        tail = next(row["id"] for row in fn["blocks"]
                    if row["terminator"]["instruction"]["op"] == "return_fault")
        old_fault = block["terminator"]["instruction"]["fault"]
        block["terminator"]["instruction"]["fault"] = tail
        block["edges"][1]["target"] = tail
        fn["blocks"] = [row for row in fn["blocks"] if row["id"] != old_fault]
        wire.write_text(json.dumps(data))
        obj.unlink(missing_ok=True)
        result = subprocess.run([str(driver), str(wire), str(obj)], text=True,
                                capture_output=True, env=ENV)
        assert result.returncode != 0 and not obj.exists(), result.stderr
        print(path.name, "reject", "Fault remaining-slot bypass")
        for label in ("missing-object-inventory", "nonowned-array-slot", "wrong-child"):
            data = copy.deepcopy(original)
            kind = "field_set" if label == "nonowned-array-slot" else "object_field_set"
            fn, block = (next((fn, block) for fn in data["functions"]
                if fn["role"] == "birth_unit" for block in fn["blocks"]
                if operation(block).get("kind") == "field_set" and
                any(i not in row["owned_residences"] for row in data["layouts"]
                    if row["object_id"] == operation(block)["object_id"]
                    for i in range(row["field_count"])))
                if label == "nonowned-array-slot" else select(data, kind))
            op = operation(block)
            layout = next(row for row in data["layouts"] if row["object_id"] == op["object_id"])
            if label == "missing-object-inventory":
                layout.pop("owned_object_residences")
            elif label == "nonowned-array-slot":
                op["field_ordinal"] = next(i for i in range(layout["field_count"])
                                          if i not in layout["owned_residences"])
            else:
                op["child_object_id"] = next(row["object_id"] for row in data["layouts"]
                                            if row["object_id"] != op["child_object_id"])
            wire = work / "bad-inventory.json"
            wire.write_text(json.dumps(data))
            obj.unlink(missing_ok=True)
            result = subprocess.run([str(driver), str(wire), str(obj)], text=True,
                                    capture_output=True, env=ENV)
            assert result.returncode != 0 and not obj.exists(), (label, result.stderr)
            print(path.name, "reject", label)

        # The source Birth issuer uses Invoke stores with explicit Fault cleanup.
        # A scalar instruction overwrite must not preserve an old owned marker.
        for field_kind in ("field_set", "object_field_set"):
            data = copy.deepcopy(original)
            fn, store = select(data, field_kind)
            op = operation(store)
            end = next(b for b in fn["blocks"] if
                       b["terminator"]["instruction"]["op"] == "return")
            dst = 1 + max([fn["receiver"]] + [r["instruction"].get("dst", 0)
                for b in fn["blocks"] for r in b["instructions"]])
            rows = end["instructions"]
            rows.append({"index": len(rows), "instruction":
                {"op": "const_i64", "dst": dst, "value": 0}})
            rows.append({"index": len(rows), "instruction":
                {"op": "field_set", "base": fn["receiver"],
                 "object_id": op["object_id"], "field_ordinal": op["field_ordinal"],
                 "value": dst, "site": 999999, "exact_numeric_runtime_check": {"kind": "none"}}})
            end["terminator"]["index"] = len(rows)
            wire = work / "instruction-owned-overwrite.json"
            wire.write_text(json.dumps(data))
            obj.unlink(missing_ok=True)
            result = subprocess.run([str(driver), str(wire), str(obj)],
                text=True, capture_output=True, env=ENV)
            assert result.returncode != 0 and "unsupported-cohort" in result.stderr, (
                field_kind, result.returncode, result.stderr)
            assert not obj.exists()
            print(path.name, "reject", field_kind, "instruction-owned-overwrite")

        # Fresh acquired values isolate duplicate marker rejection from old lease reuse.
        for kind, ordinal in (("field_set", 1), ("object_field_set", 2)):
            data = copy.deepcopy(original)
            fn, block = next((fn, b) for fn in data["functions"]
                if fn["role"] == "birth_unit" for b in fn["blocks"]
                if operation(b).get("kind") == kind and
                operation(b).get("field_ordinal") == ordinal)
            operation(block)["field_ordinal"] = ordinal - 1
            wire = work / "fresh-duplicate-store.json"
            wire.write_text(json.dumps(data))
            obj.unlink(missing_ok=True)
            result = subprocess.run([str(driver), str(wire), str(obj)],
                text=True, capture_output=True, env=ENV)
            assert result.returncode != 0 and "unsupported-cohort" in result.stderr, result.stderr
            assert not obj.exists()
            print(path.name, "reject", kind, "fresh-lease-duplicate-slot")
