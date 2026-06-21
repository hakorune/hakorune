#!/usr/bin/env python3
"""Typed operation IR for verified MirBuilder family Hako emission."""

from __future__ import annotations

from dataclasses import dataclass, field as dataclass_field
from typing import Any


@dataclass(frozen=True)
class HakoOperation:
    kind: str
    field: str | None = None
    key: str | None = None
    value: str | None = None
    return_value: int | None = None
    extra: dict[str, Any] = dataclass_field(default_factory=dict)

    def to_json(self) -> dict[str, Any]:
        data: dict[str, Any] = {"kind": self.kind}
        for key, value in {
            "field": self.field,
            "key": self.key,
            "value": self.value,
            "return_value": self.return_value,
        }.items():
            if value is not None:
                data[key] = value
        data.update(self.extra)
        return data


@dataclass(frozen=True)
class HakoMethodIR:
    signature: str
    operations: list[HakoOperation]

    def to_json(self) -> dict[str, Any]:
        return {
            "signature": self.signature,
            "operations": [operation.to_json() for operation in self.operations],
        }


def op(kind: str, **kwargs: Any) -> HakoOperation:
    known: dict[str, Any] = {}
    for key in ("field", "key", "value", "return_value"):
        if key in kwargs:
            known[key] = kwargs.pop(key)
    return HakoOperation(kind=kind, extra=kwargs, **known)
