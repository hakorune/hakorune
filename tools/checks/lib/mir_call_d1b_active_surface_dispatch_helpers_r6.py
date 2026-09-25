"""Extended R6-S2/S3 check owners for the stable dispatcher."""

from __future__ import annotations

from pathlib import Path


R6S2_PUBLISHED_VIEW_GLOBAL_STOP_S2_ROW = (
    "MIR-CALL-R6S2-PUBLISHED-VIEW-GLOBAL-STOP-S2"
)
R6S2_PUBLISHED_VIEW_GLOBAL_STOP_S2_CARD_REL = Path(
    "docs/development/current/main/investigations/"
    "mir-call-r6s2-published-view-global-stop-s2-2026-09-25.md"
)


def check_r6s2_published_view_global_stop_s2(state: dict, root: Path, api) -> None:
    """Pin the R6-S2 published-view legacy-Global stop surface."""
    row = R6S2_PUBLISHED_VIEW_GLOBAL_STOP_S2_ROW
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{row} must be fast or closeout")
    if state.get("current_execution_row") != row:
        api.fail(f"{row} pointer row drifted")
    if not str(state.get("current_design_stop", "")).startswith("none"):
        api.fail(f"{row} must clear current_design_stop")
    if not str(state.get("next_design_card", "")).startswith("none"):
        api.fail(f"{row} must not open a second design card")
    expected_next = row if mode == "fast" else "none"
    if not str(state.get("next_execution_card", "")).startswith(expected_next):
        api.fail(f"{row} next_execution_card drifted")
    card_rel = str(R6S2_PUBLISHED_VIEW_GLOBAL_STOP_S2_CARD_REL)
    if state.get("next_execution_card_path") != card_rel:
        api.fail(f"{row} next_execution_card_path drifted")
    if state.get("latest_card_path") != card_rel:
        api.fail(f"{row} latest_card_path drifted")

    card_text = (root / card_rel).read_text(encoding="utf-8")
    for token in (row, "SelectedNormalUsesLegacyCallV0", "UnsupportedBeforeObject"):
        if token not in card_text:
            api.fail(f"{row} contract is missing: {token}")

    view = (
        root
        / "src/mir/compiler/normal_default_pipeline/published_backend_view.rs"
    ).read_text(encoding="utf-8")
    if "!canonical_call && func == ValueId::INVALID" not in view:
        api.fail(f"{row} try_new lost the legacy-Global stop arm")
    if "legacy_global" not in view:
        api.fail(f"{row} selected admission lost the Global stop flag")
    if view.count("SelectedNormalUsesLegacyCallV0") < 3:
        api.fail(f"{row} selected admission lost its named terminal")

    tests = (
        root / "src/mir/function/published_backend_view_tests.rs"
    ).read_text(encoding="utf-8")
    if "published_view_stops_legacy_global_before_object" not in tests:
        api.fail(f"{row} legacy-Global view stop pin missing")
    admission_tests = (
        root
        / "src/mir/function/published_backend_view_selected_admission_tests.rs"
    ).read_text(encoding="utf-8")
    if "selected_normal_admission_stops_legacy_only_global_input" not in (
        admission_tests
    ):
        api.fail(f"{row} selected admission stop pin missing")

    rel = "src/mir/compiler/normal_default_pipeline/published_backend_view.rs"
    path = root / rel
    if not path.is_file():
        api.fail(f"{row} implementation owner is missing: {rel}")
    if sum(1 for _ in path.open(encoding="utf-8")) >= 800:
        api.fail(f"{row} implementation owner reached 800 lines: {rel}")
    print(f"[{api.TAG}] row={row} delegated=r6s2-published-view-global-stop")


R6S3_SELECTED_DYNAMIC_LEGACY_STOP_S3_ROW = (
    "MIR-CALL-R6S3-SELECTED-DYNAMIC-LEGACY-STOP-S3"
)
R6S3_SELECTED_DYNAMIC_LEGACY_STOP_S3_CARD_REL = Path(
    "docs/development/current/main/investigations/"
    "mir-call-r6s3-selected-dynamic-legacy-stop-s3-2026-09-25.md"
)


def check_r6s3_selected_dynamic_legacy_stop_s3(
    state: dict, root: Path, api
) -> None:
    """Pin the R6-S3 selected-Dynamic legacy-carrier stop surface."""
    row = R6S3_SELECTED_DYNAMIC_LEGACY_STOP_S3_ROW
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{row} must be fast or closeout")
    if state.get("current_execution_row") != row:
        api.fail(f"{row} pointer row drifted")
    if not str(state.get("current_design_stop", "")).startswith("none"):
        api.fail(f"{row} must clear current_design_stop")
    if not str(state.get("next_design_card", "")).startswith("none"):
        api.fail(f"{row} must not open a second design card")
    expected_next = row if mode == "fast" else "none"
    if not str(state.get("next_execution_card", "")).startswith(expected_next):
        api.fail(f"{row} next_execution_card drifted")
    card_rel = str(R6S3_SELECTED_DYNAMIC_LEGACY_STOP_S3_CARD_REL)
    if state.get("next_execution_card_path") != card_rel:
        api.fail(f"{row} next_execution_card_path drifted")
    if state.get("latest_card_path") != card_rel:
        api.fail(f"{row} latest_card_path drifted")

    card_text = (root / card_rel).read_text(encoding="utf-8")
    for token in (row, "call-legacy-carrier", "LegacyCallV0"):
        if token not in card_text:
            api.fail(f"{row} contract is missing: {token}")

    lane = (root / "src/runner/product/llvm/mod.rs").read_text(encoding="utf-8")
    for token in (
        "selected_dynamic_callsite_reject_code",
        "call-legacy-carrier",
        "legacy_callsite_reject_code",
        "selected_legacy_callsite_scan_rejects_every_legacy_carrier",
        "selected_legacy_callsite_scan_rejects_legacy_carrier_in_terminator",
    ):
        if token not in lane:
            api.fail(f"{row} selected lane lost {token}")

    allowlists = (
        root / "src/mir/contracts/backend_core_ops/allowlists.rs"
    ).read_text(encoding="utf-8")
    if "call-legacy-carrier" in allowlists:
        api.fail(f"{row} shared predicate widened into compat lanes")

    rel = "src/runner/product/llvm/mod.rs"
    path = root / rel
    if not path.is_file():
        api.fail(f"{row} implementation owner is missing: {rel}")
    if sum(1 for _ in path.open(encoding="utf-8")) >= 800:
        api.fail(f"{row} implementation owner reached 800 lines: {rel}")
    print(f"[{api.TAG}] row={row} delegated=r6s3-selected-dynamic-stop")


UNIFIED_OFF_VALUE_MINT_PROMOTE_S4_ROW = (
    "MIR-CALL-UNIFIED-OFF-VALUE-MINT-PROMOTE-S4"
)
UNIFIED_OFF_VALUE_MINT_PROMOTE_S4_CARD_REL = Path(
    "docs/development/current/main/investigations/"
    "mir-call-unified-off-value-mint-promote-s4-2026-09-25.md"
)


def check_unified_off_value_mint_promote_s4(
    state: dict, root: Path, api
) -> None:
    """Pin the S4 unified-off Value mint promote surface."""
    row = UNIFIED_OFF_VALUE_MINT_PROMOTE_S4_ROW
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{row} must be fast or closeout")
    if state.get("current_execution_row") != row:
        api.fail(f"{row} pointer row drifted")
    if not str(state.get("current_design_stop", "")).startswith("none"):
        api.fail(f"{row} must clear current_design_stop")
    if not str(state.get("next_design_card", "")).startswith("none"):
        api.fail(f"{row} must not open a second design card")
    expected_next = row if mode == "fast" else "none"
    if not str(state.get("next_execution_card", "")).startswith(expected_next):
        api.fail(f"{row} next_execution_card drifted")
    card_rel = str(UNIFIED_OFF_VALUE_MINT_PROMOTE_S4_CARD_REL)
    if state.get("next_execution_card_path") != card_rel:
        api.fail(f"{row} next_execution_card_path drifted")
    if state.get("latest_card_path") != card_rel:
        api.fail(f"{row} latest_card_path drifted")

    card_text = (root / card_rel).read_text(encoding="utf-8")
    for token in (row, "Callee::Value", "canonical-call"):
        if token not in card_text:
            api.fail(f"{row} contract is missing: {token}")

    compat = (
        root / "src/mir/builder/calls/unified_emitter/compat_entrypoints.rs"
    ).read_text(encoding="utf-8")
    for token in ("MirInstruction::call(", "Callee::Value(func_val)"):
        if token not in compat:
            api.fail(f"{row} typed Value mint lost {token}")
    if "LegacyCallV0" in compat:
        api.fail(f"{row} legacy Value mint re-entered compat_entrypoints")

    receipt_tests = (
        root
        / "src/mir/builder/calls/unified_emitter/physical_receipt_tests.rs"
    ).read_text(encoding="utf-8")
    if "ordinary_value_call_under_disabled_profile_mints_canonical_value_callee" not in (
        receipt_tests
    ):
        api.fail(f"{row} unified-off Value mint pin missing")

    interpreter = (
        root / "src/backend/mir_interpreter/handlers/calls/mod.rs"
    ).read_text(encoding="utf-8")
    for token in (
        "canonical_value_call_rejects_at_global_only_boundary",
        "[vm-reference/canonical-call] only Global targets are admitted",
    ):
        if token not in interpreter:
            api.fail(f"{row} interpreter canonical Value stop pin missing")

    emit_tests = (
        root / "src/runner/mir_json_emit/emitters/calls.rs"
    ).read_text(encoding="utf-8")
    if "v0_value_call_wire_is_identical_across_typed_and_legacy_carriers" not in (
        emit_tests
    ):
        api.fail(f"{row} v0 wire parity pin missing")

    v0_module = (root / "src/runner/mir_json_v0/module.rs").read_text(
        encoding="utf-8"
    )
    if "boxcall" not in v0_module:
        api.fail(f"{row} quarantined v0 boxcall ingress was removed")

    for rel in (
        "src/mir/builder/calls/unified_emitter/compat_entrypoints.rs",
        "src/mir/builder/calls/unified_emitter/physical_receipt_tests.rs",
        "src/backend/mir_interpreter/handlers/calls/mod.rs",
        "src/runner/mir_json_emit/emitters/calls.rs",
    ):
        path = root / rel
        if not path.is_file():
            api.fail(f"{row} implementation owner is missing: {rel}")
        if sum(1 for _ in path.open(encoding="utf-8")) >= 800:
            api.fail(f"{row} implementation owner reached 800 lines: {rel}")
    print(f"[{api.TAG}] row={row} delegated=unified-off-value-mint-promote")


V0_BOXCALL_MINT_PROMOTE_S5_ROW = (
    "MIR-CALL-V0-BOXCALL-MINT-PROMOTE-S5"
)
V0_BOXCALL_MINT_PROMOTE_S5_CARD_REL = Path(
    "docs/development/current/main/investigations/"
    "mir-call-v0-boxcall-mint-promote-s5-2026-09-25.md"
)


def check_v0_boxcall_mint_promote_s5(
    state: dict, root: Path, api
) -> None:
    """Pin the S5 v0 boxcall mint promote surface."""
    row = V0_BOXCALL_MINT_PROMOTE_S5_ROW
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{row} must be fast or closeout")
    if state.get("current_execution_row") != row:
        api.fail(f"{row} pointer row drifted")
    if not str(state.get("current_design_stop", "")).startswith("none"):
        api.fail(f"{row} must clear current_design_stop")
    if not str(state.get("next_design_card", "")).startswith("none"):
        api.fail(f"{row} must not open a second design card")
    expected_next = row if mode == "fast" else "none"
    if not str(state.get("next_execution_card", "")).startswith(expected_next):
        api.fail(f"{row} next_execution_card drifted")
    card_rel = str(V0_BOXCALL_MINT_PROMOTE_S5_CARD_REL)
    if state.get("next_execution_card_path") != card_rel:
        api.fail(f"{row} next_execution_card_path drifted")
    if state.get("latest_card_path") != card_rel:
        api.fail(f"{row} latest_card_path drifted")

    card_text = (root / card_rel).read_text(encoding="utf-8")
    for token in (row, "Callee::Method", "boxcall"):
        if token not in card_text:
            api.fail(f"{row} contract is missing: {token}")

    v0_module = (root / "src/runner/mir_json_v0/module.rs").read_text(
        encoding="utf-8"
    )
    for token in ("\"boxcall\"", "method_call("):
        if token not in v0_module:
            api.fail(f"{row} v0 boxcall ingress lost {token}")
    if "LegacyCallV0" in v0_module:
        api.fail(f"{row} legacy carrier mint re-entered mir_json_v0/module")

    v0_tests = (root / "src/runner/mir_json_v0/tests.rs").read_text(
        encoding="utf-8"
    )
    for token in (
        "boxcall_mints_canonical_method_call_carrier",
        "boxcall_without_optional_fields_uses_runtime_data_defaults",
    ):
        if token not in v0_tests:
            api.fail(f"{row} parser pin missing: {token}")

    interpreter = (
        root / "src/backend/mir_interpreter/handlers/calls/mod.rs"
    ).read_text(encoding="utf-8")
    for token in (
        "canonical_method_call_rejects_at_global_only_boundary",
        "[vm-reference/canonical-call] only Global targets are admitted",
    ):
        if token not in interpreter:
            api.fail(f"{row} interpreter canonical Method stop pin missing")

    emit_tests = (
        root / "src/runner/mir_json_emit/emitters/calls.rs"
    ).read_text(encoding="utf-8")
    if "compatibility_profile_without_methodize_keeps_boxcall" not in (
        emit_tests
    ):
        api.fail(f"{row} v0 boxcall wire projection pin missing")

    for rel in (
        "src/runner/mir_json_v0/module.rs",
        "src/runner/mir_json_v0/tests.rs",
        "src/backend/mir_interpreter/handlers/calls/mod.rs",
        "src/runner/mir_json_emit/emitters/calls.rs",
    ):
        path = root / rel
        if not path.is_file():
            api.fail(f"{row} implementation owner is missing: {rel}")
        if sum(1 for _ in path.open(encoding="utf-8")) >= 800:
            api.fail(f"{row} implementation owner reached 800 lines: {rel}")
    print(f"[{api.TAG}] row={row} delegated=v0-boxcall-mint-promote")


CANONICALIZE_LEGACY_METHOD_ARM_DELETE_S6_ROW = (
    "MIR-CALL-R7-CANONICALIZE-LEGACY-METHOD-ARM-DELETE-S6"
)
CANONICALIZE_LEGACY_METHOD_ARM_DELETE_S6_CARD_REL = Path(
    "docs/development/current/main/investigations/"
    "mir-call-r7-canonicalize-legacy-method-arm-delete-s6-2026-09-25.md"
)


def check_canonicalize_legacy_method_arm_delete_s6(
    state: dict, root: Path, api
) -> None:
    """Pin the S6 canonicalize legacy Method arm deletion surface."""
    row = CANONICALIZE_LEGACY_METHOD_ARM_DELETE_S6_ROW
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{row} must be fast or closeout")
    if state.get("current_execution_row") != row:
        api.fail(f"{row} pointer row drifted")
    if not str(state.get("current_design_stop", "")).startswith("none"):
        api.fail(f"{row} must clear current_design_stop")
    if not str(state.get("next_design_card", "")).startswith("none"):
        api.fail(f"{row} must not open a second design card")
    expected_next = row if mode == "fast" else "none"
    if not str(state.get("next_execution_card", "")).startswith(
        expected_next
    ):
        api.fail(f"{row} next_execution_card drifted")
    card_rel = str(CANONICALIZE_LEGACY_METHOD_ARM_DELETE_S6_CARD_REL)
    if state.get("next_execution_card_path") != card_rel:
        api.fail(f"{row} next_execution_card_path drifted")
    if state.get("latest_card_path") != card_rel:
        api.fail(f"{row} latest_card_path drifted")

    card_text = (root / card_rel).read_text(encoding="utf-8")
    for token in (row, "callsite_canonicalize", "LegacyCallV0"):
        if token not in card_text:
            api.fail(f"{row} contract is missing: {token}")

    pass_rel = "src/mir/passes/callsite_canonicalize/pass.rs"
    pass_text = (root / pass_rel).read_text(encoding="utf-8")
    for token in (
        "known_user_box_name_from_value",
        "collect_known_user_boxes",
        "method_call(",
        "value_types",
    ):
        if token in pass_text:
            api.fail(f"{row} deleted arm plumbing remains: {token}")
    for token in (
        "Callee::Closure",
        "Callee::Global",
        "rewrite_cfg_stable_receiver_operands",
    ):
        if token not in pass_text:
            api.fail(f"{row} retained arm lost: {token}")

    helpers_path = root / "src/mir/passes/callsite_canonicalize/helpers.rs"
    if helpers_path.exists():
        api.fail(f"{row} helpers.rs must be deleted")

    ucm_tests = (
        root / "src/mir/passes/callsite_canonicalize/tests/ucm.rs"
    ).read_text(encoding="utf-8")
    if "ucm1_residual_legacy_method_call_is_never_silently_rewritten" not in (
        ucm_tests
    ):
        api.fail(f"{row} no-laundering pin missing")

    for rel in (
        pass_rel,
        "src/mir/passes/callsite_canonicalize/tests/ucm.rs",
    ):
        path = root / rel
        if not path.is_file():
            api.fail(f"{row} implementation owner is missing: {rel}")
        if sum(1 for _ in path.open(encoding="utf-8")) >= 800:
            api.fail(f"{row} implementation owner reached 800 lines: {rel}")
    print(f"[{api.TAG}] row={row} delegated=canonicalize-legacy-method-arm-delete")


LEGACY_ARRAY_WRITE_CANON_DELETE_S7_ROW = (
    "MIR-CALL-R7-LEGACY-ARRAY-WRITE-CANON-DELETE-S7"
)
LEGACY_ARRAY_WRITE_CANON_DELETE_S7_CARD_REL = Path(
    "docs/development/current/main/investigations/"
    "mir-call-r7-legacy-array-write-canon-delete-s7-2026-09-25.md"
)


def check_legacy_array_write_canon_delete_s7(
    state: dict, root: Path, api
) -> None:
    """Pin the S7 legacy array-write repair deletion surface."""
    row = LEGACY_ARRAY_WRITE_CANON_DELETE_S7_ROW
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{row} must be fast or closeout")
    if state.get("current_execution_row") != row:
        api.fail(f"{row} pointer row drifted")
    if not str(state.get("current_design_stop", "")).startswith("none"):
        api.fail(f"{row} must clear current_design_stop")
    if not str(state.get("next_design_card", "")).startswith("none"):
        api.fail(f"{row} must not open a second design card")
    expected_next = row if mode == "fast" else "none"
    if not str(state.get("next_execution_card", "")).startswith(
        expected_next
    ):
        api.fail(f"{row} next_execution_card drifted")
    card_rel = str(LEGACY_ARRAY_WRITE_CANON_DELETE_S7_CARD_REL)
    if state.get("next_execution_card_path") != card_rel:
        api.fail(f"{row} next_execution_card_path drifted")
    if state.get("latest_card_path") != card_rel:
        api.fail(f"{row} latest_card_path drifted")

    card_text = (root / card_rel).read_text(encoding="utf-8")
    for token in (row, "canonicalize_legacy_array_write_calls",
                  "residual_call"):
        if token not in card_text:
            api.fail(f"{row} contract is missing: {token}")

    owner_rel = "src/mir/array_element_write.rs"
    owner_text = (root / owner_rel).read_text(encoding="utf-8")
    if "fn canonicalize_legacy_array_write_calls" in owner_text:
        api.fail(f"{row} deleted repair fn still present")
    for token in (
        "fn reject_residual_calls",
        "residual_legacy_array_push_call_rejects_instead_of_upgrading",
    ):
        if token not in owner_text:
            api.fail(f"{row} residual stop or pin lost: {token}")

    contracts_text = (
        root / "src/mir/semantic_refresh/contracts.rs"
    ).read_text(encoding="utf-8")
    if "canonicalize_legacy_array_write_calls" in contracts_text:
        api.fail(f"{row} dead call site remains in contracts.rs")

    for rel in (owner_rel, "src/mir/semantic_refresh/contracts.rs"):
        path = root / rel
        if not path.is_file():
            api.fail(f"{row} implementation owner is missing: {rel}")
        if sum(1 for _ in path.open(encoding="utf-8")) >= 800:
            api.fail(f"{row} implementation owner reached 800 lines: {rel}")
    print(f"[{api.TAG}] row={row} delegated=legacy-array-write-canon-delete")


CANONICALIZE_LEGACY_CLOSURE_ARM_DELETE_S8_ROW = (
    "MIR-CALL-R7-CANONICALIZE-LEGACY-CLOSURE-ARM-DELETE-S8"
)
CANONICALIZE_LEGACY_CLOSURE_ARM_DELETE_S8_CARD_REL = Path(
    "docs/development/current/main/investigations/"
    "mir-call-r7-canonicalize-legacy-closure-arm-delete-s8-2026-09-25.md"
)


def check_canonicalize_legacy_closure_arm_delete_s8(
    state: dict, root: Path, api
) -> None:
    """Pin the S8 legacy closure repair-arm deletion surface."""
    row = CANONICALIZE_LEGACY_CLOSURE_ARM_DELETE_S8_ROW
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{row} must be fast or closeout")
    if state.get("current_execution_row") != row:
        api.fail(f"{row} pointer row drifted")
    if not str(state.get("current_design_stop", "")).startswith("none"):
        api.fail(f"{row} must clear current_design_stop")
    if not str(state.get("next_design_card", "")).startswith("none"):
        api.fail(f"{row} must not open a second design card")
    expected_next = row if mode == "fast" else "none"
    if not str(state.get("next_execution_card", "")).startswith(
        expected_next
    ):
        api.fail(f"{row} next_execution_card drifted")
    card_rel = str(CANONICALIZE_LEGACY_CLOSURE_ARM_DELETE_S8_CARD_REL)
    if state.get("next_execution_card_path") != card_rel:
        api.fail(f"{row} next_execution_card_path drifted")
    if state.get("latest_card_path") != card_rel:
        api.fail(f"{row} latest_card_path drifted")

    card_text = (root / card_rel).read_text(encoding="utf-8")
    for token in (row, "Callee::Closure", "no-laundering"):
        if token not in card_text:
            api.fail(f"{row} contract is missing: {token}")

    owner_rel = "src/mir/passes/callsite_canonicalize/pass.rs"
    owner_text = (root / owner_rel).read_text(encoding="utf-8")
    for token in ("Callee::Closure", "classify_closure_call_shape",
                  "ClosureCallShape"):
        if token in owner_text:
            api.fail(f"{row} deleted closure repair remains: {token}")
    tests_rel = "src/mir/passes/callsite_canonicalize/tests/ncl.rs"
    tests_text = (root / tests_rel).read_text(encoding="utf-8")
    pin = "ncl0_residual_legacy_closure_call_is_never_silently_rewritten"
    if pin not in tests_text:
        api.fail(f"{row} no-laundering pin lost: {pin}")

    for rel in (owner_rel, tests_rel):
        path = root / rel
        if not path.is_file():
            api.fail(f"{row} implementation owner is missing: {rel}")
        if sum(1 for _ in path.open(encoding="utf-8")) >= 800:
            api.fail(f"{row} implementation owner reached 800 lines: {rel}")
    print(f"[{api.TAG}] row={row} delegated=canonicalize-legacy-closure-arm-delete")


CANONICALIZE_LEGACY_GLOBAL_NOOP_ARM_DELETE_S9_ROW = (
    "MIR-CALL-R7-CANONICALIZE-LEGACY-GLOBAL-NOOP-ARM-DELETE-S9"
)
CANONICALIZE_LEGACY_GLOBAL_NOOP_ARM_DELETE_S9_CARD_REL = Path(
    "docs/development/current/main/investigations/"
    "mir-call-r7-canonicalize-legacy-global-noop-arm-delete-s9-2026-09-25.md"
)


def check_canonicalize_legacy_global_noop_arm_delete_s9(
    state: dict, root: Path, api
) -> None:
    """Pin the S9 legacy Global no-op arm deletion surface."""
    row = CANONICALIZE_LEGACY_GLOBAL_NOOP_ARM_DELETE_S9_ROW
    mode = state.get("work_mode")
    if mode not in {"fast", "closeout"}:
        api.fail(f"{row} must be fast or closeout")
    if state.get("current_execution_row") != row:
        api.fail(f"{row} pointer row drifted")
    if not str(state.get("current_design_stop", "")).startswith("none"):
        api.fail(f"{row} must clear current_design_stop")
    if not str(state.get("next_design_card", "")).startswith("none"):
        api.fail(f"{row} must not open a second design card")
    expected_next = row if mode == "fast" else "none"
    if not str(state.get("next_execution_card", "")).startswith(
        expected_next
    ):
        api.fail(f"{row} next_execution_card drifted")
    card_rel = str(CANONICALIZE_LEGACY_GLOBAL_NOOP_ARM_DELETE_S9_CARD_REL)
    if state.get("next_execution_card_path") != card_rel:
        api.fail(f"{row} next_execution_card_path drifted")
    if state.get("latest_card_path") != card_rel:
        api.fail(f"{row} latest_card_path drifted")

    card_text = (root / card_rel).read_text(encoding="utf-8")
    for token in (row, "Callee::Global", "passthrough"):
        if token not in card_text:
            api.fail(f"{row} contract is missing: {token}")

    owner_rel = "src/mir/passes/callsite_canonicalize/pass.rs"
    owner_text = (root / owner_rel).read_text(encoding="utf-8")
    if "Callee" in owner_text:
        api.fail(f"{row} deleted Global arm or callee import remains")
    if "MirInstruction::LegacyCallV0 { .. } => 0" not in owner_text:
        api.fail(f"{row} legacy catch-all passthrough arm lost")
    tests_rel = "src/mir/passes/callsite_canonicalize/tests/mcl.rs"
    tests_text = (root / tests_rel).read_text(encoding="utf-8")
    for pin in (
        "mcl5_keeps_typed_global_callee_without_suffix_repair",
        "mcl6_keeps_typed_global_target_without_runtime_method_repair",
    ):
        if pin not in tests_text:
            api.fail(f"{row} Global passthrough pin lost: {pin}")

    for rel in (owner_rel, tests_rel):
        path = root / rel
        if not path.is_file():
            api.fail(f"{row} implementation owner is missing: {rel}")
        if sum(1 for _ in path.open(encoding="utf-8")) >= 800:
            api.fail(f"{row} implementation owner reached 800 lines: {rel}")
    print(f"[{api.TAG}] row={row} delegated=canonicalize-legacy-global-noop-arm-delete")


