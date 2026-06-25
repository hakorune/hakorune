#!/usr/bin/env python3
"""One-command entrypoint for the MirBuilder lightweight-facts converters."""

from __future__ import annotations

import argparse
import sys
from pathlib import Path

from mirbuilder_carrier_snapshot_artifacts import (
    run_variable_context_carrier_snapshot_artifact_generator,
    run_variable_context_explicit_carrier_snapshot_artifact_generator,
)
from mirbuilder_explicit_phi_artifacts import run_explicit_phi_artifact_generator
from mirbuilder_family_artifacts import run_mirbuilder_family_artifact_generator
from mirbuilder_metadata_context_scalar_artifacts import run_metadata_context_scalar_artifact_generator
from mirbuilder_metadata_region_parent_artifacts import run_metadata_region_parent_artifact_generator
from mirbuilder_metadata_value_caller_artifacts import run_metadata_value_caller_artifact_generator
from mirbuilder_literal_integer_artifacts import run_literal_integer_lowering_artifact_generator
from mir_function_constructor_artifacts import run_mir_function_constructor_shell_artifact_generator
from mir_module_minimal_shell_artifacts import run_mir_module_minimal_shell_artifact_generator
from mirbuilder_prepared_state_install_artifacts import run_prepared_state_install_artifact_generator
from mirbuilder_multi_exit_phi_artifacts import run_multi_exit_phi_artifact_generator
from mirbuilder_next_value_id_prepared_state_kernel_artifacts import run_prepared_state_kernel_generator
from mirbuilder_region_observer_artifacts import run_region_observer_artifact_generator
from mirbuilder_single_scalar_loop_carrier_artifacts import run_single_scalar_loop_carrier_artifact_generator
from mirbuilder_structured_loop_artifacts import run_structured_loop_artifact_generator
from mirbuilder_type_context_origin_map_artifacts import run_type_context_origin_map_artifact_generator
from mirbuilder_type_context_string_literal_artifacts import run_type_context_string_literal_artifact_generator
from mirbuilder_type_context_snapshot_restore_artifacts import run_type_context_snapshot_restore_artifact_generator
from mirbuilder_type_context_value_kind_artifacts import run_type_context_value_kind_artifact_generator
from mirbuilder_type_context_value_type_artifacts import run_type_context_value_type_artifact_generator

ROOT = Path(__file__).resolve().parents[2]
FAMILY_GENERATORS = {
    "binding-context": lambda *, check: run_mirbuilder_family_artifact_generator("binding_context", check=check),
    "box-compilation-context": lambda *, check: run_mirbuilder_family_artifact_generator("box_compilation_context", check=check),
    "canonical-explicit-phi": run_explicit_phi_artifact_generator,
    "core-context": lambda *, check: run_mirbuilder_family_artifact_generator("core_context", check=check),
    "metadata-context-scalar-source-file": run_metadata_context_scalar_artifact_generator,
    "metadata-context-region-parent": run_metadata_region_parent_artifact_generator,
    "metadata-context-value-caller": run_metadata_value_caller_artifact_generator,
    "mir-function-constructor-shell": run_mir_function_constructor_shell_artifact_generator,
    "mir-module-minimal-shell": run_mir_module_minimal_shell_artifact_generator,
    "mirbuilder-prepared-state-install": run_prepared_state_install_artifact_generator,
    "mirbuilder-literal-integer-lowering": run_literal_integer_lowering_artifact_generator,
    "mirbuilder-next-value-id-prepared-state-kernel": run_prepared_state_kernel_generator,
    "multi-carrier-exit-phi": run_multi_exit_phi_artifact_generator,
    "region-observer-slot-metadata": run_region_observer_artifact_generator,
    "single-scalar-loop-carrier": run_single_scalar_loop_carrier_artifact_generator,
    "structured-loop-without-carried-state": run_structured_loop_artifact_generator,
    "type-context-origin-map": run_type_context_origin_map_artifact_generator,
    "type-context-string-literal": run_type_context_string_literal_artifact_generator,
    "type-context-snapshot-restore": run_type_context_snapshot_restore_artifact_generator,
    "type-context-value-kind": run_type_context_value_kind_artifact_generator,
    "type-context-value-type": run_type_context_value_type_artifact_generator,
    "variable-context-simple-map": lambda *, check: run_mirbuilder_family_artifact_generator("variable_context_simple_map", check=check),
    "variable-context-immutable-borrow": lambda *, check: run_mirbuilder_family_artifact_generator("variable_context_immutable_borrow", check=check),
    "variable-context-snapshot-restore": lambda *, check: run_mirbuilder_family_artifact_generator("variable_context_snapshot_restore", check=check),
    "variable-context-carrier-snapshot": run_variable_context_carrier_snapshot_artifact_generator,
    "variable-context-explicit-carrier-snapshot": run_variable_context_explicit_carrier_snapshot_artifact_generator,
}
FAMILY_ORDER = tuple(FAMILY_GENERATORS)


def _run_family(name: str, *, check: bool) -> int:
    try:
        FAMILY_GENERATORS[name](check=check)
    except SystemExit as exc:
        code = exc.code
        if code is None:
            return 0
        if isinstance(code, int):
            return code
        print(code, file=sys.stderr)
        return 1
    return 0


def main() -> int:
    parser = argparse.ArgumentParser()
    group = parser.add_mutually_exclusive_group(required=True)
    group.add_argument(
        "--family",
        choices=sorted(FAMILY_GENERATORS),
        help="family to convert with lightweight facts",
    )
    group.add_argument("--all", action="store_true", help="run every lightweight converter family")
    parser.add_argument("--check", action="store_true", help="fail if generated files differ")
    args = parser.parse_args()

    if args.all:
        for name in FAMILY_ORDER:
            return_code = _run_family(name, check=args.check)
            if return_code != 0:
                return return_code
        return 0
    return _run_family(args.family, check=args.check)


if __name__ == "__main__":
    raise SystemExit(main())
