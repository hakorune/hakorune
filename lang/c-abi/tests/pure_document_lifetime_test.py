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
         ("malformed-json", "{", -1, 0, "typed", "json read error:"),
         ("schema", '{"schema_version":42}', -1, 1, "typed", "invalid schema_version")]
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
cases.append(("pattern-success", json.dumps(pattern), 0, 1, "generic", None))
if len(sys.argv) > 2:
    cases.append(("source-dynamic", Path(sys.argv[2]).read_text(), 0, 1, "generic", None))
# Document counters cover its leaks; ASan covers invalid access/double frees.
# LLVM/global allocations are outside this narrow document ownership assertion.
env = dict(os.environ, ASAN_OPTIONS="detect_leaks=0", HAKO_BACKEND_COMPILE_RECIPE="pure-first")
with tempfile.TemporaryDirectory(prefix="hakorune-document-test-") as directory:
    work = Path(directory)
    for label, text, rc, parsed, mode, error in cases:
        source, output = work / (label + ".json"), work / (label + ".o")
        source.write_text(text)
        result = subprocess.run([sys.argv[1], str(source), str(output), mode],
                                text=True, capture_output=True, env=env)
        assert result.returncode == 0, (label, result.stdout, result.stderr)
        assert result.stdout.strip() == f"rc={rc} reads=1 parsed={parsed} freed={parsed}", result
        if error:
            assert error in result.stderr, result
        assert output.exists() == (rc == 0), label
        print(label, result.stdout.strip())
