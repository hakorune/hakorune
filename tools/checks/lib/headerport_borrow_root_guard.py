#!/usr/bin/env python3
"""Reusable HEADERPORT0 whole-invocation borrow proof."""

from __future__ import annotations

import pathlib


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def require_exact_count(text: str, fragment: str, count: int, label: str) -> None:
    actual = text.count(fragment)
    if actual != count:
        raise AssertionError(
            f"{label} count mismatch: expected={count} actual={actual} fragment={fragment!r}"
        )


def verify_borrow_root_p0(
    root: pathlib.Path,
    builder_mod: str,
    card: str,
    state: str,
) -> None:
    matrix_path = root / "src/mir/builder/module_finalization_candidate_p0.rs"
    matrix = matrix_path.read_text()
    for fragment in (
        "RootBatchPreflight",
        "DeclarationFactsSeal",
        "DrainPreflight",
        "PostDrainFinalize",
        "const FAILURE_ROWS: [ModuleFinalizationFailureRowV1; 8]",
        "root_batch_shell_drain_and_finalizer_failures_have_no_fallback_route",
    ):
        require(matrix, fragment, "BORROW-P0-ROOT failure matrix")
    production_calls = []
    excluded = {
        root / "src/mir/builder.rs",
        root / "src/mir/builder/drained_module_candidate_p0.rs",
        root / "src/mir/builder/module_invocation_drain_s0_tests.rs",
    }
    watched = (
        "ModuleLoweringInvocationDrainOwnerV1::new(",
        "InvocationDrainExpectationV1::new(",
        "CompletedInvocationInventoryV1::new(",
        "DrainedModuleCandidateV1::from_drained_module(",
        "DrainedModuleFinalizationInputV1::new(",
    )
    for path in (root / "src/mir").rglob("*.rs"):
        if path in excluded:
            continue
        source = path.read_text().split("#[cfg(test)]", 1)[0]
        if any(fragment in source for fragment in watched):
            production_calls.append(str(path.relative_to(root)))
    if production_calls:
        raise AssertionError(
            "BORROW-P0-ROOT production orchestration consumers: "
            + ", ".join(production_calls)
        )

    require(card, "WIRING-I0-BORROW-P0-ROOT-P0a closeout", "root P0a closeout")
    require(card, "WIRING-I0-BORROW-P0-ROOT-P0b closeout", "root P0b closeout")
    require(card, "WIRING-I0-BORROW-P0-ROOT-P0c closeout", "root P0c closeout")
    require(card, "WIRING-I0-BORROW-P0-ROOT-P0d closeout", "root P0d closeout")
    require(card, "WIRING-I0-BORROW-P0-ROOT-G0 closeout", "root G0 closeout")
    require(card, "WIRING-I0-BORROW-G0 closeout", "whole BORROW G0 closeout")
    if not any(
        pointer in state
        for pointer in (
            "BORROW-P0-ROOT-G0 and WIRING-I0-BORROW-G0 are closed; WIRING-I0-HDR0-M0 is next",
            "BORROW-P0-ROOT-G0 and WIRING-I0-BORROW-G0 are closed; WIRING-I0-HDR0-M0 is closed and WIRING-I0-HDR0-P0 is next",
            "BORROW-P0-ROOT-G0 and WIRING-I0-BORROW-G0 are closed; WIRING-I0-HDR0-M0 is closed; HDR0-P0 Q1-Q4 decisions are accepted and HDR0-P0-AUTHORITY-ERASURE0 is next",
            "BORROW-P0-ROOT-G0 and WIRING-I0-BORROW-G0 are closed; WIRING-I0-HDR0-M0 is closed; HDR0-P0 Q1-Q4 decisions are accepted, HDR0-P0-AUTHORITY-ERASURE0 is closed, and HDR0-P0-METHODTAIL-COMPAT0 is next",
            "BORROW-P0-ROOT-G0 and WIRING-I0-BORROW-G0 are closed; WIRING-I0-HDR0-M0 is closed; HDR0-P0 Q1-Q4 decisions are accepted, HDR0-P0-AUTHORITY-ERASURE0 and HDR0-P0-METHODTAIL-COMPAT0 are closed, and HDR0-P0-CALLER-CENSUS0 is next",
            "BORROW-P0-ROOT-G0 and WIRING-I0-BORROW-G0 are closed; WIRING-I0-HDR0-M0 is closed; HDR0-P0 Q1-Q4 decisions are accepted, HDR0-P0-AUTHORITY-ERASURE0, HDR0-P0-METHODTAIL-COMPAT0, and HDR0-P0-CALLER-CENSUS0 are closed, and HDR0-G0 is next",
            "BORROW-P0-ROOT-G0 and WIRING-I0-BORROW-G0 are closed; WIRING-I0-HDR0-M0 is closed; HDR0-P0 Q1-Q4 decisions are accepted, HDR0-P0-AUTHORITY-ERASURE0, HDR0-P0-METHODTAIL-COMPAT0, HDR0-P0-CALLER-CENSUS0, and HDR0-G0 are closed; CUT0-COMPAT-POLICY-CONSULT0-DESIGN-STOP is next",
            "BORROW-P0-ROOT-G0 and WIRING-I0-BORROW-G0 are closed; WIRING-I0-HDR0-M0 is closed; HDR0-P0 Q1-Q4 decisions are accepted, HDR0-P0-AUTHORITY-ERASURE0, HDR0-P0-METHODTAIL-COMPAT0, HDR0-P0-CALLER-CENSUS0, and HDR0-G0 are closed; CUT0-COMPAT-POLICY-CONSULT0 is closed with Candidate S-prime; CUT0-S0 is next",
            "BORROW-P0-ROOT-G0 and WIRING-I0-BORROW-G0 are closed; WIRING-I0-HDR0-M0 is closed; HDR0-P0 Q1-Q4 decisions are accepted, HDR0-P0-AUTHORITY-ERASURE0, HDR0-P0-METHODTAIL-COMPAT0, HDR0-P0-CALLER-CENSUS0, and HDR0-G0 are closed; CUT0-COMPAT-POLICY-CONSULT0 is closed with Candidate S-prime; CUT0-S0-OWNER0, CUT0-S0-COMPAT0, and CUT0-P0 are closed as disconnected proofs, CUT0-I0 is next",
        )
    ):
        raise AssertionError("missing BORROW-G0 pointer or its documented HDR0 follow-on")
