"""Checked Python projection of the neutral TextScan AOT export facts.

This module contains no provider lookup, selector dispatch, runtime address, or
VM route.  It is used by parity checks until a later AOT admission consumes the
symbolic entries.
"""

CONTRACT_ID = "hako.text.scan@1"
ABI_REVISION = 1
PROFILE_CODEPOINT_CLAMPED = 1
SUSPENSION_NON_SUSPENDING = 0
CALL_ABI_REVISION = 1
CALL_OK = 0
CALL_INVALID_OUTPUT = 1
CALL_OUT_WIRE_REVISION = 2
CALL_TRANSPORT_RETURN = "u32"
CALL_OUT_PARAMETER = "HakoDynamicV2CallOutV1*"
PARAM_TYPE_U64 = 1
PARAM_TYPE_I64 = 2
PARAM_TYPE_OUT_POINTER = 3

ENTRY_SUBSTRING = 1
ENTRY_INDEX_OF = 2

VALUE_HOST_HANDLE = 1
VALUE_IMMEDIATE_I64 = 2

LEASE_NONE = 0
LEASE_END_AUTHORIZED = 1

EXPORT_FACTS = (
    {
        "entry": ENTRY_SUBSTRING,
        "symbol": "hako.text.scan.substring.v1",
        "arity": 2,
        "receiver_lane": VALUE_HOST_HANDLE,
        "argument_lanes": (VALUE_IMMEDIATE_I64, VALUE_IMMEDIATE_I64),
        "result_lane": VALUE_HOST_HANDLE,
        "lease": LEASE_END_AUTHORIZED,
        "call_abi": {
            "logical_arity": 2,
            "abi_revision": CALL_ABI_REVISION,
            "out_wire_revision": CALL_OUT_WIRE_REVISION,
            "transport_return": CALL_TRANSPORT_RETURN,
            "out_parameter": CALL_OUT_PARAMETER,
            "parameter_types": (
                PARAM_TYPE_U64,
                PARAM_TYPE_I64,
                PARAM_TYPE_I64,
                PARAM_TYPE_OUT_POINTER,
            ),
        },
    },
    {
        "entry": ENTRY_INDEX_OF,
        "symbol": "hako.text.scan.index_of.v1",
        "arity": 1,
        "receiver_lane": VALUE_HOST_HANDLE,
        "argument_lanes": (VALUE_HOST_HANDLE,),
        "result_lane": VALUE_IMMEDIATE_I64,
        "lease": LEASE_NONE,
        "call_abi": {
            "logical_arity": 1,
            "abi_revision": CALL_ABI_REVISION,
            "out_wire_revision": CALL_OUT_WIRE_REVISION,
            "transport_return": CALL_TRANSPORT_RETURN,
            "out_parameter": CALL_OUT_PARAMETER,
            "parameter_types": (
                PARAM_TYPE_U64,
                PARAM_TYPE_U64,
                PARAM_TYPE_OUT_POINTER,
            ),
        },
    },
)
