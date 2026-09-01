"""Guard for the bounded ordinary-static target-only cutover."""

from pathlib import Path


ORDINARY_STATIC_TARGET_ONLY_I0_ROW = (
    "MIR-CALL-SAME-MODULE-ORDINARY-STATIC-CATALOGED-TARGET-ONLY-I0"
)
ORDINARY_STATIC_TARGET_ONLY_I0_KEY = (
    "same_module_ordinary_static_cataloged_target_only_i0_2026_08_30"
)
ORDINARY_STATIC_TARGET_ONLY_RESIDUAL_I0_ROW = (
    "MIR-CALL-SAME-MODULE-ORDINARY-STATIC-TARGET-ONLY-RESIDUAL-I0"
)
ORDINARY_STATIC_TARGET_ONLY_RESIDUAL_I0_KEY = (
    "same_module_ordinary_static_target_only_residual_corrective_d0_2026_09_01"
)
D0_KEY = "same_module_ordinary_static_cataloged_target_only_d0_2026_08_30"
PARENT_KEY = "same_module_all_producer_disposition_r0_2026_08_30"


def _text(root: Path, rel: str) -> str:
    path = root / rel
    if not path.is_file():
        raise SystemExit(f"[mir-call-d1b-target-only] missing owner: {rel}")
    if sum(1 for _ in path.open()) >= 760:
        raise SystemExit(f"[mir-call-d1b-target-only] owner reached 760 lines: {rel}")
    return path.read_text()


def check_ordinary_static_target_only_i0(
    state: dict, card: dict, root: Path, api: object
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        api.fail("ordinary-static target-only I0 requires fast or closeout work_mode")
    if state.get("current_execution_row") != ORDINARY_STATIC_TARGET_ONLY_I0_ROW:
        api.fail("ordinary-static target-only I0 is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        api.fail("ordinary-static target-only I0 must clear current_design_stop")
    if state.get("next_execution_card") != ORDINARY_STATIC_TARGET_ONLY_I0_ROW:
        api.fail("ordinary-static target-only I0 pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("ordinary-static target-only I0 card pointer drifted")

    parent = card.get(PARENT_KEY)
    if not isinstance(parent, dict) or parent.get("status") != "accepted_with_open_blockers":
        api.fail("ordinary-static target-only I0 requires the blocker-open parent")
    if parent.get("implementation_permission") is not False:
        api.fail("ordinary-static target-only I0 cannot open broad implementation")

    d0 = card.get(D0_KEY)
    if not isinstance(d0, dict) or d0.get("status") != "accepted_design_ready_for_i0":
        api.fail("ordinary-static target-only D0 is not accepted")

    row = card.get(ORDINARY_STATIC_TARGET_ONLY_I0_KEY)
    if not isinstance(row, dict) or row.get("task_id") != ORDINARY_STATIC_TARGET_ONLY_I0_ROW:
        api.fail("ordinary-static target-only I0 card is missing")
    if row.get("status") not in {"fast_open", "landed"}:
        api.fail("ordinary-static target-only I0 status is not finite")
    if row.get("implementation_permission") is not (row.get("status") == "fast_open"):
        api.fail("ordinary-static target-only I0 permission/status drifted")

    owner = "src/mir/callable_result_representation/static_call_result_publication_owner.rs"
    owner_tests = "src/mir/callable_result_representation/tests/static_call_result_publication_owner.rs"
    ingress = "src/mir/builder/static_result_publication_ingress.rs"
    handlers = "src/mir/builder/method_call_handlers.rs"
    member = "src/mir/builder/calls/member_route.rs"
    bridge = "src/mir/builder/calls/static_result_publication_physical_bridge.rs"
    terminal = "src/mir/builder/calls/method_call_terminal.rs"
    calls_mod = "src/mir/builder/calls/mod.rs"
    allowed = {
        owner,
        owner_tests,
        ingress,
        handlers,
        member,
        bridge,
        terminal,
        calls_mod,
        "src/mir/builder/README.md",
        str(api.HELPER_REL),
        "tools/checks/lib/mir_call_d1b_same_module_target_only_guard.py",
        str(api.STATE_REL),
        str(api.CARD_REL),
        "docs/development/current/main/workstreams/mirbuilder-inplace-replacement-current.md",
    }
    declared = row.get("allowed_files")
    if not isinstance(declared, list) or set(declared) != allowed:
        api.fail("ordinary-static target-only I0 allowed-file boundary drifted")

    base = row.get("base_commit")
    if not isinstance(base, str) or not base:
        api.fail("ordinary-static target-only I0 base_commit is missing")
    changed = api.git_diff_paths(root, base)
    if not changed <= allowed:
        api.fail(f"ordinary-static target-only I0 changed paths escaped: {sorted(changed - allowed)}")

    texts = {
        rel: _text(root, rel)
        for rel in (owner, owner_tests, ingress, handlers, member, bridge, terminal, calls_mod)
    }
    if row.get("status") == "fast_open":
        return

    for rel in (owner, ingress, handlers, member, bridge):
        if rel == bridge:
            if "lower_target_only_static_result_publication_v1" not in texts[rel]:
                api.fail(f"target-only I0 missing TargetOnly consumer evidence: {rel}")
            continue
        if "TargetOnly" not in texts[rel] and rel != owner:
            api.fail(f"target-only I0 missing TargetOnly consumer evidence: {rel}")
    for token in ("TargetOnly(", "NoExactStaticTarget", "consumed"):
        if token not in texts[owner]:
            api.fail(f"target-only owner lacks lifecycle token: {token}")
    if "StaticCallResultPublicationTakeV1::Unselected" in texts[owner] + texts[ingress]:
        api.fail("target-only I0 left the collapsed Unselected state")
    if "StaticResultPublicationIngressV1::Absent" in texts[ingress] + texts[handlers] + texts[member]:
        api.fail("target-only I0 left Absent as a fallback selector")
    if "lower_target_only_static_result_publication_v1" not in texts[bridge]:
        api.fail("target-only I0 lacks the physical target-only consumer")
    if "emit_static_global_target_value_terminal_v1" not in texts[terminal]:
        api.fail("target-only I0 lacks a typed-target terminal")
    changed_test_names = row.get("changed_test_names")
    if not isinstance(changed_test_names, list) or not changed_test_names:
        api.fail("target-only I0 must record its changed focused tests")
    for test_name in changed_test_names:
        if not isinstance(test_name, str) or f"fn {test_name}(" not in texts[owner_tests] + texts[ingress]:
            api.fail(f"target-only I0 focused test is missing: {test_name}")
    projection_at = texts[bridge].find("canonical_global_target_v1()")
    descent_at = texts[bridge].find("descent.lower_all(builder)")
    if projection_at < 0 or descent_at < 0 or projection_at > descent_at:
        api.fail("target-only I0 must project the typed target before argument descent")


def check_ordinary_static_target_only_residual_i0(
    state: dict, card: dict, root: Path, api: object
) -> None:
    if state.get("work_mode") not in {"fast", "closeout"}:
        api.fail("target-only residual I0 requires fast or closeout work_mode")
    if state.get("current_execution_row") != ORDINARY_STATIC_TARGET_ONLY_RESIDUAL_I0_ROW:
        api.fail("target-only residual I0 is not selected by CURRENT_STATE")
    if state.get("current_design_stop") != "none":
        api.fail("target-only residual I0 must clear current_design_stop")
    if state.get("next_execution_card") != ORDINARY_STATIC_TARGET_ONLY_RESIDUAL_I0_ROW:
        api.fail("target-only residual I0 pointer drifted")
    if state.get("next_execution_card_path") != str(api.CARD_REL):
        api.fail("target-only residual I0 card pointer drifted")

    row = card.get(ORDINARY_STATIC_TARGET_ONLY_RESIDUAL_I0_KEY)
    if not isinstance(row, dict) or row.get("task_id") != ORDINARY_STATIC_TARGET_ONLY_RESIDUAL_I0_ROW:
        api.fail("target-only residual I0 card is missing")
    status = row.get("status")
    if status not in {"fast_open", "landed"}:
        api.fail("target-only residual I0 status is not finite")
    if row.get("implementation_permission") is not (status == "fast_open"):
        api.fail("target-only residual I0 permission/status drifted")

    allowed = row.get("allowed_files")
    if not isinstance(allowed, list) or not allowed:
        api.fail("target-only residual I0 allowed_files are missing")
    allowed_set = set(allowed)
    required = {
        "src/mir/callable_result_representation/static_call_result_publication_owner.rs",
        "src/mir/callable_result_representation/tests/static_call_result_publication_owner.rs",
        "src/mir/builder/module_draft_collector/static_result_publication_owner.rs",
        "src/mir/builder/module_draft_collector/normal_collector_drain_lifecycle.rs",
        "src/mir/builder/program_root_lowering.rs",
        "src/mir/builder/normal_default_root_catalog_post_install.rs",
        "tools/checks/lib/mir_call_d1b_same_module_target_only_guard.py",
        "tools/checks/lib/mir_call_d1b_active_surface_dispatch.py",
        str(api.STATE_REL),
        str(api.CARD_REL),
    }
    if not required <= allowed_set:
        api.fail(f"target-only residual I0 allowed_files omit {sorted(required - allowed_set)}")
    base = row.get("base_commit")
    if not isinstance(base, str) or not base:
        api.fail("target-only residual I0 base_commit is missing")
    changed = api.git_diff_paths(root, base)
    if not changed <= allowed_set:
        api.fail(f"target-only residual I0 changed paths escaped: {sorted(changed - allowed_set)}")

    owner = root / "src/mir/callable_result_representation/static_call_result_publication_owner.rs"
    tests = root / "src/mir/callable_result_representation/tests/static_call_result_publication_owner.rs"
    post_install = root / "src/mir/builder/normal_default_root_catalog_post_install.rs"
    program = root / "src/mir/builder/program_root_lowering.rs"
    collector = root / "src/mir/builder/module_draft_collector/static_result_publication_owner.rs"
    lifecycle = root / "src/mir/builder/module_draft_collector/normal_collector_drain_lifecycle.rs"
    for path in (owner, tests, post_install, program, collector, lifecycle):
        rel = path.relative_to(root).as_posix()
        _text(root, rel)
    if status == "fast_open":
        return

    owner_text = owner.read_text()
    if "StaticCallResultPublicationOwnerFinishErrorV1" not in owner_text:
        api.fail("target-only residual owner lacks typed finish error")
    if "finish_empty" not in owner_text:
        api.fail("target-only residual owner lacks finish_empty")
    if "target_only_targets" not in owner_text or "rows" not in owner_text:
        api.fail("target-only residual owner lacks both pending dispositions")
    if "Option<VerifiedStaticCallResultPublicationOwnerV1>" not in program.read_text():
        api.fail("program lowering does not carry the optional source-backed owner")
    if "if let Some(owner)" not in program.read_text():
        api.fail("program lowering does not conditionally install the source-backed owner")
    if "finish_empty" not in lifecycle.read_text():
        api.fail("normal drain does not check publication-owner residuals")
    if "VerifiedStaticCallResultPublicationOwnerV1::issue" in post_install.read_text():
        api.fail("compatibility post-install still reconstructs a publication owner")
    changed_test_names = row.get("changed_test_names")
    if not isinstance(changed_test_names, list) or not changed_test_names:
        api.fail("target-only residual I0 must record changed focused tests")
    test_text = tests.read_text()
    for test_name in changed_test_names:
        if not isinstance(test_name, str) or f"fn {test_name}(" not in test_text:
            api.fail(f"target-only residual focused test is missing: {test_name}")
