#!/usr/bin/env python3
"""
Nyash LLVM Python Backend - Main Builder
Following the design principles in docs/development/design/legacy/LLVM_LAYER_OVERVIEW.md
"""

import json
import sys
import os
import re
from typing import Dict, Any, Optional, List, Tuple
import llvmlite.ir as ir
import llvmlite.binding as llvm

from builders.legacy_block_lower import (
    setup_phi_placeholders as _legacy_setup_phi_placeholders,
    lower_block as _legacy_lower_block,
    finalize_phis as _legacy_finalize_phis,
)
from trace import debug as trace_debug
from build_opts import create_target_machine_for_target, resolve_build_options
from mir_analysis import scan_call_arities

from resolver import Resolver
from mir_reader import build_builder_input

_FUNC_DECL_RE = re.compile(r"^define\b.*@([^(]+)\(")
_BLOCK_LABEL_RE = re.compile(r"^([A-Za-z$._][\w$.-]*):")
_BR_LABEL_RE = re.compile(r"label\s+%([A-Za-z$._][\w$.-]*)")
_PHI_DEF_RE = re.compile(r"^([%A-Za-z$._][\w$.\-]*)\s*=\s*phi\b")
_PHI_INCOMING_PRED_RE = re.compile(r",\s*%([A-Za-z$._][\w$.-]*)\s*\]")


def _first_phi_verify_mismatch(ir_text: str) -> Optional[Dict[str, Any]]:
    """Best-effort detector for PHI predecessor/incoming mismatch from textual IR."""
    current_func: Optional[str] = None
    current_block: Optional[str] = None
    block_preds: Dict[str, set[str]] = {}
    phi_rows: List[Dict[str, Any]] = []

    def flush_current_function() -> Optional[Dict[str, Any]]:
        if current_func is None:
            return None
        for row in phi_rows:
            cfg_preds = sorted(block_preds.get(row["block"], set()))
            incoming_preds = list(row["incoming_preds"])
            if len(incoming_preds) != len(cfg_preds) or set(incoming_preds) != set(cfg_preds):
                return {
                    "function": current_func,
                    "block": row["block"],
                    "phi": row["phi"],
                    "incoming_count": len(incoming_preds),
                    "pred_count": len(cfg_preds),
                    "incoming_preds": incoming_preds,
                    "cfg_preds": cfg_preds,
                    "line": row["line"],
                }
        return None

    for raw_line in ir_text.splitlines():
        line = raw_line.strip()
        if not line:
            continue

        if current_func is None:
            m_func = _FUNC_DECL_RE.match(line)
            if m_func:
                current_func = m_func.group(1)
                current_block = None
                block_preds = {}
                phi_rows = []
            continue

        if line.startswith("}"):
            mismatch = flush_current_function()
            if mismatch is not None:
                return mismatch
            current_func = None
            current_block = None
            continue

        m_label = _BLOCK_LABEL_RE.match(line)
        if m_label:
            current_block = m_label.group(1)
            block_preds.setdefault(current_block, set())
            continue

        if current_block is None:
            continue

        for target in _BR_LABEL_RE.findall(line):
            block_preds.setdefault(target, set()).add(current_block)

        if " = phi " not in line:
            continue

        m_phi = _PHI_DEF_RE.match(line)
        if m_phi is None:
            continue
        phi_rows.append(
            {
                "block": current_block,
                "phi": m_phi.group(1),
                "incoming_preds": _PHI_INCOMING_PRED_RE.findall(line),
                "line": line,
            }
        )

    return flush_current_function()


class NyashLLVMBuilder:
    """Main LLVM IR builder for Nyash MIR"""
    
    def __init__(self):
        # Initialize LLVM (llvm.initialize() is deprecated in newer llvmlite)
        # llvm.initialize()  # Removed - now handled automatically
        llvm.initialize_native_target()
        llvm.initialize_native_asmprinter()
        
        # Module and basic types
        self.module = ir.Module(name="nyash_module")
        self.i64 = ir.IntType(64)
        self.i32 = ir.IntType(32)
        self.i8 = ir.IntType(8)
        self.i1 = ir.IntType(1)
        self.i8p = self.i8.as_pointer()
        self.f64 = ir.DoubleType()
        self.void = ir.VoidType()
        
        # Value and block maps
        self.vmap: Dict[int, ir.Value] = {}  # value_id -> LLVM value
        self.bb_map: Dict[int, ir.Block] = {}  # block_id -> LLVM block
        
        # PHI deferrals for sealed block approach: (block_id, dst_vid, incoming)
        self.phi_deferrals: List[Tuple[int, int, List[Tuple[int, int]]]] = []
        # Predecessor map and per-block end snapshots
        self.preds: Dict[int, List[int]] = {}
        # Phase 132-P1: Legacy storage (replaced by FunctionLowerContext Box per-function)
        # These are now only used as fallback/backward compatibility
        self.block_end_values: Dict[int, Dict[int, ir.Value]] = {}
        # Definition map: value_id -> set(block_id) where the value is defined
        # Used as a lightweight lifetime hint to avoid over-localization
        self.def_blocks: Dict[int, set] = {}
        
        # Resolver for unified value resolution
        self.resolver = Resolver(self.vmap, self.bb_map)
        # P0-1: Connect builder's SSOT structures to resolver
        self.resolver.def_blocks = self.def_blocks
        self.resolver.block_end_values = self.block_end_values

        # Statistics
        self.loop_count = 0
        # Heuristics for minor gated fixes
        self.current_function_name: Optional[str] = None
        self._last_substring_vid: Optional[int] = None
        # Phase 132-Post: PHI Management Box (replaces predeclared_ret_phis dict)
        from phi_manager import PhiManager
        self.phi_manager = PhiManager()
        # Legacy support for code that still uses predeclared_ret_phis
        self.predeclared_ret_phis: Dict[Tuple[int, int], ir.Instruction] = {}
        
    def build_from_mir(self, mir_json: Dict[str, Any]) -> str:
        """Build LLVM IR from MIR JSON"""
        builder_input = build_builder_input(mir_json, scan_call_arities)
        self.user_box_decls = builder_input.user_box_decls
        functions = builder_input.functions
        self.call_arities = builder_input.call_arities
        
        if not functions:
            # No functions - create dummy ny_main
            return self._create_dummy_main()
        
        # Pre-declare all functions with default i64 signature to allow cross-calls
        for func_data in functions:
            name = func_data.get("name", "unknown")
            # Derive arity:
            # - For method-like names (Class.method/N), include implicit 'me' by using len(params)
            # - Otherwise, prefer suffix '/N' when present; fallback to params length
            m = re.search(r"/(\d+)$", name)
            params_list = func_data.get("params", []) or []
            if "." in name:
                arity = len(params_list)
                # Dev fallback: when params missing for Box.method, use call-site arity
                if arity == 0:
                    try:
                        arity = int(self.call_arities.get(name, 0))
                    except Exception:
                        pass
            else:
                arity = int(m.group(1)) if m else len(params_list)
            if name == "ny_main":
                # Align with runtime expectation: ny_main returns i64
                fty = ir.FunctionType(self.i64, [])
            else:
                fty = ir.FunctionType(self.i64, [self.i64] * arity)
            exists = False
            for f in self.module.functions:
                if f.name == name:
                    exists = True
                    break
            if not exists:
                ir.Function(self.module, fty, name=name)
        
        # Process each function (finalize PHIs per function to avoid cross-function map collisions)
        for func_data in functions:
            self.lower_function(func_data)

        # Create ny_main wrapper if necessary (delegated builder; no legacy fallback)
        try:
            from builders.entry import ensure_ny_main as _ensure_ny_main
            _ensure_ny_main(self)
        except Exception as _e:
            try:
                trace_debug(f"[Python LLVM] ensure_ny_main failed: {_e}")
            except Exception:
                pass
        
        ir_text = str(self.module)
        # Optional IR dump to file for debugging
        try:
            dump_path = os.environ.get('NYASH_LLVM_DUMP_IR')
            if dump_path:
                os.makedirs(os.path.dirname(dump_path), exist_ok=True)
                with open(dump_path, 'w') as f:
                    f.write(ir_text)
            else:
                # Default dump location when verbose and not explicitly set
                if os.environ.get('NYASH_CLI_VERBOSE') == '1':
                    os.makedirs('tmp', exist_ok=True)
                    with open('tmp/nyash_harness.ll', 'w') as f:
                        f.write(ir_text)
        except Exception:
            pass
        return ir_text
    
    def _create_dummy_main(self) -> str:
        """Create dummy ny_main that returns 0"""
        ny_main_ty = ir.FunctionType(self.i64, [])
        ny_main = ir.Function(self.module, ny_main_ty, name="ny_main")
        block = ny_main.append_basic_block(name="entry")
        builder = ir.IRBuilder(block)
        builder.ret(ir.Constant(self.i32, 0))
        return str(self.module)
    
    def lower_function(self, func_data: Dict[str, Any]):
        """Lower a single MIR function to LLVM IR (delegated, no legacy fallback)."""
        try:
            from builders.function_lower import lower_function as _lower
            return _lower(self, func_data)
        except Exception as _e:
            try:
                trace_debug(f"[Python LLVM] lower_function failed: {_e}")
                # Always print traceback for debugging (Phase 21.1)
                import traceback
                traceback.print_exc(file=sys.stderr)
            except Exception:
                pass
            raise


    def setup_phi_placeholders(self, blocks: List[Dict[str, Any]]):
        return _legacy_setup_phi_placeholders(self, blocks)
    
    def lower_block(self, bb: ir.Block, block_data: Dict[str, Any], func: ir.Function):
        return _legacy_lower_block(self, bb, block_data, func)
    
    def lower_instruction(self, builder: ir.IRBuilder, inst: Dict[str, Any], func: ir.Function):
        from builders.instruction_lower import lower_instruction as _li
        return _li(self, builder, inst, func)
    
    # NOTE: regular while lowering is implemented in
    # instructions/controlflow/while_.py::lower_while_regular and invoked
    # from NyashLLVMBuilder.lower_instruction(). This legacy helper is removed
    # to avoid divergence between two implementations.

    def _lower_instruction_list(self, builder: ir.IRBuilder, insts: List[Dict[str, Any]], func: ir.Function):
        """Lower a flat list of instructions using current builder and function.
        Structural guard: truncate at first terminator (ret/branch/jump) to keep IR valid.
        """
        # Sanitize: stop at first terminator in the MIR list
        effective: List[Dict[str, Any]] = []
        try:
            for it in insts:
                op = (it or {}).get('op')
                effective.append(it)
                if op in ('ret', 'branch', 'jump'):
                    break
        except Exception:
            effective = list(insts)
        for sub in effective:
            # If current block already has a terminator, stop lowering further instructions
            # to keep LLVM IR structurally valid. Any residuals should be split upstream.
            try:
                if builder.block is not None and builder.block.terminator is not None:
                    break
            except Exception:
                pass
            self.lower_instruction(builder, sub, func)
    
    def finalize_phis(self):
        return _legacy_finalize_phis(self)
    
    def compile_to_object(self, output_path: str):
        """Compile module to object file"""
        build_opts = resolve_build_options()
        # Create target machine
        target = llvm.Target.from_default_triple()
        target_machine = create_target_machine_for_target(
            target,
            opt_level=build_opts.opt_level,
        )
        try:
            trace_debug(f"[Python LLVM] opt-level={build_opts.opt_level}")
        except Exception:
            pass
        
        # Compile
        ir_text = str(self.module)
        # Optional IR dump for debugging (Phase 131-7)
        if os.environ.get('NYASH_LLVM_DUMP_IR') == '1':
            try:
                ir_dump_path = output_path.replace('.o', '.ll')
                with open(ir_dump_path, 'w') as f:
                    f.write(ir_text)
                print(f"[llvm_builder] IR dumped to: {ir_dump_path}", file=sys.stderr)
            except Exception as e:
                print(f"[llvm_builder] IR dump failed: {e}", file=sys.stderr)
        # Optional sanitize: drop any empty PHI rows (no incoming list) to satisfy IR parser.
        # Gate with NYASH_LLVM_SANITIZE_EMPTY_PHI=1. Additionally, auto-enable when harness is requested.
        if build_opts.sanitize_empty_phi:
            try:
                fixed_lines = []
                for line in ir_text.splitlines():
                    if (" = phi  i64" in line or " = phi i64" in line) and ("[" not in line):
                        # Skip malformed PHI without incoming pairs
                        continue
                    fixed_lines.append(line)
                ir_text = "\n".join(fixed_lines)
            except Exception:
                pass
        mod = llvm.parse_assembly(ir_text)
        # Allow skipping verifier for iterative bring-up
        if build_opts.verify_ir:
            try:
                mod.verify()
            except Exception:
                mismatch = _first_phi_verify_mismatch(ir_text)
                if mismatch is not None:
                    print(
                        (
                            "[llvm_builder/phi_verify] "
                            f"func={mismatch['function']} "
                            f"block={mismatch['block']} "
                            f"phi={mismatch['phi']} "
                            f"incoming={mismatch['incoming_count']} "
                            f"preds={mismatch['pred_count']} "
                            f"incoming_preds={mismatch['incoming_preds']} "
                            f"cfg_preds={mismatch['cfg_preds']} "
                            f"line='{mismatch['line']}'"
                        ),
                        file=sys.stderr,
                    )
                raise

        # PERF-only fast path: run standard LLVM module optimization passes before codegen.
        # Keep default behavior unchanged; this is gated by NYASH_LLVM_FAST=1.
        if build_opts.fast_ir_passes:
            try:
                pmb = llvm.create_pass_manager_builder()
                pmb.opt_level = int(build_opts.opt_level)
                pmb.size_level = 0
                pmb.loop_vectorize = pmb.opt_level >= 2
                pmb.slp_vectorize = pmb.opt_level >= 2
                mpm = llvm.create_module_pass_manager()
                pmb.populate(mpm)
                mpm.run(mod)
            except Exception as _e:
                try:
                    trace_debug(f"[Python LLVM] fast IR passes skipped: {_e}")
                except Exception:
                    pass
        
        # Generate object code
        obj = target_machine.emit_object(mod)
        
        # Write to file
        with open(output_path, 'wb') as f:
            f.write(obj)

def default_output_file():
    return os.path.join('tmp', 'nyash_llvm_py.o')


def parse_cli_args(argv):
    output_file = default_output_file()
    args = list(argv)
    dummy = False

    if not args:
        print("Usage: llvm_builder.py <input.mir.json> [-o output.o] | --dummy [-o output.o]")
        raise SystemExit(1)

    if "-o" in args:
        idx = args.index("-o")
        if idx + 1 < len(args):
            output_file = args[idx + 1]
            del args[idx:idx+2]

    if args and args[0] == "--dummy":
        dummy = True
        del args[0]

    input_file = None
    if not dummy:
        if not args:
            print("error: missing input MIR JSON (or use --dummy)", file=sys.stderr)
            raise SystemExit(2)
        input_file = args[0]

    return dummy, input_file, output_file


def ensure_output_dir(output_file):
    try:
        os.makedirs(os.path.dirname(output_file), exist_ok=True)
    except Exception:
        pass


def load_input_mir_json(input_file):
    with open(input_file, 'r') as f:
        return json.load(f)


def build_dummy_object(output_file, builder=None):
    if builder is None:
        builder = NyashLLVMBuilder()
    ir_text = builder._create_dummy_main()
    trace_debug(f"[Python LLVM] Generated dummy IR:\n{ir_text}")
    ensure_output_dir(output_file)
    builder.compile_to_object(output_file)


def build_object_from_input_file(input_file, output_file, builder=None):
    if builder is None:
        builder = NyashLLVMBuilder()
    mir_json = load_input_mir_json(input_file)
    builder.build_from_mir(mir_json)
    trace_debug("[Python LLVM] Generated LLVM IR (see NYASH_LLVM_DUMP_IR or tmp/nyash_harness.ll)")
    ensure_output_dir(output_file)
    builder.compile_to_object(output_file)


def emit_cli_output(dummy, input_file, output_file):
    builder = NyashLLVMBuilder()
    if dummy:
        build_dummy_object(output_file, builder=builder)
    else:
        build_object_from_input_file(input_file, output_file, builder=builder)
    print(f"Compiled to {output_file}")


def main(argv=None):
    # CLI:
    #   llvm_builder.py <input.mir.json> [-o output.o]
    #   llvm_builder.py --dummy [-o output.o]
    dummy, input_file, output_file = parse_cli_args(sys.argv[1:] if argv is None else argv)
    emit_cli_output(dummy, input_file, output_file)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
