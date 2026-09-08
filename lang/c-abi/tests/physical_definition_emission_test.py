#!/usr/bin/env python3
"""Physical definition metadata -> actual selected C consumers. No source claim.

Optional library argument selects the parent for exact diagnostic/code comparison.
"""
import ctypes
import hashlib
import json
import os
from pathlib import Path
import runpy
import subprocess
import tempfile

support = runpy.run_path(str(Path(__file__).with_name("named_allocation_emission_test.py")))
Row, compile_input, free_error = [support[k] for k in ("Row", "compile_input", "free_error")]
const = {"op": "const", "dst": 1, "value": {"type": "i64", "value": 30}}
ret = {"op": "ret", "value": 1}
call = {"op": "mir_call", "mir_call": {
    "callee": {"type": "Global", "name": "anchor"}, "args": []}}
row = Row(function_name=b"main", instruction_index=1, target_symbol=b"anchor", arity=0, kind=3)


def function(name, instructions, metadata=None):
    return dict(name=name, params=[], metadata=metadata or {},
                blocks=[dict(id=0, instructions=instructions)])


def definition(name, kind):
    return dict(target_symbol=name, definition_kind=kind)


reports = []
for numeric in (False, True):
    for mode in ("same", "leaf", "both", "duplicate", "missing-target", "unknown-kind", "null", "malformed"):
        rows = [definition("anchor", "same_module_function")]
        if mode in ("same", "both", "duplicate"):
            rows.append(definition("candidate", "same_module_function"))
        if mode in ("leaf", "both"):
            rows.append(definition("candidate", "leaf_i64_function"))
        if mode == "duplicate":
            rows += [rows[-1], definition("anchor", "same_module_function")]
        if mode == "missing-target":
            rows.append(definition("absent", "same_module_function"))
        if mode == "unknown-kind":
            rows.append(definition("candidate", "unknown"))
        if mode == "null":
            rows = None
        if mode == "malformed":
            rows = 42
        body = [const, ret] if numeric else [
            const, {"op": "newbox", "dst": 2, "type": "MapBox", "args": []}, ret]
        data = {"functions": [
            function("main", [const, call, ret], {"same_module_function_definitions": rows}),
            function("anchor", [const, ret]), function("candidate", body)]}
        with tempfile.TemporaryDirectory(prefix="hakorune-definition-") as directory:
            work = Path(directory)
            src, obj = work / "input.json", work / "output.o"
            src.write_text(json.dumps(data))
            error = ctypes.c_char_p()
            rc = compile_input(os.fsencode(src), ctypes.byref(row), 1,
                               os.fsencode(obj), ctypes.byref(error))
            message = error.value.decode() if error.value else None
            if error:
                free_error(error)
            report = dict(numeric=numeric, mode=mode, rc=rc, error=message)
            if mode == "null":
                assert rc != 0 and "no_lowering_variant" in message
            elif mode in ("unknown-kind", "malformed"):
                assert rc != 0 and "same_module_function_plan_failed" in message
            else:
                assert rc == 0, report
                symbols = subprocess.check_output(["nm", obj], text=True)
                expected_definition = mode in ("same", "both", "duplicate") or mode == "leaf" and numeric
                assert (" T candidate\n" in symbols) == expected_definition, (report, symbols)
                machine = subprocess.check_output(["objdump", "-dr", obj], text=True)
                report["code_relocation_sha256"] = hashlib.sha256(
                    "\n".join(machine.splitlines()[2:]).encode()).hexdigest()
            assert obj.exists() == (rc == 0), report
            reports.append(report)
# Boundaries exercise the real metadata reader, including duplicate-at-capacity.
for kind, capacity in (("leaf_i64_function", 256), ("same_module_function", 1024)):
    for extra in ("none", "duplicate", "overflow"):
        rows = [definition("anchor", "same_module_function")]
        count = capacity - (kind == "same_module_function")
        rows += [definition(f"absent{i}", kind) for i in range(count)]
        if extra == "duplicate":
            rows.append(rows[-1])
        if extra == "overflow":
            rows.append(definition("one_more", kind))
        data = {"functions": [
            function("main", [const, call, ret], {"same_module_function_definitions": rows}),
            function("anchor", [const, ret])]}
        with tempfile.TemporaryDirectory(prefix="hakorune-plan-capacity-") as directory:
            src, obj = Path(directory) / "input.json", Path(directory) / "out.o"
            src.write_text(json.dumps(data))
            error = ctypes.c_char_p()
            rc = compile_input(os.fsencode(src), ctypes.byref(row), 1,
                               os.fsencode(obj), ctypes.byref(error))
            message = error.value.decode() if error.value else None
            if error:
                free_error(error)
            assert (rc != 0) == (extra == "overflow"), (kind, extra, message)
            assert obj.exists() == (rc == 0)
            reports.append(dict(kind=kind, extra=extra, rc=rc, error=message))
print(json.dumps(reports, indent=2))
