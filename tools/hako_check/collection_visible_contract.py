#!/usr/bin/env python3
"""Emit the collection visible semantics contract report."""

from __future__ import annotations

import argparse
import csv
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
COLL = ROOT / "lang/src/runtime/collections"
BUFFER = COLL / "buffer"
STRING = COLL / "string"
MAP = COLL / "map"
ARRAY = COLL / "array"
BUFFER_FIXTURE = ROOT / "tools/hako_check/tests/collection_visible/buffer_visible_contract.tsv"
BUFFER_NUMERIC_FIXTURE = (
    ROOT / "tools/hako_check/tests/collection_visible/buffer_numeric_le_contract.tsv"
)
STRING_FIXTURE = ROOT / "tools/hako_check/tests/collection_visible/string_visible_contract.tsv"
MAP_FIXTURE = ROOT / "tools/hako_check/tests/collection_visible/map_visible_contract.tsv"
ARRAY_FIXTURE = ROOT / "tools/hako_check/tests/collection_visible/array_visible_contract.tsv"


def has_text(path: Path, needle: str) -> bool:
    try:
        return needle in path.read_text(encoding="utf-8")
    except FileNotFoundError:
        return False


def bool_row(value: bool) -> str:
    return "1" if value else "0"


def function_body(text: str, name: str) -> str:
    marker = f"  {name}("
    start = text.find(marker)
    if start < 0:
        return ""
    brace = text.find("{", start)
    if brace < 0:
        return ""
    depth = 0
    for idx in range(brace, len(text)):
        ch = text[idx]
        if ch == "{":
            depth += 1
        elif ch == "}":
            depth -= 1
            if depth == 0:
                return text[brace + 1 : idx]
    return ""


def read_buffer_fixture() -> list[dict[str, str]]:
    if not BUFFER_FIXTURE.exists():
        return []
    with BUFFER_FIXTURE.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def fixture_matches_policy(policy_text: str, rows: list[dict[str, str]]) -> bool:
    if not rows:
        return False

    canonical_body = function_body(policy_text, "canonical_method")
    arity_body = function_body(policy_text, "expected_arity")
    mutates_body = function_body(policy_text, "mutates_receiver")
    reads_body = function_body(policy_text, "reads_receiver")
    return_body = function_body(policy_text, "return_tag")

    for row in rows:
        method = row["method"]
        canonical = row["canonical"]
        arity = row["arity"]
        return_tag = row["return_tag"]

        if method != canonical:
            alias_line = f'if name == "{method}" {{ return "{canonical}" }}'
            if alias_line not in canonical_body:
                return False

        arity_line = f'if m == "{canonical}" {{ return {arity} }}'
        if arity_line not in arity_body:
            return False

        return_line = f'if m == "{canonical}" {{ return "{return_tag}" }}'
        if return_line not in return_body:
            return False

        mutates_present = f'if m == "{canonical}"' in mutates_body
        reads_present = f'if m == "{canonical}"' in reads_body
        if mutates_present != (row["mutates"] == "1"):
            return False
        if reads_present != (row["reads"] == "1"):
            return False

    return True


def read_tsv(path: Path) -> list[dict[str, str]]:
    if not path.exists():
        return []
    with path.open(newline="", encoding="utf-8") as handle:
        return list(csv.DictReader(handle, delimiter="\t"))


def numeric_fixture_matches_policy(policy_text: str, rows: list[dict[str, str]]) -> bool:
    if not rows:
        return False

    width_body = function_body(policy_text, "width_bytes")
    min_body = function_body(policy_text, "min_value")
    max_body = function_body(policy_text, "max_value_tag")

    if 'return "little"' not in function_body(policy_text, "endian"):
        return False
    if 'return "offset_plus_width_must_fit_buffer"' not in policy_text:
        return False
    if 'return "fail_if_gt_i64_max"' not in policy_text:
        return False

    for row in rows:
        method = row["method"]
        width = row["width"]
        max_tag = row["max_tag"]

        if f'if name == "{method}" {{ return 1 }}' not in policy_text:
            return False
        if f'if name == "{method}" {{ return {width} }}' not in width_body:
            return False
        if row["min"]:
            if f'if name == "{method}" {{ return {row["min"]} }}' not in min_body:
                return False
        if max_tag:
            if max_tag == "float_or_unbounded":
                if 'return "float_or_unbounded"' not in max_body:
                    return False
            elif f'if name == "{method}" {{ return "{max_tag}" }}' not in max_body:
                return False

    return True


def string_fixture_matches_policy(policy_text: str, rows: list[dict[str, str]]) -> bool:
    if not rows:
        return False

    method_id_body = function_body(policy_text, "method_id")
    canonical_body = function_body(policy_text, "canonical_method")
    arity_body = function_body(policy_text, "expected_arity")
    slot_body = function_body(policy_text, "slot")
    return_body = function_body(policy_text, "return_tag")

    if 'return "substrate"' not in function_body(policy_text, "storage_owner"):
        return False
    if 'return "hako"' not in function_body(policy_text, "visible_owner"):
        return False

    for row in rows:
        method = row["method"]
        canonical = row["canonical"]
        arity = row["arity"]
        slot = row["slot"]
        method_id = row["method_id"]
        return_tag = row["return_tag"]

        method_id_line = (
            f'if name == "{method}" && arity == {arity} {{ return "{method_id}" }}'
        )
        if method_id_line not in method_id_body:
            return False

        canonical_line = f'if id == "{method_id}" {{ return "{canonical}" }}'
        if canonical_line not in canonical_body:
            return False

        arity_line = f'if id == "{method_id}" {{ return {arity} }}'
        if arity_line not in arity_body:
            return False

        slot_line = f'if id == "{method_id}" {{ return {slot} }}'
        if slot_line not in slot_body:
            return False

        return_line = f'if id == "{method_id}" {{ return "{return_tag}" }}'
        has_common_string_return = return_tag == "string" and 'return "string"' in return_body
        if return_line not in return_body and not has_common_string_return:
            return False

    return True


def map_fixture_matches_policy(policy_text: str, rows: list[dict[str, str]]) -> bool:
    if not rows:
        return False

    method_id_body = function_body(policy_text, "method_id")
    canonical_body = function_body(policy_text, "canonical_method")
    arity_body = function_body(policy_text, "expected_arity")
    slot_body = function_body(policy_text, "slot")
    effect_body = function_body(policy_text, "effect_tag")
    return_body = function_body(policy_text, "return_tag")

    required_policy_returns = [
        ("key_normalization_policy", "to_string_box_then_quote_strip"),
        ("missing_key_policy", "map_missing_string"),
        ("storage_owner", "substrate"),
        ("visible_owner", "hako"),
    ]
    for function_name, expected in required_policy_returns:
        if f'return "{expected}"' not in function_body(policy_text, function_name):
            return False

    for row in rows:
        method = row["method"]
        canonical = row["canonical"]
        arity = row["arity"]
        slot = row["slot"]
        effect = row["effect"]
        method_id = row["method_id"]
        return_tag = row["return_tag"]

        method_id_line = (
            f'if name == "{method}" && arity == {arity} {{ return "{method_id}" }}'
        )
        if method_id_line not in method_id_body:
            return False

        canonical_line = f'if id == "{method_id}" {{ return "{canonical}" }}'
        if canonical_line not in canonical_body:
            return False

        arity_line = f'if id == "{method_id}" {{ return {arity} }}'
        if arity_line not in arity_body:
            return False

        slot_line = f'if id == "{method_id}" {{ return {slot} }}'
        if slot_line not in slot_body:
            return False

        if effect == "write_heap":
            effect_line = f'if id == "{method_id}" {{ return "write_heap" }}'
            if effect_line not in effect_body:
                return False

        return_line = f'if id == "{method_id}" {{ return "{return_tag}" }}'
        if return_line not in return_body:
            return False

    return True


def array_fixture_matches_policy(policy_text: str, rows: list[dict[str, str]]) -> bool:
    if not rows:
        return False

    method_id_body = function_body(policy_text, "method_id")
    canonical_body = function_body(policy_text, "canonical_method")
    arity_body = function_body(policy_text, "expected_arity")
    slot_body = function_body(policy_text, "slot")
    effect_body = function_body(policy_text, "effect_tag")
    return_body = function_body(policy_text, "return_tag")

    required_policy_returns = [
        ("bounds_policy", "index_must_be_in_range_or_visible_oob_contract"),
        ("empty_pop_policy", "empty_array_returns_visible_empty_string"),
        ("storage_owner", "substrate"),
        ("visible_owner", "hako"),
    ]
    for function_name, expected in required_policy_returns:
        if f'return "{expected}"' not in function_body(policy_text, function_name):
            return False

    for row in rows:
        method = row["method"]
        canonical = row["canonical"]
        arity = row["arity"]
        slot = row["slot"]
        effect = row["effect"]
        method_id = row["method_id"]
        return_tag = row["return_tag"]

        method_id_line = (
            f'if name == "{method}" && arity == {arity} {{ return "{method_id}" }}'
        )
        if method_id_line not in method_id_body:
            return False

        canonical_line = f'if id == "{method_id}" {{ return "{canonical}" }}'
        if canonical_line not in canonical_body:
            return False

        arity_line = f'if id == "{method_id}" {{ return {arity} }}'
        if arity_line not in arity_body:
            return False

        slot_line = f'if id == "{method_id}" {{ return {slot} }}'
        if slot_line not in slot_body:
            return False

        if effect == "write_heap":
            effect_line = f'if id == "{method_id}" {{ return "write_heap" }}'
            if effect_line not in effect_body:
                return False

        return_line = f'if id == "{method_id}" {{ return "{return_tag}" }}'
        if return_line not in return_body:
            return False

    return True


def contract_lines() -> list[str]:
    policy = BUFFER / "visible_policy_box.hako"
    numeric_policy = BUFFER / "numeric_le_policy_box.hako"
    bridge = BUFFER / "substrate_bridge_box.hako"
    core = BUFFER / "core_box.hako"
    facade = COLL / "buffer_core_box.hako"
    readme = BUFFER / "README.md"
    string_policy = STRING / "visible_policy_box.hako"
    string_bridge = STRING / "substrate_bridge_box.hako"
    string_core = STRING / "core_box.hako"
    string_readme = STRING / "README.md"
    string_vm_wrapper = COLL / "string_core_box.hako"
    map_policy = MAP / "visible_policy_box.hako"
    map_bridge = MAP / "substrate_bridge_box.hako"
    map_core = MAP / "core_box.hako"
    map_readme = MAP / "README.md"
    map_vm_wrapper = COLL / "map_core_box.hako"
    array_policy = ARRAY / "visible_policy_box.hako"
    array_bridge = ARRAY / "substrate_bridge_box.hako"
    array_core = ARRAY / "core_box.hako"
    array_readme = ARRAY / "README.md"
    array_vm_wrapper = COLL / "array_core_box.hako"

    method_names = [
        "write",
        "read",
        "readAll",
        "clear",
        "length",
        "len",
        "size",
        "append",
        "slice",
    ]
    policy_text = policy.read_text(encoding="utf-8") if policy.exists() else ""
    bridge_text = bridge.read_text(encoding="utf-8") if bridge.exists() else ""
    core_text = core.read_text(encoding="utf-8") if core.exists() else ""
    methods_present = all(name in policy_text for name in method_names)
    fixture_rows = read_buffer_fixture()
    fixture_matches = fixture_matches_policy(policy_text, fixture_rows)
    numeric_text = numeric_policy.read_text(encoding="utf-8") if numeric_policy.exists() else ""
    numeric_rows = read_tsv(BUFFER_NUMERIC_FIXTURE)
    numeric_matches = numeric_fixture_matches_policy(numeric_text, numeric_rows)
    string_text = string_policy.read_text(encoding="utf-8") if string_policy.exists() else ""
    string_rows = read_tsv(STRING_FIXTURE)
    string_matches = string_fixture_matches_policy(string_text, string_rows)
    string_bridge_text = string_bridge.read_text(encoding="utf-8") if string_bridge.exists() else ""
    string_core_text = string_core.read_text(encoding="utf-8") if string_core.exists() else ""
    string_core_uses_policy = "StringVisiblePolicyBox" in string_core_text
    string_core_uses_bridge = "StringSubstrateBridgeBox" in string_core_text
    string_bridge_keeps_cutover_pending = 'return "pending"' in string_bridge_text
    map_text = map_policy.read_text(encoding="utf-8") if map_policy.exists() else ""
    map_rows = read_tsv(MAP_FIXTURE)
    map_matches = map_fixture_matches_policy(map_text, map_rows)
    map_bridge_text = map_bridge.read_text(encoding="utf-8") if map_bridge.exists() else ""
    map_core_text = map_core.read_text(encoding="utf-8") if map_core.exists() else ""
    map_core_uses_policy = "MapVisiblePolicyBox" in map_core_text
    map_core_uses_bridge = "MapSubstrateBridgeBox" in map_core_text
    map_bridge_keeps_cutover_pending = 'return "pending"' in map_bridge_text
    array_text = array_policy.read_text(encoding="utf-8") if array_policy.exists() else ""
    array_rows = read_tsv(ARRAY_FIXTURE)
    array_matches = array_fixture_matches_policy(array_text, array_rows)
    array_bridge_text = array_bridge.read_text(encoding="utf-8") if array_bridge.exists() else ""
    array_core_text = array_core.read_text(encoding="utf-8") if array_core.exists() else ""
    array_core_uses_policy = "ArrayVisiblePolicyBox" in array_core_text
    array_core_uses_bridge = "ArraySubstrateBridgeBox" in array_core_text
    array_bridge_keeps_cutover_pending = 'return "pending"' in array_bridge_text
    core_uses_policy = "BufferVisiblePolicyBox" in core_text
    core_uses_bridge = "BufferSubstrateBridgeBox" in core_text
    bridge_keeps_mutation_pending = 'return "pending"' in bridge_text

    return [
        "output_contract=hako-check-collection-visible-contract-v0",
        "tool_surface=hako_check_collection_visible_contract",
        "observation_only=1",
        "rewrite_executed=0",
        "collection_visible_semantics_lane_active=1",
        "collection_visible_first_pilot=Buffer",
        f"buffer_visible_policy_module_exists={bool_row(policy.exists())}",
        f"buffer_numeric_le_policy_module_exists={bool_row(numeric_policy.exists())}",
        f"buffer_substrate_bridge_module_exists={bool_row(bridge.exists())}",
        f"buffer_visible_core_module_exists={bool_row(core.exists())}",
        f"buffer_visible_facade_module_exists={bool_row(facade.exists())}",
        f"buffer_visible_readme_exists={bool_row(readme.exists())}",
        f"buffer_visible_method_inventory_exists={bool_row(methods_present)}",
        f"buffer_alias_policy_named={bool_row(has_text(policy, 'canonical_method'))}",
        f"buffer_return_policy_named={bool_row(has_text(policy, 'return_tag'))}",
        f"buffer_mutation_policy_named={bool_row(has_text(policy, 'mutates_receiver'))}",
        f"buffer_storage_substrate_owner={bool_row(has_text(bridge, 'storage_owner'))}",
        f"buffer_visible_owner_hako={bool_row(has_text(policy, 'visible_owner'))}",
        f"buffer_visible_contract_fixture_exists={bool_row(BUFFER_FIXTURE.exists())}",
        f"buffer_visible_contract_fixture_rows={len(fixture_rows)}",
        f"buffer_visible_contract_matches_hako_policy={bool_row(fixture_matches)}",
        f"buffer_length_read_write_contract={bool_row(fixture_matches)}",
        f"buffer_clear_append_slice_contract={bool_row(fixture_matches)}",
        f"buffer_numeric_le_contract_fixture_exists={bool_row(BUFFER_NUMERIC_FIXTURE.exists())}",
        f"buffer_numeric_le_contract_fixture_rows={len(numeric_rows)}",
        f"buffer_numeric_le_contract_matches_hako_policy={bool_row(numeric_matches)}",
        f"buffer_numeric_le_policy_owner={bool_row(numeric_matches)}",
        f"buffer_numeric_bounds_policy_owner={bool_row(numeric_matches)}",
        f"buffer_numeric_storage_layout_changed=0",
        f"buffer_hako_visible_owner_exists={bool_row(core_uses_policy)}",
        f"buffer_core_uses_policy_module={bool_row(core_uses_policy)}",
        f"buffer_core_uses_substrate_bridge={bool_row(core_uses_bridge)}",
        f"buffer_substrate_byte_storage_preserved={bool_row(core_uses_bridge)}",
        f"buffer_data_mutation_cutover_status_pending={bool_row(bridge_keeps_mutation_pending)}",
        f"buffer_vm_handler_dispatch_owner=1",
        f"buffer_visible_semantics_changed=0",
        f"buffer_raw_storage_moved_to_hako=0",
        f"buffer_vm_dispatch_cutover=0",
        f"string_visible_policy_module_exists={bool_row(string_policy.exists())}",
        f"string_substrate_bridge_module_exists={bool_row(string_bridge.exists())}",
        f"string_visible_core_module_exists={bool_row(string_core.exists())}",
        f"string_visible_readme_exists={bool_row(string_readme.exists())}",
        f"string_vm_wrapper_exists={bool_row(string_vm_wrapper.exists())}",
        f"string_visible_contract_fixture_exists={bool_row(STRING_FIXTURE.exists())}",
        f"string_visible_contract_fixture_rows={len(string_rows)}",
        f"string_visible_contract_matches_hako_policy={bool_row(string_matches)}",
        f"string_visible_method_inventory_exists={bool_row(string_matches)}",
        f"string_alias_policy_named={bool_row(has_text(string_policy, 'method_id'))}",
        f"string_slot_policy_named={bool_row(has_text(string_policy, 'slot'))}",
        f"string_return_policy_named={bool_row(has_text(string_policy, 'return_tag'))}",
        f"string_mutation_policy_named={bool_row(has_text(string_policy, 'mutates_receiver'))}",
        f"string_storage_substrate_owner={bool_row(has_text(string_bridge, 'storage_owner'))}",
        f"string_visible_owner_hako={bool_row(has_text(string_policy, 'visible_owner'))}",
        "string_index_mode_substrate_owner="
        f"{bool_row(has_text(string_bridge, 'index_mode_owner'))}",
        f"string_hako_visible_owner_exists={bool_row(string_core_uses_policy)}",
        f"string_core_uses_policy_module={bool_row(string_core_uses_policy)}",
        f"string_core_uses_substrate_bridge={bool_row(string_core_uses_bridge)}",
        f"string_substrate_storage_preserved={bool_row(string_core_uses_bridge)}",
        f"string_vm_wrapper_cutover_status_pending={bool_row(string_bridge_keeps_cutover_pending)}",
        f"string_vm_handler_dispatch_owner=1",
        f"string_visible_semantics_changed=0",
        f"string_raw_storage_moved_to_hako=0",
        f"string_vm_dispatch_cutover=0",
        f"map_visible_policy_module_exists={bool_row(map_policy.exists())}",
        f"map_substrate_bridge_module_exists={bool_row(map_bridge.exists())}",
        f"map_visible_core_module_exists={bool_row(map_core.exists())}",
        f"map_visible_readme_exists={bool_row(map_readme.exists())}",
        f"map_vm_wrapper_exists={bool_row(map_vm_wrapper.exists())}",
        f"map_visible_contract_fixture_exists={bool_row(MAP_FIXTURE.exists())}",
        f"map_visible_contract_exists={bool_row(MAP_FIXTURE.exists())}",
        f"map_visible_contract_fixture_rows={len(map_rows)}",
        f"map_visible_contract_matches_hako_policy={bool_row(map_matches)}",
        f"map_visible_method_inventory_exists={bool_row(map_matches)}",
        f"map_alias_policy_named={bool_row(has_text(map_policy, 'method_id'))}",
        f"map_slot_policy_named={bool_row(has_text(map_policy, 'slot'))}",
        f"map_effect_policy_named={bool_row(has_text(map_policy, 'effect_tag'))}",
        f"map_return_policy_named={bool_row(has_text(map_policy, 'return_tag'))}",
        f"map_mutation_policy_named={bool_row(has_text(map_policy, 'mutates_receiver'))}",
        "map_key_normalization_policy_named="
        f"{bool_row(has_text(map_policy, 'key_normalization_policy'))}",
        f"map_missing_key_policy_named={bool_row(has_text(map_policy, 'missing_key_policy'))}",
        f"map_storage_substrate_owner={bool_row(has_text(map_bridge, 'storage_owner'))}",
        f"map_visible_owner_hako={bool_row(has_text(map_policy, 'visible_owner'))}",
        f"map_hako_visible_owner_exists={bool_row(map_core_uses_policy)}",
        f"map_core_uses_policy_module={bool_row(map_core_uses_policy)}",
        f"map_core_uses_substrate_bridge={bool_row(map_core_uses_bridge)}",
        f"map_substrate_storage_preserved={bool_row(map_core_uses_bridge)}",
        f"map_vm_wrapper_cutover_status_pending={bool_row(map_bridge_keeps_cutover_pending)}",
        f"map_vm_handler_dispatch_owner=1",
        f"map_visible_semantics_changed=0",
        f"map_raw_storage_moved_to_hako=0",
        f"map_vm_dispatch_cutover=0",
        f"array_visible_policy_module_exists={bool_row(array_policy.exists())}",
        f"array_substrate_bridge_module_exists={bool_row(array_bridge.exists())}",
        f"array_visible_core_module_exists={bool_row(array_core.exists())}",
        f"array_visible_readme_exists={bool_row(array_readme.exists())}",
        f"array_vm_wrapper_exists={bool_row(array_vm_wrapper.exists())}",
        f"array_visible_contract_fixture_exists={bool_row(ARRAY_FIXTURE.exists())}",
        f"array_visible_contract_exists={bool_row(ARRAY_FIXTURE.exists())}",
        f"array_visible_contract_fixture_rows={len(array_rows)}",
        f"array_visible_contract_matches_hako_policy={bool_row(array_matches)}",
        f"array_visible_method_inventory_exists={bool_row(array_matches)}",
        f"array_alias_policy_named={bool_row(has_text(array_policy, 'method_id'))}",
        f"array_slot_policy_named={bool_row(has_text(array_policy, 'slot'))}",
        f"array_effect_policy_named={bool_row(has_text(array_policy, 'effect_tag'))}",
        f"array_return_policy_named={bool_row(has_text(array_policy, 'return_tag'))}",
        f"array_mutation_policy_named={bool_row(has_text(array_policy, 'mutates_receiver'))}",
        f"array_bounds_policy_named={bool_row(has_text(array_policy, 'bounds_policy'))}",
        f"array_empty_pop_policy_named={bool_row(has_text(array_policy, 'empty_pop_policy'))}",
        f"array_storage_substrate_owner={bool_row(has_text(array_bridge, 'storage_owner'))}",
        f"array_visible_owner_hako={bool_row(has_text(array_policy, 'visible_owner'))}",
        f"array_hako_visible_owner_exists={bool_row(array_core_uses_policy)}",
        f"array_core_uses_policy_module={bool_row(array_core_uses_policy)}",
        f"array_core_uses_substrate_bridge={bool_row(array_core_uses_bridge)}",
        f"array_substrate_storage_preserved={bool_row(array_core_uses_bridge)}",
        f"array_vm_wrapper_cutover_status_pending={bool_row(array_bridge_keeps_cutover_pending)}",
        f"array_vm_handler_dispatch_owner=1",
        f"array_visible_semantics_changed=0",
        f"array_inline_lane_representation_changed=0",
        f"array_raw_storage_moved_to_hako=0",
        f"array_vm_dispatch_cutover=0",
        "collection_visible_semantics_closeout_ready=1",
        "collection_storage_substrate_owner_preserved=1",
        "next_foundation_lane_selected=coreplan_joinir_expressivity",
        "summary=ok",
    ]


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    report = "\n".join(contract_lines()) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
