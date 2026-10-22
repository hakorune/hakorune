#!/usr/bin/env python3
"""Consume Rust-captured Script inputs; malformed mutations are physical evidence only."""
import copy
import hashlib
import time
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
work = Path(sys.argv[1])
string_only = len(sys.argv) > 2 and sys.argv[2] == "--checked-string"
owned_only = len(sys.argv) > 2 and sys.argv[2] == "--owned-array"
work.mkdir(parents=True, exist_ok=True)
inputs = [] if string_only or owned_only else [json.loads(path.read_text()) for path in sorted(work.glob("*.json"), key=lambda p: int(p.stem))]
assert string_only or owned_only or len(inputs) == 39


def checked(args):
    result = subprocess.run([str(x) for x in args], text=True, capture_output=True)
    assert result.returncode == 0, (args, result.stdout, result.stderr)
    return result


library_dir = Path(os.environ.get("HAKO_TEST_FFI_DIR", ROOT / "target/release"))
driver = work / "driver"
checked(["cc", ROOT / "lang/c-abi/tests/published_lifecycle_v4_driver.c",
         "-L" + str(library_dir), "-lhako_llvmc_ffi",
         "-Wl,-rpath," + str(library_dir), "-o", driver])
obj = work / "out.o"


def compile_input(data, success):
    path = work / "input.json"
    path.write_text(json.dumps(data))
    before = obj.read_bytes() if obj.exists() else None
    result = subprocess.run([str(driver), str(path), str(obj)], text=True, capture_output=True)
    assert (result.returncode == 0) == success, (result.stdout, result.stderr, data)
    assert not list(work.glob("out.o.*")), "temporary leak"
    if success:
        assert obj.read_bytes().startswith(b"\x7fELF")
    else:
        assert "[freeze:contract]" in result.stderr
        assert obj.read_bytes() == before, "rejection changed existing output"
    return result


def instructions(data):
    for block in data["functions"][0]["blocks"]:
        for row in block["instructions"]:
            yield row["instruction"]
        yield block["terminator"]["instruction"]


def first(data, kind):
    return next(row for row in instructions(data) if row.get("operation", {}).get("kind") == kind)


archive = Path(os.environ.get("HAKO_TEST_LIFECYCLE_ARCHIVE", ROOT / "target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a"))
assert archive.is_file()
def checked_string_physical():
    # Independent physical ABI fixtures. These do not claim source activation.
    literal = lambda text: {"kind": "static_text", "utf8": text}
    owned = lambda value: {"kind": "owned_string", "value": value}
    def inv(kind, normal, fault, **operands):
        return {"op": "invoke", "operation": {"kind": kind, "site": 7, **operands},
                "fault_frame": 0, "normal": normal, "fault": fault}
    def end(value, normal, fault):
        return inv("string_end", normal, fault, value=value)
    def block(id, term, rows=()):
        edges = ([{"target": term[k], "args": None} for k in ["normal", "fault"]]
                 if term["op"] == "invoke" else
                 [{"target": term["target"], "args": None}] if term["op"] == "jump" else
                 [{"target": term[k], "args": None} for k in ["then", "else"]] if term["op"] == "branch" else [])
        return {"id": id, "instructions": [{"index": i, "instruction": r} for i, r in enumerate(rows)],
                "terminator": {"index": len(rows), "instruction": term}, "edges": edges}
    entry = [{"op": "fault_frame_enter", "dst": 0, "mode": "root_owned"},
             {"op": "const_i64", "dst": 1, "value": -9223372036854775808}]
    fault = {"op": "return_fault", "fault_frame": 0}
    rows = [block(0, inv("string_add", 1, 90, left=literal("猫\0"), right=literal("")), entry),
            block(1, inv("string_add", 2, 10, left=owned(2), right={"kind": "normal_integer", "value": 1}),
                  [{"op": "invoke_normal_result", "invoke_block": 0, "dst": 2}]),
            block(2, inv("string_print", 3, 20, value=owned(3)),
                  [{"op": "invoke_normal_result", "invoke_block": 1, "dst": 3}]),
            block(3, end(3, 4, 21)), block(4, end(2, 5, 90)),
            block(5, {"op": "return", "value": 4}, [{"op": "const_i64", "dst": 4, "value": 0}]),
            block(10, end(2, 90, 91)), block(20, end(3, 21, 22)),
            block(21, end(2, 90, 91)), block(22, end(2, 90, 91)), block(90, fault), block(91, fault)]
    # All Invoke Normal landings, including cleanup, remain exclusive.
    for b in list(rows):
        t = b["terminator"]["instruction"]
        if t["op"] == "invoke" and t["operation"]["kind"] != "string_add":
            target = t["normal"]; landing = 1000 + b["id"]; t["normal"] = landing
            b["edges"][0]["target"] = landing
            rows.append(block(landing, {"op": "jump", "target": target, "args": None}))
    for i, b in enumerate(rows):
        if b["terminator"]["instruction"]["op"] == "invoke":
            b["terminator"]["instruction"]["operation"]["site"] = 100 + i
    fn = {"name": "checked_string", "role": "root_i64", "receiver": None,
          "receiver_object": None, "params": [], "entry": 0, "blocks": rows}
    base = {"schema": "hako.published-lifecycle-physical-program.v2", "fault_abi_version": 1,
            "process_result_site": 99, "functions": [fn], "storage_profile": 1, "layouts": []}
    wraps = ["nyash.array.checked_new_v1", "nyash.array.checked_claim_v1",
             "nyash.array.checked_append_i64_v1", "nyash.array.checked_append_bool_v1",
             "nyash.array.checked_append_f64_v1", "nyrt_handle_release_h",
             "nyash.string.checked_add_v1", "nyash.string.checked_print_v1", "nyash.string.checked_end_v1",
             "nyash.string.checked_materialize_v1", "nyash.string.checked_length_v1", "nyash.string.checked_substring_v1", "nyash.string.checked_substring_operands_v1", "nyash.string.checked_index_of_v1", "nyash.string.checked_compare_v1",
             "nyash.fault.report_final_v1", "nyash.fault.frame_dispose_v1"]
    def execute(data, defines=(), status=0, endings=(), payload=None, primary=None):
        compile_input(data, True)
        exe = work / "checked-string"
        checked(["cc", obj, ROOT / "lang/c-abi/tests/published_native_array_runtime_probe.c", archive,
                 "-DTEST_CHECKED_STRING", *defines, *["-Wl,--wrap=" + name for name in wraps],
                 "-lpthread", "-ldl", "-lm", "-o", exe])
        r = subprocess.run([str(exe)], capture_output=True, env=dict(os.environ, NYASH_NYRT_SILENT_RESULT="1", HAKO_NYRT_PLUGIN_HOST="off"))
        assert r.returncode == status, (r.returncode, r.stdout, r.stderr)
        if payload is not None: assert payload in r.stdout, r.stdout
        ends = [line for line in r.stdout.splitlines() if line.startswith(b"STRING_END ")]
        assert ends == [f"STRING_END {i}".encode() for i in endings], r.stdout
        if primary is not None: assert f"FAULT {primary} ".encode() in r.stdout, r.stdout
        if "-DTEST_FAIL_STRING_PRINT" in defines and "-DTEST_FAIL_STRING_END_AT=1" in defines:
            assert b"STRING_SUPPRESSED 9303" in r.stdout, r.stdout
    for native in [False, True]:
        data = copy.deepcopy(base)
        if native:
            del data["storage_profile"]; del data["layouts"]
            data["runtime_requirements"] = {"kind": "native_array", "abi_version": 1}
        execute(data, endings=[1, 0], payload="猫\0-9223372036854775808\n".encode())
        for attempt, endings in [(1, []), (2, [0])]:
            execute(data, [f"-DTEST_FAIL_STRING_ADD_AT={attempt}"], 70, endings, primary=9301)
        execute(data, ["-DTEST_FAIL_STRING_PRINT"], 70, [1, 0], primary=9302)
        execute(data, ["-DTEST_FAIL_STRING_END_AT=1"], 70, [1, 0], primary=9303)
        execute(data, ["-DTEST_FAIL_STRING_PRINT", "-DTEST_FAIL_STRING_END_AT=1"], 70, [1, 0], primary=9302)
        # Reuse the installed-Home runtime slot for a true owned replacement:
        # old End precedes Print; its Fault ends new once and never retries old.
        installed = copy.deepcopy(data)
        installed_rows = installed["functions"][0]["blocks"]
        for block_id,value in [(1,2),(2,3)]:
            b = next(b for b in installed_rows if b["id"] == block_id)
            event = {"op":"string_residence_install","value":value}
            if block_id == 2: event["replaces"] = 2
            b["instructions"].append({"index":1,"instruction":event})
            b["terminator"]["index"] = 2
        for block_id,operation in [(2,{"kind":"string_end","value":2}),
                                   (3,{"kind":"string_print","value":owned(3)})]:
            b = next(b for b in installed_rows if b["id"] == block_id)
            operation["site"] = b["terminator"]["instruction"]["operation"]["site"]
            b["terminator"]["instruction"]["operation"] = operation
        b = next(b for b in installed_rows if b["id"] == 2)
        b["terminator"]["instruction"]["fault"] = b["edges"][1]["target"] = 21
        for block_id in [4,21]:
            next(b for b in installed_rows if b["id"] == block_id)["terminator"]["instruction"]["operation"]["value"] = 3
        installed_rows[:] = [b for b in installed_rows if b["id"] not in [20,22,1020,1022]]
        execute(installed, endings=[0,1])
        execute(installed, ["-DTEST_FAIL_STRING_PRINT"], 70, [0,1], primary=9302)
        execute(installed, ["-DTEST_FAIL_STRING_END_AT=1"], 70, [0,1], primary=9303)
        both_homes = copy.deepcopy(data)
        for block_id,value in [(1,2),(2,3)]:
            b = next(b for b in both_homes["functions"][0]["blocks"] if b["id"] == block_id)
            b["instructions"].append({"index":1,"instruction":{"op":"string_residence_install","value":value}})
            b["terminator"]["index"] = 2
        execute(both_homes, endings=[1,0])
        for block_id,value in [(2,2),(2,1),(2,3),(10,2)]:
            bad = copy.deepcopy(installed)
            b = next(b for b in bad["functions"][0]["blocks"] if b["id"] == block_id)
            event = {"index":0,"instruction":{"op":"string_residence_install","value":value}}
            if block_id == 2 and value == 2: b["instructions"].insert(1,event)
            else: b["instructions"].append(event)
            for i,row in enumerate(b["instructions"]): row["index"] = i
            b["terminator"]["index"] = len(b["instructions"])
            compile_input(bad, False)
        bad = copy.deepcopy(data)
        b = next(b for b in bad["functions"][0]["blocks"] if b["id"] == 2)
        b["instructions"].append({"index":1,"instruction":{"op":"string_residence_install","value":3}})
        b["terminator"]["index"] = 2
        compile_input(bad, False)  # Home cannot end before the remaining child.
        bare = copy.deepcopy(data)
        bare["functions"][0]["blocks"] = [block(0, inv("string_print", 1, 2, value=literal("猫\0")), entry),
            block(1, {"op": "return", "value": 4}, [{"op": "const_i64", "dst": 4, "value": 0}]), block(2, fault)]
        execute(bare, endings=[], payload="猫\0\n".encode())
        if native:
            mixed = copy.deepcopy(data)
            f = mixed["functions"][0]; f["entry"] = 2000
            b = next(b for b in f["blocks"] if b["id"] == 0)
            b["instructions"] = [{"index": 0, "instruction": entry[1]}]; b["terminator"]["index"] = 1
            f["blocks"] += [
                block(2000, inv("array_new", 2001, 2004), [entry[0]]),
                block(2001, inv("array_claim", 2002, 2003, array=10, element_tag=4),
                      [{"op": "invoke_normal_result", "invoke_block": 2000, "dst": 10}]),
                block(2002, {"op": "jump", "target": 0, "args": None}),
                block(2003, fault, [{"op": "array_residence_release", "value": 10}]), block(2004, fault)]
            for b in f["blocks"]:
                if b["id"] in [2000, 2001]: b["terminator"]["instruction"]["operation"]["site"] = b["id"]
                if b["id"] in [5, 90, 91]:
                    b["instructions"].append({"index": len(b["instructions"]),
                        "instruction": {"op": "array_residence_release", "value": 10}})
                    b["terminator"]["index"] = len(b["instructions"])
            execute(mixed, endings=[1, 0], payload="猫\0-9223372036854775808\n".encode())
            execute(mixed, ["-DTEST_FAIL_STRING_PRINT", "-DTEST_FAIL_STRING_END_AT=1"], 70, [1, 0], primary=9302)
            bad = copy.deepcopy(mixed)
            next(b for b in bad["functions"][0]["blocks"] if b["id"] == 3)["instructions"] = [
                {"index": 0, "instruction": {"op": "array_residence_release", "value": 3}}]
            next(b for b in bad["functions"][0]["blocks"] if b["id"] == 3)["terminator"]["index"] = 1
            compile_input(bad, False)
        for block_id, value in [(3, 2), (4, 3), (10, 3)]:
            bad = copy.deepcopy(data)
            b = next(b for b in bad["functions"][0]["blocks"] if b["id"] == block_id)
            b["terminator"]["instruction"]["operation"]["value"] = value
            compile_input(bad, False)
        for change in ["integer_alias", "copy", "extra_key", "missing_end", "duplicate_site"]:
            bad = copy.deepcopy(data)
            b = next(b for b in bad["functions"][0]["blocks"] if b["id"] == 2)
            if change == "integer_alias": b["terminator"]["instruction"]["operation"]["value"]["kind"] = "normal_integer"
            elif change == "duplicate_site": b["terminator"]["instruction"]["operation"]["site"] = 100
            elif change == "extra_key": b["terminator"]["instruction"]["operation"]["value"]["token"] = 9
            elif change == "copy":
                b["instructions"].append({"index": 1, "instruction": {"op": "copy", "src": 3, "dst": 100}})
                b["terminator"]["index"] = 2
            else:
                b = next(b for b in bad["functions"][0]["blocks"] if b["id"] == 10)
                b["terminator"] = {"index": 0, "instruction": fault}; b["edges"] = []
            compile_input(bad, False)
        cp = copy.deepcopy(data)
        projection = lambda source, dst: {"op": "invoke_normal_result", "invoke_block": source, "dst": dst}
        cp_rows = [
            block(0, inv("string_materialize", 1, 90, value=literal("aé界z")),
                  [entry[0], {"op":"const_i64", "dst":1, "value":1}, {"op":"const_i64", "dst":4, "value":3},
                   {"op":"const_i64", "dst":5, "value":0}]),
            block(1, inv("string_substring", 2, 10, value=owned(2), start=1, end=4), [projection(0,2)]),
            block(2, inv("string_length", 3, 20, value=owned(3)), [projection(1,3)]),
            block(3, inv("string_index_of", 4, 20, value=owned(3), needle=literal("界")), [projection(2,7)]),
            block(4, inv("string_print", 5, 20, value={"kind":"normal_integer", "value":7}), [projection(3,8)]),
            block(5, inv("string_print", 6, 20, value={"kind":"normal_integer", "value":8})),
            block(6, inv("string_print", 7, 20, value=owned(3))),
            block(7, end(3, 8, 21)), block(8, end(2, 9, 90)), block(9, {"op":"return", "value":5}),
            block(10, end(2, 90, 91)), block(20, end(3, 21, 22)),
            block(21, end(2, 90, 91)), block(22, end(2, 90, 91)), block(90, fault), block(91, fault)]
        for b in list(cp_rows):
            term = b["terminator"]["instruction"]
            if term["op"] == "invoke" and term["operation"]["kind"] in ["string_end", "string_print"]:
                target = term["normal"]; landing = 1000 + b["id"]; term["normal"] = landing
                b["edges"][0]["target"] = landing
                cp_rows.append(block(landing, {"op":"jump", "target":target, "args":None}))
        for i, b in enumerate(cp_rows):
            if b["terminator"]["instruction"]["op"] == "invoke": b["terminator"]["instruction"]["operation"]["site"] = 200 + i
        cp["functions"][0]["blocks"] = cp_rows
        execute(cp, endings=[1,0], payload="2\n1\né界\n".encode())
        execute(cp, ["-DTEST_FAIL_STRING_SUBSTRING"], 70, [0], primary=9304)
        for read_at in [1,2]: execute(cp, [f"-DTEST_FAIL_STRING_READ_AT={read_at}"], 70, [1,0], primary=9305)
        for mutation in ["owned_bound", "integer_receiver", "integer_needle", "end_scalar", "slice_scalar"]:
            bad = copy.deepcopy(cp)
            if mutation == "owned_bound": first(bad, "string_substring")["operation"]["start"] = 2
            elif mutation == "integer_receiver": first(bad, "string_materialize")["operation"]["value"] = {"kind":"normal_integer", "value":1}
            elif mutation == "integer_needle": first(bad, "string_index_of")["operation"]["needle"] = {"kind":"normal_integer", "value":1}
            elif mutation == "end_scalar": first(bad, "string_end")["operation"]["value"] = 7
            else: first(bad, "string_print")["operation"]["value"] = {"kind":"normal_integer", "value":3}
            compile_input(bad, False)
        # Independent Bool consumers: exact Unicode scalar order and proper
        # prefixes, both Normal outcomes, live paired borrows and Fault cleanup.
        for comparison, predicate in [
            ("eq", lambda a,b: a == b), ("ne", lambda a,b: a != b),
            ("lt", lambda a,b: a < b), ("le", lambda a,b: a <= b),
            ("gt", lambda a,b: a > b), ("ge", lambda a,b: a >= b),
        ]:
            pairs = ([("é", "é"), ("é", "e\u0301")] if comparison in ["eq", "ne"] else
                     [("\ue000", "\U00010000"), ("\U00010000", "\ue000")] if comparison == "lt" else
                     [("a", "aa"), ("aa", "a")] if comparison == "le" else
                     [("a\0z", "a\0y"), ("a\0y", "a\0z")])
            for case, (left, right) in enumerate(pairs):
                expected = predicate(left, right)
                compare_rows = [
                    block(0, inv("string_add",1,90,left=literal(left),right=literal("")),
                          [entry[0], {"op":"const_i64","dst":4,"value":0}, {"op":"const_i64","dst":5,"value":1}]),
                    block(1, inv("string_compare",2,10,comparison=comparison,left=owned(2) if not case else literal(left),right=literal(right)), [projection(0,2)]),
                    block(2, {"op":"branch","condition":3,"then":3 if expected else 4,"else":4 if expected else 3,
                              "then_args":None,"else_args":None}, [projection(1,3)]),
                    block(3, end(2,5,90)), block(4, end(2,6,90)),
                    block(5, {"op":"return","value":4}), block(6, {"op":"return","value":5}),
                    block(10, end(2,90,91)), block(90,fault), block(91,fault),
                ]
                for b in list(compare_rows):
                    term = b["terminator"]["instruction"]
                    if term["op"] == "invoke" and term["operation"]["kind"] == "string_end":
                        target = term["normal"]; landing = 1000+b["id"]; term["normal"] = landing
                        b["edges"][0]["target"] = landing
                        compare_rows.append(block(landing,{"op":"jump","target":target,"args":None}))
                for i,b in enumerate(compare_rows):
                    if b["terminator"]["instruction"]["op"] == "invoke": b["terminator"]["instruction"]["operation"]["site"] = 300+i
                compared = copy.deepcopy(data); compared["functions"][0]["blocks"] = compare_rows
                execute(compared, endings=[0])
            if comparison == "eq":
                for text_operands in ["static_owned", "same_owned"]:
                    variant = copy.deepcopy(compared)
                    operation = first(variant,"string_compare")["operation"]
                    operation["left"] = literal(left) if text_operands == "static_owned" else owned(2)
                    operation["right"] = owned(2)
                    branch = next(b for b in variant["functions"][0]["blocks"] if b["id"] == 2)
                    branch["terminator"]["instruction"].update(then=3, **{"else":4})
                    branch["edges"] = [{"target":3,"args":None},{"target":4,"args":None}]
                    execute(variant, endings=[0])
            if comparison != "lt": continue
            execute(compared, ["-DTEST_FAIL_STRING_COMPARE"], 70, [0], primary=9306)
            for mutation in ["unknown", "nul_suffix", "numeric", "missing", "extra", "integer_condition", "integer_left", "integer_right", "bool_as_text", "bool_as_i64"]:
                bad = copy.deepcopy(compared); operation = first(bad,"string_compare")["operation"]
                if mutation == "unknown": operation["comparison"] = "slt"
                elif mutation == "nul_suffix": operation["comparison"] = "lt\0suffix"
                elif mutation == "numeric": operation["comparison"] = 2
                elif mutation == "integer_condition":
                    next(b for b in bad["functions"][0]["blocks"] if b["id"] == 2)["terminator"]["instruction"]["condition"] = 4
                elif mutation == "missing": del operation["comparison"]
                elif mutation == "extra": operation["extra"] = 0
                elif mutation in ["integer_left","integer_right"]:
                    operation[mutation[8:]] = {"kind":"normal_integer","value":4}
                else:
                    normal = next(b for b in bad["functions"][0]["blocks"] if b["id"] == 2)
                    branch = normal["terminator"]["instruction"]
                    term = inv("string_print",1002,10,value={"kind":"owned_string" if mutation == "bool_as_text" else "normal_integer","value":3}); term["operation"]["site"] = 999
                    normal["terminator"]["instruction"] = term
                    normal["edges"] = [{"target":1002,"args":None},{"target":10,"args":None}]
                    bad["functions"][0]["blocks"].append(block(1002,branch))
                compile_input(bad,False)
    print("checked String: indexed/native real ABI, literal NUL, MIN Integer, Normal/Fault LIFO End and malformed input refusal")


def owned_array_physical():
    # Independent physical graph, never source admission or app acceptance.
    def inv(kind, normal, fault, **args):
        operation = {"kind": kind, **args}
        if kind != "ordinary_call": operation["site"] = 1
        return {"op": "invoke", "operation": operation,
                "fault_frame": 0, "normal": normal, "fault": fault}
    def own(kind, normal, fault, **args):
        return inv("array_owned_" + kind, normal, fault,
                   array=10, object_id=1, nullable=True, **args)
    def block(id, term, rows=()):
        edges = ([{"target": term[k], "args": None} for k in ("normal", "fault")]
                 if term["op"] == "invoke" else
                 [{"target": term["target"], "args": None}] if term["op"] == "jump" else
                 [{"target": term[k], "args": None} for k in ("then", "else")]
                 if term["op"] == "branch" else [])
        return {"id": id, "edges": edges,
                "instructions": [{"index": i, "instruction": r} for i, r in enumerate(rows)],
                "terminator": {"index": len(rows), "instruction": term}}
    def result(producer, dst):
        return {"op": "invoke_normal_result", "invoke_block": producer, "dst": dst}
    fault = {"op": "return_fault", "fault_frame": 0}
    rows = [
        block(0, inv("array_new", 1, 90), [
            {"op": "fault_frame_enter", "mode": "root_owned", "dst": 0},
            {"op": "const_i64", "dst": 1, "value": 0}]),
        block(1, own("claim", 2, 91), [result(0, 10)]),
        block(2, inv("new_box", 3, 92, object_id=1)),
        block(3, own("append", 4, 93, value=20), [result(2, 20)]),
        block(4, own("get", 5, 92, index=1)),
        block(5, inv("ordinary_call", 6, 92, result="i64", call={
            "target": 1, "dst": None, "args": [{"kind": "nullable_typed_object", "value": 31}]}),
            [result(4, 30), {"op": "copy", "dst": 31, "src": 30}]),
        block(6, own("end", 7, 90), [result(5, 40)]),
        block(7, {"op": "return", "value": 40}), block(90, fault),
        block(91, fault, [{"op": "array_residence_release", "value": 10}]),
        block(92, own("end", 90, 90)),
        block(93, inv("home_release", 92, 92, object_id=1, value=20)),
        block(94, inv("new_box", 95, 92, object_id=1)),
        block(95, own("append", 4, 96, value=21), [result(94, 21)]),
        block(96, inv("home_release", 92, 92, object_id=1, value=21))]
    rows[3]["terminator"]["instruction"]["normal"] = 94
    rows[3]["edges"][0]["target"] = 94
    value_ops = {"array_new", "new_box", "array_owned_get", "ordinary_call"}
    for b in list(rows):
        term = b["terminator"]["instruction"]
        if term["op"] == "invoke" and term["operation"]["kind"] not in value_ops:
            target = term["normal"]; landing = 1000 + b["id"]
            term["normal"] = landing; b["edges"][0]["target"] = landing
            rows.append(block(landing, {"op": "jump", "target": target, "args": None}))
    for i, b in enumerate(rows):
        if b["terminator"]["instruction"]["op"] == "invoke" and b["terminator"]["instruction"]["operation"]["kind"] != "ordinary_call":
            b["terminator"]["instruction"]["operation"]["site"] = 100 + i
    worker = {"name": "observe_null", "role": "ordinary_i64", "receiver": None,
        "receiver_object": None, "params": [{"value": 1,
            "representation": "borrowed_kind_payload_v1", "object_view": 1}], "entry": 0,
        "blocks": [block(0, {"op": "branch", "condition": 3, "then": 1, "else": 2,
            "then_args": None, "else_args": None}, [
                {"op": "fault_frame_enter", "mode": "borrowed", "dst": 0},
                {"op": "const_null", "dst": 2},
                {"op": "borrowed_null_compare", "dst": 3, "lhs": 1, "rhs": 2, "predicate": "eq"}]),
            block(1, {"op": "return", "value": 4}, [{"op": "const_i64", "dst": 4, "value": 1}]),
            block(2, {"op": "return", "value": 5}, [{"op": "const_i64", "dst": 5, "value": 0}])]}
    base = {"schema": "hako.published-lifecycle-physical-program.v2", "fault_abi_version": 1,
        "process_result_site": 99, "storage_profile": 1,
        "layouts": [{"object_id": 1, "runtime_type_id": 770001, "field_count": 0,
            "fields": [], "owned_residences": [], "owned_object_residences": []}],
        "functions": [{"name": "owned_array", "role": "root_i64", "receiver": None,
            "receiver_object": None, "params": [], "entry": 0, "blocks": rows}, worker]}
    probe = work / "owned-runtime-observer.c"
    probe.write_text(r'''/* Observe the real ABI and compiled child End callback; issue no source proof. */
#include <assert.h>
#include <stdio.h>
#include "nyrt_fault_v1.h"
#ifndef TEST_FAIL_APPEND_AT
#define TEST_FAIL_APPEND_AT 0
#endif
#ifndef TEST_FAIL_CHILD_END_AT
#define TEST_FAIL_CHILD_END_AT 0
#endif
static int64_t children[8];
static unsigned made, ended, attempts, arrays_ended;
static const NyrtArrayOwnedDescriptorV1* original;
static void descriptor(const NyrtArrayOwnedDescriptorV1* d) {
  assert(d && d->profile == 1 && d->nullable == 1 && d->type_id == 770001 && d->end);
  if (!original) original = d;
  assert(original == d);
}
uint32_t real_new(void*,uint32_t,uint64_t,int64_t,const uint32_t*,size_t,int64_t*) __asm__("__real_nyash.object.checked_new_v1");
uint32_t wrap_new(void*,uint32_t,uint64_t,int64_t,const uint32_t*,size_t,int64_t*) __asm__("__wrap_nyash.object.checked_new_v1");
uint32_t wrap_new(void* f,uint32_t p,uint64_t s,int64_t t,const uint32_t* l,size_t n,int64_t* o) {
  uint32_t r=real_new(f,p,s,t,l,n,o); if (!r) { assert(made<8); children[made++]=*o; } return r;
}
uint32_t real_release(void*,uint32_t,uint64_t,int64_t,int64_t) __asm__("__real_nyash.object.home_release_plain_i64_v1");
uint32_t wrap_release(void*,uint32_t,uint64_t,int64_t,int64_t) __asm__("__wrap_nyash.object.home_release_plain_i64_v1");
uint32_t wrap_release(void* f,uint32_t p,uint64_t s,int64_t h,int64_t t) {
  assert(ended<made && h==children[made-ended-1] && t==770001);
  printf("CHILD_END %u\n",made-ended); ended++;
  uint32_t r=real_release(f,p,s,h,t); assert(r==0);
  return ended==TEST_FAIL_CHILD_END_AT ? nyrt_fault_record_static_v1(f,9402,s,ended,0) : r;
}
#define OWN_PROTO(name,args) uint32_t real_##name args __asm__("__real_nyash.array.checked_owned_" #name "_v1"); \
  uint32_t wrap_##name args __asm__("__wrap_nyash.array.checked_owned_" #name "_v1")
OWN_PROTO(claim,(void*,uint64_t,int64_t,const NyrtArrayOwnedDescriptorV1*));
uint32_t wrap_claim(void* f,uint64_t s,int64_t h,const NyrtArrayOwnedDescriptorV1* d) {
  descriptor(d); return real_claim(f,s,h,d);
}
OWN_PROTO(append,(void*,uint64_t,int64_t,const NyrtArrayOwnedDescriptorV1*,int64_t));
uint32_t wrap_append(void* f,uint64_t s,int64_t h,const NyrtArrayOwnedDescriptorV1* d,int64_t c) {
  descriptor(d); attempts++;
  if (attempts==TEST_FAIL_APPEND_AT) { printf("APPEND_FAULT %u\n",attempts); return nyrt_fault_record_static_v1(f,9401,s,attempts,0); }
  return real_append(f,s,h,d,c);
}
OWN_PROTO(get,(void*,uint64_t,int64_t,const NyrtArrayOwnedDescriptorV1*,int64_t,int64_t*));
uint32_t wrap_get(void* f,uint64_t s,int64_t h,const NyrtArrayOwnedDescriptorV1* d,int64_t i,int64_t* o) {
  descriptor(d); return real_get(f,s,h,d,i,o);
}
OWN_PROTO(end,(void*,uint64_t,int64_t,const NyrtArrayOwnedDescriptorV1*));
uint32_t wrap_end(void* f,uint64_t s,int64_t h,const NyrtArrayOwnedDescriptorV1* d) {
  descriptor(d); uint32_t r=real_end(f,s,h,d);
  assert(ended==made && arrays_ended++==0); printf("ARRAY_END %u\n",r); return r;
}
int32_t real_report(const void*) __asm__("__real_nyash.fault.report_final_v1");
int32_t wrap_report(const void*) __asm__("__wrap_nyash.fault.report_final_v1");
int32_t wrap_report(const void* f) {
  const NyrtFaultFrameV1* frame=f; assert(ended==made && arrays_ended==1);
  printf("FAULT %u\n",frame->primary.reason);
  for (unsigned i=0;i<frame->suppressed_len;i++) printf("SUPPRESSED %u\n",frame->suppressed[i].reason);
  return real_report(f);
}
uint32_t real_dispose(void*) __asm__("__real_nyash.fault.frame_dispose_v1");
uint32_t wrap_dispose(void*) __asm__("__wrap_nyash.fault.frame_dispose_v1");
uint32_t wrap_dispose(void* f) {
  assert(ended==made && arrays_ended==1); printf("DISPOSE %u %u\n",made,ended); return real_dispose(f);
}
''')
    wraps = ["nyash.object.checked_new_v1", "nyash.object.home_release_plain_i64_v1",
        *["nyash.array.checked_owned_" + kind + "_v1" for kind in ("claim", "append", "get", "end")],
        "nyash.fault.report_final_v1", "nyash.fault.frame_dispose_v1"]
    executions, rejections = [], []
    def digest(path):
        with Path(path).open("rb") as stream:
            return hashlib.file_digest(stream, "sha256").hexdigest()
    def execute(data, expected, defines=(), ends=(2, 1), primary=None, suppressed=None):
        started = time.monotonic()
        compile_input(data, True)
        object_seconds = time.monotonic() - started
        started = time.monotonic()
        exe = work / "owned-array"
        checked(["cc", obj, probe, archive, "-I" + str(ROOT / "include"),
            *defines, *["-Wl,--wrap=" + kind for kind in wraps],
            "-lpthread", "-ldl", "-lm", "-o", exe])
        link_seconds = time.monotonic() - started
        started = time.monotonic()
        run = subprocess.run([str(exe)], text=True, capture_output=True,
            env=dict(os.environ, NYASH_NYRT_SILENT_RESULT="1", HAKO_NYRT_PLUGIN_HOST="off"))
        run_seconds = time.monotonic() - started
        assert run.returncode == expected, (run.returncode, run.stdout, run.stderr)
        lines = run.stdout.splitlines()
        assert [line for line in lines if line.startswith("CHILD_END ")] == [f"CHILD_END {i}" for i in ends], lines
        assert lines[-1] == f"DISPOSE {len(ends)} {len(ends)}", lines
        assert sum(line.startswith("ARRAY_END ") for line in lines) == 1, lines
        if primary is not None: assert f"FAULT {primary}" in lines, lines
        if suppressed is not None: assert f"SUPPRESSED {suppressed}" in lines, lines
        case = work / f"owned-case-{len(executions)}.json"
        case.write_text(json.dumps(data))
        executions.append({"input": str(case), "input_sha256": digest(case),
            "object_sha256": digest(obj), "exe_sha256": digest(exe), "defines": list(defines),
            "exit": run.returncode, "stdout": run.stdout, "stderr": run.stderr,
            "object_seconds": object_seconds, "link_seconds": link_seconds, "run_seconds": run_seconds})
    for index, expected in [(0, 0), (1, 0), (2, 1), (-1, 1)]:
        data = copy.deepcopy(base)
        data["functions"][0]["blocks"][0]["instructions"][1]["instruction"]["value"] = index
        execute(data, expected)
    execute(base, 70, ["-DTEST_FAIL_APPEND_AT=2"], primary=9401)
    execute(base, 70, ["-DTEST_FAIL_CHILD_END_AT=1"], primary=9402)
    execute(base, 70, ["-DTEST_FAIL_APPEND_AT=2", "-DTEST_FAIL_CHILD_END_AT=1"], primary=9401, suppressed=9402)
    stored_null = copy.deepcopy(base)
    by_id = {b["id"]: b for b in stored_null["functions"][0]["blocks"]}
    for id, target in [(94, 95), (96, 92)]:
        by_id[id]["terminator"]["instruction"] = {"op": "jump", "target": target, "args": None}
        by_id[id]["edges"] = [{"target": target, "args": None}]
    by_id[95]["instructions"][0]["instruction"] = {"op": "const_null", "dst": 21}
    stored_null["functions"][0]["blocks"].remove(by_id[1096])
    by_id[0]["instructions"][1]["instruction"]["value"] = 1
    execute(stored_null, 1, ends=(1,))
    execute(stored_null, 70, ["-DTEST_FAIL_APPEND_AT=2"], ends=(1,), primary=9401)
    mutations = []
    def mutate(change):
        data = copy.deepcopy(base); change(data); mutations.append(data)
    def op(data, kind):
        return first(data, "array_owned_" + kind)["operation"]
    def root_block(data, id):
        return next(b for b in data["functions"][0]["blocks"] if b["id"] == id)
    mutate(lambda d: op(d, "get").update(nullable=False))
    mutate(lambda d: op(d, "get").update(index=10))
    mutate(lambda d: op(d, "append").update(value=10))
    mutate(lambda d: op(d, "get").update(object_id=999))
    mutate(lambda d: op(d, "get").pop("nullable"))
    mutate(lambda d: op(d, "get").update(nullable=1))
    mutate(lambda d: root_block(d, 7)["terminator"]["instruction"].update(value=31))
    mutate(lambda d: first(d, "ordinary_call")["operation"]["call"]["args"][0].update(kind=3))
    def late_borrow(data):
        b = root_block(data, 7)
        b["instructions"].append({"index": 0, "instruction": {"op": "copy", "src": 31, "dst": 41}})
        b["terminator"]["index"] = 1
    mutate(late_borrow)
    def bypass(data, id, target):
        b = root_block(data, id)
        b["terminator"]["instruction"] = {"op": "jump", "target": target, "args": None}
        b["edges"] = [{"target": target, "args": None}]
    mutate(lambda d: bypass(d, 93, 92))  # Fault keeps candidate cleanup owed.
    mutate(lambda d: bypass(d, 6, 7))    # Successful append still owes holder End.
    def foreign_view(data):
        layout = copy.deepcopy(data["layouts"][0]); layout.update(object_id=2, runtime_type_id=770002)
        data["layouts"].append(layout)
        data["functions"][1]["params"][0]["object_view"] = 2
    mutate(foreign_view)
    def no_layout(data):
        data.pop("storage_profile"); data.pop("layouts")
        data["runtime_requirements"] = {"kind": "native_array", "abi_version": 1}
    mutate(no_layout)
    for data in mutations:
        result = compile_input(data, False)
        rejections.append({"input_sha256": digest(work / "input.json"),
            "exit": result.returncode, "stderr": result.stderr})
    (work / "owned-runtime-results.json").write_text(json.dumps({
        "head": checked(["git", "-C", ROOT, "rev-parse", "HEAD"]).stdout.strip(),
        "script_sha256": digest(__file__), "ffi_sha256": digest(library_dir / "libhako_llvmc_ffi.so"),
        "archive": str(archive), "archive_sha256": digest(archive),
        "executions": executions, "rejections": rejections,
        "env_overrides": {"NYASH_NYRT_SILENT_RESULT": "1", "HAKO_NYRT_PLUGIN_HOST": "off"},
        "non_claim": "independent physical/runtime ABI graph, no source Recipe or app acceptance"
    }, indent=2))
    print("owned Array: real indexed nonnull/missing/stored-null Get; Normal and injected append/child-End Fault cleanup, reverse all-child End and retained primary/suppressed; malformed wire/lifetime/call refusal; no source acceptance")


if owned_only:
    owned_array_physical()
    sys.exit(0)


if string_only:
    checked_string_physical()
    sys.exit(0)


wraps = ["nyash.array.checked_new_v1", "nyash.array.checked_claim_v1",
         "nyash.array.checked_append_i64_v1", "nyash.array.checked_append_bool_v1",
         "nyash.array.checked_append_f64_v1", "nyrt_handle_release_h",
         "nyash.fault.report_final_v1", "nyash.fault.frame_dispose_v1"]
exe = work / "native"
for index, data in enumerate(inputs):
    compile_input(data, True)
    host_obj = work / f"{index}.host.o"
    assert host_obj.is_file(), "production host object required"
    checked(["cc", host_obj, ROOT / "lang/c-abi/tests/published_native_array_runtime_probe.c",
             archive, *["-Wl,--wrap=" + name for name in wraps],
             "-lpthread", "-ldl", "-lm", "-o", exe])
    result = subprocess.run([str(exe)], text=True, capture_output=True,
                            env=dict(os.environ, NYASH_NYRT_SILENT_RESULT="1", HAKO_NYRT_PLUGIN_HOST="off"))
    expected = ((0 if data["functions"][0]["role"] == "root_unit" else 30)
                if index < 28 else 70 if index < 31 else [0, 255, 70, 70][(index - 31) // 2])
    assert result.returncode == expected, (index, result)
    lines = result.stdout.splitlines()
    if index < 28 or index in range(31, 35):
        assert lines[-3:] == ["RELEASE 2 0 0", "RELEASE 1 2 10", "DISPOSE 2 2 2 0"], lines
        assert not any(line.startswith("FAULT ") for line in lines)
    elif index < 31:
        kind = ["bool", "f64", "i64"][index - 28]
        subtype = 3 if kind == "i64" else 1
        assert lines[-5:] == [f"APPEND 2 {kind} 1 0 0", "RELEASE 2 0 0", "RELEASE 1 1 7",
                              f"FAULT 202 {subtype} 0", "DISPOSE 2 2 2 1"], lines

    else:
        actual = [256, 2**63-1][(index - 35) // 2]
        assert lines[-4:] == ["RELEASE 2 0 0", "RELEASE 1 2 10",
                              f"FAULT 102 {actual} 0", "DISPOSE 2 2 2 1"], lines

# Use untouched production objects for optimized/unoptimized I64 and Unit roots.
for index in range(4):
    for failed_attempt in [1, 2]:
        checked(["cc", work / f"{index}.host.o",
                 ROOT / "lang/c-abi/tests/published_native_array_runtime_probe.c",
                 f"-DTEST_FAIL_NEW_AT={failed_attempt}", archive,
                 *["-Wl,--wrap=" + name for name in wraps],
                 "-lpthread", "-ldl", "-lm", "-o", exe])
        result = subprocess.run([str(exe)], text=True, capture_output=True,
                                env=dict(os.environ, NYASH_NYRT_SILENT_RESULT="1", HAKO_NYRT_PLUGIN_HOST="off"))
        assert result.returncode == 70, (index, failed_attempt, result)
        prefix = [] if failed_attempt == 1 else [
            "NEW 1 0", "CLAIM 1 1 0", "APPEND 1 i64 0 0 1", "APPEND 1 i64 0 1 2"]
        cleanup = [] if failed_attempt == 1 else ["RELEASE 1 2 10"]
        counts = "0 0 0" if failed_attempt == 1 else "1 2 1"
        assert result.stdout.splitlines() == prefix + [f"NEW_FAULT {failed_attempt}"] + cleanup + [
            f"FAULT 9001 {failed_attempt} 0", f"DISPOSE {counts} 1"], result

base = inputs[2]  # optimized I64, with aliases and two residences
mutations = []


def mutate(change):
    data = copy.deepcopy(base)
    change(data)
    mutations.append(data)


mutate(lambda d: d["runtime_requirements"].update(abi_version=2))
mutate(lambda d: d["runtime_requirements"].update(kind="unknown"))
mutate(lambda d: d.update(storage_profile=0))
mutate(lambda d: d.update(layouts=[]))
mutate(lambda d: d["functions"].append(copy.deepcopy(d["functions"][0])))
mutate(lambda d: d["functions"][0].update(role="birth_unit"))
mutate(lambda d: d["functions"][0].update(receiver=0))
for tag in [0, 8, -1, 2**32]:
    mutate(lambda d, tag=tag: first(d, "array_claim")["operation"].update(element_tag=tag))
mutate(lambda d: first(d, "array_append")["operation"].update(representation="bool"))
mutate(lambda d: first(d, "array_append")["operation"].update(representation="f64"))
mutate(lambda d: first(d, "array_append")["operation"].update(array=2**32-1))
mutate(lambda d: first(d, "array_append")["operation"].update(value=2**32-1))
mutate(lambda d: first(d, "array_new")["operation"].update(site=d["process_result_site"]))
mutate(lambda d: first(d, "array_claim")["operation"].update(site=first(d, "array_new")["operation"]["site"]))
mutate(lambda d: first(d, "array_new").update(fault_frame=2**32-1))
mutate(lambda d: first(d, "array_new").update(normal=first(d, "array_new")["fault"]))


def rewrite_block(data, choose, change):
    block = next(b for b in data["functions"][0]["blocks"] if choose(b))
    change(block)
    for index, row in enumerate(block["instructions"]):
        row["index"] = index
    block["terminator"]["index"] = len(block["instructions"])


def has_release(block):
    return any(row["instruction"]["op"] == "array_residence_release" for row in block["instructions"])


def duplicate_release(block):
    row = next(row for row in block["instructions"] if row["instruction"]["op"] == "array_residence_release")
    block["instructions"].append(copy.deepcopy(row))


mutate(lambda d: rewrite_block(d, has_release, duplicate_release))
mutate(lambda d: rewrite_block(d, has_release, lambda b: b["instructions"].remove(
    next(row for row in b["instructions"] if row["instruction"]["op"] == "array_residence_release"))))
mutate(lambda d: next(row for row in instructions(d) if row["op"] == "return").update(value=None))


def skip_claim(block):
    target = block["terminator"]["instruction"]["normal"]
    block["terminator"]["instruction"] = {"op": "jump", "target": target, "args": None}
    block["edges"] = [{"target": target, "args": None}]


def release_before_append(block):
    handle = block["terminator"]["instruction"]["operation"]["array"]
    block["instructions"].append({"instruction": {"op": "array_residence_release", "value": handle}})


def duplicate_result(block):
    ins = copy.deepcopy(next(r["instruction"] for r in block["instructions"] if r["instruction"]["op"] == "invoke_normal_result"))
    ins["dst"] = 100000
    block["instructions"].append({"instruction": ins})


mutate(lambda d: rewrite_block(d, lambda b: b["terminator"]["instruction"].get("operation", {}).get("kind") == "array_claim", skip_claim))
mutate(lambda d: rewrite_block(d, lambda b: b["terminator"]["instruction"].get("operation", {}).get("kind") == "array_append", release_before_append))
mutate(lambda d: rewrite_block(d, lambda b: any(r["instruction"]["op"] == "invoke_normal_result" for r in b["instructions"]), duplicate_result))
for data in mutations:
    compile_input(data, False)

for bits in [0, 2**63, 0x7ff8000000000042, 0xfff8000000000123]:
    data = copy.deepcopy(inputs[29])
    next(row for row in instructions(data) if row["op"] == "const_f64_bits")["bits"] = bits
    compile_input(data, True)
for bits in [-1, 2**64, "0", 1.25]:
    data = copy.deepcopy(inputs[29])
    next(row for row in instructions(data) if row["op"] == "const_f64_bits")["bits"] = bits
    compile_input(data, False)
print("native C: 39 production host objects; 8 returned-allocation Fault probes; ordered runtime observations, malformed wire/cleanup rejection, Float bit inputs")
