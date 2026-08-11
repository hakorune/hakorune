"""Strict loader for the sealed A-prime LLVM physical capability.

The field is a post-session receipt.  Its absence intentionally leaves all
legacy lowering behavior unchanged.  This module validates only the explicit
transport schema; it never resolves values, infers types, or performs casts.
"""

from dataclasses import dataclass
from typing import Any, Dict, Optional, Sequence, Tuple


SCHEMA_VERSION = 1
CAPABILITY_KEY = "a_prime_i64_physical_receipt"


class APrimeI64CapabilityError(ValueError):
    """Malformed, incomplete, or mismatched A-prime receipt."""


@dataclass(frozen=True)
class APrimeI64ValueRow:
    value_id: int
    lane: str


@dataclass(frozen=True)
class APrimeI64ParameterRow:
    role: str
    formal_parameter_index: int
    value_id: int
    lane: str


@dataclass(frozen=True)
class APrimeI64CallEdgeRow:
    role: str
    block: int
    instruction_index: int
    target_fingerprint: str
    arguments: Tuple[APrimeI64ValueRow, ...]
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

    def require_call_edge(self, block: int, instruction_index: int) -> APrimeI64CallEdgeRow:
        rows = [
            row
            for row in self.call_edges
            if row.block == block and row.instruction_index == instruction_index
        ]
        if len(rows) != 1:
            raise APrimeI64CapabilityError(
                f"call edge is not unique: ({block}, {instruction_index})"
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


def _load_value_row(row: Dict[str, Any], label: str) -> APrimeI64ValueRow:
    lane = _required_string(row.get("lane"), f"{label}.lane")
    if lane not in ("immediate_i64", "opaque_handle"):
        raise APrimeI64CapabilityError(f"{label}.lane is unsupported: {lane}")
    return APrimeI64ValueRow(
        value_id=_required_int(row.get("value_id"), f"{label}.value_id"),
        lane=lane,
    )


def _validate_func_params(func_data: Dict[str, Any], rows: Sequence[APrimeI64ParameterRow]) -> None:
    params = func_data.get("params")
    if not isinstance(params, list) or not params:
        raise APrimeI64CapabilityError("selected A-prime function requires explicit params")
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

    schema_version = _required_int(raw.get("schema_version"), "schema_version")
    if schema_version != SCHEMA_VERSION:
        raise APrimeI64CapabilityError(f"unsupported schema_version: {schema_version}")
    backend_family = _required_string(raw.get("backend_family"), "backend_family")
    if backend_family != "llvm":
        raise APrimeI64CapabilityError(f"unsupported backend_family: {backend_family}")
    if raw.get("fallback", False) is not False or raw.get("retry", False) is not False:
        raise APrimeI64CapabilityError("fallback/retry are forbidden in A-prime receipt")

    parameter_rows = _required_rows(raw.get("parameters"), "parameters")
    if len(parameter_rows) != 2:
        raise APrimeI64CapabilityError("A-prime receipt requires exactly two parameters")
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
        args = _required_rows(row.get("arguments"), "call.arguments")
        calls.append(
            APrimeI64CallEdgeRow(
                role=_required_string(row.get("role"), "call.role"),
                block=_required_int(row.get("block"), "call.block"),
                instruction_index=_required_int(
                    row.get("instruction_index"), "call.instruction_index"
                ),
                target_fingerprint=_required_string(
                    row.get("target_fingerprint"), "call.target_fingerprint"
                ),
                arguments=tuple(
                    _load_value_row(arg, "call.argument") for arg in args
                ),
                result_value_id=_required_int(
                    row.get("result_value_id"), "call.result_value_id"
                ),
                result_lane=_required_string(row.get("result_lane"), "call.result_lane"),
            )
        )
    if {row.role for row in calls} != {"substring", "index_of"}:
        raise APrimeI64CapabilityError("call roles must be exactly substring/index_of")
    if len({(row.block, row.instruction_index) for row in calls}) != len(calls):
        raise APrimeI64CapabilityError("duplicate call site")
    if any(row.result_lane != "opaque_handle" for row in calls):
        raise APrimeI64CapabilityError("Dynamic call results must use opaque_handle")

    return_rows = _required_rows(raw.get("returns"), "returns")
    if len(return_rows) != 2:
        raise APrimeI64CapabilityError("A-prime receipt requires exactly two returns")
    returns = tuple(
        APrimeI64ReturnRow(
            site=_required_string(row.get("site"), "return.site"),
            block=_required_int(row.get("block"), "return.block"),
            value_id=_required_int(row.get("value_id"), "return.value_id"),
            lane=_required_string(row.get("lane"), "return.lane"),
        )
        for row in return_rows
    )
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
