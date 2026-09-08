#!/usr/bin/env python3
"""Both actual Named emitters through the published C ABI -> object/rejection.

Synthetic physical MIR only, not source admission. Optional library argument
supports comparing the exact same cases with the parent during extraction.
Prescan rejection remains dependency evidence; the C selector test covers the
unreachable-by-this-prescan StringBox fallback independently.
"""
import ctypes
import hashlib
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parents[3]
lib = ctypes.CDLL(str(Path(sys.argv[1]).resolve()) if len(sys.argv) > 1 else
                  str(ROOT / "target/release/libhako_llvmc_ffi.so"))


class Row(ctypes.Structure):
    _fields_ = [("function_name", ctypes.c_char_p),
                ("block_id", ctypes.c_uint32), ("instruction_index", ctypes.c_uint32),
                ("target_symbol", ctypes.c_char_p)] + [
                    (name, ctypes.c_uint32) for name in
                    ("arity", "kind", "site_id", "receiver", "index", "value", "dst", "flags")]


compile_input = lib.hako_llvmc_compile_published_static_method_v1
compile_input.argtypes = [ctypes.c_char_p, ctypes.POINTER(Row), ctypes.c_size_t,
                          ctypes.c_char_p, ctypes.POINTER(ctypes.c_char_p)]
free_error = ctypes.CDLL(None).free
free_error.argtypes = [ctypes.c_void_p]
os.environ["HAKO_BACKEND_COMPILE_RECIPE"] = "pure-first"
row = Row(function_name=b"main", instruction_index=1, target_symbol=b"anchor", arity=0, kind=3)
reports = []


def run_case(walker, target, args, dst, plan_kind, exact):
    const = {"op": "const", "dst": 1, "value": {"type": "i64", "value": 30}}
    alloc = {"op": "newbox", "dst": dst, "type": target, "args": args}
    ret = {"op": "ret", "value": 1}
    anchor_call = {"op": "mir_call", "mir_call": {
        "callee": {"type": "Global", "name": "anchor"}, "args": []}}
    main = {"name": "main", "params": [], "metadata": {}, "blocks": [
        {"id": 0, "instructions": [const, anchor_call, alloc, ret]}]}
    main["metadata"]["same_module_function_definitions"] = [
        {"target_symbol": "anchor", "definition_kind": "same_module_function"}]
    anchor = {"name": "anchor", "params": [], "metadata": {},
              "blocks": [{"id": 0, "instructions": [const, ret]}]}
    data = {"functions": [main, anchor]}
    if walker == "same-module":
        main["blocks"][0]["instructions"] = [const, anchor_call, ret]
        main["metadata"]["same_module_function_definitions"].append(
            {"target_symbol": "nested", "definition_kind": "same_module_function"})
        data["functions"].append({"name": "nested", "params": [], "metadata": {},
                                  "blocks": [{"id": 0, "instructions": [const, alloc, ret]}]})
    if plan_kind != "missing":
        data["typed_object_plans"] = [{"box_name": target,
            "type_id": 11 if plan_kind == "valid" else 0,
            "field_count": 0, "fields": []}]
    os.environ["HAKO_ARRAY_SLOT_STORE"] = "direct_array_i64_exact" if exact else ""
    builtin = target in ("DirectArrayI64", "ArrayBox", "MapBox")
    generic_special = walker == "generic" and (
        target == "FileBox" or target == "StringBox" and dst and args)
    # Whole-program layout registration rejects type_id=0 before emission.
    # The selector unit test independently checks shadowed invalid-plan laziness.
    expected = plan_kind != "invalid" and bool(
        builtin or generic_special or plan_kind == "valid" and dst)
    alias_execution = (walker == "generic" and target == "StringBox"
                       and dst and args and plan_kind == "missing")
    if alias_execution:
        main["blocks"][0]["instructions"][-1] = {"op": "ret", "value": dst}
    with tempfile.TemporaryDirectory(prefix="hakorune-named-") as directory:
        work = Path(directory)
        src, obj = work / "input.json", work / "output.o"
        src.write_text(json.dumps(data))
        error = ctypes.c_char_p()
        rc = compile_input(os.fsencode(src), ctypes.byref(row), 1,
                           os.fsencode(obj), ctypes.byref(error))
        message = error.value.decode() if error.value else None
        if error:
            free_error(error)
        assert (rc == 0) == expected, (walker, target, args, dst, plan_kind, rc, message)
        report = dict(walker=walker, target=target, args=args, dst=dst,
                      plan=plan_kind, exact=exact, rc=rc, error=message)
        if expected:
            assert obj.is_file() and not message
            symbols = subprocess.check_output(["nm", "-u", obj], text=True)
            if builtin:
                symbol = ("nyash.map.birth_h" if target == "MapBox" else
                          "nyash.array.direct_i64.birth_h" if exact or target == "DirectArrayI64"
                          else "nyash.array.birth_h")
            elif generic_special:
                symbol = "nyash.env.box.new" if target == "FileBox" else None
            else:
                symbol = "nyash.object.new_typed_hi"
            if symbol:
                assert symbol in symbols, (report, symbols)
            if builtin or generic_special:
                assert "nyash.object.new_typed_hi" not in symbols
            # Object file symbols contain a PID-dependent temporary .ll name.
            # Compare actual machine code and relocations without the path header.
            disassembly = subprocess.check_output(["objdump", "-dr", obj], text=True)
            normalized = "\n".join(disassembly.splitlines()[2:])
            report["code_relocation_sha256"] = hashlib.sha256(normalized.encode()).hexdigest()
            if alias_execution:
                exe = work / "alias"
                driver = work / "driver.c"
                driver.write_text("extern long long ny_main(void); int main(void) { return (int)ny_main(); }\n")
                link = subprocess.run(["cc", "-no-pie", str(driver), str(obj), "-o", str(exe)],
                                      capture_output=True)
                assert link.returncode == 0, (link.stderr.decode(), subprocess.check_output(["nm", "-g", obj], text=True))
                assert subprocess.run([str(exe)], capture_output=True).returncode == 30
                report["alias_exit"] = 30
        else:
            assert message and not obj.exists(), report
        reports.append(report)


for walker in ("generic", "same-module"):
    for target in ("DirectArrayI64", "ArrayBox", "MapBox", "FileBox", "StringBox", "User"):
        for plan in ("missing", "invalid", "valid"):
            run_case(walker, target, [1], 2, plan, False)
    for target in ("DirectArrayI64", "ArrayBox", "MapBox"):
        run_case(walker, target, [], 0, "invalid", True)
        run_case(walker, target, [], 0, "missing", True)
        run_case(walker, target, [], 2, "missing", True)
    run_case(walker, "User", [], 0, "valid", False)
    run_case(walker, "StringBox", [], 2, "missing", False)
    run_case(walker, "StringBox", [1], 0, "invalid", False)
print(json.dumps(reports, indent=2))
