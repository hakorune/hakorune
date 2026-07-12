#!/usr/bin/env python3
"""Measure isolated Hako source shapes used by the U4-P1 reader diagnostic."""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import re
import subprocess
import tempfile
import time


ROOT = pathlib.Path(__file__).resolve().parents[2]
TIMING_RE = re.compile(
    r"\[mir-compile/timing\] stage=([^ ]+) (?:elapsed_ms|count)=([0-9]+)"
)
IMPORTS = (
    "using lang.compiler.analysis.bounded_body_snapshot.path_v0 as ShapePathV0Box",
    "using lang.compiler.analysis.bounded_body_snapshot.outcome_v0 as ShapeOutcomeV0Box",
    "using lang.compiler.analysis.bounded_body_snapshot.snapshot_model_v0 as ShapeModelV0Box",
    "using lang.compiler.analysis.bounded_body_snapshot.budget_v0 as ShapeBudgetV0Box",
    "using lang.compiler.analysis.bounded_body_snapshot.strict_json_tree_v0 as ShapeTreeV0Box",
)


def nested_branches(count: int) -> str:
    body = "return -1"
    for index in reversed(range(count)):
        body = f'if kind == "K{index:02d}" {{ return {index} }} else {{ {body} }}'
    return body


def combined_reader_body(use_extern: bool) -> str:
    leaves = []
    for index in range(12):
        value = (
            f"ShapeTreeV0Box.object_value_at(session, node, {index})"
            if use_extern
            else str(index)
        )
        leaves.append(f'if kind == "K{index:02d}" {{ return {value} }}')
    leaves.append("return me._read_children(session, node, depth)")
    return " else { ".join(leaves) + " }" * (len(leaves) - 1)


def source_for(case: str) -> str:
    imports: tuple[str, ...] = ()
    body = "return 0"
    helper = ""
    signature = "kind, depth"
    main_call = 'ShapeProbeV0Box.read("miss", 0)'

    if case.startswith("branch_"):
        body = nested_branches(int(case.removeprefix("branch_")))
    elif case == "recursion_none":
        body = "if depth <= 0 { return 0 } return depth - 1"
        main_call = "ShapeProbeV0Box.read(\"miss\", 2)"
    elif case == "recursion_direct":
        body = "if depth <= 0 { return 0 } return me.read(kind, depth - 1)"
        main_call = "ShapeProbeV0Box.read(\"miss\", 2)"
    elif case == "recursion_helper":
        body = "if depth <= 0 { return 0 } return me._read_one(kind, depth - 1)"
        helper = "_read_one(kind, depth) { return me.read(kind, depth) }"
        main_call = "ShapeProbeV0Box.read(\"miss\", 2)"
    elif case == "recursion_loop":
        body = (
            "local i = 0 local value = 0 "
            "loop(i < depth) { value = me.read(kind, depth - 1) i = i + 1 } "
            "return value"
        )
        main_call = "ShapeProbeV0Box.read(\"miss\", 2)"
    elif case in ("combined_plain", "combined_extern"):
        imports = IMPORTS
        signature = "session, node, depth"
        body = 'local kind = "miss" ' + combined_reader_body(case == "combined_extern")
        helper = (
            "_read_children(session, node, depth) { "
            "if depth <= 0 { return 0 } "
            "local count = 2 local i = 0 local value = 0 "
            "loop(i < count) { "
            "value = me.read(session, node, depth - 1) i = i + 1 "
            "} return value }"
        )
        main_call = "0"
    elif case.startswith("import_"):
        imports = IMPORTS[: int(case.removeprefix("import_"))]
    elif case != "baseline":
        raise ValueError(f"unknown case: {case}")

    prefix = "\n".join(imports)
    if prefix:
        prefix += "\n"
    return f"""{prefix}static box ShapeProbeV0Box {{
  read({signature}) {{ {body} }}
  {helper}
}}
static box Main {{
  main() {{ return {main_call} }}
}}
"""


def run_command(command: list[str], timeout: float) -> dict[str, object]:
    started = time.monotonic()
    try:
        completed = subprocess.run(
            command,
            cwd=ROOT,
            env=os.environ
            | {"NYASH_DISABLE_PLUGINS": "1", "NYASH_MIR_COMPILE_TRACE": "1"},
            text=True,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
            timeout=timeout,
            check=False,
        )
    except subprocess.TimeoutExpired as error:
        stderr = error.stderr or ""
        if isinstance(stderr, bytes):
            stderr = stderr.decode("utf-8", errors="replace")
        return {
            "status": "timeout",
            "elapsed_ms": round((time.monotonic() - started) * 1000),
            "stages": parse_stages(stderr),
        }
    return {
        "status": "ok" if completed.returncode == 0 else "error",
        "returncode": completed.returncode,
        "elapsed_ms": round((time.monotonic() - started) * 1000),
        "stages": parse_stages(completed.stderr),
        "stderr_tail": completed.stderr.splitlines()[-8:],
    }


def parse_stages(stderr: str) -> dict[str, int]:
    return {match.group(1): int(match.group(2)) for match in TIMING_RE.finditer(stderr)}


def measure(binary: pathlib.Path, case: str, timeout: float, optimize: bool) -> dict:
    tracked = {
        "tracked_model": "tools/checks/fixtures/bounded_body_snapshot_model_v0.hako",
        "tracked_root_reader": "tools/checks/fixtures/bounded_body_snapshot_root_reader_v0.hako",
        "tracked_leaf_reader": "tools/checks/fixtures/bounded_body_snapshot_leaf_expr_reader_v0.hako",
        "tracked_child_reader": "tools/checks/fixtures/bounded_body_snapshot_child_expr_reader_v0.hako",
    }
    if case in tracked:
        source_path = ROOT / tracked[case]
        source = source_path.read_text(encoding="utf-8")
        parse = run_command([str(binary), "--dump-ast", str(source_path)], timeout)
        compile_command = [str(binary), "--dump-mir"]
        if not optimize:
            compile_command.append("--no-optimize")
        compile_command.append(str(source_path))
        compile_result = run_command(compile_command, timeout)
    else:
        source = source_for(case)
        with tempfile.TemporaryDirectory(prefix="hako-reader-shape-") as temp_dir:
            source_path = pathlib.Path(temp_dir) / f"{case}.hako"
            source_path.write_text(source, encoding="utf-8")
            parse = run_command([str(binary), "--dump-ast", str(source_path)], timeout)
            compile_command = [str(binary), "--dump-mir"]
            if not optimize:
                compile_command.append("--no-optimize")
            compile_command.append(str(source_path))
            compile_result = run_command(compile_command, timeout)
    return {
        "case": case,
        "source_lines": len(source.splitlines()),
        "source_bytes": len(source.encode("utf-8")),
        "optimize": optimize,
        "parse": parse,
        "compile": compile_result,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--bin", type=pathlib.Path, default=ROOT / "target/release/hakorune")
    parser.add_argument("--timeout-sec", type=float, default=30.0)
    parser.add_argument("--optimize", action="store_true")
    parser.add_argument(
        "--cases",
        default=(
            "baseline,tracked_model,tracked_root_reader,tracked_leaf_reader,tracked_child_reader,"
            "branch_1,branch_4,branch_8,branch_12,"
            "recursion_none,recursion_direct,recursion_helper,recursion_loop,"
            "import_0,import_1,import_3,import_5,combined_plain,combined_extern"
        ),
    )
    parser.add_argument("--json-out", type=pathlib.Path)
    args = parser.parse_args()

    cases = [token.strip() for token in args.cases.split(",") if token.strip()]
    results = [measure(args.bin.resolve(), case, args.timeout_sec, args.optimize) for case in cases]
    report = {"contract": "HakoReaderCompileShapeDiagnosticV0", "results": results}
    encoded = json.dumps(report, ensure_ascii=False, indent=2)
    if args.json_out:
        args.json_out.write_text(encoded + "\n", encoding="utf-8")
    print(encoded)
    return 0 if all(row["parse"]["status"] == "ok" for row in results) else 1


if __name__ == "__main__":
    raise SystemExit(main())
