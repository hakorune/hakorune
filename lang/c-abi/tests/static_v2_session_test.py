#!/usr/bin/env python3
"""Public retained static session: configuration capture and stage error order."""
import ctypes as c
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

lib = c.CDLL(sys.argv[1])
libc = c.CDLL(None)
libc.free.argtypes = [c.c_void_p]
class Contract(c.Structure):
    _fields_ = [
        ("revision", c.c_uint32), ("byte_size", c.c_uint32),
        ("ingress_profile", c.c_uint32), ("flags", c.c_uint32),
        ("compile_recipe", c.c_char_p), ("compat_replay", c.c_char_p),
        ("opt_level", c.c_char_p), ("opt_tool_path", c.c_char_p),
        ("llc_tool_path", c.c_char_p), ("llc_flags", c.c_char_p),
        ("llvmc_path", c.c_char_p),
    ]
class Frame(c.Structure):
    _fields_ = [("revision", c.c_uint32), ("size", c.c_uint32)] + [
        item for name in ("calls", "maps", "values", "expanded")
        for item in ((name, c.c_void_p), (name + "_count", c.c_uint64))]
open_session = lib.hako_llvmc_static_open_v2
open_session.argtypes = [c.c_char_p, c.c_size_t, c.POINTER(c.c_void_p), c.POINTER(c.c_void_p)]
open_session.restype = c.c_int
open_with_options = lib.hako_llvmc_static_open_v2_with_options
open_with_options.argtypes = [
    c.c_char_p, c.c_size_t, c.POINTER(Contract), c.POINTER(c.c_void_p), c.POINTER(c.c_void_p)
]
open_with_options.restype = c.c_int
query = lib.hako_llvmc_static_query_v2
query.argtypes = [c.c_void_p, c.c_char_p, c.c_size_t, c.c_uint32, c.c_uint32, c.POINTER(c.c_uint32)]
query.restype = c.c_int
compile_session = lib.hako_llvmc_static_compile_v2
compile_session.argtypes = [c.c_void_p, c.POINTER(Frame), c.c_char_p, c.POINTER(c.c_void_p)]
compile_session.restype = c.c_int
close = lib.hako_llvmc_static_close_v2
close.argtypes = [c.c_void_p]

def message(error):
    if not error.value:
        return ""
    text = c.string_at(error.value).decode()
    libc.free(error)
    return text

def opened(body):
    data = body if isinstance(body, bytes) else json.dumps(body).encode()
    handle, error = c.c_void_p(), c.c_void_p()
    rc = open_session(data, len(data), c.byref(handle), c.byref(error))
    return rc, handle, message(error)

def llvm18_tool_available(env_name, names):
    candidates = ([os.environ[env_name]] if os.environ.get(env_name) else []) + names
    for candidate in candidates:
        path = shutil.which(candidate)
        if not path:
            continue
        try:
            version = subprocess.run([path, "--version"], capture_output=True,
                                     text=True, check=False, timeout=2).stdout
        except (OSError, subprocess.SubprocessError):
            continue
        if "LLVM version 18" in version:
            return True
    return False

def llvm18_available():
    return (llvm18_tool_available("NYASH_NY_LLVM_OPT_TOOL", ["opt", "opt-18"])
            and llvm18_tool_available("NYASH_NY_LLVM_LLC_TOOL", ["llc", "llc-18"]))

os.environ["HAKO_BACKEND_COMPILE_RECIPE"] = "pure-first"
os.environ.pop("HAKO_BACKEND_COMPAT_REPLAY", None)
body = {"functions": [{"name": "main", "params": [], "metadata": {}, "blocks": [{"id": 0,
    "instructions": [{"op": "newbox", "dst": 1, "type": "ArrayBox", "args": []},
                     {"op": "const", "dst": 2, "value": {"type": "i64", "value": 30}},
                     {"op": "ret", "value": 2}]}]}]}
for initial, changed, expected in [("", "direct_array_i64_exact", 0),
                                    ("direct_array_i64_exact", "", 1)]:
    os.environ["HAKO_ARRAY_SLOT_STORE"] = initial
    rc, handle, error = opened(body)
    assert rc == 0, error
    try:
        os.environ["HAKO_ARRAY_SLOT_STORE"] = changed
        for _ in range(2):
            consumer = c.c_uint32()
            assert query(handle, b"main", 4, 0, 0, c.byref(consumer)) == 0
            assert consumer.value == expected, consumer.value
    finally:
        close(handle)
os.environ.pop("HAKO_ARRAY_SLOT_STORE", None)
os.environ["HAKO_BACKEND_COMPAT_REPLAY"] = "harness"
rc, handle, error = opened(b"{")
assert rc != 0 and not handle.value and "generic-capi-compat-admission-required" in error, error
os.environ.pop("HAKO_BACKEND_COMPAT_REPLAY")
saved = os.environ.copy()
os.environ["HAKO_BACKEND_COMPILE_RECIPE"] = "ambient-wrong"
os.environ["HAKO_BACKEND_COMPAT_REPLAY"] = "none"
ambient_before = os.environ.copy()
body_bytes = json.dumps(body).encode()
with tempfile.TemporaryDirectory(prefix="hako-static-contract-") as directory:
    directory = Path(directory)
    opt_log, llc_log = directory / "opt.args", directory / "llc.args"
    tools = {}
    for name, log in (("opt", opt_log), ("llc", llc_log)):
        tool = directory / name
        tool.write_text(
            "#!/bin/sh\n"
            "set -eu\n"
            f"printf '%s\\n' \"$@\" > '{log}'\n"
            "out= input=\n"
            "while [ \"$#\" -gt 0 ]; do\n"
            "  if [ \"$1\" = \"-o\" ]; then shift; out=\"$1\";"
            " else input=\"$1\"; fi\n"
            "  shift\n"
            "done\n"
            "cp \"$input\" \"$out\"\n"
        )
        tool.chmod(0o755)
        tools[name] = tool
    contract = Contract(
        1, c.sizeof(Contract), 2, 0, b"pure-first", b"none", b"0",
        os.fsencode(tools["opt"]), os.fsencode(tools["llc"]),
        b"-static-contract-flag", None,
    )
    handle, error = c.c_void_p(), c.c_void_p()
    rc = open_with_options(body_bytes, len(body_bytes), c.byref(contract),
                           c.byref(handle), c.byref(error))
    assert rc == 0, message(error)
    try:
        consumer = c.c_uint32()
        assert query(handle, b"main", 4, 0, 0, c.byref(consumer)) == 0
        frame = Frame(2, c.sizeof(Frame))
        output = directory / "result.o"
        error = c.c_void_p()
        rc = compile_session(handle, c.byref(frame), os.fsencode(output),
                             c.byref(error))
        assert rc == 0, message(error)
        assert output.exists()
        assert "-static-contract-flag" in llc_log.read_text()
        assert "ambient-wrong" not in opt_log.read_text()
        assert os.environ == ambient_before
    finally:
        close(handle)
os.environ.clear()
os.environ.update(saved)
for null_body in (body_bytes, b"{"):
    before = os.environ.copy()
    handle, error = c.c_void_p(1), c.c_void_p()
    rc = open_with_options(null_body, len(null_body), None,
                           c.byref(handle), c.byref(error))
    null_message = message(error)
    assert rc != 0 and not handle.value and "static-v2/options-null" in null_message, null_message
    assert os.environ == before
for bad_contract, expected in [
    (Contract(1, c.sizeof(Contract), 1, 0, b"pure-first", b"none", b"0",
              None, None, None, None), "static-v2/profile"),
    (Contract(1, c.sizeof(Contract), 2, 1, b"pure-first", b"none", b"0",
              None, None, None, None), "profile-flags"),
    (Contract(1, c.sizeof(Contract) - 4, 2, 0, b"pure-first", b"none", b"0",
              None, None, None, None), "revision-size"),
    (Contract(2, c.sizeof(Contract), 2, 0, b"pure-first", b"none", b"0",
              None, None, None, None), "revision-size"),
]:
    before = os.environ.copy()
    handle, error = c.c_void_p(), c.c_void_p()
    rc = open_with_options(body_bytes, len(body_bytes), c.byref(bad_contract),
                           c.byref(handle), c.byref(error))
    bad_message = message(error)
    assert rc != 0 and not handle.value and expected in bad_message, bad_message
    assert os.environ == before
rc, handle, error = opened(b"{")
assert rc != 0 and not handle.value and "static-v2/json" in error, error
with tempfile.TemporaryDirectory(prefix="hako-static-session-") as directory:
    output = Path(directory) / "result.o"
    rc, handle, error = opened({"functions": []})
    assert rc == 0, error
    try:
        frame = Frame(0, c.sizeof(Frame))
        error = c.c_void_p()
        assert compile_session(handle, c.byref(frame), os.fsencode(output), c.byref(error)) != 0
        assert "static-v2" in message(error)
        assert not output.exists()
    finally:
        close(handle)
    if not llvm18_available():
        print("static V2 session capture/error/cancel/compile: explicit contract ok; LLVM18 object compile skipped")
    else:
        rc, handle, error = opened(body)
        assert rc == 0, error
        try:
            frame = Frame(2, c.sizeof(Frame))
            error = c.c_void_p()
            rc = compile_session(handle, c.byref(frame), os.fsencode(output), c.byref(error))
            assert rc == 0, message(error)
            assert output.exists()
        finally:
            close(handle)
        print("static V2 session capture/error/cancel/compile: ok")
