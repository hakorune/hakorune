#!/usr/bin/env python3
"""Public retained static session: configuration capture and stage error order."""
import ctypes as c
import json
import os
from pathlib import Path
import sys
import tempfile

lib = c.CDLL(sys.argv[1])
libc = c.CDLL(None)
libc.free.argtypes = [c.c_void_p]
class Frame(c.Structure):
    _fields_ = [("revision", c.c_uint32), ("size", c.c_uint32)] + [
        item for name in ("calls", "maps", "values", "expanded")
        for item in ((name, c.c_void_p), (name + "_count", c.c_uint64))]
open_session = lib.hako_llvmc_static_open_v2
open_session.argtypes = [c.c_char_p, c.c_size_t, c.POINTER(c.c_void_p), c.POINTER(c.c_void_p)]
query = lib.hako_llvmc_static_query_v2
query.argtypes = [c.c_void_p, c.c_char_p, c.c_size_t, c.c_uint32, c.c_uint32, c.POINTER(c.c_uint32)]
compile_session = lib.hako_llvmc_static_compile_v2
compile_session.argtypes = [c.c_void_p, c.POINTER(Frame), c.c_char_p, c.POINTER(c.c_void_p)]
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
