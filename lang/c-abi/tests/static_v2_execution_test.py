#!/usr/bin/env python3
"""Private V2 -> both real walkers -> object -> kernel; no source cutover claim."""
import copy
import json
import itertools
import os
from pathlib import Path
import resource
import signal
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
DRIVER, KERNEL = sys.argv[1:3]

def const(dst, value, kind="i64"):
    return dict(op="const", dst=dst, value=dict(type=kind, value=value))

def witness(nested=False, kind=1, bits=30, key="k", writes=1):
    owner = "nested" if nested else "main"
    value = const(3, None if kind == 3 else bits, "f64" if kind == 3 else "void" if kind == 4 else "i64")
    instructions = [dict(op="newbox", dst=1, target=dict(kind="intrinsic_map"), args=[]),
                    const(2, key, dict(kind="handle", box_type="StringBox")), value]
    instructions += [dict(op="map_literal_entry_write", receiver=1, key=2, value=3) for _ in range(writes)]
    instructions += [const(4, 30), dict(op="ret", value=4)]
    function = dict(name=owner, params=[], metadata={}, blocks=[dict(id=0, instructions=instructions)])
    frame = dict(maps=[dict(function=owner, block=0, instruction=i, kind=1 if i == 0 else 2)
                       for i in [0, *range(3, 3 + writes)]],
                 values=[dict(function=owner, value=3, action=1, kind=kind, payload=bits)])
    body = dict(functions=[function])
    if nested:
        body["functions"].insert(0, dict(name="main", params=[], metadata=dict(
            same_module_function_definitions=[dict(target_symbol=owner, definition_kind="same_module_function")]),
            blocks=[dict(id=0, instructions=[const(1, 0),
                dict(op="mir_call", dst=2, mir_call=dict(callee=dict(type="Global", name="unused_json_name"), args=[])),
                dict(op="ret", value=2)])]))
        frame["calls"] = [dict(function="main", block=0, instruction=1, kind=3, arity=0, target=owner)]
    return body, frame

ENV = dict(os.environ, ASAN_OPTIONS="detect_leaks=0", HAKO_BACKEND_COMPILE_RECIPE="pure-first")

def no_core():
    resource.setrlimit(resource.RLIMIT_CORE, (0, 0))

with tempfile.TemporaryDirectory(prefix="hakorune-static-v2-") as directory:
    work = Path(directory)
    def compile_case(label, body, frame, expected_error=None, settings=None):
        source, rows, obj, ir = (work / (label + suffix) for suffix in (".json", ".frame.json", ".o", ".ll"))
        source.write_text(json.dumps(body)); rows.write_text(json.dumps(frame))
        if expected_error:
            obj.write_bytes(b"previous-artifact")
        result = subprocess.run([DRIVER, str(source), str(rows), str(obj)],
            text=True, capture_output=True, env=dict(ENV, **(settings or {}), NYASH_LLVM_DUMP_IR=str(ir)))
        assert result.returncode == (1 if expected_error else 0), (label, result.stdout, result.stderr)
        if expected_error:
            assert expected_error in result.stderr, (label, result.stderr)
            assert obj.read_bytes() == b"previous-artifact", label
        else:
            assert obj.exists(), label
        assert not list(work.glob(label + ".o.v2-*")), label
        print(label, "ok")
        return obj, ir

    if sys.argv[3:] == ["--original-only"]:
        from static_v2_original_cases import run_original_cases
        run_original_cases(compile_case, witness, const, ROOT, KERNEL, ENV, no_core)
        sys.exit(0)

    for nested in (False, True):
        for kind, bits in ((1, 30), (2, 1), (3, 0x7ff8000000000042), (4, 0)):
            for key in ("k", "k\x00tail"):
                label = f"{'nested' if nested else 'entry'}-{kind}-{'nul' if chr(0) in key else 'text'}"
                body, frame = witness(nested, kind, bits, key, writes=2)
                obj, ir = compile_case(label, body, frame)
                executable = obj.with_suffix(".exe")
                link = subprocess.run(["cc", "-no-pie", str(obj),
                    str(ROOT / "lang/c-abi/tests/static_v2_runtime_probe.c"), KERNEL,
                    "-Wl,--wrap=nyash.map.literal_store_v1", "-Wl,--wrap=nyash.box.from_i8_string_const_len_v1", "-ldl", "-lpthread", "-lm", "-o", str(executable)],
                    text=True, capture_output=True)
                assert link.returncode == 0, link.stderr
                env = dict(ENV, EXPECT_KIND=str(kind), EXPECT_BITS=str(bits), EXPECT_WRITES="2", EXPECT_KEY_HEX=key.encode().hex())
                run = subprocess.run([str(executable)], text=True, capture_output=True, env=env, preexec_fn=no_core)
                assert run.returncode == 0 and "kernel-readback-ok" in run.stdout, (label, run)
                if kind == 1 and key == "k":
                    for status in (1, 2, 7):
                        run = subprocess.run([str(executable)], text=True, capture_output=True,
                            env=dict(env, FORCE_STATUS=str(status)), preexec_fn=no_core)
                        assert run.returncode == -signal.SIGILL and "after-entry" not in run.stderr, (label, status, run)

                    run = subprocess.run([str(executable)], text=True, capture_output=True,
                        env=dict(env, FORCE_STRING_ZERO="1"), preexec_fn=no_core)
                    assert run.returncode == -signal.SIGILL and "after-entry" not in run.stderr, (label, run)

    for nested in (False, True):
        for key, transform in (("", "direct"), ("猫", "direct"), ("k\0", "copy"), ("k\0tail", "concat")):
            body, frame = witness(nested, key=key)
            function = body["functions"][-1]
            instructions = function["blocks"][0]["instructions"]
            if transform == "copy":
                instructions.insert(3, dict(op="copy", src=2, dst=5))
                instructions[4]["key"] = 5
            if transform == "concat":
                instructions[1]["value"]["value"] = "k\0"
                instructions[3:3] = [const(6, "tail", dict(kind="handle", box_type="StringBox")),
                                       dict(op="binop", operation="+", lhs=2, rhs=6, dst=5)]
                instructions[5]["key"] = 5
            frame["maps"] = [dict(function=function["name"], block=0, instruction=i,
                kind=1 if ins["op"] == "newbox" else 2) for i, ins in enumerate(instructions)
                if ins["op"] in ("newbox", "map_literal_entry_write")]
            label = f"string-{nested}-{transform}-{key.encode().hex()}"
            obj, ir = compile_case(label, body, frame)
            executable = obj.with_suffix(".exe")
            link = subprocess.run(["cc", "-no-pie", str(obj), str(ROOT / "lang/c-abi/tests/static_v2_runtime_probe.c"), KERNEL,
                "-Wl,--wrap=nyash.map.literal_store_v1", "-Wl,--wrap=nyash.box.from_i8_string_const_len_v1", "-ldl", "-lpthread", "-lm", "-o", str(executable)], capture_output=True)
            assert link.returncode == 0, link.stderr
            run = subprocess.run([str(executable)], text=True, capture_output=True, preexec_fn=no_core,
                env=dict(ENV, EXPECT_KIND="1", EXPECT_BITS="30", EXPECT_WRITES="1", EXPECT_KEY_HEX=key.encode().hex()))
            assert run.returncode == 0, (label, run)

    # Backedge planning shares command order with the existing checked field store.
    for nested, field_order in itertools.product((False, True), (None, "before", "after")):
        body, frame = witness(nested)
        function = body["functions"][-1]
        prefix = function["blocks"][0]["instructions"][:3]
        function["blocks"] = [
            dict(id=0, instructions=prefix + [const(4, 30), const(10, 0), const(11, 1), const(12, 2), dict(op="jump", target=1)]),
            dict(id=1, instructions=[dict(op="phi", dst=20, incoming=[[10, 0], [21, 2]]),
                 dict(op="compare", dst=22, operation="<", lhs=20, rhs=12), dict(op="branch", cond=22, then=2, **{"else": 3})]),
            dict(id=2, instructions=[dict(op="map_literal_entry_write", receiver=1, key=2, value=3),
                 dict(op="map_literal_entry_write", receiver=1, key=2, value=3),
                 dict(op="binop", dst=21, operation="+", lhs=20, rhs=11), dict(op="jump", target=1)]),
            dict(id=3, instructions=[dict(op="ret", value=4)])]
        if field_order:
            body["typed_object_plans"] = [dict(box_name="User", type_id=11, field_count=1,
                fields=[dict(name="value", storage="i8", slot=0)])]
            function["blocks"][0]["instructions"].insert(-1, dict(op="newbox", dst=30, type="User", args=[]))
            function["blocks"][2]["instructions"].insert(0 if field_order == "before" else 2,
                dict(op="field_set", box=30, field="value", value=4))
        frame["maps"] = [frame["maps"][0]] + [dict(function=function["name"], block=2, instruction=i, kind=2)
            for i, ins in enumerate(function["blocks"][2]["instructions"]) if ins["op"] == "map_literal_entry_write"]
        obj, ir = compile_case(f"backedge-{nested}-{field_order}", body, frame)
        assert f"exact_status_continue_2_{2 if field_order else 1}" in ir.read_text()
        executable = obj.with_suffix(".exe")
        link = subprocess.run(["cc", "-no-pie", str(obj), str(ROOT / "lang/c-abi/tests/static_v2_runtime_probe.c"), KERNEL,
            "-Wl,--wrap=nyash.map.literal_store_v1", "-Wl,--wrap=nyash.box.from_i8_string_const_len_v1", "-ldl", "-lpthread", "-lm", "-o", str(executable)], capture_output=True)
        assert link.returncode == 0, link.stderr
        run = subprocess.run([str(executable)], text=True, capture_output=True, preexec_fn=no_core,
            env=dict(ENV, EXPECT_KIND="1", EXPECT_BITS="30", EXPECT_WRITES="4", EXPECT_KEY_HEX="6b"))
        assert run.returncode == 0, run

    # Each selected operation executes in both walkers. Operands carry the
    # planner's complete rows; Bool operation payloads are widened at producers.
    for nested in (False, True):
        cases = []
        def leaf(value, kind=1):
            return dict(action=1, kind=kind, payload=value, flags=1)
        def original():
            return dict(action=2, kind=5, encoding=1, flags=1)
        def operation(number):
            return dict(action=7, operation=number, flags=1,
                kind=1 if number == 1 else 5 if number == 5 else 2,
                encoding=1 if number in (1, 5) else 2)
        for symbol, left, right, result in (("+", 12, 18, 30), ("-", 42, 12, 30),
                ("*", 5, 6, 30), ("/", 91, 3, 30), ("%", 93, 31, 0)):
            cases.append((f"binary-{symbol}", [const(5, left), const(6, right),
                dict(op="binop", dst=3, lhs=5, rhs=6, operation=symbol)],
                {5: leaf(left), 6: leaf(right), 3: operation(1)}, 1, result, None))
        for symbol, result in (("<", 1), ("<=", 1), ("==", 0), ("!=", 1), (">=", 0), (">", 0)):
            cases.append((f"integer-compare-{symbol}", [const(5, 12), const(6, 30),
                dict(op="compare", dst=3, lhs=5, rhs=6, operation=symbol)],
                {5: leaf(12), 6: leaf(30), 3: operation(2)}, 2, result, None))
        cases.append(("mixed-bool-width", [const(5, 0), const(6, 1),
            dict(op="unop", dst=7, src=5, operation="not"),
            dict(op="compare", dst=3, lhs=7, rhs=6, operation="==")],
            {5: leaf(0, 2), 6: leaf(1, 2), 7: operation(7), 3: operation(3)}, 2, 1, None))
        cases.append(("integer-not", [const(5, 42), dict(op="unop", dst=3, src=5, operation="Not")],
            {5: leaf(42), 3: operation(6)}, 2, 0, None))
        string_type = dict(kind="handle", box_type="StringBox")
        for symbol, result in (("<", 1), ("==", 0), (">", 0)):
            cases.append((f"string-compare-{symbol}", [const(5, "cat", string_type), const(6, "dog", string_type),
                dict(op="compare", dst=3, lhs=5, rhs=6, operation=symbol)],
                {5: original(), 6: original(), 3: operation(4)}, 2, result, None))
        cases.append(("string-value", [const(3, "cat", string_type)], {3: original()}, 5, 0, "StringBox"))
        cases.append(("string-concat", [const(5, "cat", string_type), const(6, "dog", string_type),
            dict(op="binop", dst=3, lhs=5, rhs=6, operation="+")],
            {5: original(), 6: original(), 3: operation(5)}, 5, 0, "StringBox"))
        cases.append(("map-value", [dict(op="newbox", dst=3, target=dict(kind="intrinsic_map"), args=[])],
            {3: original()}, 5, 0, "MapBox"))
        for number, (name, instructions, rows, kind, bits, handle_type) in enumerate(cases):
            body, frame = witness(nested)
            function = body["functions"][-1]
            body_instructions = function["blocks"][0]["instructions"]
            body_instructions[2:3] = instructions
            frame["values"] = [dict(function=function["name"], value=value, **row) for value, row in rows.items()]
            frame["maps"] = [dict(function=function["name"], block=0, instruction=i, kind=1 if ins["op"] == "newbox" else 2)
                for i, ins in enumerate(body_instructions) if ins["op"] in ("newbox", "map_literal_entry_write")]
            obj, ir = compile_case(f"operation-{nested}-{number}-{name.replace('/', 'div')}", body, frame)
            if "integer-compare" in name:
                assert "dyn_lhs_is_str" not in ir.read_text()
            if "bool" in name or "compare" in name or "not" in name:
                assert "%map_payload_3 = zext i1 %r3 to i64" in ir.read_text()
            executable = obj.with_suffix(".exe")
            link = subprocess.run(["cc", "-no-pie", str(obj), str(ROOT / "lang/c-abi/tests/static_v2_runtime_probe.c"), KERNEL,
                "-Wl,--wrap=nyash.map.literal_store_v1", "-Wl,--wrap=nyash.box.from_i8_string_const_len_v1", "-ldl", "-lpthread", "-lm", "-o", str(executable)], capture_output=True)
            assert link.returncode == 0, link.stderr
            env = dict(ENV, EXPECT_KIND=str(kind), EXPECT_BITS=str(bits), EXPECT_WRITES="1", EXPECT_KEY_HEX="6b",
                EXPECT_HANDLE_TYPE=handle_type or "", EXPECT_TEXT="catdog" if name == "string-concat" else "cat")
            run = subprocess.run([str(executable)], text=True, capture_output=True, preexec_fn=no_core, env=env)
            assert run.returncode == 0 and "kernel-readback-ok" in run.stdout, (name, nested, run)
            if name == "mixed-bool-width":
                for mutation, change in (
                    ("missing-input", lambda f: f["values"].pop(0)),
                    ("wrong-input-kind", lambda f: f["values"][0].update(kind=1)),
                    ("missing-original", lambda f: f["values"][0].update(flags=0)),
                ):
                    bad = copy.deepcopy(frame); change(bad)
                    diagnostic = "body-coverage" if mutation == "missing-original" else "value-reference-closure"
                    compile_case(f"operation-{nested}-{mutation}", body, bad, diagnostic)
                for mutation, change in (
                    ("wrong-result-kind", lambda f: f["values"][-1].update(kind=1)),
                    ("wrong-encoding", lambda f: f["values"][-1].update(encoding=1)),
                    ("wrong-opcode", lambda f: f["values"][-1].update(operation=1)),
                ):
                    bad = copy.deepcopy(frame); change(bad)
                    compile_case(f"operation-{nested}-{mutation}", body, bad, "value-action")

    from static_v2_original_cases import run_original_cases
    run_original_cases(compile_case, witness, const, ROOT, KERNEL, ENV, no_core)

    body, frame = witness()
    body["functions"][0]["metadata"] = {"array_text_state_residence_route": {
        "observer_kind": "indexof", "residence": "loop_local_pointer_array", "result_repr": "scalar_i64",
        "consumer_capability": "direct_array_text_state_residence", "publication_boundary": "none",
        "temporary_indexof_seed_payload": {"variant": "leaf", "rows": 64, "ops": 400000, "flip_period": 0,
            "line_seed": "line-seed", "line_seed_len": 9, "none_seed": "none-seed", "none_seed_len": 9,
            "needle": "line", "needle_len": 4, "proof": "kilo_leaf_array_string_indexof_const_10block",
            "result_use": "found_predicate", "backend_action": "literal_membership_predicate",
            "candidate_outcomes": [{"literal": "line-seed", "outcome": "found"}, {"literal": "none-seed", "outcome": "not_found"}]}}}
    compile_case("early-pattern-residual", body, frame, "static-v2/map-row-not-consumed")

    body, frame = witness()
    for label, change, diagnostic in (
        ("revision", lambda b, f: f.update(revision=3), "static-v2/header"),
        ("size", lambda b, f: f.update(byte_size=0), "static-v2/header"),
        ("pair", lambda b, f: f.update(value_count=0), "static-v2/table-pair"),
        ("count", lambda b, f: f.update(value_count=2**64-1), "static-v2/table-pair"),
        ("missing-map", lambda b, f: f["maps"].pop(), "static-v2/body-coverage"),
        ("duplicate-map", lambda b, f: f["maps"].append(f["maps"][0]), "static-v2/map-site"),
        ("wrong-site", lambda b, f: f["maps"][0].update(instruction=1), "static-v2/map-site"),
        ("missing-value", lambda b, f: f.update(values=[]), "static-v2/body-coverage"),
        ("duplicate-value", lambda b, f: f["values"].append(f["values"][0]), "static-v2/value-action"),
        ("bool-payload", lambda b, f: f["values"][0].update(kind=2, payload=30), "static-v2/value-action"),
        ("unknown-action", lambda b, f: f["values"][0].update(action=42), "static-v2/value-action"),
        ("unimplemented-action", lambda b, f: f["values"][0].update(action=3), "static-v2/value-consumer-unsupported"),
        ("unknown-kind", lambda b, f: f["values"][0].update(kind=42), "static-v2/value-action"),
        ("reserved", lambda b, f: f["values"][0].update(operation=1), "static-v2/value-action"),
        ("original-use", lambda b, f: b["functions"][0]["blocks"][0]["instructions"][-1].update(value=3), "static-v2/body-coverage"),
        ("unknown-use", lambda b, f: b["functions"][0]["blocks"][0]["instructions"].insert(4, dict(op="unsupported")), "static-v2/body-coverage"),
        ("undemanded", lambda b, f: f["values"].append(dict(function="main", value=4, action=1, kind=1, payload=30, flags=1)), "static-v2/value-reference-closure"),
        ("generic-abort", lambda b, f: (f["values"][0].update(flags=1), b["functions"][0]["blocks"][0]["instructions"].insert(4, dict(op="unsupported"))), "unsupported"),
    ):
        b, f = copy.deepcopy(body), copy.deepcopy(frame)
        change(b, f)
        compile_case(label, b, f, diagnostic)
