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


