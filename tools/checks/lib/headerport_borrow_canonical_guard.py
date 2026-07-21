#!/usr/bin/env python3
"""Reusable HEADERPORT0 canonical candidate-borrow proof."""

from __future__ import annotations

import pathlib


def require(text: str, fragment: str, label: str) -> None:
    if fragment not in text:
        raise AssertionError(f"missing {label}: {fragment!r}")


def verify_borrow_canonical_p0(root: pathlib.Path, card: str, state: str) -> None:
    compiler_path = root / "src/mir/compiler/mod.rs"
    session_path = root / "src/mir/compiler/module_session.rs"
    proof_path = root / "src/mir/compiler/module_session_borrow_p0_tests.rs"
    matrix_path = root / "src/mir/builder/module_wiring_route_matrix_p0e.rs"
    function_input_path = root / "src/mir/compiler/function_input.rs"
    direct_call_path = root / "src/mir/builder/resolved_lowering/trivial_ssa/direct_call.rs"
    acyclic_path = root / "src/mir/compiler/acyclic_callable_module_activation_tests.rs"
    recursive_path = root / "src/mir/compiler/recursive_callable_module_activation_tests.rs"
    compiler = compiler_path.read_text()
    session = session_path.read_text()
    proof = proof_path.read_text()
    matrix = matrix_path.read_text()
    function_input = function_input_path.read_text()
    direct_call = direct_call_path.read_text()

    for path, source in ((session_path, session), (proof_path, proof)):
        if len(source.splitlines()) >= 800:
            raise AssertionError(f"BORROW-P0-CANONICAL source/proof reached 800 lines: {path}")

    first = compiler.split("fn compile_resolved_first_family(", 1)[1].split(
        "fn finish_built_canonical_module(", 1
    )[0]
    acyclic = compiler.split("pub fn compile_resolved_callable_module(", 1)[1].split(
        "pub fn compile_resolved_recursive_callable_module(", 1
    )[0]
    recursive = compiler.split("pub fn compile_resolved_recursive_callable_module(", 1)[1].split(
        "pub fn compile_legacy(", 1
    )[0]
    for route, label in ((first, "first-family"), (acyclic, "acyclic"), (recursive, "recursive")):
        phases = (
            "CanonicalModuleLoweringSessionV1::open(&self.builder)",
            "finish_built_canonical_module(",
            "commit(&mut self.builder)",
        )
        for fragment in phases:
            require(route, fragment, f"BORROW-P0-CANONICAL {label} phase")
        if not (route.index(phases[0]) < route.index(phases[1]) < route.index(phases[2])):
            raise AssertionError(f"BORROW-P0-CANONICAL {label} phase order drift")

    for fragment in (
        "candidate: MirBuilder",
        "pub(super) fn open(current: &MirBuilder)",
        "pub(super) fn commit(self, current: &mut MirBuilder)",
        "*current = self.candidate;",
    ):
        require(session, fragment, "BORROW-P0-CANONICAL candidate owner")
    for fragment in (
        "canonical_module_session_drop_preserves_live_builder_after_candidate_mutation",
        "canonical_module_session_commit_replaces_live_builder_once",
        "recursion_depth = 91",
    ):
        require(proof, fragment, "BORROW-P0-CANONICAL session proof")
    for fragment in (
        "InvocationRootFamilyV1::CanonicalAPlus",
        "InvocationRootFamilyV1::BindingSsaTrivial",
        "InvocationRootFamilyV1::BindingSsaAcyclic",
        "InvocationRootFamilyV1::BindingSsaRecursive",
    ):
        require(matrix, fragment, "BORROW-P0-CANONICAL exact family set")
    for fragment in (
        "self.source().catalog().index().lookup(key)",
        "callable_index: Some(self.source().catalog().index())",
        "callable_header: Some(header)",
    ):
        require(function_input, fragment, "canonical immutable catalog authority")
    require(direct_call, "input.callable_header()", "canonical direct-call header authority")
    for path, fixture in (
        (acyclic_path, "zero_call_and_recursive_graphs_reject_without_poisoning_the_compiler"),
        (recursive_path, "recursive_ingress_rejects_acyclic_input_without_poisoning_compiler"),
    ):
        require(path.read_text(), fixture, "canonical compiler reuse")
    require(card, "WIRING-I0-BORROW-P0-CANONICAL closeout", "canonical closeout")
    require(state, "BORROW-P0-CANONICAL is closed; WIRING-I0-BORROW-P0-ROOT is next", "canonical pointer")
