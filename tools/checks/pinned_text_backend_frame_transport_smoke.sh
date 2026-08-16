#!/usr/bin/env bash
# Reusable transport proof for the Rust-owned pinned-Text backend-frame row.
# The pure-first C consumer must accept the exact projection, reject unknown or
# malformed fields, and fail before generic IR lowering on layout drift.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"

bash "$ROOT/tools/build_hako_llvmc_ffi.sh" >/dev/null

ROOT="$ROOT" python3 - <<'PY'
import copy
import ctypes
import json
import os
import pathlib
import tempfile

root = pathlib.Path(os.environ["ROOT"])
library = ctypes.CDLL(str(root / "target/release/libhako_llvmc_ffi.so"))
compile_fn = library.hako_llvmc_compile_json_pure_first
compile_fn.argtypes = [
    ctypes.c_char_p,
    ctypes.c_char_p,
    ctypes.POINTER(ctypes.c_void_p),
]
compile_fn.restype = ctypes.c_int
library.hako_mem_free.argtypes = [ctypes.c_void_p]

fixture = root / "apps/tests/phase29z_vm_hako_s3_nop_const_add_return_mir_v0.json"
if not fixture.is_file():
    raise SystemExit(f"missing fixture: {fixture}")

base = json.loads(fixture.read_text())
contract = {
    "contract_id": "hako.pinned_text_backend_frame@1",
    "schema_revision": 1,
    "owner": {"compilation_brand": 1, "slot": 1},
    "invocation_ordinal": 1,
    "source_logical_arity": 2,
    "receiver_lane_count": 1,
    "physical_formal_lane_count": 4,
    "physical_callable_lane_count": 5,
    "exact_text_root_count": 2,
    "plan_stamp": 1,
    "plan_count": 1,
    "residence": {
        "abi_revision": "text-formal-residence-v1",
        "frame_revision": 1,
        "header_size": 32,
        "root_row_size": 16,
        "header_alignment": 8,
        "root_row_alignment": 8,
        "max_root_count": 1024,
        "max_frame_bytes": 65536,
        "derived_frame_size": 64,
    },
    "target": {
        "profile_id": "nyrt-text-residence-ptr64-as0-v1",
        "triple": "x86_64-pc-linux-gnu",
        "data_layout": (
            "e-m:e-p270:32:32-p271:32:32-p272:64:64-"
            "i64:64-i128:128-f80:128-n8:16:32:64-S128"
        ),
        "little_endian": True,
        "address_space_zero_pointer_width": 64,
        "address_space_zero_abi_alignment": 8,
        "max_root_count": 1024,
        "max_private_frame_bytes": 65536,
        "residence_abi_revision": "text-formal-residence-v1",
        "consumer_abi_revision": "hako-llvmc-pure-first-v1",
    },
}


def payload_with(contract_value):
    payload = copy.deepcopy(base)
    payload["functions"][0]["metadata"] = {
        "pinned_text_backend_frame_v1": contract_value,
    }
    return payload


def invoke(payload, path, output):
    path.write_text(json.dumps(payload, separators=(",", ":")))
    error = ctypes.c_void_p()
    old_env = os.environ.copy()
    os.environ["HAKO_BACKEND_COMPAT_REPLAY"] = "none"
    try:
        rc = compile_fn(str(path).encode(), str(output).encode(), ctypes.byref(error))
    finally:
        os.environ.clear()
        os.environ.update(old_env)
    message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
    if error.value:
        library.hako_mem_free(error)
    return rc, message


with tempfile.TemporaryDirectory(prefix="hako-pinned-text-frame-") as directory:
    directory = pathlib.Path(directory)

    rc, message = invoke(
        payload_with(copy.deepcopy(contract)),
        directory / "valid.json",
        directory / "valid.o",
    )
    if rc == 0 or "unsupported pure shape" not in message:
        raise SystemExit(f"valid contract did not reach the generic shape gate: {message}")

    bad_layout = copy.deepcopy(contract)
    bad_layout["target"]["data_layout"] = "bad"
    rc, message = invoke(
        payload_with(bad_layout),
        directory / "bad-layout.json",
        directory / "bad-layout.o",
    )
    if rc == 0 or "target layout mismatch" not in message:
        raise SystemExit(f"layout drift was not rejected before lowering: {message}")

    unknown = copy.deepcopy(contract)
    unknown["unexpected"] = 1
    rc, message = invoke(
        payload_with(unknown),
        directory / "unknown.json",
        directory / "unknown.o",
    )
    if rc == 0 or "invalid pinned Text backend-frame contract header" not in message:
        raise SystemExit(f"unknown transport field was not rejected: {message}")

    missing_target = copy.deepcopy(contract)
    del missing_target["target"]
    rc, message = invoke(
        payload_with(missing_target),
        directory / "missing-target.json",
        directory / "missing-target.o",
    )
    if rc == 0 or "target layout mismatch" not in message:
        raise SystemExit(f"missing target capability was not rejected: {message}")

print("[pinned-text-backend-frame-transport-smoke] ok (strict projection accepted; drift/unknown/missing rejected)")
PY
