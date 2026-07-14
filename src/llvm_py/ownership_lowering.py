"""Verified Ownership SSA transport for the llvm_py object-handle lane."""

from __future__ import annotations

from typing import Any, Dict, Optional


_SCHEMA = "VerifiedOwnershipSsaV1"
_PRODUCER = "rust_ownership_ssa_verifier_v1"
_BACKEND = "llvm_py"
_PROVIDER = "nyash_kernel"
_OPS = {"copy_owned", "destroy_owned"}
_KINDS = {"none", "borrowed", "owned"}


class OwnershipLoweringContractError(RuntimeError):
    pass


def _fail(reason: str, detail: str) -> None:
    raise OwnershipLoweringContractError(
        f"[freeze:contract][llvm-py/ownership:{reason}] {detail}"
    )


def _operation(block_id: int, instruction_index: int, inst: Dict[str, Any]) -> Dict[str, Any]:
    op = inst.get("op")
    if op == "copy_owned":
        if not isinstance(inst.get("dst"), int) or not isinstance(inst.get("src"), int):
            _fail("invalid_operation", f"copy_owned site=bb{block_id}.i{instruction_index}")
        return {
            "block": block_id,
            "instruction_index": instruction_index,
            "op": op,
            "dst": int(inst["dst"]),
            "src": int(inst["src"]),
        }
    if op == "destroy_owned":
        if not isinstance(inst.get("value"), int):
            _fail("invalid_operation", f"destroy_owned site=bb{block_id}.i{instruction_index}")
        return {
            "block": block_id,
            "instruction_index": instruction_index,
            "op": op,
            "value": int(inst["value"]),
        }
    _fail("invalid_operation", f"unsupported ownership op={op!r}")


def _collect_operations(func_data: Dict[str, Any]) -> tuple[list[Dict[str, Any]], bool]:
    operations: list[Dict[str, Any]] = []
    has_legacy_release = False
    blocks = func_data.get("blocks")
    if not isinstance(blocks, list):
        _fail("invalid_function", "blocks must be a list")
    for block in blocks:
        if not isinstance(block, dict) or not isinstance(block.get("id"), int):
            _fail("invalid_function", "every block needs an integer id")
        for index, inst in enumerate(block.get("instructions") or []):
            if not isinstance(inst, dict):
                _fail("invalid_function", "instruction must be an object")
            if inst.get("op") in _OPS:
                operations.append(_operation(int(block["id"]), index, inst))
            elif inst.get("op") == "release_strong":
                has_legacy_release = True
    return operations, has_legacy_release


def _required_map(metadata: Dict[str, Any], name: str) -> Dict[str, Any]:
    value = metadata.get(name)
    if not isinstance(value, dict):
        _fail("missing_metadata", f"metadata.{name} is required")
    return value


def _verify_boxref(value_id: int, value_types: Dict[str, Any], storage: Dict[str, Any]) -> None:
    key = str(value_id)
    type_row = value_types.get(key)
    if not isinstance(type_row, dict):
        _fail("missing_boxref", f"%{value_id} has no exact value type")
    if type_row.get("kind") != "handle" or not isinstance(type_row.get("box_type"), str):
        _fail("missing_boxref", f"%{value_id} is not an exact BoxRef handle")
    if not type_row["box_type"] or storage.get(key) != "box_ref":
        _fail("missing_boxref", f"%{value_id} storage is not box_ref")


class VerifiedOwnershipLoweringSessionV1:
    """Function-scoped use ledger for one transported sealed witness."""

    def __init__(self, owner: int, operations: list[Dict[str, Any]]):
        self.owner = owner
        self._expected = {
            (row["block"], row["instruction_index"]): dict(row) for row in operations
        }
        self._consumed: set[tuple[int, int]] = set()

    def claim(self, block_id: int, instruction_index: int, inst: Dict[str, Any]) -> None:
        site = (int(block_id), int(instruction_index))
        expected = self._expected.get(site)
        actual = _operation(site[0], site[1], inst)
        if expected != actual:
            _fail("foreign_operation", f"site=bb{site[0]}.i{site[1]}")
        if site in self._consumed:
            _fail("duplicate_consume", f"site=bb{site[0]}.i{site[1]}")
        self._consumed.add(site)

    def finish(self) -> None:
        missing = sorted(set(self._expected) - self._consumed)
        if missing:
            _fail("incomplete_coverage", f"unconsumed sites={missing}")


def verify_ownership_lowering_v1(
    func_data: Dict[str, Any],
) -> Optional[VerifiedOwnershipLoweringSessionV1]:
    operations, has_legacy_release = _collect_operations(func_data)
    metadata = func_data.get("metadata")
    metadata = metadata if isinstance(metadata, dict) else {}
    witness = metadata.get("ownership_ssa_v1")
    if not operations:
        if witness is not None:
            _fail("orphan_witness", "witness exists without ownership operations")
        return None
    if has_legacy_release:
        _fail("legacy_mix", "ReleaseStrong cannot coexist with Ownership SSA V1")
    if not isinstance(witness, dict):
        _fail("missing_witness", "ownership operations require metadata.ownership_ssa_v1")
    if set(witness) != {
        "schema", "producer", "owner", "backend", "provider", "value_kinds", "operations"
    }:
        _fail("witness_schema", "ownership witness fields drifted")
    if witness.get("schema") != _SCHEMA or witness.get("producer") != _PRODUCER:
        _fail("witness_schema", "unverified ownership witness producer")
    if witness.get("backend") != _BACKEND:
        _fail("backend_missing_capability", f"backend={witness.get('backend')!r}")
    if witness.get("provider") != _PROVIDER:
        _fail("provider_missing_capability", f"provider={witness.get('provider')!r}")
    owner = witness.get("owner")
    if not isinstance(owner, int) or owner < 0:
        _fail("foreign_owner", f"owner={owner!r}")
    kinds = witness.get("value_kinds")
    if not isinstance(kinds, dict) or any(kind not in _KINDS for kind in kinds.values()):
        _fail("witness_schema", "value_kinds must use the closed V1 vocabulary")
    if witness.get("operations") != operations:
        _fail("foreign_operation", "witness operation inventory differs from function")

    value_types = _required_map(metadata, "value_types")
    storage = _required_map(metadata, "storage_classes")
    for row in operations:
        values = [row["value"]] if row["op"] == "destroy_owned" else [row["src"], row["dst"]]
        for value_id in values:
            _verify_boxref(value_id, value_types, storage)
            if kinds.get(str(value_id)) not in {"borrowed", "owned"}:
                _fail("missing_kind", f"%{value_id} lacks strong ownership kind")
        if row["op"] == "copy_owned" and kinds.get(str(row["dst"])) != "owned":
            _fail("missing_kind", f"copy_owned dst %{row['dst']} must be owned")
        if row["op"] == "destroy_owned" and kinds.get(str(row["value"])) != "owned":
            _fail("missing_kind", f"destroy_owned value %{row['value']} must be owned")
    return VerifiedOwnershipLoweringSessionV1(owner, operations)
