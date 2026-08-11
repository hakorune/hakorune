"""Transport-only projection of the C-owned Dynamic CallSlot wire revision 2.

This module intentionally has no runtime dispatch or provider lookup.  It is
used by schema/parity tests until a backend capability owns a live descriptor.
"""

import ctypes


WIRE_REVISION = 2
FORWARDED_NONE = 0xFFFFFFFF

TAG_INVALID = 0
TAG_HOST_HANDLE = 1
TAG_IMMEDIATE_I64 = 2

STATUS_NORMAL = 0
STATUS_FAULT = 1
STATUS_SUSPENDED = 2

DISPOSITION_NONE = 0
DISPOSITION_FORWARDED = 1
DISPOSITION_END_AUTHORIZED = 2

FAULT_NONE = 0
FAULT_INVALID_RECEIVER = 1
FAULT_INVALID_HANDLE = 2
FAULT_ARITY = 3
FAULT_UNSUPPORTED_SLOT = 4
FAULT_TYPE_MISMATCH = 5
FAULT_RANGE = 6
FAULT_RUNTIME = 7
FAULT_INVALID_RESULT = 8

U32_MAX = (1 << 32) - 1
U64_MAX = (1 << 64) - 1


class DynamicV2WireValueV1(ctypes.Structure):
    _fields_ = [
        ("tag", ctypes.c_uint32),
        ("reserved", ctypes.c_uint32),
        ("payload", ctypes.c_uint64),
    ]


class DynamicV2CallOutV1(ctypes.Structure):
    _fields_ = [
        ("status", ctypes.c_uint32),
        ("fault_code", ctypes.c_uint32),
        ("result_tag", ctypes.c_uint32),
        ("disposition", ctypes.c_uint32),
        ("forwarded_input", ctypes.c_uint32),
        ("reserved", ctypes.c_uint32),
        ("value_payload", ctypes.c_uint64),
        ("lease_token", ctypes.c_uint64),
        ("continuation_token", ctypes.c_uint64),
    ]


def _require_keys(out):
    expected = {
        "status",
        "fault_code",
        "result_tag",
        "disposition",
        "forwarded_input",
        "reserved",
        "value_payload",
        "lease_token",
        "continuation_token",
    }
    if set(out) != expected:
        raise ValueError("DynamicV2CallOut fields are not exact")


def _require_uint(name, value, maximum):
    if type(value) is not int or not 0 <= value <= maximum:
        raise ValueError(f"{name} must be an unsigned fixed-width integer")


def validate_wire_value(tag, reserved, payload):
    _require_uint("tag", tag, U32_MAX)
    _require_uint("reserved", reserved, U32_MAX)
    _require_uint("payload", payload, U64_MAX)
    if reserved != 0:
        raise ValueError("wire reserved bits must be zero")
    if tag not in {TAG_INVALID, TAG_HOST_HANDLE, TAG_IMMEDIATE_I64}:
        raise ValueError("unknown wire tag")
    if tag == TAG_INVALID and payload != 0:
        raise ValueError("invalid tag cannot carry payload")


def validate_call_out(out, *, argc=None, allow_suspended=True):
    """Validate transport combinations; this does not select a provider."""

    _require_keys(out)
    if argc is not None:
        _require_uint("argc", argc, U32_MAX - 1)
    for key in (
        "status",
        "fault_code",
        "result_tag",
        "disposition",
        "forwarded_input",
        "reserved",
    ):
        _require_uint(key, out[key], U32_MAX)
    for key in ("value_payload", "lease_token", "continuation_token"):
        _require_uint(key, out[key], U64_MAX)
    if out["reserved"] != 0:
        raise ValueError("call-out reserved bits must be zero")
    if out["status"] not in {STATUS_NORMAL, STATUS_FAULT, STATUS_SUSPENDED}:
        raise ValueError("unknown call status")
    if out["fault_code"] not in range(FAULT_INVALID_RESULT + 1):
        raise ValueError("unknown fault code")
    if out["result_tag"] not in {TAG_INVALID, TAG_HOST_HANDLE, TAG_IMMEDIATE_I64}:
        raise ValueError("unknown result tag")
    if out["disposition"] not in {
        DISPOSITION_NONE,
        DISPOSITION_FORWARDED,
        DISPOSITION_END_AUTHORIZED,
    }:
        raise ValueError("unknown call disposition")

    status = out["status"]
    if status == STATUS_NORMAL:
        if (
            out["fault_code"] != FAULT_NONE
            or out["result_tag"] == TAG_INVALID
            or out["continuation_token"] != 0
        ):
            raise ValueError("invalid normal call outcome")
        if out["disposition"] == DISPOSITION_NONE:
            if (
                out["result_tag"] != TAG_IMMEDIATE_I64
                or out["forwarded_input"] != FORWARDED_NONE
                or out["lease_token"] != 0
            ):
                raise ValueError("invalid immediate-i64 normal outcome")
        elif out["disposition"] == DISPOSITION_FORWARDED:
            if out["result_tag"] != TAG_HOST_HANDLE:
                raise ValueError("forwarded outcome requires a host handle")
            if out["forwarded_input"] == FORWARDED_NONE or out["lease_token"] != 0:
                raise ValueError("invalid forwarded outcome")
        else:
            if out["result_tag"] != TAG_HOST_HANDLE:
                raise ValueError("end-authorized outcome requires a host handle")
            if out["forwarded_input"] != FORWARDED_NONE or out["lease_token"] == 0:
                raise ValueError("invalid end-authorized outcome")
        if (
            argc is not None
            and out["forwarded_input"] != FORWARDED_NONE
            and out["forwarded_input"] > argc
        ):
            raise ValueError("forwarded lane is outside call arity")
    elif status == STATUS_FAULT:
        if (
            out["fault_code"] == FAULT_NONE
            or out["result_tag"] != TAG_INVALID
            or out["value_payload"] != 0
            or out["disposition"] != DISPOSITION_NONE
            or out["forwarded_input"] != FORWARDED_NONE
            or out["lease_token"] != 0
            or out["continuation_token"] != 0
        ):
            raise ValueError("invalid fault outcome")
    else:
        if not allow_suspended:
            raise ValueError("suspended outcome is unsupported here")
        if (
            out["fault_code"] != FAULT_NONE
            or out["result_tag"] != TAG_INVALID
            or out["value_payload"] != 0
            or out["disposition"] != DISPOSITION_NONE
            or out["forwarded_input"] != FORWARDED_NONE
            or out["lease_token"] != 0
            or out["continuation_token"] == 0
        ):
            raise ValueError("invalid suspended outcome")
