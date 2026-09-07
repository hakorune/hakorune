#!/usr/bin/env python3
"""V4 physical input -> llc -> actual lifecycle archive; not Rust host cutover proof.

Pass JSON captured from the source-issued Pair transport test, without repair.
Range variants below test the physical ABI only, not new source acceptance.
"""
import copy
import json
import os
import shutil
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
TESTS = ROOT / "lang/c-abi/tests"
issued = json.loads(Path(sys.argv[1]).read_text())
bool_inputs = [json.loads(Path(path).read_text()) for path in sys.argv[2:]]
assert len(bool_inputs) == 2, "pass source-issued Pair JSON and both source-issued Bool JSON files"
archive = ROOT / "target/lifecycle-kernel/release/libnyash_lifecycle_kernel.a"
assert archive.is_file()
env = dict(os.environ, NYASH_NYRT_SILENT_RESULT="1", HAKO_NYRT_PLUGIN_HOST="off")
# This ambient flag must not override the selected session's explicit target.
env["NYASH_NY_LLVM_LLC_FLAGS"] = "-mtriple=i386-unknown-linux-gnu"


def run(argv, **kw):
    return subprocess.run([str(x) for x in argv], text=True, capture_output=True, **kw)


def checked(argv, **kw):
    r = run(argv, **kw)
    assert r.returncode == 0, (argv, r.stdout, r.stderr)
    return r


with tempfile.TemporaryDirectory(prefix="hako v4 execution ") as directory:
    work = Path(directory)
    driver, obj, exe = [work / name for name in ("driver", "pair.o", "pair")]
    checked(["cc", TESTS / "published_lifecycle_v4_driver.c",
             "-L" + str(ROOT / "target/release"), "-lhako_llvmc_ffi",
             "-Wl,-rpath," + str(ROOT / "target/release"), "-o", driver])
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

    # Fault-inject the LLVM call after parser admission to exercise dynamic ABI
    # rejection. The production compiler remains unchanged; this tool lives only
    # in the temporary test directory and delegates to the actual LLVM18 tool.
    real_llc = shutil.which("llc-18")
    injection = work / "inject"
    injection.mkdir()
    tool = injection / "llc-18"
    payload = call_args(bool_inputs[0])[0]["value"]
    for label, old, new in [
            ("invalid-kind", "@hako_lifecycle_birth_1(ptr %frame, i64 %v", None),
            ("invalid-bool", f"%v{payload} = add i64 0, 1", f"%v{payload} = add i64 0, 2")]:
        if label == "invalid-kind":
            old = ", i32 2, i64 %v" + str(payload)
            new = ", i32 99, i64 %v" + str(payload)
        tool.write_text("#!" + sys.executable + "\nimport pathlib,sys,os\n"
                        "p=pathlib.Path(sys.argv[-1]); s=p.read_text()\n"
                        f"assert {old!r} in s\ns=s.replace({old!r},{new!r},1); p.write_text(s)\n"
                        f"os.execv({real_llc!r},[{real_llc!r}]+sys.argv[1:])\n")
        tool.chmod(0o755)
        compile_input(bool_inputs[0], custom_env=dict(env, PATH=str(injection) + os.pathsep + env["PATH"]))
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
    compile_input(issued, False, dict(env, PATH=str(work)))  # llc unavailable
    assert obj.read_bytes() == sentinel
    obj.unlink()
    compile_input(malformed, False)
    assert not obj.exists()
    print("session mismatch, missing/colliding site and tool failure: pre-artifact rejection passed")
