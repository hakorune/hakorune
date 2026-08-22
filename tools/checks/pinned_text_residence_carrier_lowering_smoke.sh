#!/usr/bin/env bash
# Caller-zero C lowering fixture for the strict pinned-Text Residence carrier.
# This intentionally stops at textual LLVM; it is not a production/object path.
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
lower = library.hako_llvmc_emit_pinned_text_residence_carrier_fixture
lower.argtypes = [ctypes.c_char_p, ctypes.c_char_p, ctypes.POINTER(ctypes.c_void_p)]
lower.restype = ctypes.c_int
library.hako_mem_free.argtypes = [ctypes.c_void_p]

source = root / "apps/tests/phase29z_vm_hako_s3_nop_const_add_return_mir_v0.json"
base = json.loads(source.read_text())
base["functions"] = base["functions"][:1]
contract = {
    "contract_id": "hako.pinned_text_backend_frame@2",
    "schema_revision": 2,
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
        "consumer_abi_revision": "hako-llvmc-pure-first-v2",
        "object_emitter": {
            "llvm_c_api_abi_revision": "llvm-c-api-18-v1",
            "cpu": "",
            "features": "",
            "codegen_opt_level": 3,
            "relocation_model": 0,
            "code_model": 0,
        },
    },
}
carrier = {
    "contract_id": "hako.pinned_text_residence_carrier@1",
    "schema_revision": 1,
    "frame_contract": {
        "contract_id": "hako.pinned_text_backend_frame@2",
        "schema_revision": 2,
    },
    "owner": {"compilation_brand": 1, "slot": 1},
    "invocation_ordinal": 1,
    "target": {
        "profile_id": "nyrt-text-residence-ptr64-as0-v1",
        "triple": "x86_64-pc-linux-gnu",
        "data_layout": contract["target"]["data_layout"],
    },
    "residence_abi_revision": "text-formal-residence-v1",
    "plan_stamp": 1,
    "enter": {"source": 1, "normal": 2, "trap": 3},
    "finish_obligation": "finish_every_explicit_normal_return",
    "normal_exit_count": 1,
    "finish_sites": [4],
    "roots": [
        {
            "frame_row": 0,
            "logical_ordinal": 0,
            "source_binding": {
                "owner": {"compilation_brand": 1, "slot": 1},
                "binding_id": 11,
            },
            "slot_lane": 1,
            "generation_lane": 2,
        },
        {
            "frame_row": 1,
            "logical_ordinal": 1,
            "source_binding": {
                "owner": {"compilation_brand": 1, "slot": 1},
                "binding_id": 12,
            },
            "slot_lane": 3,
            "generation_lane": 4,
        },
    ],
}


def payload(frame=contract, lifecycle=carrier):
    value = copy.deepcopy(base)
    value["functions"][0]["metadata"] = {
        "pinned_text_backend_frame_v1": copy.deepcopy(frame),
        "pinned_text_residence_carrier_v1": copy.deepcopy(lifecycle),
    }
    return value


def invoke(directory, name, value):
    json_path = directory / (name + ".json")
    ll_path = directory / (name + ".ll")
    json_path.write_text(json.dumps(value, separators=(",", ":")))
    error = ctypes.c_void_p()
    rc = lower(str(json_path).encode(), str(ll_path).encode(), ctypes.byref(error))
    message = ctypes.string_at(error.value).decode(errors="replace") if error.value else ""
    if error.value:
        library.hako_mem_free(error)
    return rc, message, ll_path


with tempfile.TemporaryDirectory(prefix="hako-pinned-text-carrier-lowering-") as raw:
    directory = pathlib.Path(raw)
    rc, message, ll_path = invoke(directory, "valid", payload())
    if rc != 0 or not ll_path.is_file():
        raise SystemExit(f"valid carrier fixture rejected: {message}")
    ir = ll_path.read_text()
    required = [
        "declare i32 @hako_text_formal_residence_enter_v1",
        "declare void @hako_text_formal_residence_finish_or_abort_v1",
        "call i32 @hako_text_formal_residence_enter_v1",
        "icmp eq i32 %status, 0",
        "br i1 %ok, label %normal0, label %trap",
        "trap:\n  unreachable",
        "call void @hako_text_formal_residence_finish_or_abort_v1",
        "ret i64 0",
    ]
    for marker in required:
        if marker not in ir:
            raise SystemExit(f"fixture is missing required LLVM shape: {marker!r}")
    if ir.count("call void @hako_text_formal_residence_finish_or_abort_v1") != 1:
        raise SystemExit("fixture did not place exactly one success-only Finish")
    if "nyash.string.eq_hh" in ir or "fallback" in ir.lower() or "invoke" in ir:
        raise SystemExit("fixture contains a legacy/fallback/callable route")

    missing = payload()
    del missing["functions"][0]["metadata"]["pinned_text_residence_carrier_v1"]
    rc, message, _ = invoke(directory, "missing", missing)
    if rc == 0 or "carrier" not in message:
        raise SystemExit(f"missing carrier was accepted: {message}")

    drifted = payload()
    drifted["functions"][0]["metadata"]["pinned_text_residence_carrier_v1"]["plan_stamp"] = 2
    rc, message, _ = invoke(directory, "drifted", drifted)
    if rc == 0 or "invocation mismatch" not in message:
        raise SystemExit(f"plan drift was accepted: {message}")

    trap_finish = payload()
    trap_finish["functions"][0]["metadata"]["pinned_text_residence_carrier_v1"]["finish_sites"] = [3]
    rc, message, _ = invoke(directory, "trap-finish", trap_finish)
    if rc == 0 or "trap Finish" not in message:
        raise SystemExit(f"trap Finish was accepted: {message}")

print("[pinned-text-residence-carrier-lowering-smoke] ok (strict carrier -> textual LLVM; missing/drift/trap rejected)")
PY
