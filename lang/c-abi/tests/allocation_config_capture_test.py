#!/usr/bin/env python3
"""Physical emitter evidence for invocation-local settings, not source admission."""
import itertools
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

const = {"op": "const", "dst": 1, "value": {"type": "i64", "value": 30}}
ret = {"op": "ret", "value": 1}
ops = [const, {"op": "newbox", "dst": 2, "type": "User", "args": []},
       {"op": "field_set", "box": 2, "field": "value", "value": 1},
       {"op": "field_get", "box": 2, "field": "value", "dst": 3},
       {"op": "newbox", "dst": 4, "type": "ArrayBox", "args": []}, ret]
body = {"functions": [{"name": "main", "params": [], "metadata": {
    "same_module_function_definitions": [
        {"target_symbol": "nested", "definition_kind": "same_module_function"}]},
    "blocks": [{"id": 0, "instructions": ops}]},
    {"name": "nested", "params": [], "metadata": {},
     "blocks": [{"id": 0, "instructions": ops}]}],
    "typed_object_plans": [{"box_name": "User", "type_id": 11, "field_count": 1,
        "fields": [{"name": "value", "storage": "i64", "slot": 0}]}]}
# Existing physical get contract: both walkers must use captured array mode.
ops.insert(5, {"op": "mir_call", "dst": 5, "mir_call": {"callee": {
    "type": "Method", "box_name": "ArrayBox", "method": "get", "receiver": 4}, "args": [1]}})
for fn in body["functions"]:
    fn["metadata"]["lowering_plan"] = [dict(block=0, instruction_index=5,
        source="generic_method_routes", source_route_id="generic_method.get",
        proof="core_method_contract_manifest", route_proof="get_surface_policy",
        core_op="ArrayGet", route_kind="array_slot_load_any", tier="DirectAbi",
        symbol="nyash.array.slot_load_hi", receiver_origin_box="ArrayBox",
        receiver_value=4, key_value=1, arity=1, value_demand="read_ref")]
    fn["metadata"]["direct_array_access_plans"] = [dict(block=0, instruction_index=5,
        route_id="direct_array.access", op="load", array_kind="DirectArrayI64",
        element_type="i64", route="direct_array_i64_load", bounds_policy="checked",
        proof_kind="exact_front_contract", proof_ids=["exact_front_contract"],
        fallback_policy="allow_checked", cfg_shape="checked_branching",
        store_semantics="not_store", receiver_value=4, index_value=1, result_value=5)]
cases = list(itertools.product(("", "direct_slot_exact", "single_thread_exact"),
                               ("", "direct_array_i64_exact"), ("", "1")))
cases += [(None, None, None), ("unknown", "unknown", "1"),
          ("single_thread_exact", "", "10"), ("single_thread_exact", "", "01")]
with tempfile.TemporaryDirectory(prefix="hakorune-config-") as directory:
    work = Path(directory)
    source = work / "input.json"
    source.write_text(json.dumps(body))
    for index, (typed, array, helper) in enumerate(cases):
        env = dict(os.environ, HAKO_BACKEND_COMPILE_RECIPE="pure-first",
                   HAKO_BACKEND_COMPAT_REPLAY="none")
        for name, value in zip(("HAKO_TYPED_OBJECT_STORE", "HAKO_ARRAY_SLOT_STORE",
                                "HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER"), (typed, array, helper)):
            if value is None:
                env.pop(name, None)
            else:
                env[name] = value
        flags = int(typed == "direct_slot_exact") + 2 * int(array == "direct_array_i64_exact")
        exact = typed == "single_thread_exact" and helper == "1"
        obj, llvm = work / f"{index}.o", work / f"{index}.ll"
        result = subprocess.run([sys.argv[1], str(source), str(obj), str(llvm)],
                                env=env, text=True, capture_output=True)
        assert result.returncode == 0, (index, result.stdout, result.stderr)
        assert result.stdout.strip() == f"0 {flags} {int(exact)}", result
        ir = llvm.read_text()
        assert obj.exists()
        calls = [line for line in ir.splitlines() if "call " in line]
        joined = "\n".join(calls)
        assert ('@"nyash.runtime.require_backend_modes_i"' in joined) == bool(flags)
        if flags:
            assert f'@"nyash.runtime.require_backend_modes_i"(i64 {flags})' in joined
        assert ('@"nyash.object.exact_slot_set_i64_hii"' in joined) == exact
        assert ('@"nyash.object.exact_slot_get_i64_hii"' in joined) == exact
        birth = 'nyash.array.direct_i64.birth_h' if array == "direct_array_i64_exact" else 'nyash.array.birth_h'
        assert joined.count('@"' + birth + '"') == 2
        assert ir.count("%direct_array_i64_get_base_0_5 = ") == (2 if array == "direct_array_i64_exact" else 0)
        if exact:
            assert joined.count('@"nyash.object.exact_slot_set_i64_hii"') == 2
            assert joined.count('@"nyash.object.exact_slot_get_i64_hii"') == 2
        print(index, typed, array, helper, "ok")

    # Direct-core negative witnesses, not public/source admission positives.
    # Both unsupported lowering and failed object emission must stop locally.
    marker = work / "harness-called"
    compiler = work / "fake-compiler"
    compiler.write_text(f"#!{sys.executable}\nimport pathlib, sys\n"
        f"pathlib.Path({str(marker)!r}).write_text('called')\n"
        "pathlib.Path(sys.argv[sys.argv.index('--out') + 1]).write_text('object')\n")
    compiler.chmod(0o755)
    tool_markers = []
    tools = {}
    for name in ["opt", "llc"]:
        tool_marker = work / (name + "-called")
        tool_markers.append(tool_marker)
        tool = work / name
        tool.write_text(f"#!{sys.executable}\nimport pathlib, shutil, sys\n"
            f"pathlib.Path({str(tool_marker)!r}).write_text('called')\n" +
            ("sys.exit(1)\n" if name == "llc" else
             "out = sys.argv.index('-o')\nshutil.copyfile(sys.argv[-1], sys.argv[out + 1])\n"))
        tool.chmod(0o755)
        tools[name] = str(tool)
    for label, instructions in [
            ("unsupported", [const, {"op": "newbox", "dst": 2, "type": "UnsupportedReplayBox", "args": []}, ret]),
            ("emitter-failed", [const, ret])]:
        source.write_text(json.dumps({"functions": [{"name": "main", "params": [],
            "blocks": [{"id": 0, "instructions": instructions}]}]}))
        messages = []
        for replay in ["none", "harness"]:
            for path in tool_markers:
                path.unlink(missing_ok=True)
            obj, llvm = work / f"{label}-{replay}.o", work / f"{label}-{replay}.ll"
            env = dict(os.environ, HAKO_BACKEND_COMPILE_RECIPE="pure-first",
                HAKO_BACKEND_COMPAT_REPLAY=replay, HAKO_CAPI_TM="0",
                NYASH_LLVM_ROUTE_TRACE="1", NYASH_NY_LLVM_COMPILER=str(compiler),
                NYASH_NY_LLVM_OPT_TOOL=tools["opt"], NYASH_NY_LLVM_LLC_TOOL=tools["llc"],
                NYASH_LLVM_OPT_LEVEL="0")
            result = subprocess.run([sys.argv[1], str(source), str(obj), str(llvm)],
                                    env=env, text=True, capture_output=True)
            assert result.returncode != 0, (label, replay, result)
            assert not marker.exists() and not obj.exists(), (label, replay, result)
            assert "[llvm-route/replay] lane=none" in result.stderr, result.stderr
            assert "lane=harness" not in result.stderr and "stage=child" not in result.stderr
            diagnostic = [line for line in result.stderr.splitlines()
                          if "unsupported pure shape for current backend recipe" in line]
            assert diagnostic, result.stderr
            messages.append(diagnostic)
            if label == "emitter-failed":
                assert all(path.exists() for path in tool_markers), result.stderr
        assert messages[0] == messages[1], (label, messages)
        print(label, "same unsupported terminal; no automatic child replay")
