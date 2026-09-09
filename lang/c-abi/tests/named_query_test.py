#!/usr/bin/env python3
"""Private retained-query coordinate/error witnesses; no source/Map cutover claim.

Uses named_query_driver built with ASan and function instrumentation. Retire
with that driver when public V2 host integration covers the same boundary.
"""
import copy
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

constant = {"op": "const", "dst": 1, "value": {"type": "i64", "value": 30}}
ret = {"op": "ret", "value": 1}
alloc = {"op": "newbox", "dst": 2, "type": "StringBox", "args": [1]}
call = {"op": "mir_call", "mir_call": {
    "callee": {"type": "Global", "name": "anchor"}, "args": []}}
body = {"functions": [
    {"name": "main", "params": [], "metadata": {"same_module_function_definitions": [
        {"target_symbol": "anchor", "definition_kind": "same_module_function"}]},
     "blocks": [{"id": 0, "instructions": [constant, call, alloc, ret]}]},
    {"name": "anchor", "params": [], "metadata": {},
     "blocks": [{"id": 0, "instructions": [constant, alloc, ret]}]}],
    "typed_object_plans": [{"box_name": "StringBox", "type_id": 11,
                            "field_count": 0, "fields": []}]}
query = {"function": "main", "block": 0, "instruction": 2}
cases = []


def case(name, data, requests, expected, action="x", error=None, oom=None):
    cases.append((name, data, requests, expected, action, error, oom))


bound_alias = {"status": 0, "consumer": 4}
case("walker-and-repeat", body, [query, dict(query, function="anchor", instruction=1), query],
     [bound_alias, {"status": 0, "consumer": 5}, bound_alias])
case("not-observed", body, [dict(query, function="missing"), dict(query, block=1),
                           dict(query, instruction=0), dict(query, instruction=99)],
     [{"status": 4}] * 4)
case("nul-and-empty", body, [dict(query, function="main\0extra"), dict(query, function="")],
     [{"status": 2}] * 2)
for name, mutate in (
    ("duplicate-name", lambda d: d["functions"].append(copy.deepcopy(d["functions"][0]))),
    ("duplicate-block", lambda d: d["functions"][0]["blocks"].append(
        copy.deepcopy(d["functions"][0]["blocks"][0]))),
    ("negative-block", lambda d: d["functions"][0]["blocks"][0].update(id=-1)),
    ("overflow-block", lambda d: d["functions"][0]["blocks"][0].update(id=2**32)),
    ("float-block", lambda d: d["functions"][0]["blocks"][0].update(id=0.0)),
):
    data = copy.deepcopy(body)
    mutate(data)
    case(name, data, [query], [{"status": 2}])
for target in ({"kind": "intrinsic_map"}, None):
    data = copy.deepcopy(body)
    data["functions"][0]["blocks"][0]["instructions"][2]["target"] = target
    case(f"explicit-target-{target}", data, [query], [{"status": 4}])
case("program-unavailable-before-schema", {"schema_version": 42}, [query, query],
     [{"status": 1}] * 2, "c", "static-v2/function-index")
data = copy.deepcopy(body)
data["schema_version"] = 42
case("storage-failure-before-schema", data, [query], [{"status": 3}], "c",
     "invalid schema_version", 1)
case("storage-failure-cancel", body, [query], [{"status": 3}], oom=1)
# An actual consumer failure must clear rows and destroy this same invocation.
data = copy.deepcopy(body)
data["functions"][0]["blocks"][0]["instructions"][2]["type"] = "Unsupported"
data["functions"][1]["blocks"][0]["instructions"] = [constant, ret]
case("unsupported-compile", data, [query], [{"status": 0, "consumer": 7}],
     "c", "unsupported_newbox_type")

data = copy.deepcopy(body)
data["functions"][1]["blocks"][0]["instructions"] = [constant, ret]
case("storage-failure-demand", data, [query], [{"status": 3}], "c",
     "named_outcome_storage_failed", 1)
case("dangling-definition-before-residual", {"functions": [dict(body["functions"][0],
     blocks=[{"id": 0, "instructions": [constant, ret]}])]}, [], [], "c", "static-v2/function-index")

# Query-only physical mutation of a real Dynamic document; never compiled.
if len(sys.argv) > 2:
    data = json.loads(Path(sys.argv[2]).read_text())
    requests, expected = [], []
    for function in data["functions"]:
        block = function["blocks"][0]
        ordinal = len(block["instructions"])
        block["instructions"].append(copy.deepcopy(alloc))
        requests.append(dict(function=function["name"], block=block["id"], instruction=ordinal))
        selected = bool(function.get("metadata", {}).get("dynamic_v2_aot_call_admission_v2"))
        expected.append({"status": 0, "consumer": 4 if selected else 5})
    data["typed_object_plans"] = copy.deepcopy(body["typed_object_plans"])
    case("dynamic-helper-generic-launch-same-module", data, requests, expected)

with tempfile.TemporaryDirectory(prefix="hakorune-named-query-") as directory:
    work = Path(directory)
    for name, data, requests, expected, action, error, oom in cases:
        source, obj = work / "input.json", work / "output.o"
        source.write_text(json.dumps(data))
        obj.unlink(missing_ok=True)
        env = dict(os.environ, ASAN_OPTIONS="detect_leaks=0",
                   HAKO_BACKEND_COMPILE_RECIPE="pure-first")
        env.pop("TEST_NAMED_REALLOC_FAIL_AT", None)
        if oom:
            env["TEST_NAMED_REALLOC_FAIL_AT"] = str(oom)
        result = subprocess.run([sys.argv[1], str(source), str(obj), json.dumps(requests)],
                                input=action + "\n", capture_output=True, text=True,
                                env=env, timeout=60)
        assert result.returncode == 0, (name, result.stdout, result.stderr)
        lines = result.stdout.splitlines()
        assert json.loads(lines[0]) == expected, (name, lines)
        assert "parsed=1 freed=1" in lines[-1], (name, lines)
        if error:
            assert "rc=-1 " in lines[-1] and error in result.stderr, (name, result)
        else:
            assert "rc=0 " in lines[-1], (name, result)
        print(name, "ok")
print(f"private query: {len(cases)} passed")
