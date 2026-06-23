#!/usr/bin/env python3
"""Direct shape lowerer for easy-tier MirBuilder Rust-to-Hako conversion."""

from __future__ import annotations

from dataclasses import dataclass
from typing import Any, Callable

from mirbuilder_ordered_map_converter import (
    compile_box_compilation_context_methods,
    compile_ordered_map_family_methods,
    compile_variable_context_snapshot_restore_methods,
)
from mirbuilder_aggregate_snapshot_converter import compile_aggregate_take_restore_methods
from mirbuilder_explicit_phi_converter import compile_canonical_explicit_phi_methods
from mirbuilder_metadata_scalar_converter import compile_scalar_option_atom_methods
from mirbuilder_multi_exit_phi_converter import compile_multi_carrier_exit_phi_methods
from mirbuilder_optional_map_converter import (
    compile_optional_copy_default_map_methods,
    compile_optional_immutable_atom_map_methods,
    compile_optional_owned_recursive_enum_map_methods,
)
from mirbuilder_scalar_counter_converter import compile_core_context_scalar_methods
from mirbuilder_structured_loop_converter import (
    compile_single_scalar_loop_carrier_methods,
    compile_structured_loop_without_carried_state_methods,
)
from verified_hako_family_ir import HakoMethodIR


class DirectShapeLoweringDeny(RuntimeError):
    def __init__(self, shape: str, reason: str) -> None:
        super().__init__(f"Deny({reason}) for {shape}")
        self.shape = shape
        self.reason = reason


DirectLowerer = Callable[[dict[str, Any], dict[str, Any]], list[HakoMethodIR]]


@dataclass(frozen=True)
class DirectShapeRule:
    shape: str
    lower: DirectLowerer
    description: str


def _single_ordered_map_context(
    *,
    type_name: str,
    field_id: str,
    field_name: str,
    value_arg: str,
    include_clear: bool = False,
) -> DirectLowerer:
    def lower(facts: dict[str, Any], plan: dict[str, Any]) -> list[HakoMethodIR]:
        return compile_ordered_map_family_methods(
            facts,
            plan,
            type_name=type_name,
            field_id=field_id,
            field_name=field_name,
            value_arg=value_arg,
            include_clear=include_clear,
        )

    return lower


DIRECT_SHAPE_RULES: dict[str, DirectShapeRule] = {
    "map.optional_copy_default": DirectShapeRule(
        shape="map.optional_copy_default",
        lower=lambda facts, plan: compile_optional_copy_default_map_methods(
            facts,
            plan,
            **plan["direct_shape"]["map.optional_copy_default"],
        ),
        description="map get-option/get-default/set with copy enum values",
    ),
    "map.optional_immutable_atom": DirectShapeRule(
        shape="map.optional_immutable_atom",
        lower=lambda facts, plan: compile_optional_immutable_atom_map_methods(
            facts,
            plan,
            **plan["direct_shape"]["map.optional_immutable_atom"],
        ),
        description="map get-option/set/clear with immutable atom values",
    ),
    "map.optional_owned_recursive_enum": DirectShapeRule(
        shape="map.optional_owned_recursive_enum",
        lower=lambda facts, plan: compile_optional_owned_recursive_enum_map_methods(
            facts,
            plan,
            **plan["direct_shape"]["map.optional_owned_recursive_enum"],
        ),
        description="map get-option/set with owned recursive enum projection",
    ),
    "metadata.scalar_option_atom": DirectShapeRule(
        shape="metadata.scalar_option_atom",
        lower=lambda facts, plan: compile_scalar_option_atom_methods(
            facts,
            plan,
            **plan["direct_shape"]["metadata.scalar_option_atom"],
        ),
        description="scalar field plus optional immutable atom metadata context",
    ),
    "aggregate.take_restore_with_defaults": DirectShapeRule(
        shape="aggregate.take_restore_with_defaults",
        lower=lambda facts, plan: compile_aggregate_take_restore_methods(
            facts,
            plan,
            **plan["direct_shape"]["aggregate.take_restore_with_defaults"],
        ),
        description="aggregate field ownership transfer with default replacement",
    ),
    "control.structured_loop_without_carried_state": DirectShapeRule(
        shape="control.structured_loop_without_carried_state",
        lower=lambda facts, plan: compile_structured_loop_without_carried_state_methods(
            facts,
            plan,
            **plan["direct_shape"]["control.structured_loop_without_carried_state"],
        ),
        description="structured loop with typed condition/body and no semantic carried state",
    ),
    "control.single_scalar_loop_carrier": DirectShapeRule(
        shape="control.single_scalar_loop_carrier",
        lower=lambda facts, plan: compile_single_scalar_loop_carrier_methods(
            facts,
            plan,
            **plan["direct_shape"]["control.single_scalar_loop_carrier"],
        ),
        description="structured loop with exactly one local i64 carrier",
    ),
    "control.canonical_explicit_phi": DirectShapeRule(
        shape="control.canonical_explicit_phi",
        lower=lambda facts, plan: compile_canonical_explicit_phi_methods(
            facts,
            plan,
            **plan["direct_shape"]["control.canonical_explicit_phi"],
        ),
        description="two-input explicit scalar PHI with typed predecessor values",
    ),
    "control.multi_carrier_exit_phi": DirectShapeRule(
        shape="control.multi_carrier_exit_phi",
        lower=lambda facts, plan: compile_multi_carrier_exit_phi_methods(
            facts,
            plan,
            **plan["direct_shape"]["control.multi_carrier_exit_phi"],
        ),
        description="explicit multi-carrier PHI over break/continue/early-return exits",
    ),
    "binding_context.single_ordered_map_context": DirectShapeRule(
        shape="single_ordered_map_context",
        lower=_single_ordered_map_context(
            type_name="BindingContext",
            field_id="BindingContext.binding_map",
            field_name="binding_map",
            value_arg="binding_id",
            include_clear=True,
        ),
        description="one BTreeMap field with BindingContext clear support",
    ),
    "variable_context.single_ordered_map_context": DirectShapeRule(
        shape="single_ordered_map_context",
        lower=_single_ordered_map_context(
            type_name="VariableContext",
            field_id="VariableContext.variable_map",
            field_name="variable_map",
            value_arg="value_id",
        ),
        description="one BTreeMap field for VariableContext simple-map methods",
    ),
    "variable_context.owned_ordered_map_snapshot": DirectShapeRule(
        shape="owned_ordered_map_snapshot",
        lower=lambda facts, plan: compile_variable_context_snapshot_restore_methods(facts, plan),
        description="BTreeMap clone and owned restore replacement",
    ),
    "box_compilation_context.multi_ordered_map_context": DirectShapeRule(
        shape="multi_ordered_map_context",
        lower=lambda facts, plan: compile_box_compilation_context_methods(facts, plan),
        description="default construction plus all-fields ordered-map emptiness",
    ),
    "core_context.scalar_counter_context": DirectShapeRule(
        shape="scalar_counter_context",
        lower=lambda facts, plan: compile_core_context_scalar_methods(facts, plan),
        description="bounded i64 scalar counters with generator-object methods denied",
    ),
}


def lower_direct_shape_methods(
    rule_id: str,
    facts: dict[str, Any],
    plan: dict[str, Any],
) -> list[HakoMethodIR]:
    rule = DIRECT_SHAPE_RULES.get(rule_id)
    if rule is None:
        raise DirectShapeLoweringDeny(rule_id, "UnsupportedDirectShape")
    return rule.lower(facts, plan)
