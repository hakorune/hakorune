"""Checked projection for the selected Dynamic V2 AOT admission.

This is a transport view only.  It validates the symbolic, pre-link metadata
that a future LLVM consumer will borrow at one MIR call site.  It does not
resolve a provider, inspect MIR/AST, emit LLVM, or manufacture an executable
plan.  Missing metadata means that the function is not selected.
"""

from dataclasses import dataclass
from typing import Any, Dict, Optional, Tuple

from builders.a_prime_i64_capability import (
    APrimeI64CapabilityError,
    APrimeI64CapabilityView,
    load_selected_a_prime_capability,
)
from builders.dynamic_v2_text_scan_export_facts import (
    ABI_REVISION,
    CALL_ABI_REVISION,
    CALL_OUT_WIRE_REVISION,
    CONTRACT_ID,
    ENTRY_INDEX_OF,
    ENTRY_SUBSTRING,
    EXPORT_FACTS,
    PROFILE_CODEPOINT_CLAMPED,
    VALUE_HOST_HANDLE,
    VALUE_IMMEDIATE_I64,
    LEASE_END_AUTHORIZED,
    LEASE_NONE,
)


SCHEMA_VERSION = 2
METADATA_KEY = "dynamic_v2_aot_call_admission_v2"
WIRE_REVISION = CALL_OUT_WIRE_REVISION
U64_MAX = (1 << 64) - 1

_ROOT_KEYS = {
    "schema_version",
    "contract_id",
    "profile",
    "abi_revision",
    "wire_revision",
    "registry_generation",
    "plan_stamp",
    "calls",
}
_STAMP_KEYS = {"compiler_domain", "invocation_ordinal"}
_CALL_KEYS = {
    "role",
    "site_id",
    "entry_id",
    "symbol",
    "abi_revision",
    "wire_revision",
    "receiver_lane",
    "argument_lanes",
    "result_lane",
    "lease",
}


class DynamicV2AotAdmissionError(ValueError):
    """Selected metadata is absent, malformed, or inconsistent."""


@dataclass(frozen=True)
class DynamicV2AotCallView:
    role: str
    site_id: int
    entry_id: int
    symbol: str
    abi_revision: int
    wire_revision: int
    receiver_lane: str
    argument_lanes: Tuple[str, ...]
    result_lane: str
    lease: str


@dataclass(frozen=True)
class DynamicV2AotAdmissionView:
    schema_version: int
    contract_id: str
    profile: int
    abi_revision: int
    wire_revision: int
    registry_generation: int
    compiler_domain: int
    invocation_ordinal: int
    calls: Tuple[DynamicV2AotCallView, ...]

    def require_call_site(self, site_id: int) -> DynamicV2AotCallView:
        matches = tuple(
            row
            for row in self.calls
            if row.site_id == site_id
        )
        if len(matches) != 1:
            raise DynamicV2AotAdmissionError(
                f"selected call site is not unique: {site_id}"
            )
        return matches[0]


def _reject_unknown(value: Dict[str, Any], allowed: set[str], label: str) -> None:
    unknown = sorted(set(value) - allowed)
    if unknown:
        raise DynamicV2AotAdmissionError(
            f"{label} contains unknown fields: {', '.join(unknown)}"
        )


def _required_int(value: Any, label: str, *, positive: bool = False) -> int:
    if isinstance(value, bool) or not isinstance(value, int):
        raise DynamicV2AotAdmissionError(f"{label} must be an integer")
    if value < (1 if positive else 0):
        raise DynamicV2AotAdmissionError(f"{label} must be positive")
    return value


def _required_u64(value: Any, label: str, *, positive: bool = False) -> int:
    value = _required_int(value, label, positive=positive)
    if value > U64_MAX:
        raise DynamicV2AotAdmissionError(f"{label} exceeds u64")
    return value


def _required_text(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value:
        raise DynamicV2AotAdmissionError(f"{label} must be non-empty text")
    return value


def _required_texts(value: Any, label: str) -> Tuple[str, ...]:
    if not isinstance(value, list) or any(not isinstance(item, str) or not item for item in value):
        raise DynamicV2AotAdmissionError(f"{label} must be a text array")
    return tuple(value)


def _metadata(func_data: Dict[str, Any]) -> Dict[str, Any]:
    metadata = func_data.get("metadata", {}) if isinstance(func_data, dict) else {}
    if metadata is None:
        return {}
    if not isinstance(metadata, dict):
        raise DynamicV2AotAdmissionError("metadata must be an object")
    return metadata


def _require_a_prime(func_data: Dict[str, Any]) -> APrimeI64CapabilityView:
    try:
        receipt = load_selected_a_prime_capability(func_data)
    except APrimeI64CapabilityError as error:
        raise DynamicV2AotAdmissionError(str(error)) from error
    if receipt is None:
        raise DynamicV2AotAdmissionError("selected admission requires A-prime receipt")
    return receipt


def _validate_call(
    raw: Any,
    receipt: APrimeI64CapabilityView,
) -> DynamicV2AotCallView:
    if not isinstance(raw, dict):
        raise DynamicV2AotAdmissionError("selected call rows must be objects")
    _reject_unknown(raw, _CALL_KEYS, "selected call")
    role = _required_text(raw.get("role"), "call.role")
    expected = {
        "substring": {
            "entry_id": ENTRY_SUBSTRING,
        },
        "index_of": {
            "entry_id": ENTRY_INDEX_OF,
        },
    }.get(role)
    if expected is None:
        raise DynamicV2AotAdmissionError(f"unknown selected call role: {role}")

    site_id = _required_int(raw.get("site_id"), "call.site_id")
    expected_role_by_site = {0: "substring", 1: "index_of"}
    if site_id not in expected_role_by_site:
        raise DynamicV2AotAdmissionError("selected site_id is outside the canonical pair")
    if role != expected_role_by_site[site_id]:
        raise DynamicV2AotAdmissionError("selected role does not match canonical site")
    try:
        receipt_call = receipt.require_call_site(site_id)
    except APrimeI64CapabilityError as error:
        raise DynamicV2AotAdmissionError(str(error)) from error
    if receipt_call.role != role:
        raise DynamicV2AotAdmissionError("selected role does not match A-prime site")

    entry_id = _required_int(raw.get("entry_id"), "call.entry_id", positive=True)
    fact = next((fact for fact in EXPORT_FACTS if fact["entry"] == entry_id), None)
    if fact is None:
        raise DynamicV2AotAdmissionError("unknown symbolic entry ID")
    symbol = _required_text(raw.get("symbol"), "call.symbol")
    abi_revision = _required_int(raw.get("abi_revision"), "call.abi_revision", positive=True)
    wire_revision = _required_int(raw.get("wire_revision"), "call.wire_revision", positive=True)
    receiver_lane = _required_text(raw.get("receiver_lane"), "call.receiver_lane")
    argument_lanes = _required_texts(raw.get("argument_lanes"), "call.argument_lanes")
    result_lane = _required_text(raw.get("result_lane"), "call.result_lane")
    lease = _required_text(raw.get("lease"), "call.lease")
    lane_names = {VALUE_HOST_HANDLE: "opaque_handle", VALUE_IMMEDIATE_I64: "immediate_i64"}
    lease_names = {LEASE_NONE: "none", LEASE_END_AUTHORIZED: "end_authorized"}
    expected_from_fact = {
        "symbol": fact["symbol"],
        "receiver_lane": lane_names[fact["receiver_lane"]],
        "argument_lanes": tuple(lane_names[lane] for lane in fact["argument_lanes"]),
        "result_lane": lane_names[fact["result_lane"]],
        "lease": lease_names[fact["lease"]],
    }

    actual = {
        "entry_id": entry_id,
        "symbol": symbol,
        "receiver_lane": receiver_lane,
        "argument_lanes": argument_lanes,
        "result_lane": result_lane,
        "lease": lease,
    }
    if entry_id != expected["entry_id"]:
        raise DynamicV2AotAdmissionError(f"{role} entry mismatch")
    for key, expected_value in expected_from_fact.items():
        if actual[key] != expected_value:
            raise DynamicV2AotAdmissionError(f"{role} admission mismatch: {key}")
    if abi_revision != ABI_REVISION or wire_revision != WIRE_REVISION:
        raise DynamicV2AotAdmissionError(f"{role} ABI/wire revision mismatch")
    if fact["call_abi"]["abi_revision"] != CALL_ABI_REVISION:
        raise DynamicV2AotAdmissionError(f"{role} call ABI revision mismatch")
    if (
        fact["call_abi"]["out_wire_revision"] != wire_revision
        or fact["call_abi"]["logical_arity"] != len(argument_lanes)
    ):
        raise DynamicV2AotAdmissionError(f"{role} call ABI shape mismatch")
    if receipt_call.receiver_lane != receiver_lane:
        raise DynamicV2AotAdmissionError(f"{role} receiver lane mismatch")
    expected_arg_lanes = tuple(arg.lane for arg in receipt_call.arguments)
    if expected_arg_lanes != argument_lanes or receipt_call.result_lane != result_lane:
        raise DynamicV2AotAdmissionError(f"{role} A-prime lane mismatch")
    return DynamicV2AotCallView(
        role=role,
        site_id=site_id,
        entry_id=entry_id,
        symbol=symbol,
        abi_revision=abi_revision,
        wire_revision=wire_revision,
        receiver_lane=receiver_lane,
        argument_lanes=argument_lanes,
        result_lane=result_lane,
        lease=lease,
    )


def load_selected_dynamic_v2_aot_admission(
    func_data: Dict[str, Any],
) -> Optional[DynamicV2AotAdmissionView]:
    """Load the optional selected metadata without changing lowering routes."""

    raw = _metadata(func_data).get(METADATA_KEY)
    if raw is None:
        return None
    if not isinstance(raw, dict):
        raise DynamicV2AotAdmissionError(f"{METADATA_KEY} must be an object")
    _reject_unknown(raw, _ROOT_KEYS, METADATA_KEY)
    if _required_int(raw.get("schema_version"), "schema_version") != SCHEMA_VERSION:
        raise DynamicV2AotAdmissionError("unsupported metadata schema")
    if _required_text(raw.get("contract_id"), "contract_id") != CONTRACT_ID:
        raise DynamicV2AotAdmissionError("contract mismatch")
    if _required_int(raw.get("profile"), "profile", positive=True) != PROFILE_CODEPOINT_CLAMPED:
        raise DynamicV2AotAdmissionError("profile mismatch")
    if _required_int(raw.get("abi_revision"), "abi_revision", positive=True) != ABI_REVISION:
        raise DynamicV2AotAdmissionError("ABI revision mismatch")
    if _required_int(raw.get("wire_revision"), "wire_revision", positive=True) != WIRE_REVISION:
        raise DynamicV2AotAdmissionError("wire revision mismatch")
    generation = _required_u64(raw.get("registry_generation"), "registry_generation", positive=True)
    stamp = raw.get("plan_stamp")
    if not isinstance(stamp, dict):
        raise DynamicV2AotAdmissionError("plan_stamp must be an object")
    _reject_unknown(stamp, _STAMP_KEYS, "plan_stamp")
    compiler_domain = _required_u64(stamp.get("compiler_domain"), "compiler_domain", positive=True)
    invocation_ordinal = _required_u64(stamp.get("invocation_ordinal"), "invocation_ordinal", positive=True)
    calls = raw.get("calls")
    if not isinstance(calls, list) or len(calls) != 2:
        raise DynamicV2AotAdmissionError("selected admission requires exactly two calls")
    receipt = _require_a_prime(func_data)
    parsed = tuple(_validate_call(row, receipt) for row in calls)
    if {row.role for row in parsed} != {"substring", "index_of"}:
        raise DynamicV2AotAdmissionError("selected calls must cover substring/index_of")
    if len({row.site_id for row in parsed}) != len(parsed):
        raise DynamicV2AotAdmissionError("selected call sites must be unique")
    if len({row.entry_id for row in parsed}) != len(parsed):
        raise DynamicV2AotAdmissionError("selected entry IDs must be unique")
    return DynamicV2AotAdmissionView(
        schema_version=SCHEMA_VERSION,
        contract_id=CONTRACT_ID,
        profile=PROFILE_CODEPOINT_CLAMPED,
        abi_revision=ABI_REVISION,
        wire_revision=WIRE_REVISION,
        registry_generation=generation,
        compiler_domain=compiler_domain,
        invocation_ordinal=invocation_ordinal,
        calls=parsed,
    )
