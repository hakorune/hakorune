#!/usr/bin/env python3
"""V4 physical input -> LLVM C API -> actual lifecycle archive; not Rust host cutover proof.

Pass JSON captured from the source-issued Pair transport test, without repair.
Range variants below test the physical ABI only, not new source acceptance.
"""
import copy
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
TESTS = ROOT / "lang/c-abi/tests"
YYJSON = ROOT / "plugins/nyash-json-plugin/c/yyjson"
issued = json.loads(Path(sys.argv[1]).read_text())
bool_inputs = [json.loads(Path(path).read_text()) for path in sys.argv[2:]]
assert len(bool_inputs) == 2, "pass source-issued Pair JSON and both source-issued Bool JSON files"
archive = ROOT / "target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a"
assert archive.is_file()
env = dict(os.environ, NYASH_NYRT_SILENT_RESULT="1", HAKO_NYRT_PLUGIN_HOST="off")


def run(argv, **kw):
    return subprocess.run([str(x) for x in argv], text=True, capture_output=True, **kw)


def checked(argv, **kw):
    r = run(argv, **kw)
    assert r.returncode == 0, (argv, r.stdout, r.stderr)
    return r


with tempfile.TemporaryDirectory(prefix="hako v4 execution ") as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ("driver", "pair.o", "pair")]
    test_ffi = work / "libhako_llvmc_ffi_test.so"
    checked(["cc", "-DHAKO_LLVMC_LIFECYCLE_TEST_SEAM", "-fPIC", "-shared",
             "-I" + str(YYJSON), "-o", test_ffi,
             ROOT / "lang/c-abi/shims/hako_llvmc_ffi.c",
             ROOT / "lang/c-abi/shims/hako_aot.c",
             ROOT / "lang/c-abi/shims/hako_json_v1.c",
             YYJSON / "yyjson.c"])
    checked(["cc", TESTS / "published_lifecycle_v4_driver.c",
             "-L" + str(work), "-lhako_llvmc_ffi_test",
             "-Wl,-rpath," + str(work), "-o", driver])
    wraps = ["fault.frame_init", "fault.frame_dispose", "fault.report_final",
             "object.checked_field_set", "object.home_release_plain_i64", "object.reclaim_unpublished"]

    def compile_input(data, expected=True, custom_env=None):
        path = work / "input.json"
        path.write_text(json.dumps(data))
        r = run([driver, path, obj], env=custom_env or env)
        assert (r.returncode == 0) == expected, (r.stdout, r.stderr)
        assert not list(work.glob("pair.o.*")), "temporary artifact leak"
        return r

    def link():
        checked(["cc", obj, TESTS / "published_lifecycle_v4_runtime_probe.c", archive,
                 *["-Wl,--wrap=nyash." + name + "_v1" for name in wraps],
                 "-lpthread", "-ldl", "-lm", "-o", exe])

    def execute(mode, status, counts, fault=None):
        r = run([exe], env=dict(env, V4_PROBE_MODE=mode))
        assert r.returncode == status, (mode, r)
        assert "COUNTS " + counts + "\n" in r.stdout, (mode, r)
        if fault:
            assert fault + "\n" in r.stdout, (mode, r)
        else:
            assert "FAULT " not in r.stdout, (mode, r)
        print(mode, status, counts)

    compile_input(issued)
    assert "Advanced Micro Devices X86-64" in checked(["readelf", "-h", obj]).stdout
    link()
    execute("normal", 30, "1 2 1 0 0 1")
    sites = [row["terminator"]["instruction"]["operation"]["site"]
             for row in issued["functions"][1]["blocks"]
             if row["terminator"]["instruction"].get("operation", {}).get("kind") == "field_set"]
    execute("fault-first", 70, "1 1 0 1 1 1", f"FAULT 101 {sites[0]} 10 0 HOME 0 RECLAIM 1")
    execute("fault-second", 70, "1 2 0 1 1 1", f"FAULT 101 {sites[1]} 20 0 HOME 0 RECLAIM 1")
    execute("report-failure", 70, "1 1 0 1 1 1", f"FAULT 101 {sites[0]} 10 0 HOME 0 RECLAIM 1")
    execute("store-invalid", 70, "1 1 0 0 0 1")
    execute("init-invalid", 70, "1 0 0 0 0 0")

    # Physical CFG analysis must terminate on a reachable no-exit cycle.
    # Compile only: running this object would intentionally never terminate.
    cycle = copy.deepcopy(issued)
    cycle["layouts"] = []
    cycle["process_result_site"] = 0
    cycle["functions"] = [copy.deepcopy(issued["functions"][0])]
    root = cycle["functions"][0]

    def jump_block(block_id, target, instructions=()):
        return {"id": block_id,
                "instructions": [{"index": i, "instruction": ins}
                                 for i, ins in enumerate(instructions)],
                "terminator": {"index": len(instructions),
                               "instruction": {"op": "jump", "target": target, "args": None}},
                "edges": [{"target": target, "args": None}]}

    root["entry"] = 91
    root["blocks"] = [jump_block(7, 23), jump_block(23, 7),
                      jump_block(91, 23, [{"op": "fault_frame_enter", "dst": 0,
                                          "mode": "root_owned"}])]
    compile_input(cycle)
    # Original ordinals, not sorted block IDs, own state slots.
    root["blocks"].reverse()
    compile_input(cycle)
    entry_backedge = copy.deepcopy(cycle)
    block = next(b for b in entry_backedge["functions"][0]["blocks"] if b["id"] == 7)
    block["terminator"]["instruction"]["target"] = 91
    block["edges"][0]["target"] = 91
    compile_input(entry_backedge, False)
    unreachable = copy.deepcopy(cycle)
    unreachable["functions"][0]["blocks"].append(jump_block(42, 42))
    compile_input(unreachable, False)
    print("physical no-exit cycle compiles; entry backedge/unreachable block reject")

    # Scalar backedge through the actual parser, index, worklist and LLVM owner.
    # This physical fixture is not the source-called G0 acceptance witness.
    scalar_loop = copy.deepcopy(cycle)
    root = scalar_loop["functions"][0]
    root["blocks"] = [
        jump_block(91, 23, [{"op": "fault_frame_enter", "dst": 0, "mode": "root_owned"},
                            {"op": "const_i64", "dst": 1, "value": 0},
                            {"op": "const_i64", "dst": 2, "value": 1},
                            {"op": "const_i64", "dst": 3, "value": 3}]),
        jump_block(7, 23, [{"op": "add", "dst": 5, "lhs": 4, "rhs": 2}]),
        {"id": 23, "instructions": [
            {"index": 0, "instruction": {"op": "phi", "dst": 4,
                "inputs": [{"block": 91, "value": 1}, {"block": 7, "value": 5}]}},
            {"index": 1, "instruction": {"op": "compare", "dst": 6,
                "lhs": 4, "rhs": 3, "predicate": "slt"}}],
         "terminator": {"index": 2, "instruction": {"op": "branch", "condition": 6,
             "then": 7, "else": 42, "then_args": None, "else_args": None}},
         "edges": [{"target": 7, "args": None}, {"target": 42, "args": None}]},
        {"id": 42, "instructions": [], "terminator": {"index": 0,
            "instruction": {"op": "return", "value": 4}}, "edges": []}]
    compile_input(scalar_loop)
    link()
    execute("normal", 3, "1 0 0 0 0 1")
    for predicate in ["eq", "ne", "slt", "sle", "sgt", "sge"]:
        data = copy.deepcopy(scalar_loop)
        data["functions"][0]["blocks"][2]["instructions"][1]["instruction"]["predicate"] = predicate
        compile_input(data)  # Some predicates intentionally describe nonterminating loops.
    for label in ["unknown-predicate", "mixed-phi", "non-bool-branch", "late-phi"]:
        data = copy.deepcopy(scalar_loop)
        rows = data["functions"][0]["blocks"][2]["instructions"]
        if label == "unknown-predicate":
            rows[1]["instruction"]["predicate"] = "ult"
        elif label == "mixed-phi":
            data["functions"][0]["blocks"][0]["instructions"][1]["instruction"].update(
                op="const_bool", value=False)
        elif label == "non-bool-branch":
            data["functions"][0]["blocks"][2]["terminator"]["instruction"]["condition"] = 3
        else:
            rows.insert(0, {"index": 0, "instruction": {"op": "const_i64", "dst": 8, "value": 0}})
            for i, row in enumerate(rows):
                row["index"] = i
            data["functions"][0]["blocks"][2]["terminator"]["index"] = len(rows)
        compile_input(data, False)
    print("scalar PHI/backedge EXE3; six predicates compile; four malformed CFG/type cases reject")

    mutual = copy.deepcopy(scalar_loop)
    header = mutual["functions"][0]["blocks"][2]
    header["instructions"][1:1] = [
        {"instruction": {"op": "phi", "dst": 8,
            "inputs": [{"block": 91, "value": 1}, {"block": 7, "value": 9}]}},
        {"instruction": {"op": "phi", "dst": 9,
            "inputs": [{"block": 91, "value": 2}, {"block": 7, "value": 8}]}}]
    for index, row in enumerate(header["instructions"]):
        row["index"] = index
    header["terminator"]["index"] = len(header["instructions"])
    mutual["functions"][0]["blocks"].reverse()
    compile_input(mutual)
    link()
    execute("normal", 3, "1 0 0 0 0 1")
    for label in ["missing-input", "duplicate-pred", "undefined-value", "edge-args", "bool-compare"]:
        data = copy.deepcopy(scalar_loop)
        blocks = data["functions"][0]["blocks"]
        inputs = blocks[2]["instructions"][0]["instruction"]["inputs"]
        if label == "missing-input":
            inputs.pop()
        elif label == "duplicate-pred":
            inputs[1]["block"] = inputs[0]["block"]
        elif label == "undefined-value":
            inputs[1]["value"] = 999999
        elif label == "edge-args":
            blocks[1]["edges"][0]["args"] = {}
        else:
            blocks[0]["instructions"][3]["instruction"].update(op="const_bool", value=True)
        compile_input(data, False)
    print("seeded mutual PHIs execute in reversed order; malformed relations reject")

    # A Bool PHI remains a Bool lane, including a future Compare backedge.
    boolean_loop = copy.deepcopy(scalar_loop)
    blocks = boolean_loop["functions"][0]["blocks"]
    blocks[0]["instructions"].append({"index": 4,
        "instruction": {"op": "const_bool", "dst": 9, "value": False}})
    blocks[0]["terminator"]["index"] = 5
    blocks[1]["instructions"].append({"index": 1,
        "instruction": {"op": "compare", "dst": 10, "lhs": 5, "rhs": 3, "predicate": "slt"}})
    blocks[1]["terminator"]["index"] = 2
    blocks[2]["instructions"].insert(1, {"index": 1, "instruction": {"op": "phi", "dst": 11,
        "inputs": [{"block": 91, "value": 9}, {"block": 7, "value": 10}]}})
    blocks[2]["instructions"][2]["index"] = 2
    blocks[2]["terminator"]["index"] = 3
    compile_input(boolean_loop)
    link()
    execute("normal", 3, "1 0 0 0 0 1")

    # Signed comparison evidence must execute, not merely parse each spelling.
    for predicate, lhs, rhs, expected in [
            ("eq", -1, -1, 1), ("ne", -1, 0, 1),
            ("slt", -(2**63), 2**63 - 1, 1), ("sle", -1, -1, 1),
            ("sgt", -(2**63), 2**63 - 1, 0), ("sge", -1, 0, 0)]:
        data = copy.deepcopy(scalar_loop)
        blocks = data["functions"][0]["blocks"]
        entry = blocks[0]
        entry["instructions"][1]["instruction"]["value"] = lhs
        entry["instructions"][2]["instruction"]["value"] = rhs
        entry["instructions"][3]["instruction"] = {
            "op": "compare", "dst": 6, "lhs": 1, "rhs": 2, "predicate": predicate}
        entry["terminator"]["instruction"] = {"op": "branch", "condition": 6,
            "then": 7, "else": 42, "then_args": None, "else_args": None}
        entry["edges"] = [{"target": 7, "args": None}, {"target": 42, "args": None}]
        yes, no = copy.deepcopy(blocks[3]), copy.deepcopy(blocks[3])
        for block, block_id, value in [(yes, 7, 1), (no, 42, 0)]:
            block["id"] = block_id
            block["instructions"] = [{"index": 0,
                "instruction": {"op": "const_i64", "dst": 20 + value, "value": value}}]
            block["terminator"] = {"index": 1,
                "instruction": {"op": "return", "value": 20 + value}}
        data["functions"][0]["blocks"] = [no, entry, yes]
        compile_input(data)
        link()
        execute("normal", expected, "1 0 0 0 0 1")

    # One allocation slot may be revisited only after both outcome paths
    # discharge it. A live-resource or Fault-state backedge is not a fixed point.
    resource_loop = copy.deepcopy(cycle)
    resource_loop["layouts"] = copy.deepcopy(issued["layouts"])
    resource_loop["process_result_site"] = 12
    def invoke_block(block_id, operation, normal, fault, instructions=()):
        block = jump_block(block_id, normal, instructions)
        block["terminator"]["instruction"] = {"op": "invoke", "operation": operation,
            "fault_frame": 0, "normal": normal, "fault": fault}
        block["edges"].append({"target": fault, "args": None})
        return block
    resource_loop["functions"][0]["blocks"] = [
        jump_block(91, 23, [{"op": "fault_frame_enter", "dst": 0, "mode": "root_owned"}]),
        invoke_block(23, {"kind": "new_box", "object_id": 0, "site": 10}, 7, 42),
        invoke_block(7, {"kind": "reclaim_unpublished", "object_id": 0, "site": 11, "value": 1},
                     23, 42, [{"op": "invoke_normal_result", "dst": 1, "invoke_block": 23}]),
        {"id": 42, "instructions": [], "terminator": {"index": 0,
            "instruction": {"op": "return_fault", "fault_frame": 0}}, "edges": []}]
    compile_input(resource_loop)  # Compile-only infinite allocation/reclaim loop.
    live_backedge = copy.deepcopy(resource_loop)
    block = live_backedge["functions"][0]["blocks"][2]
    block["terminator"]["instruction"] = {"op": "jump", "target": 23, "args": None}
    block["edges"] = [{"target": 23, "args": None}]
    compile_input(live_backedge, False)
    fault_backedge = copy.deepcopy(resource_loop)
    block = fault_backedge["functions"][0]["blocks"][2]
    block["terminator"]["instruction"]["fault"] = 23
    block["edges"][1]["target"] = 23
    compile_input(fault_backedge, False)
    print("Bool PHI, signed boundaries and exact resource/Fault backedge checks passed")

    for lhs, rhs in [(0, 0), (200, 55), (-21, 20), (236, 20), (200, 200)]:
        data = copy.deepcopy(issued)
        constants = [row["instruction"] for block in data["functions"][0]["blocks"]
                     for row in block["instructions"] if row["instruction"]["op"] == "const_i64"]
        assert len(constants) == 2
        for ins, value in zip(constants, [lhs, rhs]):
            ins["value"] = value
        compile_input(data)
        link()
        value = lhs + rhs
        if 0 <= value <= 255:
            execute("normal", value, "1 2 1 0 0 1")
        else:
            execute("normal", 70, "1 2 1 0 1 1",
                    f"FAULT 102 {data['process_result_site']} {value} 0 HOME 1 RECLAIM 0")

    # Same unspecialized Birth, actual source-issued Bool values at either store.
    for index, data in enumerate(bool_inputs):
        assert data["functions"][1] == bool_inputs[0]["functions"][1]
        compile_input(data)
        link()
        execute("normal", 70, f"1 {index} 0 1 1 1",
                f"FAULT 103 {sites[index]} 1 2 HOME 0 RECLAIM 1")

    # Tagged FieldSet expands one physical Fault edge into type/store failures.
    # Both must enter the same forwarding predecessor of the scalar PHI.
    fault_phi = copy.deepcopy(bool_inputs[0])
    birth = fault_phi["functions"][1]
    entry = next(b for b in birth["blocks"] if b["id"] == birth["entry"])
    entry["instructions"].append({"index": len(entry["instructions"]),
        "instruction": {"op": "const_i64", "dst": 8, "value": 42}})
    entry["terminator"]["index"] = len(entry["instructions"])
    fault = next(b for b in birth["blocks"] if b["terminator"]["instruction"]["op"] == "return_fault")
    predecessors = [b["id"] for b in birth["blocks"]
                    if any(e["target"] == fault["id"] for e in b["edges"])]
    fault["instructions"] = [{"index": 0, "instruction": {"op": "phi", "dst": 9,
        "inputs": [{"block": pred, "value": 8} for pred in predecessors]}}]
    fault["terminator"]["index"] = 1
    compile_input(fault_phi)
    link()
    execute("normal", 70, "1 0 0 1 1 1", f"FAULT 103 {sites[0]} 1 2 HOME 0 RECLAIM 1")
    print("expanded tagged Fault edges share the physical PHI predecessor")

    def call_args(data):
        return next(block["terminator"]["instruction"]["operation"]["call"]["args"]
                    for block in data["functions"][0]["blocks"]
                    if block["terminator"]["instruction"].get("operation", {}).get("kind") == "birth_call")

    # Physical Copy must retain both lanes; it is not a new source-family proof.
    copied = copy.deepcopy(bool_inputs[0])
    birth = copied["functions"][1]
    next_id = max(row["instruction"].get("dst", 0) for block in birth["blocks"]
                  for row in block["instructions"]) + 1
    block = next(block for block in birth["blocks"]
                 if block["terminator"]["instruction"].get("operation", {}).get("kind") == "field_set")
    op = block["terminator"]["instruction"]["operation"]
    block["instructions"].append({"index": len(block["instructions"]),
                                  "instruction": {"op": "copy", "dst": next_id, "src": op["value"]}})
    block["terminator"]["index"] += 1
    op["value"] = next_id
    compile_input(copied)
    link()
    execute("normal", 70, "1 0 0 1 1 1", f"FAULT 103 {sites[0]} 1 2 HOME 0 RECLAIM 1")

    # Fault-inject the emitted LLVM text through a private compile-time seam.
    # The production build has no test hook or environment-based mutation.
    for label, mode in [("invalid-kind", "invalid-kind"), ("invalid-bool", "invalid-bool")]:
        compile_input(bool_inputs[0], custom_env=dict(env, HAKO_LIFECYCLE_TEST_SEAM=mode))
        link()
        execute("normal", 70, "1 0 0 0 0 1")
        print(label, "InvalidContract without source Fault")

    # Every failure preserves an existing published object, with no temporary debris.
    sentinel = b"existing artifact"
    obj.write_bytes(sentinel)
    path = work / "input.json"
    path.write_text(json.dumps(issued))
    r = run([driver, path, obj, "bad-session"], env=env)
    assert r.returncode != 0 and "runtime-abi" in r.stderr, r
    assert obj.read_bytes() == sentinel
    malformed = copy.deepcopy(issued)
    del malformed["process_result_site"]
    compile_input(malformed, False)
    assert obj.read_bytes() == sentinel
    malformed["process_result_site"] = 0
    compile_input(malformed, False)
    assert obj.read_bytes() == sentinel
    malformed_inputs = []
    def malformed_case(change):
        data = copy.deepcopy(issued)
        change(data)
        malformed_inputs.append(data)
    malformed_case(lambda d: d.update(schema="hako.published-lifecycle-physical-program.v1"))
    malformed_case(lambda d: d["functions"][1].update(params=[0, 1, 2]))
    malformed_case(lambda d: d["functions"][1]["params"][0].update(representation="i64"))
    malformed_case(lambda d: d["functions"][1].update(receiver=d["functions"][1]["params"][0]["value"]))
    malformed_case(lambda d: call_args(d)[0].pop("kind"))
    malformed_case(lambda d: call_args(d)[0].update(kind=99))
    malformed_case(lambda d: call_args(d)[0].update(kind=2))  # Integer payload is not Bool
    malformed_case(lambda d: call_args(d)[0].update(value=999999))
    malformed_case(lambda d: call_args(d)[0].update(value=next(
        b["terminator"]["instruction"]["operation"]["call"]["receiver"] for b in d["functions"][0]["blocks"]
        if b["terminator"]["instruction"].get("operation", {}).get("kind") == "birth_call")))
    for wrong in [0, 1, "true", None]:
        data = copy.deepcopy(bool_inputs[0])
        const = next(r["instruction"] for b in data["functions"][0]["blocks"] for r in b["instructions"]
                     if r["instruction"]["op"] == "const_bool")
        const["value"] = wrong
        malformed_inputs.append(data)
    bad_add = copy.deepcopy(copied)
    row = next(r["instruction"] for b in bad_add["functions"][1]["blocks"] for r in b["instructions"]
               if r["instruction"]["op"] == "copy")
    row.update(op="add", lhs=row.pop("src"), rhs=bad_add["functions"][1]["params"][0]["value"])
    malformed_inputs.append(bad_add)
    for data in malformed_inputs:
        compile_input(data, False)
        assert obj.read_bytes() == sentinel
    print(len(malformed_inputs), "tag/schema/type negative inputs preserve artifact")

    # Object emission stays in-process; hiding llc from PATH must not change it.
    compile_input(issued, custom_env=dict(env, PATH=str(work)))
    link()
    execute("normal", 30, "1 2 1 0 0 1")

    for mode in ["missing-library", "missing-symbol", "malformed-ir", "verify-invalid",
                 "target-drift", "emit-failure", "empty-output"]:
        obj.write_bytes(sentinel)
        compile_input(issued, False, dict(env, HAKO_LIFECYCLE_TEST_SEAM=mode))
        assert obj.read_bytes() == sentinel
    bad_parent = work / "missing-parent" / "pair.o"
    failed = run([driver, work / "input.json", bad_parent], env=env)
    assert failed.returncode != 0
    obj.unlink()
    compile_input(malformed, False)
    assert not obj.exists()
    print("session mismatch, private library seam and temp cleanup: pre-artifact rejection passed")
