"""Strict loader for the sealed A-prime LLVM physical capability.

The field is a post-session receipt.  Its absence intentionally leaves all
legacy lowering behavior unchanged.  This module validates only the explicit
transport schema; it never resolves values, infers types, or performs casts.
"""

from dataclasses import dataclass
from typing import Any, Dict, Optional, Sequence, Tuple


SCHEMA_VERSION = 2
CAPABILITY_KEY = "a_prime_i64_physical_receipt"
FORMAL_PARAMETER_COUNT = 4

_RECEIPT_KEYS = {
    "schema_version",
    "backend_family",
    "formal_parameter_count",
    "fallback",
    "retry",
    "parameters",
    "call_edges",
    "returns",
}
_PARAMETER_KEYS = {"role", "formal_parameter_index", "value_id", "lane"}
_CALL_KEYS = {
    "site_id",
    "role",
    "target_fingerprint",
    "receiver_role",
    "receiver_value_id",
    "receiver_lane",
    "arguments",
    "result_value_id",
    "result_lane",
}
_ARGUMENT_KEYS = {"ordinal", "role", "value_id", "lane"}
_RETURN_KEYS = {"site", "block", "value_id", "lane"}


class APrimeI64CapabilityError(ValueError):
    """Malformed, incomplete, or mismatched A-prime receipt."""


@dataclass(frozen=True)
class APrimeI64ParameterRow:
    role: str
    formal_parameter_index: int
    value_id: int
    lane: str


@dataclass(frozen=True)
class APrimeI64ArgumentRow:
    ordinal: int
    role: str
    value_id: int
    lane: str


@dataclass(frozen=True)
class APrimeI64CallEdgeRow:
    site_id: int
    role: str
    target_fingerprint: str
    receiver_role: str
    receiver_value_id: int
    receiver_lane: str
    arguments: Tuple[APrimeI64ArgumentRow, ...]
    result_value_id: int
    result_lane: str


@dataclass(frozen=True)
class APrimeI64ReturnRow:
    site: str
    block: int
    value_id: int
    lane: str


@dataclass(frozen=True)
class APrimeI64CapabilityView:
    schema_version: int
    backend_family: str
    parameters: Tuple[APrimeI64ParameterRow, ...]
    call_edges: Tuple[APrimeI64CallEdgeRow, ...]
    returns: Tuple[APrimeI64ReturnRow, ...]

    def require_parameter(self, role: str) -> APrimeI64ParameterRow:
        rows = [row for row in self.parameters if row.role == role]
        if len(rows) != 1:
            raise APrimeI64CapabilityError(f"parameter role is not unique: {role}")
        return rows[0]

    def require_call_site(self, site_id: int) -> APrimeI64CallEdgeRow:
        rows = [
            row
            for row in self.call_edges
            if row.site_id == site_id
        ]
        if len(rows) != 1:
            raise APrimeI64CapabilityError(
                f"call site is not unique: {site_id}"
            )
        return rows[0]

    def require_return(self, site: str) -> APrimeI64ReturnRow:
        rows = [row for row in self.returns if row.site == site]
        if len(rows) != 1:
            raise APrimeI64CapabilityError(f"return site is not unique: {site}")
        return rows[0]


def _metadata(func_data: Dict[str, Any]) -> Dict[str, Any]:
    metadata = func_data.get("metadata", {}) if isinstance(func_data, dict) else {}
    if metadata is None:
        return {}
    if not isinstance(metadata, dict):
        raise APrimeI64CapabilityError("metadata must be an object")
    return metadata


def _required_int(value: Any, label: str) -> int:
    # bool is an int subclass but is never a valid MIR identifier/index.
    if isinstance(value, bool) or not isinstance(value, int) or value < 0:
        raise APrimeI64CapabilityError(f"{label} must be a non-negative integer")
    return value


def _required_string(value: Any, label: str) -> str:
    if not isinstance(value, str) or not value:
        raise APrimeI64CapabilityError(f"{label} must be a non-empty string")
    return value


def _required_rows(value: Any, label: str) -> Sequence[Dict[str, Any]]:
    if not isinstance(value, list) or not value:
        raise APrimeI64CapabilityError(f"{label} must be a non-empty array")
    if any(not isinstance(row, dict) for row in value):
        raise APrimeI64CapabilityError(f"{label} rows must be objects")
    return value


def _reject_unknown_keys(value: Dict[str, Any], allowed: set[str], label: str) -> None:
    unknown = sorted(set(value) - allowed)
    if unknown:
        raise APrimeI64CapabilityError(
            f"{label} contains unknown fields: {', '.join(unknown)}"
        )


def _validate_func_params(func_data: Dict[str, Any], rows: Sequence[APrimeI64ParameterRow]) -> None:
    params = func_data.get("params")
    if not isinstance(params, list) or len(params) != FORMAL_PARAMETER_COUNT:
        raise APrimeI64CapabilityError(
            "selected A-prime function requires exactly four formal params"
        )
    for index, value in enumerate(params):
        _required_int(value, f"params[{index}]")
    if len(set(params)) != len(params):
        raise APrimeI64CapabilityError("formal parameter ValueIds must be unique")
    if len(params) <= max(row.formal_parameter_index for row in rows):
        raise APrimeI64CapabilityError("parameter contract index is outside params")
    for row in rows:
        if params[row.formal_parameter_index] != row.value_id:
            raise APrimeI64CapabilityError(
                f"parameter ValueId mismatch at index {row.formal_parameter_index}"
            )


def load_selected_a_prime_capability(
    func_data: Dict[str, Any],
) -> Optional[APrimeI64CapabilityView]:
    """Load and strictly validate the optional selected receipt.

    ``None`` means the function is not selected for the A-prime lane; callers
    must preserve legacy behavior in that case.
    """

    metadata = _metadata(func_data)
    raw = metadata.get(CAPABILITY_KEY)
    if raw is None:
        return None
    if not isinstance(raw, dict):
        raise APrimeI64CapabilityError(f"{CAPABILITY_KEY} must be an object")
    _reject_unknown_keys(raw, _RECEIPT_KEYS, CAPABILITY_KEY)

    schema_version = _required_int(raw.get("schema_version"), "schema_version")
    if schema_version != SCHEMA_VERSION:
        raise APrimeI64CapabilityError(f"unsupported schema_version: {schema_version}")
    backend_family = _required_string(raw.get("backend_family"), "backend_family")
    if backend_family != "llvm":
        raise APrimeI64CapabilityError(f"unsupported backend_family: {backend_family}")
    formal_parameter_count = _required_int(
        raw.get("formal_parameter_count"), "formal_parameter_count"
    )
    if formal_parameter_count != FORMAL_PARAMETER_COUNT:
        raise APrimeI64CapabilityError("A-prime receipt requires four formal params")
    if (
        "fallback" not in raw
        or type(raw["fallback"]) is not bool
        or raw["fallback"] is not False
        or "retry" not in raw
        or type(raw["retry"]) is not bool
        or raw["retry"] is not False
    ):
        raise APrimeI64CapabilityError("fallback/retry are forbidden in A-prime receipt")

    parameter_rows = _required_rows(raw.get("parameters"), "parameters")
    if len(parameter_rows) != 2:
        raise APrimeI64CapabilityError("A-prime receipt requires exactly two parameters")
    for row in parameter_rows:
        _reject_unknown_keys(row, _PARAMETER_KEYS, "parameter")
    parameters = tuple(
        APrimeI64ParameterRow(
            role=_required_string(row.get("role"), "parameter.role"),
            formal_parameter_index=_required_int(
                row.get("formal_parameter_index"), "parameter.formal_parameter_index"
            ),
            value_id=_required_int(row.get("value_id"), "parameter.value_id"),
            lane=_required_string(row.get("lane"), "parameter.lane"),
        )
        for row in parameter_rows
    )
    if {row.role for row in parameters} != {"pos", "end"}:
        raise APrimeI64CapabilityError("parameter roles must be exactly pos/end")
    expected_indices = {"pos": 1, "end": 2}
    for row in parameters:
        if row.formal_parameter_index != expected_indices[row.role]:
            raise APrimeI64CapabilityError(
                f"{row.role} must use formal parameter index "
                f"{expected_indices[row.role]}"
            )
    if len({row.formal_parameter_index for row in parameters}) != len(parameters):
        raise APrimeI64CapabilityError("duplicate parameter index")
    if len({row.value_id for row in parameters}) != len(parameters):
        raise APrimeI64CapabilityError("duplicate parameter ValueId")
    if any(row.lane != "immediate_i64" for row in parameters):
        raise APrimeI64CapabilityError("parameters must use immediate_i64")
    _validate_func_params(func_data, parameters)

    call_rows = _required_rows(raw.get("call_edges"), "call_edges")
    if len(call_rows) != 2:
        raise APrimeI64CapabilityError("A-prime receipt requires exactly two call edges")
    calls = []
    for row in call_rows:
        _reject_unknown_keys(row, _CALL_KEYS, "call")
        args = _required_rows(row.get("arguments"), "call.arguments")
        for arg in args:
            _reject_unknown_keys(arg, _ARGUMENT_KEYS, "call.argument")
        calls.append(
            APrimeI64CallEdgeRow(
                site_id=_required_int(row.get("site_id"), "call.site_id"),
                role=_required_string(row.get("role"), "call.role"),
                target_fingerprint=_required_string(
                    row.get("target_fingerprint"), "call.target_fingerprint"
                ),
                receiver_role=_required_string(row.get("receiver_role"), "call.receiver_role"),
                receiver_value_id=_required_int(
                    row.get("receiver_value_id"), "call.receiver_value_id"
                ),
                receiver_lane=_required_string(row.get("receiver_lane"), "call.receiver_lane"),
                arguments=tuple(
                    APrimeI64ArgumentRow(
                        ordinal=_required_int(arg.get("ordinal"), "call.argument.ordinal"),
                        role=_required_string(arg.get("role"), "call.argument.role"),
                        value_id=_required_int(
                            arg.get("value_id"), "call.argument.value_id"
                        ),
                        lane=_required_string(arg.get("lane"), "call.argument.lane"),
                    )
                    for arg in args
                ),
                result_value_id=_required_int(
                    row.get("result_value_id"), "call.result_value_id"
                ),
                result_lane=_required_string(row.get("result_lane"), "call.result_lane"),
            )
        )
    if {row.role for row in calls} != {"substring", "index_of"}:
        raise APrimeI64CapabilityError("call roles must be exactly substring/index_of")
    if {row.site_id for row in calls} != {0, 1}:
        raise APrimeI64CapabilityError("call sites must be exactly 0/1")
    if len({row.site_id for row in calls}) != len(calls):
        raise APrimeI64CapabilityError("duplicate call site")
    call_results = set()
    for row in calls:
        # Keep the transport key identical to the source CallSlot authority:
        # receiver excluded (`substring/2`, `indexOf/1`).  A receiver-inclusive
        # ABI spelling belongs to a future explicit field, never this key.
        expected_target = "substring/2" if row.role == "substring" else "indexOf/1"
        expected_site = 0 if row.role == "substring" else 1
        if row.site_id != expected_site:
            raise APrimeI64CapabilityError("call site does not match role")
        expected_receiver = "src" if row.role == "substring" else "pred_chars"
        expected_argument_roles = ("start", "end") if row.role == "substring" else ("ch",)
        expected_argument_lanes = (
            ("immediate_i64", "immediate_i64")
            if row.role == "substring"
            else ("opaque_handle",)
        )
        if row.target_fingerprint != expected_target:
            raise APrimeI64CapabilityError("call target fingerprint does not match role")
        if row.receiver_role != expected_receiver or row.receiver_lane != "opaque_handle":
            raise APrimeI64CapabilityError("call receiver does not match role")
        if len(row.arguments) != len(expected_argument_roles):
            raise APrimeI64CapabilityError("call argument count does not match role")
        for ordinal, (arg, expected_role, expected_lane) in enumerate(
            zip(row.arguments, expected_argument_roles, expected_argument_lanes)
        ):
            if arg.ordinal != ordinal or arg.role != expected_role or arg.lane != expected_lane:
                raise APrimeI64CapabilityError("call argument identity does not match role")
        expected_result_lane = (
            "opaque_handle" if row.role == "substring" else "immediate_i64"
        )
        if row.result_lane != expected_result_lane:
            raise APrimeI64CapabilityError(
                f"{row.role} result must use {expected_result_lane}"
            )
        if row.result_value_id in call_results:
            raise APrimeI64CapabilityError("duplicate call result ValueId")
        call_results.add(row.result_value_id)

    return_rows = _required_rows(raw.get("returns"), "returns")
    if len(return_rows) != 2:
        raise APrimeI64CapabilityError("A-prime receipt requires exactly two returns")
    for row in return_rows:
        _reject_unknown_keys(row, _RETURN_KEYS, "return")
    returns = tuple(
        APrimeI64ReturnRow(
            site=_required_string(row.get("site"), "return.site"),
            block=_required_int(row.get("block"), "return.block"),
            value_id=_required_int(row.get("value_id"), "return.value_id"),
            lane=_required_string(row.get("lane"), "return.lane"),
        )
        for row in return_rows
    )
    if {row.site for row in returns} != {"inner", "outer"}:
        raise APrimeI64CapabilityError("return sites must be exactly inner/outer")
    if len({row.site for row in returns}) != len(returns):
        raise APrimeI64CapabilityError("duplicate return site")
    if any(row.lane != "immediate_i64" for row in returns):
        raise APrimeI64CapabilityError("returns must use immediate_i64")

    return APrimeI64CapabilityView(
        schema_version=schema_version,
        backend_family=backend_family,
        parameters=parameters,
        call_edges=tuple(calls),
        returns=returns,
    )


def preflight_selected_a_prime_capability(
    func_data: Dict[str, Any],
) -> Optional[APrimeI64CapabilityView]:
    """Pre-effect hook; absent marker deliberately preserves legacy routes."""

    return load_selected_a_prime_capability(func_data)
