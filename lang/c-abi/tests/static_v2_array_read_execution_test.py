#!/usr/bin/env python3
"""Lane-A published ArrayGet row -> both walkers -> slot_load_hi -> kernel.

Physical consumer evidence only: the MIR document and call rows are fixture
bytes, not a source/publication acceptance proof. Each `array_element_read`
must carry one exact-coordinate kind-9 row; a missing or mismatched row is a
hard stop, never a silent generic fallback.
"""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
TESTS = ROOT / 'lang/c-abi/tests'
DRIVER, KERNEL = sys.argv[1:3]
ENV = dict(os.environ, HAKO_BACKEND_COMPILE_RECIPE="pure-first",
           HAKO_NYRT_PLUGIN_HOST='off')
ENV.pop('V4_PROBE_FAULT_AT', None)

INDEX = 2   # HAKO_LLVMC_PUBLISHED_ROW_FLAG_INDEX_PRESENT
DST = 1     # HAKO_LLVMC_PUBLISHED_ROW_FLAG_DST_PRESENT
ARRAY_SET = 6
ARRAY_GET = 9
FREE_FUNCTION = 3


def const(dst, value):
    return dict(op="const", dst=dst, value=dict(type="i64", value=value))


def read_fn(name, read_index_value):
    # newbox ArrayBox; set(0, 7); get(<read_index_value>) -> ret
    return dict(name=name, params=[], metadata={}, blocks=[dict(id=0, instructions=[
        dict(op="newbox", dst=1, type="ArrayBox", args=[]),
        const(2, 0),
        const(3, 7),
        dict(op="array_element_write", kind="set", site_id=5,
             receiver=1, index=2, value=3),
        const(5, read_index_value),
        dict(op="array_element_read", site_id=6, dst=4, receiver=1, index=5),
        dict(op="ret", value=4)])])


def rows(owner, read_index_value, read_slot=5):
    return [
        dict(function=owner, block=0, instruction=3, kind=ARRAY_SET,
             site_id=5, receiver=1, index=2, value=3, flags=INDEX),
        dict(function=owner, block=0, instruction=read_slot, kind=ARRAY_GET,
             site_id=6, receiver=1, index=read_slot, dst=4, flags=INDEX | DST),
    ]


def entry_body(read_index_value):
    return dict(functions=[read_fn("main", read_index_value)])


def nested_body(read_index_value):
    # The same-module walker shares the row take/emitter, but its prepass
    # does not admit array_element_* ops yet — same boundary that already
    # gates array_element_write. Kept here as the honest parked edge: the
    # read consumer is wired, upstream admission is a separate decision.
    main = dict(name="main", params=[], metadata=dict(
        same_module_function_definitions=[
            dict(target_symbol="nested", definition_kind="same_module_function")]),
        blocks=[dict(id=0, instructions=[
            const(1, 0),
            dict(op="mir_call", dst=2, mir_call=dict(
                callee=dict(type="Global", name="unused_json_name"), args=[])),
            dict(op="ret", value=2)])])
    call_row = dict(function="main", block=0, instruction=1,
                    kind=FREE_FUNCTION, arity=0, target="nested")
    return (dict(functions=[main, read_fn("nested", read_index_value)]),
            [call_row, *rows("nested", read_index_value)])


with tempfile.TemporaryDirectory(prefix="hakorune-static-v2-array-read-") as directory:
    work = Path(directory)

    def compile_case(label, body, frame, expected_error=None):
        source, calls, obj, ir = (work / (label + suffix)
                                  for suffix in (".json", ".frame.json", ".o", ".ll"))
        source.write_text(json.dumps(body))
        calls.write_text(json.dumps(dict(calls=frame, maps=[], values=[], expanded=[])))
        if expected_error:
            obj.write_bytes(b"previous-artifact")
        result = subprocess.run([DRIVER, str(source), str(calls), str(obj)],
                                text=True, capture_output=True,
                                env=dict(ENV, NYASH_LLVM_DUMP_IR=str(ir)))
        assert result.returncode == (1 if expected_error else 0), (
            label, result.stdout, result.stderr)
        if expected_error:
            assert expected_error in result.stderr, (label, result.stderr)
            assert obj.read_bytes() == b"previous-artifact", label
        else:
            assert obj.exists(), label
        print(label, "ok")
        return obj, ir

    def run_case(label, body, frame, expected):
        obj, ir = compile_case(label, body, frame)
        text = ir.read_text()
        assert 'call i64 @nyash.array.slot_load_hi' in text, (label, text)
        exe = obj.with_suffix(".exe")
        link = subprocess.run(
            ["cc", str(obj), KERNEL, "-lpthread", "-ldl", "-lm", "-o", str(exe)],
            text=True, capture_output=True)
        assert link.returncode == 0, (label, link.stderr)
        run = subprocess.run([str(exe)], text=True, capture_output=True, env=ENV)
        assert run.returncode == expected, (label, run.returncode, run.stdout, run.stderr)
        print(label, "executes through slot_load_hi ->", expected)

    for scope, make in (("entry", lambda i: (entry_body(i), rows("main", i))),):
        body, frame = make(0)
        run_case(scope + "-stored", body, frame, 7)
        body, frame = make(5)
        run_case(scope + "-oob", body, frame, 0)

    # Same-module admission boundary: the prepass whitelist predates the
    # row consumer — the read must fail there for the same reason a write
    # already does, not silently succeed through one walker only.
    body, frame = nested_body(0)
    compile_case("nested-prepass-boundary", body, frame,
                 "module_generic_prepass_failed")

    body, frame = make_entry = entry_body(0), rows("main", 0)
    compile_case("missing-read-row", body, frame[:1],
                 "published_array_read_row_mismatch")

    bad_kind = [dict(r) for r in frame]
    bad_kind[1]["kind"] = ARRAY_SET
    bad_kind[1]["value"] = 9
    compile_case("wrong-kind-read-row", body, bad_kind,
                 "published_array_read_row_mismatch")

    bad_flags = [dict(r) for r in frame]
    bad_flags[1]["flags"] = INDEX
    bad_flags[1]["dst"] = 0
    compile_case("dst-less-read-row", body, bad_flags,
                 "malformed typed row")

    bad_shape = json.loads(json.dumps(body))
    del bad_shape["functions"][0]["blocks"][0]["instructions"][5]["index"]
    compile_case("missing-index-field", bad_shape, frame,
                 "published_array_read_row_mismatch")

    print("Lane-A array read: stored/oob execute; malformed rows fail closed; "
          "same-module prepass boundary pinned")
