#!/usr/bin/env python3
"""
Nyash LLVM Python Backend - Main Builder
Following the design principles in docs/development/design/legacy/LLVM_LAYER_OVERVIEW.md
"""

import json
import sys
import os
from typing import Dict, Any, Optional, List, Tuple
import llvmlite.ir as ir
import llvmlite.binding as llvm

from builders.legacy_block_lower import (
    setup_phi_placeholders as _legacy_setup_phi_placeholders,
    lower_block as _legacy_lower_block,
    finalize_phis as _legacy_finalize_phis,
)
from trace import debug as trace_debug
from build_opts import create_target_machine_for_target, parse_opt_level_env
from mir_analysis import scan_call_arities

from resolver import Resolver
from mir_reader import MIRReader

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
        # Phase 285LLVM-1.1: Extract user box declarations for registration
        self.user_box_decls = mir_json.get("user_box_decls", [])

        # Parse MIR
        reader = MIRReader(mir_json)
        functions = reader.get_functions()

        try:
            self.call_arities = scan_call_arities(functions)
        except Exception:
            self.call_arities = {}
        
        if not functions:
            # No functions - create dummy ny_main
            return self._create_dummy_main()
        
        # Pre-declare all functions with default i64 signature to allow cross-calls
        import re
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
        # Create target machine
        target = llvm.Target.from_default_triple()
        target_machine = create_target_machine_for_target(target)
        try:
            trace_debug(f"[Python LLVM] opt-level={parse_opt_level_env()}")
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
        if os.environ.get('NYASH_LLVM_SANITIZE_EMPTY_PHI') == '1' or os.environ.get('NYASH_LLVM_USE_HARNESS') == '1':
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
        if os.environ.get('NYASH_LLVM_SKIP_VERIFY') != '1':
            mod.verify()

        # PERF-only fast path: run standard LLVM module optimization passes before codegen.
        # Keep default behavior unchanged; this is gated by NYASH_LLVM_FAST=1.
        if os.environ.get('NYASH_LLVM_FAST') == '1' and os.environ.get('NYASH_LLVM_FAST_IR_PASSES', '1') == '1':
            try:
                pmb = llvm.create_pass_manager_builder()
                pmb.opt_level = int(parse_opt_level_env())
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

def main():
    # CLI:
    #   llvm_builder.py <input.mir.json> [-o output.o]
    #   llvm_builder.py --dummy [-o output.o]
    output_file = os.path.join('tmp', 'nyash_llvm_py.o')
    args = sys.argv[1:]
    dummy = False

    if not args:
        print("Usage: llvm_builder.py <input.mir.json> [-o output.o] | --dummy [-o output.o]")
        sys.exit(1)

    if "-o" in args:
        idx = args.index("-o")
        if idx + 1 < len(args):
            output_file = args[idx + 1]
            del args[idx:idx+2]

    if args and args[0] == "--dummy":
        dummy = True
        del args[0]

    builder = NyashLLVMBuilder()

    if dummy:
        # Emit dummy ny_main
        ir_text = builder._create_dummy_main()
        trace_debug(f"[Python LLVM] Generated dummy IR:\n{ir_text}")
        try:
            os.makedirs(os.path.dirname(output_file), exist_ok=True)
        except Exception:
            pass
        builder.compile_to_object(output_file)
        print(f"Compiled to {output_file}")
        return

    if not args:
        print("error: missing input MIR JSON (or use --dummy)", file=sys.stderr)
        sys.exit(2)

    input_file = args[0]
    with open(input_file, 'r') as f:
        mir_json = json.load(f)

    llvm_ir = builder.build_from_mir(mir_json)
    trace_debug("[Python LLVM] Generated LLVM IR (see NYASH_LLVM_DUMP_IR or tmp/nyash_harness.ll)")

    try:
        os.makedirs(os.path.dirname(output_file), exist_ok=True)
    except Exception:
        pass
    builder.compile_to_object(output_file)
    print(f"Compiled to {output_file}")

if __name__ == "__main__":
    main()
