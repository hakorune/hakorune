#!/usr/bin/env bash
# Focused proof for the Boundary pure-first invocation-owned compile options.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="llvm-compile-options-contract-smoke"

bash "$ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null

ROOT="$ROOT" python3 - <<'PY'
import ctypes
import os
import pathlib
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
lib.hako_mem_free.argtypes = [ctypes.c_void_p]

fixture = root / "apps/tests/mir_shape_guard/ret_const_min_v1.mir.json"
exact_fixture = root / "apps/tests/mir_shape_guard/substring_concat_loop_pure_min_v1.mir.json"
if not fixture.is_file():
    raise SystemExit(f"missing fixture: {fixture}")
if not exact_fixture.is_file():
    raise SystemExit(f"missing fixture: {exact_fixture}")

with tempfile.TemporaryDirectory(prefix="hako-options-contract-") as temp:
    temp = pathlib.Path(temp)
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

print("[llvm-compile-options-contract-smoke] explicit options, propagation, restore, and pre-effect reject: ok")
PY
