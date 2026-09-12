"""One native Windows DLL invocation with environment fixed at process birth.

Do not synchronize CRT caches with putenv: that deletes present-empty values.
The smoke supplies subprocess.env before either CRT or the tested DLL loads.
"""
import ctypes
import json
import os
import pathlib
import sys

assert os.name == "nt", "Windows invocation worker requires native Python"
keys = ("HAKO_LLVM_OPT_LEVEL", "NYASH_LLVM_OPT_LEVEL")
kernel = ctypes.WinDLL("kernel32", use_last_error=True)
kernel.GetEnvironmentVariableW.argtypes = [ctypes.c_wchar_p, ctypes.c_wchar_p, ctypes.c_uint32]
kernel.GetEnvironmentVariableW.restype = ctypes.c_uint32
crts = [ctypes.CDLL(name) for name in ("msvcrt.dll", "ucrtbase.dll")]
for crt in crts:
    crt.getenv.argtypes = [ctypes.c_char_p]
    crt.getenv.restype = ctypes.c_char_p

def snapshot():
    native = []
    for key in keys:
        ctypes.set_last_error(0)
        size = kernel.GetEnvironmentVariableW(key, None, 0)
        if not size:
            native.append(None if ctypes.get_last_error() == 203 else "")
        else:
            value = ctypes.create_unicode_buffer(size)
            kernel.GetEnvironmentVariableW(key, value, size)
            native.append(value.value)
    cached = [[crt.getenv(key.encode()) for key in keys] for crt in crts]
    return [native] + [[None if v is None else v.decode() for v in pair] for pair in cached]

lib_path, name, fixture, output = sys.argv[1:]
lib = ctypes.CDLL(lib_path)
fn = getattr(lib, name)
fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]
fn.restype = ctypes.c_int
lib.hako_mem_free.argtypes = [ctypes.c_void_p]
before = snapshot()
expected = [os.environ.get(key) for key in keys]
if before[0] != expected:
    raise SystemExit(f"Windows worker environment admission drift: {before[0]} != {expected}")
err = ctypes.c_void_p()
rc = fn(fixture.encode(), output.encode(), ctypes.byref(err))
message = ctypes.string_at(err.value).decode(errors="replace") if err.value else ""
if err.value:
    lib.hako_mem_free(err)
after = snapshot()
print(json.dumps([rc, pathlib.Path(output).is_file(), message, before, after]))
