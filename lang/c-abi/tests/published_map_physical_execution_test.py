#!/usr/bin/env python3
"""Physical C Map consumer proof. Synthetic CFG, not source cutover evidence."""
import copy
import json
import os
import signal
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
TESTS = ROOT / "lang/c-abi/tests"
ARCHIVE = Path(sys.argv[1]).resolve()


def run(args, **kwargs):
    return subprocess.run([str(a) for a in args], capture_output=True, text=True, **kwargs)


def checked(args, **kwargs):
    result = run(args, **kwargs)
    assert result.returncode == 0, (args, result.stdout, result.stderr)
    return result


class Graph:
    def __init__(self):
        self.blocks = []
        self.value = 2
        self.site = 0
        self.current = self.block()
        self.row(self.current, dict(op="fault_frame_enter", dst=0, mode="root_owned"))
        self.row(self.current, dict(op="const_i64", dst=1, value=30))
        self.failure = self.block()
        self.term(self.failure, dict(op="return_fault", fault_frame=0))

    def block(self):
        result = dict(id=len(self.blocks), instructions=[], edges=[])
        self.blocks.append(result)
        return result

    def row(self, block, instruction):
        block["instructions"].append(dict(index=len(block["instructions"]), instruction=instruction))

    def term(self, block, instruction):
        block["terminator"] = dict(index=len(block["instructions"]), instruction=instruction)
        if instruction["op"] == "invoke":
            block["edges"] = [dict(target=instruction[k], args=None) for k in ("normal", "fault")]

    def invoke_term(self, block, op, normal, fault):
        op = dict(op, site=self.site)
        self.site += 1
        self.term(block, dict(op="invoke", operation=op, fault_frame=0, normal=normal, fault=fault))

    def cleanup(self, ops):
        tail = self.failure["id"]
        for op in reversed(ops):
            block = self.block()
            self.invoke_term(block, op, tail, tail)
            tail = block["id"]
        return tail

    def invoke(self, op, cleanup, result=False):
        old = self.current
        normal = self.block()
        self.invoke_term(old, op, normal["id"], self.cleanup(cleanup))
        self.current = normal
        if result:
            value = self.value
            self.value += 1
            self.row(normal, dict(op="invoke_normal_result", invoke_block=old["id"], dst=value))
            return value


def program(keys):
    graph = Graph()
    outer = []
    for i in range(len(keys)):
        value = graph.invoke(dict(kind="new_box", object_id=i), list(reversed(outer)), True)
        outer.append(dict(kind="home_release", object_id=i, value=value))
    values = list(outer)
    map_value = graph.invoke(dict(kind="map_new"), list(reversed(outer)), True)
    end = dict(kind="map_end", map=map_value)
    for i, text in enumerate(keys):
        failure = [end] + list(reversed(outer))
        key = graph.invoke(dict(kind="map_prepare_key", utf8=text), failure, True)
        outcome = graph.invoke(dict(kind="map_install_indexed", map=map_value, key=key,
                                    object_id=i, value=values[i]["value"]), failure, True)
        outer.pop(0)
        graph.invoke(dict(kind="map_end_outcome", outcome=outcome), [end] + list(reversed(outer)))
    graph.invoke(end, [])
    graph.term(graph.current, dict(op="return", value=1))
    return dict(schema="hako.published-lifecycle-physical-program.v2", storage_profile=1,
                fault_abi_version=1, process_result_site=graph.site,
        layouts=[dict(object_id=i, runtime_type_id=900+i, field_count=0, fields=[])
                 for i in range(len(keys))],
        functions=[dict(name="main", role="root_i64", entry=0, params=[], receiver=None,
                                receiver_object=None, blocks=graph.blocks)])


def value_program(kinds):
    """Explicit physical scalars; shared key exercises value/Home replacement."""
    graph = Graph()
    map_value = graph.invoke(dict(kind="map_new"), [], True)
    end = dict(kind="map_end", map=map_value)
    scalars = {}
    for kind, payload in kinds:
        if kind == "home":
            value = graph.invoke(dict(kind="new_box", object_id=0), [end], True)
            failure = [dict(kind="home_release", object_id=0, value=value), end]
        else:
            if (kind, payload) not in scalars:
                value = graph.value
                graph.value += 1
                graph.row(graph.current, dict(op="const_" + kind, dst=value, value=payload))
                for _ in range(2):
                    alias = graph.value
                    graph.value += 1
                    graph.row(graph.current, dict(op="copy", dst=alias, src=value))
                    value = alias
                scalars[kind, payload] = value
            value = scalars[kind, payload]
            failure = [end]
        key = graph.invoke(dict(kind="map_prepare_key", utf8="same"), failure, True)
        operation = dict(kind="map_install_value", map=map_value, key=key,
                         value=value, value_kind=1 if kind == "i64" else 2)
        if kind == "home":
            operation.pop("value_kind")
            operation.update(kind="map_install_indexed", object_id=0)
        outcome = graph.invoke(operation, failure, True)
        graph.invoke(dict(kind="map_end_outcome", outcome=outcome), [end])
    graph.invoke(end, [])
    graph.term(graph.current, dict(op="return", value=1))
    data = program(["unused"])
    data["functions"][0]["blocks"] = graph.blocks
    data["process_result_site"] = graph.site
    return data


def mixed_program():
    """Two live Maps, with later ordinary allocations under their cleanup."""
    graph = Graph()
    ends = []
    for object_id in range(2):
        map_value = graph.invoke(dict(kind="map_new"), list(reversed(ends)), True)
        ends.append(dict(kind="map_end", map=map_value))
        cleanup = list(reversed(ends))
        value = graph.invoke(dict(kind="new_box", object_id=object_id), cleanup, True)
        release = dict(kind="home_release", object_id=object_id, value=value)
        key = graph.invoke(dict(kind="map_prepare_key", utf8="key"), [release] + cleanup, True)
        outcome = graph.invoke(dict(kind="map_install_indexed", map=map_value, key=key,
                                    object_id=object_id, value=value), [release] + cleanup, True)
        graph.invoke(dict(kind="map_end_outcome", outcome=outcome), cleanup)
    while ends:
        graph.invoke(ends.pop(), list(reversed(ends)))
    graph.term(graph.current, dict(op="return", value=1))
    data = program(["a", "b"])
    data["functions"][0]["blocks"] = graph.blocks
    data["process_result_site"] = graph.site
    return data


with tempfile.TemporaryDirectory(prefix="hako map physical ") as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ("driver", "map.o", "map")]
    checked(["cc", "-DHAKO_TEST_RUNTIME_DESCRIPTOR", TESTS / "published_lifecycle_v4_driver.c",
             "-L" + str(ROOT / "target/release"), "-lhako_llvmc_ffi", ARCHIVE,
             "-Wl,-rpath," + str(ROOT / "target/release"), "-ldl", "-lpthread", "-lm", "-o", driver])
    main = work / "main.c"
    main.write_text("extern long ny_main(void); int main(void) { return (int)ny_main(); }\n")
    env = dict(os.environ, NYASH_NYRT_SILENT_RESULT="1", HAKO_NYRT_PLUGIN_HOST="off")

    def compile_input(data, success=True):
        path = work / "input.json"
        path.write_text(json.dumps(data))
        before = obj.read_bytes() if obj.exists() else None
        result = run([driver, path, obj], env=env)
        assert (result.returncode == 0) == success, (result.stdout, result.stderr, data)
        assert not list(work.glob("map.o.*")), "temporary artifact leak"
        if not success:
            assert obj.read_bytes() == before, "rejected input changed object"
        return result

    def assert_named_reject(data, marker):
        """A negative is evidence only when the intended owner names it."""
        result = compile_input(data, False)
        assert marker in result.stderr, (marker, result.stderr)
        return result

    for keys in [[], ["a"], ["a", "b"], ["a", "a"], ["a\x00b", "a\x00b"], ["", "01"]]:
        data = program(keys)
        compile_input(data)
        checked(["cc", main, obj, ARCHIVE, "-ldl", "-lpthread", "-lm", "-o", exe])
        result = run([exe], env=env)
        assert result.returncode == 30, (keys, result.returncode, result.stderr)
    print("six Map physical programs -> actual descriptor/OBJ/linked EXE30")

    # Block IDs and document ordinals are distinct coordinates. Exercise the
    # index with the entry and result-origin blocks out of numeric order.
    reordered = program(["a", "b"])
    reordered["functions"][0]["blocks"].reverse()
    compile_input(reordered)
    checked(["cc", main, obj, ARCHIVE, "-ldl", "-lpthread", "-lm", "-o", exe])
    assert run([exe], env=env).returncode == 30
    print("reversed block document order -> linked EXE30")
    # MapEnd expands bookkeeping before its normal edge. Its scalar PHI must
    # consume the shared physical-edge label, not the original block label.
    phi_return = program(["a"])
    blocks = phi_return["functions"][0]["blocks"]
    terminal = next(b for b in blocks if b["terminator"]["instruction"]["op"] == "return")
    predecessors = [b["id"] for b in blocks
                    if any(e["target"] == terminal["id"] for e in b["edges"])]
    value_id = max(r["instruction"].get("dst", 0) for b in blocks for r in b["instructions"]) + 1
    terminal["instructions"].insert(0, dict(index=0, instruction=dict(op="phi", dst=value_id,
        inputs=[dict(block=pred, value=1) for pred in predecessors])))
    for index, row in enumerate(terminal["instructions"]):
        row["index"] = index
    terminal["terminator"] = dict(index=len(terminal["instructions"]),
                                  instruction=dict(op="return", value=value_id))
    compile_input(phi_return)
    checked(["cc", main, obj, ARCHIVE, "-ldl", "-lpthread", "-lm", "-o", exe])
    assert run([exe], env=env).returncode == 30
    print("Map bookkeeping -> shared edge label -> scalar PHI -> EXE30")
    compile_input(mixed_program())
    checked(["cc", main, obj, ARCHIVE, "-ldl", "-lpthread", "-lm", "-o", exe])
    assert run([exe], env=env).returncode == 30
    print("two live Maps with later ordinary allocations -> linked EXE30")

    compile_input(program(["same", "same"]))
    wraps = ["storage_init", "storage_dispose", "key_init", "key_dispose",
             "outcome_init", "outcome_dispose", "checked_new", "key_prepare_utf8",
             "checked_install_indexed", "outcome_end", "checked_end"]
    checked(["cc", TESTS / "published_map_fault_probe.c", obj, ARCHIVE,
             *["-Wl,--wrap=nyash.map." + name + "_v1" for name in wraps],
             "-ldl", "-lpthread", "-lm", "-o", exe])
    for mode, keys, outcomes in [("normal", 2, 2), ("new-fault", 0, 0),
                                 ("prepare-fault", 1, 0), ("install-fault", 1, 1),
                                 ("outcome-fault", 1, 1), ("end-fault", 2, 2)]:
        result = run([exe, mode], env=env)
        expected = 30 if mode == "normal" else 70
        assert result.returncode == expected, (mode, result.returncode, result.stderr)
        assert result.stdout.strip() == f"{expected} 1 1 {keys} {keys} {outcomes} {outcomes}", result.stdout
    for mode in ["invalid", "unknown"]:
        result = run([exe, mode], env=env)
        assert result.returncode == -signal.SIGILL, (mode, result.returncode, result.stderr)
    print("six completion paths dispose exactly once; InvalidContract/unknown trap")

    for entries in [[("i64", -(2**63)), ("i64", 2**63-1)],
                    [("bool", False), ("bool", True), ("bool", True)],
                    [("i64", 30), ("home", None), ("bool", True), ("home", None)]]:
        compile_input(value_program(entries))
        checked(["cc", main, obj, ARCHIVE, "-ldl", "-lpthread", "-lm", "-o", exe])
        assert run([exe], env=env).returncode == 30
    print("I64/Bool and alternating Value/Home replacements -> linked EXE30")

    compile_input(value_program([("bool", True), ("i64", 30)]))
    checked(["cc", "-DHAKO_MAP_VALUE_PROBE", TESTS / "published_map_fault_probe.c", obj, ARCHIVE,
             *["-Wl,--wrap=nyash.map." + name + "_v1" for name in wraps + ["checked_install_value"]],
             "-ldl", "-lpthread", "-lm", "-o", exe])
    for mode, keys, outcomes in [("normal", 2, 2), ("new-fault", 0, 0),
                                ("prepare-fault", 1, 0), ("install-fault", 1, 1),
                                ("outcome-fault", 1, 1), ("end-fault", 2, 2)]:
        result = run([exe, mode], env=env)
        expected = 30 if mode == "normal" else 70
        assert result.returncode == expected, (mode, result.stderr)
        assert result.stdout.strip() == f"{expected} 1 1 {keys} {keys} {outcomes} {outcomes}", result.stdout
    print("Value status paths preserve Key/Outcome disposal")

    base = program(["a", "b"])

    # Reuse the valid normal/fault-cleanup graph and mutate one parser-owned
    # field. The parser's named rejection proves this is not a generic V4
    # unsupported-cohort negative.
    abi_layout = copy.deepcopy(base)
    abi_layout["layouts"][0]["fields"] = [{
        "declaration_ordinal": 0, "runtime_slot": 0, "storage_kind": 99
    }]
    assert_named_reject(
        abi_layout,
        "[freeze:contract][published-lifecycle-physical-parser/abi-layout]",
    )

    def operation(data, kind):
        return next(b["terminator"]["instruction"]["operation"]
                    for b in data["functions"][0]["blocks"]
                    if b["terminator"]["instruction"].get("operation", {}).get("kind") == kind)

    malformed = []
    for field, bad in [("value_kind", 0), ("value_kind", 3), ("value_kind", 2),
                       ("value", 999999), ("object_id", 0)]:
        data = value_program([("i64", 30)])
        operation(data, "map_install_value")[field] = bad
        malformed.append(data)
    for operand in ["map", "key"]:
        data = value_program([("i64", 30)])
        op = operation(data, "map_install_value")
        op["value"] = op[operand]
        malformed.append(data)

    for key in ["value_kind", "value", "site"]:
        data = value_program([("i64", 30)])
        del operation(data, "map_install_value")[key]
        malformed.append(data)
    data = value_program([("i64", 30)])
    for block in data["functions"][0]["blocks"]:
        for row in block["instructions"]:
            ins = row["instruction"]
            if ins.get("op") == "const_i64" and ins["dst"] != 1:
                ins.update(op="const_f64_bits", bits=0)
                del ins["value"]
    malformed.append(data)
    data = value_program([("home", None)])
    op = operation(data, "map_install_indexed")
    op.update(kind="map_install_value", value_kind=1)
    del op["object_id"]
    malformed.append(data)
    data = value_program([("bool", True)])
    data["storage_profile"] = 2
    malformed.append(data)
    for key, value in [("map", 0), ("key", 0), ("value", 0), ("object_id", 999), ("site", 0)]:
        data = copy.deepcopy(base)
        operation(data, "map_install_indexed")[key] = value
        malformed.append(data)
    for kind in ["map_new", "map_prepare_key", "map_install_indexed", "map_end", "map_end_outcome"]:
        data = copy.deepcopy(base)
        del operation(data, kind)["site"]
        malformed.append(data)
    data = copy.deepcopy(base)
    data["storage_profile"] = 2
    malformed.append(data)

    # Keep operand availability intact while removing the final lifetime end.
    data = copy.deepcopy(base)
    blocks = data["functions"][0]["blocks"]
    finish = next(b for b in blocks if b["terminator"]["instruction"].get("operation", {}).get("kind")
                  == "map_end" and any(t["id"] == b["terminator"]["instruction"]["normal"]
                  and t["terminator"]["instruction"]["op"] == "return" for t in blocks))
    target = finish["terminator"]["instruction"]["normal"]
    finish["terminator"]["instruction"] = dict(op="jump", target=target, args=None)
    finish["edges"] = [dict(target=target, args=None)]
    malformed.append(data)

    # A second end uses a dominating, correctly typed Map value, but its
    # storage has already been consumed on the preceding Normal edge.
    data = copy.deepcopy(base)
    blocks = data["functions"][0]["blocks"]
    terminal = next(b for b in blocks if b["terminator"]["instruction"]["op"] == "return")
    new_id = max(b["id"] for b in blocks) + 1
    blocks.append(dict(id=new_id, instructions=[], edges=[],
                       terminator=dict(index=0, instruction=dict(op="return", value=1))))
    end = dict(operation(data, "map_end"), site=data["process_result_site"])
    data["process_result_site"] += 1
    terminal["terminator"]["instruction"] = dict(op="invoke", operation=end, fault_frame=0,
                                                 normal=new_id, fault=1)
    terminal["edges"] = [dict(target=new_id, args=None), dict(target=1, args=None)]
    malformed.append(data)
    for index, data in enumerate(malformed):
        result = compile_input(data, False)
        if index >= len(malformed) - 2:
            assert "unsupported-cohort" in result.stderr, result.stderr
    print(len(malformed), "malformed physical inputs preserve object")
