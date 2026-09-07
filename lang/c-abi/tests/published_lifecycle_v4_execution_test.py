#!/usr/bin/env python3
"""V4 physical input -> llc -> actual lifecycle archive; not Rust host cutover proof.

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
issued = json.loads(Path(sys.argv[1]).read_text())
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
    compile_input(issued, False, dict(env, PATH=str(work)))  # llc unavailable
    assert obj.read_bytes() == sentinel
    obj.unlink()
    compile_input(malformed, False)
    assert not obj.exists()
    print("session mismatch, missing/colliding site and tool failure: pre-artifact rejection passed")
