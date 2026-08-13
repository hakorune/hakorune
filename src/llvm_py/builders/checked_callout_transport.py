"""Transport-only view of the neutral MIR CheckedCallOut vocabulary.

The Rust MIR JSON emitter is the transport owner for these four operations.
This module only performs a strict JSON-to-Python projection for future W6
consumers; it does not resolve a provider, select a target, emit LLVM, or
fall through to a generic call route.  Production dispatcher integration is
intentionally a later part of the atomic activation cell.
"""

from dataclasses import dataclass
from typing import Any, Dict, Mapping, Tuple, Union


class CheckedCallOutTransportError(ValueError):
    """The transport payload is missing, malformed, or has unknown fields."""


_U32_MAX = (1 << 32) - 1
_U16_MAX = (1 << 16) - 1
_KNOWN_EFFECT_BITS = 0x7FFF

_TERMINATOR_KEYS = {
    "op",
    "site_id",
    "receiver",
    "args",
    "normal",
    "fault",
    "effects",
}
_NORMAL_RESULT_KEYS = {"op", "site_id", "dst"}
_END_KEYS = {"op", "site_id", "lease_slot"}
_FAULT_KEYS = {"op", "site_id"}


@dataclass(frozen=True)
class CheckedCallOutTerminatorView:
    site_id: int
    receiver: int
    arguments: Tuple[int, ...]
    normal_landing: int
    fault_landing: int
    effects: int

    def to_json(self) -> Dict[str, Any]:
        return {
            "op": "checked_callout",
            "site_id": self.site_id,
            "receiver": self.receiver,
            "args": list(self.arguments),
            "normal": self.normal_landing,
            "fault": self.fault_landing,
            "effects": self.effects,
        }


@dataclass(frozen=True)
class CheckedCallOutNormalResultView:
    site_id: int
    dst: int

    def to_json(self) -> Dict[str, Any]:
        return {"op": "checked_callout_normal_result", "site_id": self.site_id, "dst": self.dst}


@dataclass(frozen=True)
class CheckedCallOutEndView:
    site_id: int
    lease_slot: int

    def to_json(self) -> Dict[str, Any]:
        return {"op": "checked_callout_end", "site_id": self.site_id, "lease_slot": self.lease_slot}


@dataclass(frozen=True)
class CheckedCallOutFaultView:
    site_id: int

    def to_json(self) -> Dict[str, Any]:
        return {"op": "checked_callout_fault", "site_id": self.site_id}


CheckedCallOutTransportView = Union[
    CheckedCallOutTerminatorView,
    CheckedCallOutNormalResultView,
    CheckedCallOutEndView,
    CheckedCallOutFaultView,
]


def _reject_unknown(raw: Mapping[str, Any], allowed: set[str], label: str) -> None:
    unknown = sorted(set(raw) - allowed)
    if unknown:
        raise CheckedCallOutTransportError(
            f"{label} contains unknown fields: {', '.join(unknown)}"
        )


def _u32(raw: Any, label: str) -> int:
    if isinstance(raw, bool) or not isinstance(raw, int):
        raise CheckedCallOutTransportError(f"{label} must be an integer")
    if raw < 0 or raw > _U32_MAX:
        raise CheckedCallOutTransportError(f"{label} exceeds u32")
    return raw


def _u16_effects(raw: Any) -> int:
    value = _u32(raw, "checked_callout effects")
    if value > _U16_MAX:
        raise CheckedCallOutTransportError("checked_callout effects exceeds u16")
    if value & ~_KNOWN_EFFECT_BITS:
        raise CheckedCallOutTransportError(
            f"checked_callout effects contain unknown bits: 0x{value:04x}"
        )
    return value


def _required_mapping(raw: Any) -> Mapping[str, Any]:
    if not isinstance(raw, Mapping):
        raise CheckedCallOutTransportError("CheckedCallOut transport must be an object")
    return raw


def _required_args(raw: Any) -> Tuple[int, ...]:
    if not isinstance(raw, list):
        raise CheckedCallOutTransportError("checked_callout args must be an array")
    return tuple(_u32(value, f"checked_callout args[{index}]") for index, value in enumerate(raw))


def parse_checked_callout_transport(raw: Mapping[str, Any]) -> CheckedCallOutTransportView:
    """Parse one canonical MIR JSON CheckedCallOut operation.

    The operation name is the only dispatch key.  No selector/name lookup or
    compatibility alias is accepted here, so malformed transport fails before
    a future LLVM consumer can observe it.
    """

    value = _required_mapping(raw)
    operation = value.get("op")
    if operation == "checked_callout":
        _reject_unknown(value, _TERMINATOR_KEYS, operation)
        site_id = _u32(value.get("site_id"), "checked_callout site_id")
        normal_landing = _u32(value.get("normal"), "checked_callout normal landing")
        fault_landing = _u32(value.get("fault"), "checked_callout fault landing")
        if normal_landing == fault_landing:
            raise CheckedCallOutTransportError("checked_callout landings must be distinct")
        return CheckedCallOutTerminatorView(
            site_id=site_id,
            receiver=_u32(value.get("receiver"), "checked_callout receiver"),
            arguments=_required_args(value.get("args")),
            normal_landing=normal_landing,
            fault_landing=fault_landing,
            effects=_u16_effects(value.get("effects")),
        )
    if operation == "checked_callout_normal_result":
        _reject_unknown(value, _NORMAL_RESULT_KEYS, operation)
        return CheckedCallOutNormalResultView(
            site_id=_u32(value.get("site_id"), "checked_callout normal result site_id"),
            dst=_u32(value.get("dst"), "checked_callout normal result dst"),
        )
    if operation == "checked_callout_end":
        _reject_unknown(value, _END_KEYS, operation)
        return CheckedCallOutEndView(
            site_id=_u32(value.get("site_id"), "checked_callout end site_id"),
            lease_slot=_u32(value.get("lease_slot"), "checked_callout end lease_slot"),
        )
    if operation == "checked_callout_fault":
        _reject_unknown(value, _FAULT_KEYS, operation)
        return CheckedCallOutFaultView(
            site_id=_u32(value.get("site_id"), "checked_callout fault site_id")
        )
    raise CheckedCallOutTransportError(
        f"unsupported CheckedCallOut transport operation: {operation!r}"
    )


__all__ = [
    "CheckedCallOutTransportError",
    "CheckedCallOutTransportView",
    "CheckedCallOutTerminatorView",
    "CheckedCallOutNormalResultView",
    "CheckedCallOutEndView",
    "CheckedCallOutFaultView",
    "parse_checked_callout_transport",
]
