"""Manifest-backed evidence checks for the selected Script R4 ratchet."""

from __future__ import annotations

from collections.abc import Callable
from pathlib import Path
from typing import Any


COMPLETE_FLOOR_V1 = frozenset(
    "integer_0_real_source script_andor_lexical_closure "
    "script_array_literal_compositional_allocation script_await_lexical_closure "
    "script_binary_lexical_closure script_binding_rebind script_check_lexical_closure "
    "script_compound_binding_rebind script_context_scope_diagnostic_boundary "
    "script_enum_declaration_completion script_enum_match_scrutinee "
    "script_enum_variant_producer script_fastmem_scope script_final_root_return "
    "script_fully_explicit_record_literal script_grouped_binding_rebind "
    "script_local_lexical_binding script_map_literal_compositional_mutation "
    "script_nowait_lexical_async_binding script_outbox_semantic_materialization "
    "script_print_lexical_closure script_prior_local_array_index_write "
    "script_pure_block_expr script_root_if_control script_root_match_control "
    "script_root_qmark_propagation script_scopebox script_selected_lambda_child_owner "
    "script_selected_unsupported_diagnostic_boundary script_static_const_u16_completion "
    "script_task_scope_lexical_preflight script_unary_lexical_closure "
    "script_weak_reference_compositional".split()
)

DEFERRED_FLOOR_V1 = {
    "script_weak_unary": "UnsafeRuntimeStatement",
    "script_undefined_variable": "UndefinedVariable",
    "script_function_call_preflight": "FunctionCallPreflightAuthority",
}

DEFERRED_RESIDUALS_V1 = {
    "function_call": ("DirectPortAwareExpression", "FunctionCall", "CallObjectPreflight"),
    "call": ("DirectPortAwareExpression", "Call", "CallObject"),
    "method_call": ("DirectPortAwareExpression", "MethodCall", "CallObject"),
    "loop": ("DirectPortAwareExpression", "Loop", "JoinIrLoop"),
    "field_access": ("DirectPortAwareExpression", "FieldAccess", "CallObject"),
    "index": ("DirectPortAwareExpression", "Index", "CallObject"),
    "new": ("DirectPortAwareExpression", "New", "CallObject"),
    "record_update": ("DirectPortAwareExpression", "RecordUpdate", "CallObject"),
    "box_runtime": ("NonPlainInstanceFullLifecycle", "BoxDeclaration", "BoxRuntimeLifecycle"),
    "try_catch": ("DirectPortAwareExpression", "TryCatch", "ControlResult"),
    "throw": ("DirectPortAwareExpression", "Throw", "ControlResult"),
    "nonfinal_return": ("DirectPortAwareExpression", "Return", "ControlResult"),
}


def validate_script_r4_ratchet_evidence(
    root: Path,
    caller_manifest: dict[str, Any],
    require: Callable[[str, str, str], None],
) -> None:
    """Validate Script R4 floors from the shared caller manifest only."""
    sunsets = caller_manifest["compatibility_sunsets"]
    ratchet = sunsets["SCRIPT-EXISTING-ROOT-LOWER-COMPAT-SUNSET-001"]
    complete_fixtures = ratchet["complete_fixtures"]
    if not COMPLETE_FLOOR_V1 <= complete_fixtures.keys():
        raise AssertionError("Script Complete fixture ratchet regressed")
    _require_fixtures(root, complete_fixtures, require)

    deferred_fixtures = ratchet["deferred_fixtures"]
    _require_fixtures(root, deferred_fixtures, require, DEFERRED_FLOOR_V1)

    profile_residuals = ratchet["script_noncomplete_profile_residuals"]
    if set(profile_residuals) != {"raw_reference_lambda_capture_publication"}:
        raise AssertionError("Script profile residual registry drift")
    for entry_id, entry in profile_residuals.items():
        _require_fields(
            entry,
            ("semantic_terminal", "transport_owner", "operation_owner", "release_condition"),
            f"{entry_id} Script profile residual ownership incomplete",
        )
        fixture = entry["fixture"]
        require(
            (root / fixture["path"]).read_text(),
            fixture["anchor"],
            f"{entry_id} profile residual fixture",
        )

    residuals = ratchet["script_deferred_residuals"]
    if set(residuals) != set(DEFERRED_RESIDUALS_V1):
        raise AssertionError("Script R4 residual registry drift")
    for residual_id, expected in DEFERRED_RESIDUALS_V1.items():
        residual = residuals[residual_id]
        actual = (residual["admission"], residual["shape"], residual["family"])
        if actual != expected:
            raise AssertionError(f"{residual_id} Script residual classification drift")
        _require_fields(
            residual,
            ("semantic_terminal", "transport_owner", "operation_owner", "release_condition"),
            f"{residual_id} Script residual ownership incomplete",
        )
        fixture = residual["fixture"]
        require(
            (root / fixture["path"]).read_text(),
            fixture["anchor"],
            f"{residual_id} residual fixture",
        )


def _require_fixtures(
    root: Path,
    fixtures: dict[str, Any],
    require: Callable[[str, str, str], None],
    expected_reasons: dict[str, str] | None = None,
) -> None:
    for fixture_id, receipt in fixtures.items():
        require(
            (root / receipt["path"]).read_text(),
            receipt["anchor"],
            f"{fixture_id} fixture",
        )
        if expected_reasons is not None and receipt["reason"] != expected_reasons[fixture_id]:
            raise AssertionError(f"{fixture_id} Deferred reason drift")


def _require_fields(entry: dict[str, Any], fields: tuple[str, ...], message: str) -> None:
    if not all(entry.get(field) for field in fields):
        raise AssertionError(message)
