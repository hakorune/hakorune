#!/usr/bin/env python3
"""Sealed retain-all paired wall-clock plan and session validator."""

from __future__ import annotations

import hashlib
import json
import math
import statistics
from typing import Iterable


PROTOCOL = "s6c-meso-paired-wallclock-plan-v1"
PAIR_COUNT = 51
BLOCK_COUNT = 3
BLOCK_SIZE = 17
THRESHOLD = 1.15
FAMILIES = ("ascii", "width2", "width3", "width4", "mixed")
SIZES = (32, 256, 4096, 1048576)
POSITIONS = ("first", "middle", "last", "miss")
CANONICAL_CASES = tuple(f"{family}/{size}/{position}" for family in FAMILIES
                        for size in SIZES for position in POSITIONS)


class InvalidSession(ValueError):
    pass


def _canonical(value: object) -> bytes:
    return json.dumps(value, sort_keys=True, separators=(",", ":")).encode()


def _block_orders(binary_sha256: str, case: str, block: int, session_index: int) -> list[str]:
    ab_count = 9 if (block + session_index) % 2 == 0 else 8
    ranked = sorted(range(BLOCK_SIZE), key=lambda slot: hashlib.sha256(
        f"{PROTOCOL}:{binary_sha256}:{case}:{session_index}:{block}:{slot}".encode()).digest())
    ab_slots = set(ranked[:ab_count])
    return ["AB" if slot in ab_slots else "BA" for slot in range(BLOCK_SIZE)]


def seal_plan(
        *, commit: str, binary_sha256: str, cases: Iterable[str],
        environment_class: str, session_index: int = 0) -> dict[str, object]:
    if environment_class != "wsl_development":
        raise InvalidSession("unknown environment authority class")
    case_list = sorted(set(cases))
    if not case_list or len(binary_sha256) != 64 or len(commit) != 40 or session_index not in (0, 1):
        raise InvalidSession("plan identity/case census is incomplete")
    schedules = {case: [order for block in range(BLOCK_COUNT)
                        for order in _block_orders(binary_sha256, case, block, session_index)]
                 for case in case_list}
    plan = {
        "schema": PROTOCOL,
        "commit": commit,
        "binary_sha256": binary_sha256,
        "environment_class": environment_class,
        "session_index": session_index,
        "cases": case_list,
        "pair_count": PAIR_COUNT,
        "blocks": BLOCK_COUNT,
        "pairs_per_block": BLOCK_SIZE,
        "minimum_arm_ns": 30_000_000,
        "threshold_4k_plus_p50": THRESHOLD,
        "sample_policy": "retain_all_no_retry_no_outlier_removal",
        "p95_policy": "nearest_rank_diagnostic_only",
        "schedules": schedules,
    }
    return {**plan, "plan_sha256": hashlib.sha256(_canonical(plan)).hexdigest()}


def nearest_rank(values: list[float], percentile: float) -> float:
    return sorted(values)[math.ceil(percentile * len(values)) - 1]


def _median(values: list[float]) -> float:
    if not values:
        raise InvalidSession("empty ratio stratum")
    return statistics.median(values)


def validate_session(plan: dict[str, object], rows: list[dict[str, object]]) -> dict[str, object]:
    if plan.get("schema") != PROTOCOL or not plan.get("plan_sha256"):
        raise InvalidSession("unsealed plan")
    expected_ids = {(case, slot) for case in plan["cases"] for slot in range(PAIR_COUNT)}
    observed_ids: set[tuple[str, int]] = set()
    grouped = {case: [] for case in plan["cases"]}
    for row in rows:
        case, slot = row.get("case"), row.get("slot")
        identity = (case, slot)
        if identity not in expected_ids or identity in observed_ids:
            raise InvalidSession("missing/duplicate/foreign pair identity")
        observed_ids.add(identity)
        block, block_slot = divmod(slot, BLOCK_SIZE)
        expected_order = plan["schedules"][case][slot]
        if row.get("block") != block or row.get("block_slot") != block_slot or \
                row.get("order") != expected_order:
            raise InvalidSession("sealed pair order/block drift")
        hako_ns, c_ns = row.get("hako_ns"), row.get("c_ns")
        if not isinstance(hako_ns, int) or not isinstance(c_ns, int) or \
                min(hako_ns, c_ns) < plan["minimum_arm_ns"]:
            raise InvalidSession("missing or short measured arm")
        if row.get("oracle_equal") is not True or row.get("attempt") != 1:
            raise InvalidSession("oracle drift or retry detected")
        ratio = hako_ns / c_ns
        if not math.isfinite(ratio) or ratio <= 0:
            raise InvalidSession("invalid paired ratio")
        grouped[case].append({"slot": slot, "block": block, "order": expected_order,
                              "ratio": ratio, "hako_ns": hako_ns, "c_ns": c_ns})
    if observed_ids != expected_ids:
        raise InvalidSession("incomplete retain-all session")

    case_stats = {}
    gated_states = []
    for case, samples in grouped.items():
        samples.sort(key=lambda row: row["slot"])
        ratios = [row["ratio"] for row in samples]
        overall = _median(ratios)
        strata = {order: _median([row["ratio"] for row in samples if row["order"] == order])
                  for order in ("AB", "BA")}
        blocks = [_median([row["ratio"] for row in samples if row["block"] == block])
                  for block in range(BLOCK_COUNT)]
        size = int(case.split("/")[1])
        state = "informational"
        if size >= 4096:
            if overall > THRESHOLD:
                state = "red"
            elif any(value > THRESHOLD for value in strata.values()) or \
                    sum(value <= THRESHOLD for value in blocks) < 2:
                state = "inconclusive"
            else:
                state = "green"
            gated_states.append(state)
        case_stats[case] = {"p50": overall, "p95_diagnostic": nearest_rank(ratios, 0.95),
                            "order_strata_p50": strata, "block_p50": blocks,
                            "state": state, "samples": samples}
    if "red" in gated_states:
        outcome = "red"
    elif "inconclusive" in gated_states:
        outcome = "inconclusive"
    else:
        outcome = "green"
    return {"schema": "s6c-meso-paired-wallclock-session-v1",
            "plan_sha256": plan["plan_sha256"], "authority":
            "development-evidence-only",
            "outcome": f"development_{outcome}", "cases": case_stats,
            "session_index": plan["session_index"], "all_pairs_retained": True,
            "pair_count_per_case": PAIR_COUNT}


def _rows(plan: dict[str, object], ratios: dict[int, float] | None = None) -> list[dict[str, object]]:
    ratios = ratios or {}
    rows = []
    for case in plan["cases"]:
        for slot, order in enumerate(plan["schedules"][case]):
            ratio = ratios.get(slot, 1.0)
            rows.append({"case": case, "slot": slot, "block": slot // BLOCK_SIZE,
                         "block_slot": slot % BLOCK_SIZE, "order": order,
                         "hako_ns": int(40_000_000 * ratio), "c_ns": 40_000_000,
                         "oracle_equal": True, "attempt": 1})
    return rows


def self_test() -> None:
    plan = seal_plan(commit="a" * 40, binary_sha256="b" * 64,
                     cases=["mixed/4096/first"], environment_class="wsl_development")
    orders = plan["schedules"]["mixed/4096/first"]
    assert [orders[i * 17:(i + 1) * 17].count("AB") for i in range(3)] == [9, 8, 9]
    assert orders.count("AB") == 26 and orders.count("BA") == 25
    assert validate_session(plan, _rows(plan))["outcome"] == "development_green"
    one_outlier = _rows(plan, {0: 10.0})
    report = validate_session(plan, one_outlier)
    assert report["outcome"] == "development_green"
    assert report["cases"]["mixed/4096/first"]["p95_diagnostic"] == 1.0
    assert validate_session(plan, _rows(plan, {slot: 1.2 for slot in range(26)}))[
        "outcome"] == "development_red"
    ab_slots = [slot for slot, order in enumerate(orders) if order == "AB"][:14]
    assert validate_session(plan, _rows(plan, {slot: 1.2 for slot in ab_slots}))[
        "outcome"] == "development_inconclusive"
    two_red_blocks = {slot: 1.2 for block in (0, 1)
                      for slot in range(block * BLOCK_SIZE, block * BLOCK_SIZE + 9)}
    assert validate_session(plan, _rows(plan, two_red_blocks))[
        "outcome"] == "development_inconclusive"
    mutation = _rows(plan)
    mutation[0]["order"] = "BA" if mutation[0]["order"] == "AB" else "AB"
    try:
        validate_session(plan, mutation)
    except InvalidSession:
        pass
    else:
        raise AssertionError("order mutation accepted")
    missing = _rows(plan)[:-1]
    try:
        validate_session(plan, missing)
    except InvalidSession:
        pass
    else:
        raise AssertionError("missing pair accepted")
    second = seal_plan(commit="a" * 40, binary_sha256="b" * 64,
                       cases=["mixed/4096/first"], environment_class="wsl_development",
                       session_index=1)
    assert second["schedules"]["mixed/4096/first"].count("AB") == 25
    try:
        seal_plan(commit="a" * 40, binary_sha256="b" * 64,
                  cases=["mixed/4096/first"], environment_class="native_final")
    except InvalidSession:
        pass
    else:
        raise AssertionError("latent native wall-clock role accepted")


if __name__ == "__main__":
    self_test()
    print("[s6c-paired-wallclock-plan] self-test ok")
