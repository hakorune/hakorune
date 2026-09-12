#!/usr/bin/env bash
# Runtime proof for the generic-vs-named Hako AOT C-ABI boundary.
# The generic entry must never inherit harness replay; only the named
# compatibility entry may reach the frozen llvmlite keep lane.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
TAG="llvm-hako-aot-ffi-admission-smoke"
MODE="${1:-all}"
case "$MODE" in
  all|child-env) ;;
  *)
    echo "usage: $0 [all|child-env]" >&2
    exit 2
    ;;
esac

bash "$ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null

MODE="$MODE" ROOT="$ROOT" python3 - <<'PY'
import ctypes
import json
import os
import pathlib
import tempfile

root = pathlib.Path(os.environ["ROOT"])
mode = os.environ["MODE"]
lib_name = "hako_llvmc_ffi.dll" if os.name == "nt" else "libhako_llvmc_ffi.so"
lib = ctypes.CDLL(str(root / "target/release" / lib_name))
for name in (
    "hako_aot_compile_json",
    "hako_aot_compile_json_compat_harness",
    "hako_llvmc_compile_json",
    "hako_llvmc_compile_json_compat_harness",
):
    fn = getattr(lib, name)
    fn.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]
    fn.restype = ctypes.c_int
lib.hako_mem_free.argtypes = [ctypes.c_void_p]
libc = ctypes.CDLL(None)
libc.getenv.argtypes = [ctypes.c_char_p]
libc.getenv.restype = ctypes.c_char_p

OPT_KEYS = ("HAKO_LLVM_OPT_LEVEL", "NYASH_LLVM_OPT_LEVEL")

fixture = root / "apps/tests/mir_shape_guard/ret_const_min_v1.mir.json"
if not fixture.is_file():
    raise SystemExit(f"missing fixture: {fixture}")

def native_env_pair():
    values = []
    for key in OPT_KEYS:
        value = libc.getenv(key.encode())
        values.append(None if value is None else value.decode())
    return tuple(values)

def call(name, env, out, exact_opt_env=False):
    old = os.environ.copy()
    try:
        if exact_opt_env:
            for key in OPT_KEYS:
                os.environ.pop(key, None)
        os.environ.update(env)
        before = native_env_pair()
        err = ctypes.c_void_p()
        rc = getattr(lib, name)(
            str(fixture).encode(), str(out).encode(), ctypes.byref(err)
        )
        message = ctypes.string_at(err.value).decode(errors="replace") if err.value else ""
        if err.value:
            lib.hako_mem_free(err)
        after = native_env_pair()
        return rc, out.is_file(), message, before, after
    finally:
        os.environ.clear()
        os.environ.update(old)

def run_child_env_checks(temp, out):
    record = pathlib.Path(temp) / "child_env.json"
    if os.name == "nt":
        probe = pathlib.Path(temp) / "child_env_probe.cmd"
        probe.write_text(
            "@echo off\r\n"
            "set \"OUT=\"\r\n"
            ":next\r\n"
            "if \"%~1\"==\"--out\" set \"OUT=%~2\"\r\n"
            "if \"%~1\"==\"\" goto done\r\n"
            "shift\r\n"
            "goto next\r\n"
            ":done\r\n"
            "> \"%HAKO_AOT_CHILD_ENV_RECORD%\" echo {\"hako\":\"%HAKO_LLVM_OPT_LEVEL%\",\"nyash\":\"%NYASH_LLVM_OPT_LEVEL%\"}\r\n"
            "> \"%OUT%\" echo probe\r\n",
            encoding="utf-8",
            newline="",
        )
    else:
        probe = pathlib.Path(temp) / "child_env_probe.py"
        probe.write_text(
            "#!/usr/bin/env python3\n"
            "import json, os, pathlib, sys\n"
            "pathlib.Path(os.environ['HAKO_AOT_CHILD_ENV_RECORD']).write_text(\n"
            "    json.dumps({'hako': os.environ.get('HAKO_LLVM_OPT_LEVEL'),\n"
            "               'nyash': os.environ.get('NYASH_LLVM_OPT_LEVEL')}),\n"
            "    encoding='utf-8')\n"
            "out = pathlib.Path(sys.argv[sys.argv.index('--out') + 1])\n"
            "out.write_bytes(b'probe')\n",
            encoding="utf-8",
        )
        probe.chmod(0o755)
    probe_env = {
        "HAKO_AOT_USE_FFI": "0",
        "HAKO_BACKEND_COMPAT_REPLAY": "none",
        "NYASH_NY_LLVM_COMPILER": str(probe),
        "HAKO_AOT_CHILD_ENV_RECORD": str(record),
    }

    cases = (
        ("both-unset", {}, ("0", "0")),
        ("hako-only", {"HAKO_LLVM_OPT_LEVEL": "3"}, ("3", "0")),
        ("nyash-only", {"NYASH_LLVM_OPT_LEVEL": "1"}, ("0", "1")),
        (
            "both-present",
            {"HAKO_LLVM_OPT_LEVEL": "3", "NYASH_LLVM_OPT_LEVEL": "1"},
            ("3", "1"),
        ),
        (
            "both-empty",
            {"HAKO_LLVM_OPT_LEVEL": "", "NYASH_LLVM_OPT_LEVEL": ""},
            ("", ""),
        ),
    )
    for label, opt_env, expected in cases:
        record.unlink(missing_ok=True)
        rc, exists, message, before, after = call(
            "hako_aot_compile_json_compat_harness",
            {**probe_env, **opt_env},
            out / f"child-env-{label}.o",
            exact_opt_env=True,
        )
        if rc != 0 or not exists:
            raise SystemExit(f"child env case failed: {label}: {message}")
        if after != before:
            raise SystemExit(f"parent opt environment changed: {label}: {before} -> {after}")
        if not record.is_file():
            raise SystemExit(f"child probe was not launched: {label}")
        observed = json.loads(record.read_text(encoding="utf-8"))
        actual = (observed.get("hako"), observed.get("nyash"))
        if actual != expected:
            raise SystemExit(f"child opt environment drift: {label}: {actual} != {expected}")

    record.unlink(missing_ok=True)
    overflow_out = out / "child-env-command-error.o"
    rc, exists, message, before, after = call(
        "hako_aot_compile_json_compat_harness",
        {
            **probe_env,
            "HAKO_LLVM_OPT_LEVEL": "x" * 3000,
            "NYASH_LLVM_OPT_LEVEL": "0",
        },
        overflow_out,
        exact_opt_env=True,
    )
    if (
        rc == 0
        or exists
        or record.exists()
        or "command too long" not in message
        or after != before
    ):
        raise SystemExit("command construction error launched a child or changed the parent")

with tempfile.TemporaryDirectory(prefix="hako-aot-ffi-admission-") as temp:
    out = pathlib.Path(temp)
    run_child_env_checks(temp, out)
    if mode == "child-env":
        print("[llvm-hako-aot-ffi-admission-smoke] ok (child opt env, parent preservation, and no-child command error passed)")
        raise SystemExit(0)
    print("[llvm-hako-aot-ffi-admission-smoke] child-env preflight passed")

    rc, exists, message, _, _ = call(
        "hako_aot_compile_json",
        {"HAKO_AOT_USE_FFI": "0", "HAKO_BACKEND_COMPAT_REPLAY": "harness"},
        out / "generic-direct.o",
    )
    if rc == 0 or exists or "aot-compat-admission-required" not in message:
        raise SystemExit("generic direct AOT inherited harness replay")

    rc, exists, message, _, _ = call(
        "hako_aot_compile_json_compat_harness",
        {"HAKO_AOT_USE_FFI": "0", "HAKO_BACKEND_COMPAT_REPLAY": "none"},
        out / "named-direct.o",
    )
    if rc != 0 or not exists:
        raise SystemExit(f"named direct compatibility lane failed: {message}")

    rc, exists, message, _, _ = call(
        "hako_aot_compile_json",
        {
            "HAKO_AOT_USE_FFI": "1",
            "HAKO_BACKEND_COMPILE_RECIPE": "pure-first",
            "HAKO_BACKEND_COMPAT_REPLAY": "harness",
        },
        out / "generic-ffi-replay.o",
    )
    if rc == 0 or exists or "aot-compat-admission-required" not in message:
        raise SystemExit("generic FFI AOT inherited harness replay")

    rc, exists, message, _, _ = call(
        "hako_aot_compile_json_compat_harness",
        {"HAKO_AOT_USE_FFI": "1", "HAKO_BACKEND_COMPAT_REPLAY": "none"},
        out / "named-ffi.o",
    )
    if rc != 0 or not exists:
        raise SystemExit(f"named FFI compatibility lane failed: {message}")

    rc, exists, message, _, _ = call(
        "hako_llvmc_compile_json",
        {"HAKO_BACKEND_COMPILE_RECIPE": "pure-first", "HAKO_BACKEND_COMPAT_REPLAY": "harness"},
        out / "generic-capi-replay.o",
    )
    if rc == 0 or exists or "generic-capi-compat-admission-required" not in message:
        raise SystemExit("generic C ABI inherited harness replay")

    rc, exists, message, _, _ = call(
        "hako_llvmc_compile_json_compat_harness",
        {"HAKO_BACKEND_COMPAT_REPLAY": "none"},
        out / "named-capi.o",
    )
    if rc != 0 or not exists:
        raise SystemExit(f"named C ABI compatibility lane failed: {message}")

print("[llvm-hako-aot-ffi-admission-smoke] ok (replay gates, child opt env, parent preservation, and no-child command error passed)")
PY
