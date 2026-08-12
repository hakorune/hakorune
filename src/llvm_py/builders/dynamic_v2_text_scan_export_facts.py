"""Checked Python projection of the neutral TextScan AOT export facts.

This module contains no provider lookup, selector dispatch, runtime address, or
VM route.  It is used by parity checks until a later AOT admission consumes the
symbolic entries.
"""

CONTRACT_ID = "hako.text.scan@1"
ABI_REVISION = 1
PROFILE_CODEPOINT_CLAMPED = 1
SUSPENSION_NON_SUSPENDING = 0

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
    },
    {
        "entry": ENTRY_INDEX_OF,
        "symbol": "hako.text.scan.index_of.v1",
        "arity": 1,
        "receiver_lane": VALUE_HOST_HANDLE,
        "argument_lanes": (VALUE_HOST_HANDLE,),
        "result_lane": VALUE_IMMEDIATE_I64,
        "lease": LEASE_NONE,
    },
)
