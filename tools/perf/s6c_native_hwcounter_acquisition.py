#!/usr/bin/env python3
"""Private bounded clean-pair acquisition owner for S6C PMU evidence."""

from __future__ import annotations

import hashlib
import json
import time
from typing import Callable


BLOCK_COUNT = 3
ACCEPTED_PER_BLOCK = 51
MAX_ATTEMPTS_PER_BLOCK = 68
MAX_REJECTIONS_PER_BLOCK = 17
PROTOCOL_VERSION = "s6c-meso-clean-pair-acquisition-v1"


class FatalPairObservation(RuntimeError):
    def __init__(self, code: str, detail: str, arms: dict[str, object] | None = None):
        super().__init__(detail)
        self.code = code
        self.detail = detail
        self.arms = arms or {}


def seal_plan(fields: dict[str, object]) -> dict[str, object]:
    plan = {
        "schema": PROTOCOL_VERSION,
        "block_count": BLOCK_COUNT,
        "accepted_pairs_per_block": ACCEPTED_PER_BLOCK,
        "max_attempts_per_block": MAX_ATTEMPTS_PER_BLOCK,
        "max_scheduler_rejections_per_block": MAX_REJECTIONS_PER_BLOCK,
        "accepted_slot_order": "AB when (block + slot) is even, otherwise BA",
        "scheduler_contamination": [
            "voluntary_context_switches > 0",
            "involuntary_context_switches > 0",
            "single-CPU affinity or CPU identity drift",
        ],
        **fields,
    }
    encoded = json.dumps(plan, sort_keys=True, separators=(",", ":")).encode()
    return {**plan, "plan_sha256": hashlib.sha256(encoded).hexdigest()}


def accepted_order(block: int, slot: int) -> tuple[str, str]:
    if not 0 <= block < BLOCK_COUNT or not 0 <= slot < ACCEPTED_PER_BLOCK:
        raise ValueError("accepted slot is outside the fixed plan")
    return ("hako", "c") if (block + slot) % 2 == 0 else ("c", "hako")


def _scheduler_reasons_for_scope(
        arm: str, scope: str, row: dict[str, object], cpu: int) -> list[str]:
    reasons = []
    for field in ("voluntary_context_switches", "involuntary_context_switches"):
        if row[field] > 0:
            reasons.append(f"{arm}.{scope}.{field}")
    for field in ("affinity_count_before", "affinity_count_after"):
        if row[field] != 1:
            reasons.append(f"{arm}.{scope}.{field}")
    for field in ("affinity_cpu_before", "affinity_cpu_after"):
        if row[field] != cpu:
            reasons.append(f"{arm}.{scope}.{field}")
    return reasons


def scheduler_reasons(arms: dict[str, dict[str, object]], cpu: int) -> list[str]:
    reasons = []
    for arm in ("hako", "c"):
        sample = arms[arm]["sample"]
        for scope in ("arm_envelope", "primary", "frontend"):
            reasons.extend(_scheduler_reasons_for_scope(arm, scope, sample[scope], cpu))
    return reasons


def _block_summary(
        block: int, attempt_ids: list[str], accepted_ids: list[str],
        rejected_ids: list[str], fatal_id: str | None = None) -> dict[str, object]:
    return {
        "block": block,
        "attempt_count": len(attempt_ids),
        "accepted_count": len(accepted_ids),
        "scheduler_rejected_count": len(rejected_ids),
        "rejection_rate": len(rejected_ids) / len(attempt_ids) if attempt_ids else 0.0,
        "accepted_AB": sum((block + slot) % 2 == 0 for slot in range(len(accepted_ids))),
        "accepted_BA": sum((block + slot) % 2 == 1 for slot in range(len(accepted_ids))),
        "attempt_ids": attempt_ids,
        "accepted_attempt_ids": accepted_ids,
        "scheduler_rejected_attempt_ids": rejected_ids,
        "fatal_attempt_id": fatal_id,
    }


def acquire(
        plan: dict[str, object], cpu: int,
        run_complete_pair: Callable[[tuple[str, str]], dict[str, dict[str, object]]],
) -> dict[str, object]:
    if plan.get("schema") != PROTOCOL_VERSION or not plan.get("plan_sha256"):
        raise ValueError("unsealed acquisition plan")
    attempts: list[dict[str, object]] = []
    blocks = []
    global_attempt = 0
    for block in range(BLOCK_COUNT):
        block_attempt_ids: list[str] = []
        accepted_ids: list[str] = []
        rejected_ids: list[str] = []
        replacement_ordinal = 0
        while len(accepted_ids) < ACCEPTED_PER_BLOCK:
            if len(block_attempt_ids) >= MAX_ATTEMPTS_PER_BLOCK:
                blocks.append(_block_summary(
                    block, block_attempt_ids, accepted_ids, rejected_ids))
                return _terminal(plan, attempts, blocks, "NoSafeSlice",
                                 "scheduler_attempt_cap_exhausted")
            slot = len(accepted_ids)
            order = accepted_order(block, slot)
            attempt_id = f"b{block}-a{len(block_attempt_ids)}-g{global_attempt}"
            started = time.monotonic_ns()
            try:
                arms = run_complete_pair(order)
            except FatalPairObservation as error:
                ended = time.monotonic_ns()
                attempt = {
                    "attempt_id": attempt_id,
                    "global_attempt_index": global_attempt,
                    "block": block,
                    "block_attempt_index": len(block_attempt_ids),
                    "accepted_slot": slot,
                    "replacement_ordinal": replacement_ordinal,
                    "order": list(order),
                    "started_monotonic_ns": started,
                    "ended_monotonic_ns": ended,
                    "disposition": "fatal_invalid",
                    "analysis_eligible": False,
                    "reasons": [error.code],
                    "fatal_detail": error.detail,
                    "arms": error.arms,
                }
                attempts.append(attempt)
                block_attempt_ids.append(attempt_id)
                blocks.append(_block_summary(
                    block, block_attempt_ids, accepted_ids, rejected_ids, attempt_id))
                return _terminal(plan, attempts, blocks, "NoSafeSlice", error.code)
            ended = time.monotonic_ns()
            try:
                reasons = scheduler_reasons(arms, cpu)
            except (KeyError, TypeError, ValueError) as error:
                attempt = {
                    "attempt_id": attempt_id, "global_attempt_index": global_attempt,
                    "block": block, "block_attempt_index": len(block_attempt_ids),
                    "accepted_slot": slot, "replacement_ordinal": replacement_ordinal,
                    "order": list(order), "started_monotonic_ns": started,
                    "ended_monotonic_ns": ended, "disposition": "fatal_invalid",
                    "analysis_eligible": False, "reasons": ["malformed_pair_observation"],
                    "fatal_detail": str(error), "arms": arms,
                }
                attempts.append(attempt)
                block_attempt_ids.append(attempt_id)
                blocks.append(_block_summary(
                    block, block_attempt_ids, accepted_ids, rejected_ids, attempt_id))
                return _terminal(plan, attempts, blocks, "NoSafeSlice",
                                 "malformed_pair_observation")
            disposition = "scheduler_rejected" if reasons else "accepted"
            attempt = {
                "attempt_id": attempt_id,
                "global_attempt_index": global_attempt,
                "block": block,
                "block_attempt_index": len(block_attempt_ids),
                "accepted_slot": slot,
                "replacement_ordinal": replacement_ordinal,
                "order": list(order),
                "started_monotonic_ns": started,
                "ended_monotonic_ns": ended,
                "disposition": disposition,
                "analysis_eligible": not reasons,
                "reasons": reasons,
                "arms": arms,
            }
            attempts.append(attempt)
            block_attempt_ids.append(attempt_id)
            if reasons:
                rejected_ids.append(attempt_id)
                replacement_ordinal += 1
                if len(rejected_ids) > MAX_REJECTIONS_PER_BLOCK:
                    blocks.append(_block_summary(
                        block, block_attempt_ids, accepted_ids, rejected_ids))
                    return _terminal(plan, attempts, blocks, "NoSafeSlice",
                                     "scheduler_rejection_cap_exhausted")
            else:
                accepted_ids.append(attempt_id)
                replacement_ordinal = 0
            global_attempt += 1
        blocks.append(_block_summary(block, block_attempt_ids, accepted_ids, rejected_ids))
    return _terminal(plan, attempts, blocks, "accepted", None)


def _terminal(
        plan: dict[str, object], attempts: list[dict[str, object]],
        blocks: list[dict[str, object]], outcome: str, reason: str | None) -> dict[str, object]:
    return {
        "schema": "s6c-meso-clean-pair-terminal-v1",
        "plan": plan,
        "terminal_outcome": outcome,
        "evidence_eligible": outcome == "accepted",
        "terminal_reason": reason,
        "attempts": attempts,
        "blocks": blocks,
    }


def self_test() -> None:
    plan = seal_plan({"test": True})

    def sample(arm: str, dirty: bool = False) -> dict[str, object]:
        scope = {
            "voluntary_context_switches": 0,
            "involuntary_context_switches": int(dirty),
            "affinity_cpu_before": 2,
            "affinity_cpu_after": 2,
            "affinity_count_before": 1,
            "affinity_count_after": 1,
        }
        return {"sample": {"arm": arm, "arm_envelope": dict(scope),
                           "primary": dict(scope), "frontend": dict(scope)},
                "process": {"returncode": 0}}

    clean = acquire(plan, 2, lambda order: {arm: sample(arm) for arm in order})
    assert clean["terminal_outcome"] == "accepted"
    assert [row["accepted_AB"] for row in clean["blocks"]] == [26, 25, 26]
    assert [row["accepted_BA"] for row in clean["blocks"]] == [25, 26, 25]
    calls = 0

    def seventeen_then_clean(order: tuple[str, str]) -> dict[str, object]:
        nonlocal calls
        dirty = calls < 17
        calls += 1
        return {arm: sample(arm, dirty and arm == order[0]) for arm in order}

    bounded = acquire(plan, 2, seventeen_then_clean)
    assert bounded["terminal_outcome"] == "accepted"
    assert bounded["blocks"][0]["attempt_count"] == 68
    calls = 0

    def eighteen_dirty(order: tuple[str, str]) -> dict[str, object]:
        nonlocal calls
        calls += 1
        return {arm: sample(arm, calls <= 18) for arm in order}

    rejected = acquire(plan, 2, eighteen_dirty)
    assert rejected["terminal_outcome"] == "NoSafeSlice"
    assert rejected["blocks"][0]["scheduler_rejected_count"] == 18
    assert len(rejected["attempts"]) == 18
    fatal = acquire(plan, 2, lambda order: (_ for _ in ()).throw(
        FatalPairObservation("test.fatal", "fatal")))
    assert fatal["terminal_outcome"] == "NoSafeSlice"
    assert len(fatal["attempts"]) == 1
    missing = acquire(plan, 2, lambda order: {"hako": sample("hako")})
    assert missing["terminal_reason"] == "malformed_pair_observation"


if __name__ == "__main__":
    self_test()
    print("[s6c-native-acquisition] self-test ok")
