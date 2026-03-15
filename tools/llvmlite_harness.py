#!/usr/bin/env python3
"""
Nyash llvmlite harness (internal)

Primary AOT/EXE pipeline is the ny-llvmc crate backend. This script serves as
an internal harness that ny-llvmc delegates to for object emission.

Usage (debugging only):
  - python3 tools/llvmlite_harness.py --out out.o                # dummy ny_main -> object
  - python3 tools/llvmlite_harness.py --in mir.json --out out.o  # MIR(JSON) -> object (partial support)

Notes:
  - Without --in, emits a trivial ny_main that returns 0.
  - With --in, delegates to src/llvm_py/llvm_builder.py.
"""

import argparse
import json
import os
import sys
from pathlib import Path


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


def bootstrap_builder_paths(root: Path) -> tuple[Path, Path]:
    builder = root / "src" / "llvm_py" / "llvm_builder.py"
    llvm_py_dir = root / "src" / "llvm_py"
    if str(llvm_py_dir) not in sys.path:
        sys.path.insert(0, str(llvm_py_dir))
    return builder, llvm_py_dir


ROOT = resolve_repo_root()
PY_BUILDER, LLVM_PY_DIR = bootstrap_builder_paths(ROOT)

from build_opts import create_target_machine_for_target, parse_opt_level_env

def _maybe_trace_opt(source: str) -> None:
    if os.environ.get("NYASH_CLI_VERBOSE") == "1":
        try:
            level = parse_opt_level_env()
            print(f"[llvmlite harness] opt-level={level} ({source})", file=sys.stderr)
        except Exception:
            pass

def run_dummy(out_path: str) -> None:
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

def run_from_json(in_path: str, out_path: str) -> None:
    # Delegate to python builder to keep code unified
    import runpy

    # Enable safe defaults for prepasses unless explicitly disabled by env
    os.environ.setdefault('NYASH_LLVM_PREPASS_LOOP', os.environ.get('NYASH_LLVM_PREPASS_LOOP', '0'))
    os.environ.setdefault('NYASH_LLVM_PREPASS_IFMERGE', os.environ.get('NYASH_LLVM_PREPASS_IFMERGE', '1'))
    builder_dir = str(PY_BUILDER.parent)
    if builder_dir not in sys.path:
        sys.path.insert(0, builder_dir)
    # Simulate "python llvm_builder.py <in> -o <out>"
    sys.argv = [str(PY_BUILDER), str(in_path), "-o", str(out_path)]
    runpy.run_path(str(PY_BUILDER), run_name="__main__")


def parse_cli_args(argv=None):
    ap = argparse.ArgumentParser()
    ap.add_argument("--in", dest="infile", help="MIR JSON input", default=None)
    ap.add_argument("--out", dest="outfile", help="output object (.o)", required=True)
    return ap.parse_args(argv)


def main(argv=None) -> int:
    args = parse_cli_args(argv)
    if args.infile is None:
        run_dummy(args.outfile)
        print(f"[harness] dummy object written: {args.outfile}")
        return 0
    else:
        run_from_json(args.infile, args.outfile)
        print(f"[harness] object written: {args.outfile}")
        return 0

if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as e:
        import traceback
        print(f"[harness] error: {e}", file=sys.stderr)
        if os.environ.get('NYASH_CLI_VERBOSE') == '1':
            traceback.print_exc()
        sys.exit(1)
