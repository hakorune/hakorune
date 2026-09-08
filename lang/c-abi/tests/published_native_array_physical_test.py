#!/usr/bin/env python3
"""Consume Rust-captured Script inputs; malformed mutations are physical evidence only."""
import copy
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[3]
work = Path(sys.argv[1])
inputs = [json.loads(path.read_text()) for path in sorted(work.glob("*.json"), key=lambda p: int(p.stem))]
assert len(inputs) == 39


def checked(args):
    result = subprocess.run([str(x) for x in args], text=True, capture_output=True)
    assert result.returncode == 0, (args, result.stdout, result.stderr)
    return result


driver = work / "driver"
checked(["cc", ROOT / "lang/c-abi/tests/published_lifecycle_v4_driver.c",
         "-L" + str(ROOT / "target/release"), "-lhako_llvmc_ffi",
         "-Wl,-rpath," + str(ROOT / "target/release"), "-o", driver])
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


def instructions(data):
    for block in data["functions"][0]["blocks"]:
        for row in block["instructions"]:
            yield row["instruction"]
        yield block["terminator"]["instruction"]


def first(data, kind):
    return next(row for row in instructions(data) if row.get("operation", {}).get("kind") == kind)


archive = ROOT / "target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a"
assert archive.is_file()
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
