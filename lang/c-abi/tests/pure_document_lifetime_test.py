#!/usr/bin/env python3
"""Run the instrumented/ASan C driver; optional source-issued Dynamic body.

No source admission is inferred from the synthetic ABI rejection cases.
"""
import copy
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

const = {"op": "const", "dst": 1, "value": {"type": "i64", "value": 30}}
ret = {"op": "ret", "value": 1}
call = {"op": "mir_call", "mir_call": {
    "callee": {"type": "Global", "name": "anchor"}, "args": []}}
body = {"functions": [
    {"name": "main", "params": [], "metadata": {"same_module_function_definitions": [
        {"target_symbol": "anchor", "definition_kind": "same_module_function"}]},
     "blocks": [{"id": 0, "instructions": [const, call, ret]}]},
    {"name": "anchor", "params": [], "metadata": {},
     "blocks": [{"id": 0, "instructions": [const, ret]}]}]}
cases = [("success", json.dumps(body), 0, 1, "typed", None),
         ("malformed-json", "{", -1, 0, "typed", "static-v2/json"),
         ("schema", '{"schema_version":42}', -1, 1, "typed", "static-v2/function-index")]
# V2 frame binding precedes core schema; the earlier dual-invalid input stays.
schema_only = copy.deepcopy(body)
schema_only["schema_version"] = 42
cases.append(("schema-with-bound-frame", json.dumps(schema_only), -1, 1, "typed", "invalid schema_version"))
invalid = copy.deepcopy(body)
invalid["functions"][0]["blocks"][0]["instructions"].insert(2, {
    "op": "newbox", "dst": 2, "type": "Unsupported", "args": []})
cases.append(("generic-abort", json.dumps(invalid), -1, 1, "typed", "unsupported_newbox_type"))
invalid = copy.deepcopy(body)
invalid["functions"][0]["metadata"]["exact_seed_backend_route"] = dict(
    tag="array_string_store_micro", source_route="array_string_store_micro_seed_route",
    proof="kilo_micro_array_string_store_8block")
cases.append(("exact-seed-reject", json.dumps(invalid), -1, 1, "typed", "exact-seed route forbidden"))
invalid = copy.deepcopy(body)
invalid["functions"][1]["metadata"]["dynamic_v2_aot_call_admission_v2"] = {}
cases.append(("dynamic-reject", json.dumps(invalid), -1, 1, "typed", "selected Dynamic entry requires"))
# Empty call rows still select the typed consumer. No Map capability is claimed.
empty = {"functions": [copy.deepcopy(body["functions"][1])]}
empty["functions"][0]["name"] = "main"
cases.extend([
    ("empty-v2-success", json.dumps(empty), 0, 1, "selected-empty-v2", None),
    ("empty-v2-json", "{", -1, 0, "selected-empty-v2", "json read error:"),
    ("empty-v2-schema", '{"schema_version":42}', -1, 1, "selected-empty-v2", "invalid schema_version"),
])
for label, metadata, instructions, error in (
    ("global", {}, [const, {"op": "mir_call", "mir_call": {
        "callee": {"type": "Global", "name": "print"}, "args": [1]}}, ret], None),
    ("extern", {}, [const, {"op": "mir_call", "mir_call": {
        "callee": {"type": "Extern", "name": "nyash.console.log"}, "args": [1]}}, ret],
     "published_extern_not_allowed"),
    ("seed", {"exact_seed_backend_route": dict(
        tag="array_string_store_micro", source_route="array_string_store_micro_seed_route",
        proof="kilo_micro_array_string_store_8block")}, [const, ret], "exact-seed route forbidden"),
):
    value = copy.deepcopy(empty)
    value["functions"][0]["metadata"] = metadata
    value["functions"][0]["blocks"][0]["instructions"] = instructions
    cases.append(("empty-v2-" + label, json.dumps(value), -1, 1, "selected-empty-v2", error))
# Existing metadata-selected pattern return, before definition-plan validation.
# Deliberately malformed later plan proves that its validation was not hoisted.
pattern = copy.deepcopy(body)
pattern["functions"][0]["metadata"] = {
    "same_module_function_definitions": 42,
    "array_text_state_residence_route": {
        "observer_kind": "indexof", "residence": "loop_local_pointer_array",
        "result_repr": "scalar_i64", "consumer_capability": "direct_array_text_state_residence",
        "publication_boundary": "none", "temporary_indexof_seed_payload": {
            "variant": "leaf", "rows": 64, "ops": 400000, "flip_period": 0,
            "line_seed": "line-seed", "line_seed_len": 9,
            "none_seed": "none-seed", "none_seed_len": 9,
            "needle": "line", "needle_len": 4,
            "proof": "kilo_leaf_array_string_indexof_const_10block",
            "result_use": "found_predicate", "backend_action": "literal_membership_predicate",
            "candidate_outcomes": [{"literal": "line-seed", "outcome": "found"},
                                   {"literal": "none-seed", "outcome": "not_found"}]}}}
# A bad unrequested Named outcome must not override the earlier pattern success.
pattern["functions"][0]["blocks"][0]["instructions"].insert(2, {
    "op": "newbox", "dst": 2, "type": "Unsupported", "args": []})
cases.append(("pattern-success", json.dumps(pattern), 0, 1, "generic", None))
if len(sys.argv) > 2:
    cases.append(("source-dynamic", Path(sys.argv[2]).read_text(), 0, 1, "generic", None))
# Deferred storage failure: early schema/pattern terminals win, actual demand fails.
oom_cases = []
for fail_at, count in ((1, 1), (2, 17)):
    for label, base, expected, mode, error in (
        ("schema", body, -1, "typed", "invalid schema_version"),
        ("pattern", pattern, 0, "generic", None),
        ("generic-demand", body, -1, "typed", "named_outcome_storage_failed"),
        ("same-module-demand", body, -1, "typed", "named_outcome_storage_failed"),
    ):
        value = copy.deepcopy(base)
        if label == "schema":
            value["schema_version"] = 42
        index = 0
        if label == "same-module-demand":
            # The anchor can be replaced by the existing constant-call pattern.
            # A separate registered definition reaches the actual same-module walker.
            value["functions"][0]["metadata"]["same_module_function_definitions"].append(
                {"target_symbol": "nested", "definition_kind": "same_module_function"})
            nested = copy.deepcopy(body["functions"][1])
            nested["name"] = "nested"
            value["functions"].append(nested)
            index = 2
        value["functions"][index]["blocks"][0]["instructions"][2:2] = [
            {"op": "newbox", "dst": 2 + i, "type": "MapBox", "args": []}
            for i in range(count)]
        # Place the same-module demand before its ret.
        if index:
            instructions = value["functions"][index]["blocks"][0]["instructions"]
            instructions.append(instructions.pop(1))
        oom_cases.append((f"oom-{fail_at}-{label}", json.dumps(value), expected, 1,
                          mode, error, fail_at))
# Document counters cover its leaks; ASan covers invalid access/double frees.
# LLVM/global allocations are outside this narrow document ownership assertion.
env = dict(os.environ, ASAN_OPTIONS="detect_leaks=0", HAKO_BACKEND_COMPILE_RECIPE="pure-first")
with tempfile.TemporaryDirectory(prefix="hakorune-document-test-") as directory:
    work = Path(directory)
    for label, text, rc, parsed, mode, error, fail_at in (
            [(*case, 0) for case in cases] + oom_cases):
        source, output = work / (label + ".json"), work / (label + ".o")
        source.write_text(text)
        result = subprocess.run([sys.argv[1], str(source), str(output), mode],
                                text=True, capture_output=True, env=dict(
                                    env, TEST_NAMED_REALLOC_FAIL_AT=str(fail_at)))
        assert result.returncode == 0, (label, result.stdout, result.stderr)
        assert result.stdout.strip() == f"rc={rc} reads=1 parsed={parsed} freed={parsed}", result
        if error:
            assert error in result.stderr, result
        assert output.exists() == (rc == 0), label
        print(label, result.stdout.strip())
