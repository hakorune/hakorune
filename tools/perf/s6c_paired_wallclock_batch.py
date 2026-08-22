#!/usr/bin/env python3
"""Pure append-only MeasurementBatch V3 model for S6C wall-clock evidence."""

from __future__ import annotations

import hashlib
import json
from typing import Callable

from s6c_paired_wallclock_plan import CANONICAL_CASES, seal_plan


MANIFEST_SCHEMA = "s6c-meso-wallclock-batch-manifest-v3"
SESSION_SCHEMA = "s6c-meso-wallclock-session-terminal-v3"
TERMINAL_SCHEMA = "s6c-meso-wallclock-batch-terminal-v3"
PROTOCOL = "s6c-meso-wallclock-batch-v3"
SESSION_SLOTS = (0, 1)
COMPLETE_OUTCOMES = {
    "development_green", "development_red", "development_inconclusive",
}
REPEAT_REASONS = {
    "Incomplete": "incomplete_predecessor",
    "Complete": "confirmatory_development",
}


class InvalidBatch(ValueError):
    pass


def _canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode()


def _digest(value: object) -> str:
    return hashlib.sha256(_canonical(value)).hexdigest()


def _is_hex(value: object, length: int) -> bool:
    return isinstance(value, str) and len(value) == length and all(
        character in "0123456789abcdef" for character in value)


def _seal(value: dict[str, object], field: str) -> dict[str, object]:
    return {**value, field: _digest(value)}


def _require_seal(value: dict[str, object], field: str) -> None:
    sealed = value.get(field)
    body = {key: item for key, item in value.items() if key != field}
    if not _is_hex(sealed, 64) or sealed != _digest(body):
        raise InvalidBatch(f"{field} drift")


def issue_manifest(
        *, commit: str, binary_sha256: str, build_id: str,
        alignment_sha256: str, cpu: int,
        predecessor: dict[str, object] | None = None,
        repeat_reason: str | None = None) -> dict[str, object]:
    """Issue one immutable WSL-development batch manifest before measurement."""
    if not _is_hex(commit, 40) or not _is_hex(binary_sha256, 64) or \
            not _is_hex(alignment_sha256, 64) or not _is_hex(build_id, 40) or \
            not isinstance(cpu, int) or cpu < 0:
        raise InvalidBatch("candidate identity or CPU is incomplete")
    candidate = {
        "commit": commit,
        "binary_sha256": binary_sha256,
        "build_id": build_id,
        "alignment_sha256": alignment_sha256,
    }
    candidate_sha256 = _digest(candidate)
    ordinal, predecessor_sha256 = 0, None
    if predecessor is None:
        if repeat_reason is not None:
            raise InvalidBatch("initial batch cannot have a repeat reason")
    else:
        _require_terminal(predecessor)
        state = predecessor["terminal_state"]
        if REPEAT_REASONS.get(state) != repeat_reason:
            raise InvalidBatch("repeat reason does not match predecessor state")
        if predecessor["candidate_sha256"] != candidate_sha256:
            raise InvalidBatch("successor candidate cohort drift")
        ordinal = predecessor["batch_ordinal"] + 1
        predecessor_sha256 = predecessor["terminal_sha256"]
    plans = [seal_plan(
        commit=commit, binary_sha256=binary_sha256, cases=CANONICAL_CASES,
        environment_class="wsl_development", session_index=slot,
    ) for slot in SESSION_SLOTS]
    body = {
        "schema": MANIFEST_SCHEMA,
        "protocol": PROTOCOL,
        "batch_ordinal": ordinal,
        "predecessor_terminal_sha256": predecessor_sha256,
        "repeat_reason": repeat_reason,
        "candidate": candidate,
        "candidate_sha256": candidate_sha256,
        "environment": {"role": "wsl_development", "cpu": cpu},
        "cases": list(CANONICAL_CASES),
        "session_slots": list(SESSION_SLOTS),
        "session_plan_sha256": [plan["plan_sha256"] for plan in plans],
        "threshold_4k_plus_p50": 1.15,
        "sample_policy": "retain_all_completed_pairs_no_replacement",
    }
    sealed = _seal(body, "batch_id")
    return sealed


def _require_manifest(manifest: dict[str, object]) -> None:
    if manifest.get("schema") != MANIFEST_SCHEMA or manifest.get("protocol") != PROTOCOL:
        raise InvalidBatch("foreign batch manifest")
    _require_seal(manifest, "batch_id")
    candidate, environment = manifest.get("candidate", {}), manifest.get("environment", {})
    ordinal = manifest.get("batch_ordinal")
    if manifest.get("session_slots") != list(SESSION_SLOTS) or \
            manifest.get("cases") != list(CANONICAL_CASES) or \
            manifest.get("environment", {}).get("role") != "wsl_development" or \
            manifest.get("threshold_4k_plus_p50") != 1.15 or \
            manifest.get("sample_policy") != "retain_all_completed_pairs_no_replacement" or \
            not isinstance(ordinal, int) or ordinal < 0 or \
            not _is_hex(candidate.get("commit"), 40) or \
            not _is_hex(candidate.get("binary_sha256"), 64) or \
            not _is_hex(candidate.get("build_id"), 40) or \
            not _is_hex(candidate.get("alignment_sha256"), 64) or \
            manifest.get("candidate_sha256") != _digest(candidate) or \
            not isinstance(environment.get("cpu"), int) or environment["cpu"] < 0:
        raise InvalidBatch("batch census or role drift")
    if (ordinal == 0) != (manifest.get("predecessor_terminal_sha256") is None) or \
            (ordinal == 0) != (manifest.get("repeat_reason") is None):
        raise InvalidBatch("batch lineage shape drift")
    expected_plans = [seal_plan(
        commit=candidate["commit"], binary_sha256=candidate["binary_sha256"],
        cases=CANONICAL_CASES, environment_class="wsl_development", session_index=slot,
    )["plan_sha256"] for slot in SESSION_SLOTS]
    if manifest.get("session_plan_sha256") != expected_plans:
        raise InvalidBatch("session-plan projection drift")


def validate_manifest(manifest: dict[str, object]) -> None:
    """Public read-only validation seam for the append-only store."""
    _require_manifest(manifest)


def issue_session_terminal(
        manifest: dict[str, object], *, slot: int, terminal_state: str,
        outcome: str | None = None, raw_csv_sha256: str | None = None,
        diagnostic_raw_csv_sha256: str | None = None,
        reason: str | None = None) -> dict[str, object]:
    """Issue one terminal receipt; partial rows never enter this product."""
    _require_manifest(manifest)
    if slot not in SESSION_SLOTS:
        raise InvalidBatch("foreign session slot")
    if terminal_state == "Complete":
        if outcome not in COMPLETE_OUTCOMES or not _is_hex(raw_csv_sha256, 64) or \
                diagnostic_raw_csv_sha256 is not None or reason:
            raise InvalidBatch("complete session payload drift")
    elif terminal_state in {"Incomplete", "IntegrityInvalid"}:
        if outcome is not None or raw_csv_sha256 is not None or not reason or \
                diagnostic_raw_csv_sha256 is not None and not _is_hex(
                    diagnostic_raw_csv_sha256, 64):
            raise InvalidBatch("ineligible session carried evidence")
    else:
        raise InvalidBatch("unknown session terminal state")
    body = {
        "schema": SESSION_SCHEMA,
        "batch_id": manifest["batch_id"],
        "candidate_sha256": manifest["candidate_sha256"],
        "slot": slot,
        "session_plan_sha256": manifest["session_plan_sha256"][slot],
        "terminal_state": terminal_state,
        "outcome": outcome,
        "raw_csv_sha256": raw_csv_sha256,
        "diagnostic_raw_csv_sha256": diagnostic_raw_csv_sha256,
        "reason": reason,
    }
    return _seal(body, "receipt_sha256")


def _require_session(manifest: dict[str, object], receipt: dict[str, object]) -> None:
    if receipt.get("schema") != SESSION_SCHEMA:
        raise InvalidBatch("foreign session receipt")
    _require_seal(receipt, "receipt_sha256")
    slot = receipt.get("slot")
    if receipt.get("batch_id") != manifest["batch_id"] or slot not in SESSION_SLOTS or \
            receipt.get("candidate_sha256") != manifest["candidate_sha256"] or \
            receipt.get("session_plan_sha256") != manifest["session_plan_sha256"][slot]:
        raise InvalidBatch("session receipt cohort drift")
    state = receipt.get("terminal_state")
    if state == "Complete":
        if receipt.get("outcome") not in COMPLETE_OUTCOMES or \
                not _is_hex(receipt.get("raw_csv_sha256"), 64) or \
                receipt.get("diagnostic_raw_csv_sha256") is not None or \
                receipt.get("reason") is not None:
            raise InvalidBatch("complete session receipt payload drift")
    elif state in {"Incomplete", "IntegrityInvalid"}:
        diagnostic = receipt.get("diagnostic_raw_csv_sha256")
        if receipt.get("outcome") is not None or receipt.get("raw_csv_sha256") is not None or \
                not receipt.get("reason") or diagnostic is not None and not _is_hex(diagnostic, 64):
            raise InvalidBatch("ineligible session receipt payload drift")
    else:
        raise InvalidBatch("unknown session receipt state")


def close_batch(
        manifest: dict[str, object],
        receipts: list[dict[str, object]]) -> dict[str, object]:
    """Close one batch from every declared session terminal exactly once."""
    _require_manifest(manifest)
    if len(receipts) != len(SESSION_SLOTS):
        raise InvalidBatch("batch terminal requires every declared slot")
    for receipt in receipts:
        _require_session(manifest, receipt)
    by_slot = {receipt["slot"]: receipt for receipt in receipts}
    if set(by_slot) != set(SESSION_SLOTS) or len(by_slot) != len(receipts):
        raise InvalidBatch("duplicate or missing session terminal")
    states = [by_slot[slot]["terminal_state"] for slot in SESSION_SLOTS]
    outcomes = [by_slot[slot]["outcome"] for slot in SESSION_SLOTS]
    if "IntegrityInvalid" in states:
        state, classification = "IntegrityInvalid", None
    elif "Incomplete" in states:
        state, classification = "Incomplete", None
    else:
        state = "Complete"
        if outcomes == ["development_green", "development_green"]:
            classification = "development_keeper"
        elif outcomes == ["development_red", "development_red"]:
            classification = "development_red"
        else:
            classification = "development_inconclusive"
    body = {
        "schema": TERMINAL_SCHEMA,
        "protocol": PROTOCOL,
        "batch_id": manifest["batch_id"],
        "candidate_sha256": manifest["candidate_sha256"],
        "batch_ordinal": manifest["batch_ordinal"],
        "terminal_state": state,
        "evidence_eligible": state == "Complete",
        "classification": classification,
        "session_receipt_sha256": [by_slot[slot]["receipt_sha256"]
                                   for slot in SESSION_SLOTS],
        "native_promotion_authority": False,
        "closed": True,
    }
    return _seal(body, "terminal_sha256")


def _require_terminal(terminal: dict[str, object]) -> None:
    if terminal.get("schema") != TERMINAL_SCHEMA or terminal.get("protocol") != PROTOCOL or \
            terminal.get("closed") is not True or terminal.get("terminal_state") not in {
                "Complete", "Incomplete", "IntegrityInvalid",
            }:
        raise InvalidBatch("foreign batch terminal")
    _require_seal(terminal, "terminal_sha256")
    state, classification = terminal["terminal_state"], terminal.get("classification")
    if not isinstance(terminal.get("batch_ordinal"), int) or \
            not _is_hex(terminal.get("batch_id"), 64) or \
            not _is_hex(terminal.get("candidate_sha256"), 64) or \
            terminal.get("native_promotion_authority") is not False or \
            len(terminal.get("session_receipt_sha256", [])) != len(SESSION_SLOTS) or \
            any(not _is_hex(item, 64) for item in terminal["session_receipt_sha256"]):
        raise InvalidBatch("batch terminal census drift")
    if state == "Complete":
        if terminal.get("evidence_eligible") is not True or classification not in {
                "development_keeper", "development_red", "development_inconclusive"}:
            raise InvalidBatch("complete batch outcome drift")
    elif terminal.get("evidence_eligible") is not False or classification is not None:
        raise InvalidBatch("ineligible batch carried a classification")


def _expect_invalid(action: Callable[[], object]) -> None:
    try:
        action()
    except InvalidBatch:
        return
    raise AssertionError("invalid batch transition accepted")


def self_test() -> None:
    identity = dict(commit="a" * 40, binary_sha256="b" * 64,
                    build_id="c" * 40, alignment_sha256="d" * 64, cpu=0)
    first = issue_manifest(**identity)
    green = [issue_session_terminal(first, slot=slot, terminal_state="Complete",
             outcome="development_green", raw_csv_sha256=str(slot + 1) * 64)
             for slot in SESSION_SLOTS]
    keeper = close_batch(first, green)
    assert keeper["classification"] == "development_keeper"
    successor = issue_manifest(**identity, predecessor=keeper,
                               repeat_reason="confirmatory_development")
    assert successor["batch_id"] != first["batch_id"]
    assert successor["predecessor_terminal_sha256"] == keeper["terminal_sha256"]

    incomplete_receipts = [
        issue_session_terminal(first, slot=0, terminal_state="Incomplete",
                               reason="short_measured_arm",
                               diagnostic_raw_csv_sha256="9" * 64), green[1],
    ]
    incomplete = close_batch(first, incomplete_receipts)
    assert not incomplete["evidence_eligible"] and incomplete["classification"] is None
    resumed = issue_manifest(**identity, predecessor=incomplete,
                             repeat_reason="incomplete_predecessor")
    assert resumed["batch_ordinal"] == 1

    red = [issue_session_terminal(successor, slot=slot, terminal_state="Complete",
           outcome="development_red", raw_csv_sha256=str(slot + 3) * 64)
           for slot in SESSION_SLOTS]
    assert close_batch(successor, red)["classification"] == "development_red"
    mixed = [red[0], issue_session_terminal(
        successor, slot=1, terminal_state="Complete",
        outcome="development_green", raw_csv_sha256="f" * 64)]
    assert close_batch(successor, mixed)["classification"] == "development_inconclusive"

    foreign = json.loads(json.dumps(green[0]))
    foreign["schema"] = "s6c-meso-wallclock-session-terminal-v2"
    tampered = json.loads(json.dumps(first))
    tampered["candidate"]["binary_sha256"] = "e" * 64
    changed_identity = {**identity, "binary_sha256": "e" * 64}
    invalid_terminal = close_batch(first, [
        issue_session_terminal(first, slot=0, terminal_state="IntegrityInvalid",
                               reason="oracle_drift"), green[1]])
    for action in (
        lambda: close_batch(first, [green[0], green[0]]),
        lambda: close_batch(first, [foreign, green[1]]),
        lambda: close_batch(tampered, green),
        lambda: issue_manifest(**identity, predecessor=keeper,
                               repeat_reason="incomplete_predecessor"),
        lambda: issue_manifest(**identity, predecessor=invalid_terminal,
                               repeat_reason="incomplete_predecessor"),
        lambda: issue_manifest(**changed_identity, predecessor=keeper,
                               repeat_reason="confirmatory_development"),
        lambda: close_batch(successor, green),
        lambda: issue_session_terminal(first, slot=0, terminal_state="Complete",
                                       outcome="development_green"),
        lambda: issue_session_terminal(first, slot=0, terminal_state="Incomplete",
                                       reason="short", raw_csv_sha256="a" * 64),
        lambda: issue_session_terminal(first, slot=0, terminal_state="Incomplete",
                                       reason="short", diagnostic_raw_csv_sha256="bad"),
    ):
        _expect_invalid(action)


if __name__ == "__main__":
    self_test()
    print("[s6c-paired-wallclock-batch] self-test ok")
