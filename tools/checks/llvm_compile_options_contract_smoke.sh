#!/usr/bin/env bash
# Focused proof for the Boundary and AOT Generic invocation-owned options.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="llvm-compile-options-contract-smoke"

bash "$ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null

ROOT="$ROOT" python3 - <<'PY'
import ctypes
from concurrent.futures import ThreadPoolExecutor
import json
import os
import pathlib
import shutil
import subprocess
import sys
import tempfile

root = pathlib.Path(os.environ["ROOT"])
lib = ctypes.CDLL(str(root / "target/release/libhako_llvmc_ffi.so"))

class Contract(ctypes.Structure):
    _fields_ = [
        ("revision", ctypes.c_uint32),
        ("byte_size", ctypes.c_uint32),
        ("ingress_profile", ctypes.c_uint32),
        ("flags", ctypes.c_uint32),
        ("compile_recipe", ctypes.c_char_p),
        ("compat_replay", ctypes.c_char_p),
        ("opt_level", ctypes.c_char_p),
        ("opt_tool_path", ctypes.c_char_p),
        ("llc_tool_path", ctypes.c_char_p),
        ("llc_flags", ctypes.c_char_p),
        ("llvmc_path", ctypes.c_char_p),
    ]

compile_with_options = lib.hako_llvmc_compile_json_with_options_v1
compile_with_options.argtypes = [
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.POINTER(Contract),
    ctypes.POINTER(ctypes.c_void_p),
]
compile_with_options.restype = ctypes.c_int
compile_public = lib.hako_llvmc_compile_json
compile_public.argtypes = [
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.POINTER(ctypes.c_void_p),
]
compile_public.restype = ctypes.c_int
hako_aot_compile = lib.hako_aot_compile_json
hako_aot_compile.argtypes = [
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.POINTER(ctypes.c_void_p),
]
hako_aot_compile.restype = ctypes.c_int
lib.hako_mem_free.argtypes = [ctypes.c_void_p]

fixture = root / "apps/tests/mir_shape_guard/ret_const_min_v1.mir.json"
exact_fixture = root / "apps/tests/mir_shape_guard/substring_concat_loop_pure_min_v1.mir.json"
if not fixture.is_file():
    raise SystemExit(f"missing fixture: {fixture}")
if not exact_fixture.is_file():
    raise SystemExit(f"missing fixture: {exact_fixture}")

with tempfile.TemporaryDirectory(prefix="hako-options-contract-") as temp:
    temp = pathlib.Path(temp)
    ownership = temp / "options-ownership"
    subprocess.run(["cc", "-std=gnu11", "-O2",
        str(root / "lang/c-abi/tests/physical_options_ownership_test.c"),
        "-o", str(ownership)], check=True)
    subprocess.run([str(ownership)], check=True)
    harness_log = temp / "harness-log-ownership"
    subprocess.run(["cc", "-std=gnu11", "-Wall", "-Wextra", "-Werror",
        str(root / "lang/c-abi/tests/harness_log_ownership_test.c"),
        "-o", str(harness_log)], check=True)
    subprocess.run([str(harness_log)], check=True)
    capture = temp / "legacy-capi-invocation-capture"
    subprocess.run(["cc", "-std=gnu11", "-O2",
        "-I" + str(root / "plugins/nyash-json-plugin/c/yyjson"),
        str(root / "lang/c-abi/tests/legacy_capi_invocation_capture_test.c"),
        str(root / "lang/c-abi/shims/hako_aot.c"),
        str(root / "lang/c-abi/shims/hako_json_v1.c"),
        str(root / "plugins/nyash-json-plugin/c/yyjson/yyjson.c"),
        "-ldl", "-o", str(capture)], check=True)
    subprocess.run([str(capture)], check=True)
    fast_capture = temp / "fast-invocation-capture"
    subprocess.run(["cc", "-std=gnu11", "-O2",
        "-I" + str(root / "plugins/nyash-json-plugin/c/yyjson"),
        str(root / "lang/c-abi/tests/fast_invocation_capture_test.c"),
        str(root / "lang/c-abi/shims/hako_aot.c"),
        str(root / "lang/c-abi/shims/hako_json_v1.c"),
        str(root / "plugins/nyash-json-plugin/c/yyjson/yyjson.c"),
        "-ldl", "-o", str(fast_capture)], check=True)
    subprocess.run([str(fast_capture)], check=True)
    opt_log = temp / "opt.args"
    llc_log = temp / "llc.args"
    scripts = {}
    for name, log in (("opt", opt_log), ("llc", llc_log)):
        script = temp / name
        script.write_text(
            "#!/bin/sh\n"
            f"printf '%s\\n' \"$@\" > '{log}'\n"
            "out=\ninput=\n"
            "while [ \"$#\" -gt 0 ]; do\n"
            "  if [ \"$1\" = \"-o\" ]; then shift; out=\"$1\"; else input=\"$1\"; fi\n"
            "  shift\n"
            "done\n"
            "cp \"$input\" \"$out\"\n"
        )
        script.chmod(0o755)
        scripts[name] = script

    good = Contract(
        1,
        ctypes.sizeof(Contract),
        1,
        0,
        b"pure-first",
        b"none",
        b"0",
        str(scripts["opt"]).encode(),
        str(scripts["llc"]).encode(),
        b"-contract-flag",
        None,
    )
    before = os.environ.copy()
    os.environ.update({
        "HAKO_BACKEND_COMPILE_RECIPE": "ambient-wrong",
        "HAKO_BACKEND_COMPAT_REPLAY": "harness",
        "NYASH_LLVM_OPT_LEVEL": "3",
        "NYASH_NY_LLVM_LLC_FLAGS": "-ambient-flag",
    })
    expected = os.environ.copy()
    out = temp / "good.o"
    error = ctypes.c_void_p()
    rc = compile_with_options(
        str(fixture).encode(), str(out).encode(), ctypes.byref(good), ctypes.byref(error)
    )
    message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
    if error.value:
        lib.hako_mem_free(error)
    after = os.environ.copy()
    os.environ.clear()
    os.environ.update(before)
    if rc != 0 or not out.is_file():
        raise SystemExit(f"explicit compile failed rc={rc}: {message}")
    if "-contract-flag" not in llc_log.read_text() or "-ambient-flag" in llc_log.read_text():
        raise SystemExit("contract llc flags were not consumed")
    if "mem2reg" not in opt_log.read_text() or after != expected:
        raise SystemExit("explicit options changed ambient state or opt route")

    exact_out = temp / "exact.o"
    error = ctypes.c_void_p()
    rc = compile_with_options(
        str(exact_fixture).encode(), str(exact_out).encode(), ctypes.byref(good), ctypes.byref(error)
    )
    message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
    if error.value:
        lib.hako_mem_free(error)
    if rc != 0 or not exact_out.is_file():
        raise SystemExit(f"explicit exact-seed compile failed rc={rc}: {message}")
    if "-contract-flag" not in llc_log.read_text() or "-ambient-flag" in llc_log.read_text():
        raise SystemExit("exact-seed route did not consume contract llc flags")
    if "mem2reg" not in opt_log.read_text():
        raise SystemExit("exact-seed route did not consume contract opt tool")

    generic = Contract(
        1,
        ctypes.sizeof(Contract),
        0,
        0,
        b"pure-first",
        b"none",
        b"0",
        str(scripts["opt"]).encode(),
        str(scripts["llc"]).encode(),
        b"-generic-contract-flag",
        None,
    )
    generic_out = temp / "generic.o"
    error = ctypes.c_void_p()
    rc = compile_with_options(
        str(fixture).encode(), str(generic_out).encode(), ctypes.byref(generic), ctypes.byref(error)
    )
    message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
    if error.value:
        lib.hako_mem_free(error)
    if rc != 0 or not generic_out.is_file():
        raise SystemExit(f"generic profile compile failed rc={rc}: {message}")
    if "-generic-contract-flag" not in llc_log.read_text():
        raise SystemExit("generic profile did not consume contract llc flags")

    public_before = os.environ.copy()
    os.environ.update({
        "HAKO_BACKEND_COMPILE_RECIPE": "pure-first",
        "HAKO_BACKEND_COMPAT_REPLAY": "unknown-replay",
        "NYASH_LLVM_OPT_LEVEL": "2legacy-suffix",
        "NYASH_NY_LLVM_OPT_TOOL": str(scripts["opt"]),
        "NYASH_NY_LLVM_LLC_TOOL": str(scripts["llc"]),
        "NYASH_NY_LLVM_LLC_FLAGS": "-public-effective-flag",
    })
    os.environ.pop("HAKO_CAPI_PURE", None)
    public_expected = os.environ.copy()
    public_out = temp / "public-generic.o"
    error = ctypes.c_void_p()
    rc = compile_public(
        str(fixture).encode(), str(public_out).encode(), ctypes.byref(error)
    )
    message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
    if error.value:
        lib.hako_mem_free(error)
    public_after = os.environ.copy()
    os.environ.clear()
    os.environ.update(public_before)
    if rc != 0 or not public_out.is_file():
        raise SystemExit(f"public Generic options bridge failed rc={rc}: {message}")
    if "default<O2>" not in opt_log.read_text():
        raise SystemExit("public Generic bridge did not preserve opt first-character behavior")
    if "-public-effective-flag" not in llc_log.read_text():
        raise SystemExit("public Generic bridge did not capture effective llc flags")
    if public_after != public_expected:
        raise SystemExit("public Generic bridge changed ambient state")

    def expect_public_reject(label, updates, needle):
        before = os.environ.copy()
        os.environ.pop("HAKO_CAPI_PURE", None)
        os.environ.update(updates)
        out = temp / ("public-" + label + ".o")
        error = ctypes.c_void_p()
        rc = compile_public(
            str(fixture).encode(), str(out).encode(), ctypes.byref(error)
        )
        message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
        if error.value:
            lib.hako_mem_free(error)
        os.environ.clear()
        os.environ.update(before)
        if rc == 0 or out.exists() or needle not in message:
            raise SystemExit(f"public {label} admission was not pre-effect: rc={rc} {message!r}")

    expect_public_reject(
        "recipe-reject",
        {"HAKO_BACKEND_COMPILE_RECIPE": "ambient-wrong", "HAKO_BACKEND_COMPAT_REPLAY": "none"},
        "generic-capi-recipe-required",
    )
    expect_public_reject(
        "replay-reject",
        {"HAKO_BACKEND_COMPILE_RECIPE": "pure-first", "HAKO_BACKEND_COMPAT_REPLAY": "harness"},
        "generic-capi-compat-admission-required",
    )
    expect_public_reject(
        "alias-reject",
        {"HAKO_BACKEND_COMPILE_RECIPE": "pure-first", "HAKO_BACKEND_COMPAT_REPLAY": "none", "HAKO_CAPI_PURE": "1"},
        "hako_capi_pure_retired",
    )

    os.environ.update({
        "HAKO_AOT_USE_FFI": "1",
        "HAKO_AOT_FFI_LIB": str(root / "target/release/libhako_llvmc_ffi.so"),
        "HAKO_BACKEND_COMPILE_RECIPE": "pure-first",
        "HAKO_BACKEND_COMPAT_REPLAY": "none",
        "NYASH_LLVM_OPT_LEVEL": "0",
        "NYASH_NY_LLVM_OPT_TOOL": str(scripts["opt"]),
        "NYASH_NY_LLVM_LLC_TOOL": str(scripts["llc"]),
        "NYASH_NY_LLVM_LLC_FLAGS": "-aot-contract-flag",
    })
    aot_out = temp / "aot-generic.o"
    error = ctypes.c_void_p()
    rc = hako_aot_compile(
        str(fixture).encode(), str(aot_out).encode(), ctypes.byref(error)
    )
    message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
    if error.value:
        lib.hako_mem_free(error)
    if rc != 0 or not aot_out.is_file():
        raise SystemExit(f"AOT generic FFI compile failed rc={rc}: {message}")
    if "-aot-contract-flag" not in llc_log.read_text():
        raise SystemExit("AOT generic did not pass llc flags through options")

    os.environ["HAKO_BACKEND_COMPILE_RECIPE"] = "ambient-wrong"
    recipe_out = temp / "aot-recipe-reject.o"
    error = ctypes.c_void_p()
    rc = hako_aot_compile(
        str(fixture).encode(), str(recipe_out).encode(), ctypes.byref(error)
    )
    message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
    if error.value:
        lib.hako_mem_free(error)
    os.environ["HAKO_BACKEND_COMPILE_RECIPE"] = "pure-first"
    if rc == 0 or recipe_out.exists() or "generic-aot-recipe-required" not in message:
        raise SystemExit(f"AOT recipe was not rejected before effects: rc={rc} {message!r}")

    os.environ["HAKO_BACKEND_COMPAT_REPLAY"] = "harness"
    replay_out = temp / "aot-replay-reject.o"
    error = ctypes.c_void_p()
    rc = hako_aot_compile(
        str(fixture).encode(), str(replay_out).encode(), ctypes.byref(error)
    )
    message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
    if error.value:
        lib.hako_mem_free(error)
    os.environ["HAKO_BACKEND_COMPAT_REPLAY"] = "none"
    if rc == 0 or replay_out.exists() or "aot-compat-admission-required" not in message:
        raise SystemExit(f"AOT replay was not rejected before effects: rc={rc} {message!r}")

    os.environ["HAKO_CAPI_PURE"] = "1"
    pure_alias_out = temp / "aot-pure-alias.o"
    error = ctypes.c_void_p()
    rc = hako_aot_compile(
        str(fixture).encode(), str(pure_alias_out).encode(), ctypes.byref(error)
    )
    message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
    if error.value:
        lib.hako_mem_free(error)
    os.environ.pop("HAKO_CAPI_PURE")
    if rc == 0 or pure_alias_out.exists() or "hako_capi_pure_retired" not in message:
        raise SystemExit(f"AOT HAKO_CAPI_PURE was not rejected before effects: rc={rc} {message!r}")

    # Named Harness owns only its compiler path. Input and child settings stay
    # opaque; the public options ingress above must continue rejecting profile3.
    harness = lib.hako_llvmc_compile_json_compat_harness
    harness.argtypes = compile_public.argtypes
    harness.restype = ctypes.c_int
    aot_harness = lib.hako_aot_compile_json_compat_harness
    aot_harness.argtypes = compile_public.argtypes
    aot_harness.restype = ctypes.c_int
    fake = temp / "fake harness"
    capture = temp / "harness.json"
    fake.write_text(
        f"#!{sys.executable}\nimport json, os, pathlib, sys, time\n"
        f"pathlib.Path({str(capture)!r}).write_text(json.dumps([sys.argv[1:], dict(os.environ)]))\n"
        "mode = os.environ.get('HARNESS_TEST_MODE', '')\n"
        "if mode == 'fail':\n sys.stderr.write('first child error\\nsecond line\\n'); sys.exit(7)\n"
        "if mode == 'overlap-fail':\n"
        " out = pathlib.Path(sys.argv[sys.argv.index('--out') + 1])\n"
        " sys.stderr.write('overlap:' + out.name + '\\n'); sys.stderr.flush(); time.sleep(0.25); sys.exit(7)\n"
        "if mode != 'missing':\n pathlib.Path(sys.argv[sys.argv.index('--out') + 1]).write_bytes(b'object')\n"
    )
    fake.chmod(0o755)
    harness_before = os.environ.copy()
    os.environ.update({"NYASH_NY_LLVM_COMPILER": str(fake),
        "HAKO_BACKEND_COMPILE_RECIPE": "wrong-parent-recipe",
        "HAKO_BACKEND_COMPAT_REPLAY": "unknown",
        "NYASH_LLVM_OPT_LEVEL": "", "HAKO_LLVM_OPT_LEVEL": "3"})
    opaque = temp / "malformed.json"
    opaque.write_text("not JSON")
    def harness_call(entry, output, input_path=opaque):
        expected = os.environ.copy()
        error = ctypes.c_void_p()
        rc = entry(str(input_path).encode(), str(output).encode(), ctypes.byref(error))
        message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
        if error.value:
            lib.hako_mem_free(error)
        assert os.environ.copy() == expected, "named harness changed parent environment"
        return rc, message
    for entry, level in [(entry, level) for entry in [harness, aot_harness]
                         for level in [None, "", "O3"]]:
        if level is None:
            os.environ.pop("NYASH_LLVM_OPT_LEVEL", None)
        else:
            os.environ["NYASH_LLVM_OPT_LEVEL"] = level
        output = temp / "harness.o"
        rc, message = harness_call(entry, output)
        assert rc == 0 and output.read_bytes() == b"object", (rc, message)
        args, child_env = json.loads(capture.read_text())
        assert args == ["--driver", "harness", "--in", str(opaque), "--emit", "obj", "--out", str(output)]
        for key in ["NYASH_LLVM_OPT_LEVEL", "HAKO_LLVM_OPT_LEVEL",
                    "HAKO_BACKEND_COMPILE_RECIPE", "HAKO_BACKEND_COMPAT_REPLAY"]:
            assert child_env.get(key) == os.environ.get(key), (key, child_env.get(key))
    os.environ["NYASH_NY_LLVM_COMPILER"] = str(temp / "absent")
    output.write_bytes(b"sentinel")
    rc, message = harness_call(harness, output)
    assert rc != 0 and "ny-llvmc not found" in message and output.read_bytes() == b"sentinel"
    os.environ["NYASH_NY_LLVM_COMPILER"] = str(fake)
    for mode, needle in [("fail", "first child error"), ("missing", "finished without object")]:
        os.environ["HARNESS_TEST_MODE"] = mode
        rc, message = harness_call(harness, output)
        assert rc != 0 and needle in message and not output.exists(), (rc, message)
        assert "second line" not in message
    errorless_output = temp / "harness-errorless.o"
    rc = harness(str(opaque).encode(), str(errorless_output).encode(), None)
    assert rc != 0 and not errorless_output.exists()
    os.environ["HARNESS_TEST_MODE"] = "overlap-fail"
    overlap_outputs = [temp / "overlap-direct.o", temp / "overlap-aot.o"]
    with ThreadPoolExecutor(max_workers=2) as pool:
        overlap_results = list(pool.map(
            lambda item: harness_call(item[0], item[1]),
            [(harness, overlap_outputs[0]), (aot_harness, overlap_outputs[1])]))
    for result, overlap_output in zip(overlap_results, overlap_outputs):
        rc, message = result
        assert rc != 0 and f"overlap:{overlap_output.name}" in message
        assert not overlap_output.exists()
    os.environ.pop("HARNESS_TEST_MODE")
    output.write_bytes(b"sentinel")
    old_tmp = os.environ.get("TMPDIR")
    os.environ["TMPDIR"] = "/" + "x" * 1100
    rc, message = harness_call(harness, output)
    assert rc != 0 and "log path too long" in message and output.read_bytes() == b"sentinel"
    if old_tmp is None:
        os.environ.pop("TMPDIR")
    else:
        os.environ["TMPDIR"] = old_tmp
    blocked_tmp = temp / "tmp-is-a-file"
    blocked_tmp.write_text("not a directory")
    capture_before = capture.read_bytes()
    os.environ["TMPDIR"] = str(blocked_tmp)
    output.write_bytes(b"sentinel")
    rc, message = harness_call(harness, output)
    assert rc != 0 and "log create failed" in message
    assert output.read_bytes() == b"sentinel" and capture.read_bytes() == capture_before
    if old_tmp is None:
        os.environ.pop("TMPDIR")
    else:
        os.environ["TMPDIR"] = old_tmp
    rc, message = harness_call(harness, output, "x" * 4096)
    assert rc != 0 and "command too long" in message and output.read_bytes() == b"sentinel"
    fake.chmod(0o644)
    rc, message = harness_call(harness, output)
    assert rc != 0 and "not found (NYASH_NY_LLVM_COMPILER)" not in message and not output.exists()
    os.environ.clear()
    os.environ.update(harness_before)

    no_flags = Contract(
        1,
        ctypes.sizeof(Contract),
        1,
        0,
        b"pure-first",
        b"none",
        b"0",
        str(scripts["opt"]).encode(),
        str(scripts["llc"]).encode(),
        None,
        None,
    )
    no_flags_out = temp / "no-flags.o"
    error = ctypes.c_void_p()
    rc = compile_with_options(
        str(fixture).encode(), str(no_flags_out).encode(), ctypes.byref(no_flags), ctypes.byref(error)
    )
    message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
    if error.value:
        lib.hako_mem_free(error)
    if rc != 0 or not no_flags_out.is_file():
        raise SystemExit(f"explicit unset-flags compile failed rc={rc}: {message}")
    if "-ambient-flag" in llc_log.read_text():
        raise SystemExit("explicit unset llc flags inherited ambient state")

    bad = Contract(2, ctypes.sizeof(Contract), 1, 0, b"pure-first", b"none", b"0", None, None, None, None)
    bad_out = temp / "bad.o"
    error = ctypes.c_void_p()
    rc = compile_with_options(
        str(fixture).encode(), str(bad_out).encode(), ctypes.byref(bad), ctypes.byref(error)
    )
    message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
    if error.value:
        lib.hako_mem_free(error)
    if rc == 0 or bad_out.exists() or "revision-size" not in message:
        raise SystemExit(f"invalid revision was not rejected before effects: rc={rc} {message!r}")

    unsupported_profile = Contract(1, ctypes.sizeof(Contract), 3, 0, b"pure-first", b"none", b"0", None, None, None, None)
    unsupported_out = temp / "unsupported-profile.o"
    error = ctypes.c_void_p()
    rc = compile_with_options(
        str(fixture).encode(), str(unsupported_out).encode(), ctypes.byref(unsupported_profile), ctypes.byref(error)
    )
    message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
    if error.value:
        lib.hako_mem_free(error)
    if rc == 0 or unsupported_out.exists() or "profile" not in message:
        raise SystemExit(f"explicit harness profile was not rejected: rc={rc} {message!r}")

    real_opt = shutil.which("opt-18")
    real_llc = shutil.which("llc-18")
    if real_opt and real_llc:
        real_fixture = root / "apps/tests/mir_shape_guard/substring_concat_loop_pure_min_v1.mir.json"
        if not real_fixture.is_file():
            raise SystemExit(f"missing FAST real-object fixture: {real_fixture}")
        real_before = os.environ.copy()
        real_contract = Contract(
            1, ctypes.sizeof(Contract), 0, 0, b"pure-first", b"none", b"0",
            real_opt.encode(), real_llc.encode(), b"", None,
        )
        try:
            for fast in ("0", "1"):
                os.environ["NYASH_LLVM_FAST"] = fast
                real_out = temp / f"fast-real-{fast}.o"
                error = ctypes.c_void_p()
                rc = compile_with_options(
                    str(real_fixture).encode(), str(real_out).encode(),
                    ctypes.byref(real_contract), ctypes.byref(error)
                )
                message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
                if error.value:
                    lib.hako_mem_free(error)
                if rc != 0 or not real_out.is_file() or real_out.stat().st_size == 0:
                    raise SystemExit(f"FAST={fast} real LLVM object failed: rc={rc} {message}")
        finally:
            os.environ.clear()
            os.environ.update(real_before)
    else:
        print("[llvm-compile-options-contract-smoke] real LLVM18 FAST object: SKIP (opt-18/llc-18 unavailable)")

print("[llvm-compile-options-contract-smoke] Boundary/AOT/Generic options and named-harness ownership/order: ok")
PY
