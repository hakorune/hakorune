#!/usr/bin/env python3
"""
Nyash llvmlite harness (compat/canary keep)

Primary AOT/EXE pipeline is the ny-llvmc crate backend. This script serves as
an internal compat harness that ny-llvmc can still delegate to for object emission.

Usage (debugging only):
  - python3 tools/llvmlite_harness.py --out out.o                # dummy ny_main -> object
  - python3 tools/llvmlite_harness.py --in mir.json --out out.o  # MIR(JSON) -> object (partial support)

Notes:
  - Without --in, emits a trivial ny_main that returns 0.
  - With --in, delegates to src/llvm_py/llvm_builder.py.
"""

import argparse
import os
import sys
from pathlib import Path


KEEP_LANE_TAG = "[llvmlite-keep]"


def resolve_repo_root() -> Path:
    default_root = Path(__file__).resolve().parents[1]
    try:
        env_root_str = os.environ.get("NYASH_ROOT")
        if env_root_str:
            candidate = Path(env_root_str).resolve()
            if (candidate / "src" / "llvm_py" / "llvm_builder.py").exists():
                return candidate
    except Exception:
        pass
    return default_root


def resolve_llvm_py_dir(root: Path) -> Path:
    return root / "src" / "llvm_py"


def bootstrap_llvm_py_import_path(root: Path) -> Path:
    llvm_py_dir = resolve_llvm_py_dir(root)
    if str(llvm_py_dir) not in sys.path:
        sys.path.insert(0, str(llvm_py_dir))
    return llvm_py_dir


def bootstrap_harness_context() -> tuple[Path, Path]:
    root = resolve_repo_root()
    llvm_py_dir = bootstrap_llvm_py_import_path(root)
    return root, llvm_py_dir


ROOT, LLVM_PY_DIR = bootstrap_harness_context()

from build_opts import create_target_machine_for_target, parse_opt_level_env
from llvm_builder import build_object_from_input_file


def _maybe_trace_opt(source: str) -> None:
    if os.environ.get("NYASH_CLI_VERBOSE") == "1":
        try:
            level = parse_opt_level_env()
            print(f"[llvmlite harness] opt-level={level} ({source})", file=sys.stderr)
        except Exception:
            pass


def emit_dummy_object(out_path: str) -> None:
    # Minimal llvmlite program: ny_main() -> i32 0
    import llvmlite.ir as ir
    import llvmlite.binding as llvm

    llvm.initialize()
    llvm.initialize_native_target()
    llvm.initialize_native_asmprinter()

    mod = ir.Module(name="nyash_module")
    i32 = ir.IntType(32)
    ny_main_ty = ir.FunctionType(i32, [])
    ny_main = ir.Function(mod, ny_main_ty, name="ny_main")
    entry = ny_main.append_basic_block("entry")
    b = ir.IRBuilder(entry)
    b.ret(ir.Constant(i32, 0))

    # Emit object via target machine
    m = llvm.parse_assembly(str(mod))
    m.verify()
    target = llvm.Target.from_default_triple()
    tm = create_target_machine_for_target(target)
    _maybe_trace_opt("dummy")
    obj = tm.emit_object(m)
    Path(out_path).parent.mkdir(parents=True, exist_ok=True)
    with open(out_path, "wb") as f:
        f.write(obj)


def configure_builder_keep_env() -> None:
    # Enable safe defaults for prepasses unless explicitly disabled by env
    os.environ.setdefault(
        "NYASH_LLVM_USE_HARNESS",
        os.environ.get("NYASH_LLVM_USE_HARNESS", "1"),
    )
    os.environ.setdefault(
        "NYASH_LLVM_PREPASS_LOOP",
        os.environ.get("NYASH_LLVM_PREPASS_LOOP", "0"),
    )
    os.environ.setdefault(
        "NYASH_LLVM_PREPASS_IFMERGE",
        os.environ.get("NYASH_LLVM_PREPASS_IFMERGE", "1"),
    )


def run_builder_keep(in_path: str, out_path: str) -> None:
    # Delegate to python builder to keep compat/probe code unified
    configure_builder_keep_env()
    build_object_from_input_file(in_path, out_path)


def parse_cli_args(argv=None):
    ap = argparse.ArgumentParser(
        description=(
            "Nyash llvmlite compat/probe keep harness. "
            "This is not the daily mainline backend route."
        )
    )
    ap.add_argument(
        "--in",
        dest="infile",
        help="MIR JSON input (compat/canary keep route)",
        default=None,
    )
    ap.add_argument("--out", dest="outfile", help="output object (.o)", required=True)
    return ap.parse_args(argv)


def run_selected_mode(infile: str | None, outfile: str) -> int:
    if infile is None:
        emit_dummy_object(outfile)
        print(f"{KEEP_LANE_TAG} dummy object written: {outfile}")
        return 0
    run_builder_keep(infile, outfile)
    print(f"{KEEP_LANE_TAG} object written: {outfile}")
    return 0


def main(argv=None) -> int:
    args = parse_cli_args(argv)
    return run_selected_mode(args.infile, args.outfile)


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as e:
        import traceback

        print(f"{KEEP_LANE_TAG} error: {e}", file=sys.stderr)
        if os.environ.get("NYASH_CLI_VERBOSE") == "1":
            traceback.print_exc()
        sys.exit(1)
