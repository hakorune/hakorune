"""
Resolver API (Python version)
Based on src/backend/llvm/compiler/codegen/instructions/resolver.rs
"""

from typing import Dict, Optional, Any, Tuple, Set
import os
from trace import phi as trace_phi
from trace import values as trace_values
import llvmlite.ir as ir
from phi_wiring.debug_helper import is_phi_debug_enabled

class Resolver:
    """
    Centralized value resolution with per-block caching.
    Following the Core Invariants from docs/development/design/legacy/LLVM_LAYER_OVERVIEW.md:
    - Resolver-only reads
    - Localize at block start (PHI creation)
    - Cache per (block, value) to avoid redundant PHIs
    """
    
    def __init__(self, a, b=None):
        """Flexible init: either (builder, module) or (vmap, bb_map) for legacy wiring."""
        if hasattr(a, 'position_at_end'):
            # a is IRBuilder
            self.builder = a
            self.module = b
        else:
            # Legacy constructor (vmap, bb_map) — builder/module will be set later when available
            self.builder = None
            self.module = None
            try:
                # Keep references to global maps when provided
                self.global_vmap = a if isinstance(a, dict) else None
                self.global_bb_map = b if isinstance(b, dict) else None
            except Exception:
                self.global_vmap = None
                self.global_bb_map = None

        # Phase 132-P1: Context reference (will be set by bind_context)
        self.context = None

        # Caches: (block_name, value_id) -> llvm value
        # Phase 132-P1: These are now managed via context, but kept for backward compatibility
        self.i64_cache: Dict[Tuple[str, int], ir.Value] = {}
        self.ptr_cache: Dict[Tuple[str, int], ir.Value] = {}
        self.f64_cache: Dict[Tuple[str, int], ir.Value] = {}
        self.binop_expr_cache: Dict[Tuple[str, str, str, int, int], ir.Value] = {}
        self.compare_expr_cache: Dict[Tuple[str, str, str, int, int], ir.Value] = {}
        # String literal map: value_id -> Python string (for by-name calls)
        self.string_literals: Dict[int, str] = {}
        # Optional: value_id -> i8* pointer for string constants (lower_const can populate)
        self.string_ptrs: Dict[int, ir.Value] = {}
        # Track value-ids that are known to represent string handles (i64)
        # This is a best-effort tag used to decide '+' as string concat when both sides are i64.
        self.string_ids: set[int] = set()
        # Track value-ids that are known to represent ArrayBox handles.
        self.array_ids: set[int] = set()
        # Cache for repeated string length queries when immutably known
        self.length_cache: Dict[int, ir.Value] = {}
        self.fast_branch_only_compare_dsts: Set[int] = set()
        self.entry_block_id: Optional[int] = None
        self.entry_block: Optional[ir.Block] = None
        self.reachable_block_ids: Set[int] = set()
        self.hoisted_string_handles: Dict[str, ir.Value] = {}
        self.hoisted_string_ptrs: Dict[str, ir.Value] = {}
        self.non_negative_ids: Set[int] = set()
        self.integerish_ids: Set[int] = set()

        # Type shortcuts
        self.i64 = ir.IntType(64)
        self.i8p = ir.IntType(8).as_pointer()
        self.f64_type = ir.DoubleType()
        # Cache for recursive end-of-block i64 resolution
        self._end_i64_cache: Dict[Tuple[int, int], ir.Value] = {}
        # Lifetime hint: value_id -> set(block_id) where it's known to be defined
        # Populated by the builder when available.
        # Phase 132-P1: Now managed via context, but kept for backward compatibility
        self.def_blocks = {}
        # Optional: block -> { dst_vid -> [(pred_bid, val_vid), ...] } for PHIs from MIR JSON
        self.block_phi_incomings = {}
        # Optional: (block_id, dst_vid) -> src_vid for trivial PHI aliases.
        self.phi_trivial_aliases: Dict[Tuple[int, int], int] = {}
        # P0-1: SSOT for end-of-block values (snapshots)
        # Phase 132-P1: Now managed via context, but kept for backward compatibility
        self.block_end_values = {}
        # P0-3: Circular reference detection (hang prevention)
        self._visited: Set[Tuple[int, int]] = set()

    def bind_context(self, context):
        """Phase 132-P1: Bind resolver to function-local context Box.

        This connects the resolver's caches to the context's function-local storage,
        enabling automatic isolation between functions.

        Args:
            context: FunctionLowerContext instance
        """
        self.context = context
        # Redirect to context-managed storage
        self.i64_cache = context.resolver_i64_cache
        self.ptr_cache = context.resolver_ptr_cache
        self.f64_cache = context.resolver_f64_cache
        self._end_i64_cache = context.resolver_end_i64_cache
        self.binop_expr_cache = context.resolver_binop_expr_cache
        self.compare_expr_cache = context.resolver_compare_expr_cache
        self.string_ids = context.resolver_string_ids
        self.array_ids = context.resolver_array_ids
        self.string_literals = context.resolver_string_literals
        self.string_ptrs = context.resolver_string_ptrs
        self.length_cache = context.resolver_length_cache
        self.fast_branch_only_compare_dsts = context.fast_branch_only_compare_dsts
        self.entry_block_id = context.entry_block_id
        self.entry_block = context.entry_block
        self.reachable_block_ids = context.reachable_block_ids
        self.hoisted_string_handles = context.resolver_hoisted_string_handles
        self.hoisted_string_ptrs = context.resolver_hoisted_string_ptrs
        self.non_negative_ids = context.non_negative_value_ids
        self.integerish_ids = context.integerish_value_ids
        self.def_blocks = context.def_blocks
        self.block_phi_incomings = context.block_phi_incomings
        self.phi_trivial_aliases = context.phi_trivial_aliases
        # Note: block_end_values access goes through context methods

    def mark_string(self, value_id: int) -> None:
        try:
            vid = int(value_id)
            self.string_ids.add(vid)
            # TypeFacts SSOT: keep value_types in sync so downstream decisions
            # (e.g., '+' concat tag checks) can treat this as a StringBox handle.
            try:
                if not hasattr(self, 'value_types') or self.value_types is None:
                    self.value_types = {}
                cur = self.value_types.get(vid) if isinstance(self.value_types, dict) else None
                is_already_string = False
                if isinstance(cur, dict):
                    if cur.get('kind') == 'string':
                        is_already_string = True
                    if cur.get('kind') == 'handle' and cur.get('box_type') == 'StringBox':
                        is_already_string = True
                if not is_already_string and isinstance(self.value_types, dict):
                    self.value_types[vid] = {'kind': 'handle', 'box_type': 'StringBox'}
            except Exception:
                pass
        except Exception:
            pass

    def is_stringish(self, value_id: int) -> bool:
        try:
            vid = int(value_id)
            if vid in self.string_ids:
                return True
            # TypeFacts fallback: some loop/phi paths carry only metadata kind=string.
            vtypes = getattr(self, "value_types", None)
            if isinstance(vtypes, dict):
                meta = vtypes.get(vid)
                if isinstance(meta, dict):
                    kind = meta.get("kind")
                    if kind == "string":
                        return True
                    if kind == "handle" and meta.get("box_type") == "StringBox":
                        return True
                elif isinstance(meta, str):
                    if meta in ("string", "String", "StringBox"):
                        return True
            return False
        except Exception:
            return False

    def is_arrayish(self, value_id: int) -> bool:
        try:
            return int(value_id) in self.array_ids
        except Exception:
            return False

    def _check_cycle(self, block_id: int, value_id: int):
        """P0-3: Circular reference detection (hang prevention)"""
        key = (block_id, value_id)
        if key in self._visited:
            raise RuntimeError(
                f"[LLVM_PY] Circular reference detected: bb{block_id} v{value_id}"
            )
        self._visited.add(key)

    def resolve_cur(self, block_id: int, value_id: int, vmap_cur: Dict[int, ir.Value]) -> ir.Value:
        """P0-1: Same-block instruction lowering (vmap_cur as primary source)

        Used for lowering instructions within the same basic block where the value
        is defined and used. Checks vmap_cur first, then applies fail-fast checks.

        Args:
            block_id: Current basic block ID
            value_id: Value ID to resolve
            vmap_cur: Current block's value map (def->use tracking)

        Returns:
            LLVM IR value (i64)
        """
        # 1. Check vmap_cur first
        val = vmap_cur.get(value_id)
        if val is not None:
            return val

        # 2. Fail-Fast: def_blocks has bb but vmap_cur doesn't → lowerer bug
        if value_id in self.def_blocks and block_id in self.def_blocks[value_id]:
            if os.environ.get('NYASH_LLVM_STRICT') == '1':
                raise RuntimeError(
                    f"[LLVM_PY/STRICT] resolve_cur: v{value_id} defined in bb{block_id} "
                    f"but not in vmap_cur. Lowerer order bug?"
                )

        # 3. vmap_cur miss → undefined error
        if os.environ.get('NYASH_LLVM_STRICT') == '1':
            raise RuntimeError(
                f"[LLVM_PY/STRICT] resolve_cur: v{value_id} not found in bb{block_id} vmap_cur. "
                f"Available: {sorted(vmap_cur.keys())}"
            )

        # Non-STRICT: fallback to 0
        return ir.Constant(ir.IntType(64), 0)

    def resolve_same_block_then_snapshot_i64(
        self,
        value_id: int,
        current_block: ir.Block,
        preds: Dict[int, list],
        block_end_values: Dict[int, Dict[int, Any]],
        vmap: Dict[int, Any],
        bb_map: Optional[Dict[int, ir.Block]] = None,
        fallback_zero: bool = True,
    ) -> Optional[ir.Value]:
        """Resolve i64 with same-block vmap priority, then snapshot fallback.

        This is the SSOT lookup order for instructions (e.g. select) that can
        consume values produced earlier in the same basic block.
        """
        direct = vmap.get(value_id)
        if direct is not None:
            return direct

        try:
            block_id = int(str(current_block.name).replace("bb", ""))
        except Exception:
            block_id = 0

        resolved = self._value_at_end_i64(
            value_id, block_id, preds, block_end_values, vmap, bb_map
        )
        if resolved is not None:
            return resolved
        if fallback_zero:
            return ir.Constant(self.i64, 0)
        return None

    def resolve_incoming(self, pred_block_id: int, value_id: int, context=None) -> ir.Value:
        """P0-2: PHI incoming resolution (snapshot-only reference)
        Phase 132-P1: Use context Box for function-local state isolation

        Used for resolving PHI incoming values from predecessor blocks.
        Only looks at block_end_values snapshot, never vmap_cur.

        Args:
            pred_block_id: Predecessor block ID
            value_id: Value ID to resolve from predecessor
            context: FunctionLowerContext Box (if None, uses self.context)

        Returns:
            LLVM IR value (i64)
        """
        # Phase 132-P1: Use context.get_block_snapshot (simple block_id key)
        ctx = context if context is not None else self.context
        if ctx is not None:
            snapshot = ctx.get_block_snapshot(pred_block_id)
        else:
            # Fallback for backward compatibility (legacy code path)
            snapshot = self.block_end_values.get(pred_block_id, {})

        val = snapshot.get(value_id)
        if val is not None:
            return val

        # Fail-Fast: snapshot miss → structural bug
        if os.environ.get('NYASH_LLVM_STRICT') == '1':
            func_name = ctx.func_name if ctx else "unknown"
            raise RuntimeError(
                f"[LLVM_PY/STRICT] resolve_incoming: v{value_id} not in {func_name}:bb{pred_block_id} snapshot. "
                f"Available: {sorted(snapshot.keys())}"
            )

        # Non-STRICT: fallback to 0
        return ir.Constant(ir.IntType(64), 0)

    def resolve_i64(
        self,
        value_id: int,
        current_block: ir.Block,
        preds: Dict[int, list],
        block_end_values: Dict[int, Dict[int, Any]],
        vmap: Dict[int, Any],
        bb_map: Optional[Dict[int, ir.Block]] = None
    ) -> ir.Value:
        """
        Resolve a MIR value as i64 dominating the current block.
        Creates PHI at block start if needed, caches the result.
        """
        cache_key = (current_block.name, value_id)

        if is_phi_debug_enabled():
            try:
                bid = int(str(current_block.name).replace('bb',''))
                in_def_blocks = value_id in self.def_blocks
                if in_def_blocks:
                    def_in_blocks = list(self.def_blocks.get(value_id, set()))
                else:
                    def_in_blocks = []
                trace_phi(f"[resolve_i64/entry] bb{bid} v{value_id} in_def_blocks={in_def_blocks} def_in={def_in_blocks}")
            except Exception as e:
                trace_phi(f"[resolve_i64/entry] ERROR: {e}")

        # Check cache
        if cache_key in self.i64_cache:
            return self.i64_cache[cache_key]

        # Do not trust global vmap across blocks unless we know it's defined in this block.

        # If this block has a declared MIR PHI for the value, prefer that placeholder
        # and avoid creating any PHI here. Incoming is wired by finalize_phis().
        try:
            try:
                block_id = int(str(current_block.name).replace('bb',''))
            except Exception:
                block_id = -1
            if isinstance(self.block_phi_incomings, dict):
                bmap = self.block_phi_incomings.get(block_id)
                if isinstance(bmap, dict) and value_id in bmap:
                    # Trivial PHI alias path:
                    # If setup tagged this (bb,dst) as copy-like merge, resolve source value
                    # directly in the current block context instead of forcing a PHI placeholder.
                    try:
                        alias_src = None
                        if isinstance(self.phi_trivial_aliases, dict):
                            alias_src = self.phi_trivial_aliases.get((int(block_id), int(value_id)))
                        if isinstance(alias_src, int) and alias_src != int(value_id):
                            alias_val = self.resolve_i64(
                                alias_src,
                                current_block,
                                preds,
                                block_end_values,
                                vmap,
                                bb_map,
                            )
                            if alias_val is not None:
                                self.i64_cache[cache_key] = alias_val
                                return alias_val
                    except Exception:
                        pass
                    existing_cur = vmap.get(value_id)
                    # Fallback: try builder/global vmap when local map lacks placeholder
                    try:
                        if (existing_cur is None or not hasattr(existing_cur, 'add_incoming')) and hasattr(self, 'global_vmap') and isinstance(self.global_vmap, dict):
                            gcand = self.global_vmap.get(value_id)
                            if gcand is not None and hasattr(gcand, 'add_incoming'):
                                existing_cur = gcand
                    except Exception:
                        pass
                    # If a placeholder PHI already exists in this block, reuse it.
                    try:
                        if existing_cur is not None and hasattr(existing_cur, 'add_incoming'):
                            cur_bb_name = getattr(getattr(existing_cur, 'basic_block', None), 'name', None)
                            cbn = current_block.name if hasattr(current_block, 'name') else None
                            try:
                                if isinstance(cur_bb_name, bytes):
                                    cur_bb_name = cur_bb_name.decode()
                            except Exception:
                                pass
                            try:
                                if isinstance(cbn, bytes):
                                    cbn = cbn.decode()
                            except Exception:
                                pass
                            if cur_bb_name == cbn:
                                self.i64_cache[cache_key] = existing_cur
                                return existing_cur
                    except Exception:
                        pass
                    # Otherwise, materialize a placeholder PHI at the block head now
                    # so that comparisons and terminators can dominate subsequent uses.
                    # As a last resort, fall back to zero (should be unreachable when
                    # placeholders are properly predeclared during lowering/tagging).
                    zero = ir.Constant(self.i64, 0)
                    self.i64_cache[cache_key] = zero
                    return zero
        except Exception:
            pass
        
        # Get predecessor blocks
        try:
            bid = int(str(current_block.name).replace('bb',''))
        except Exception:
            bid = -1
        pred_ids = [p for p in preds.get(bid, []) if p != bid]

        # Lifetime hint: if value is defined in this block, and present in vmap as i64, reuse it.
        try:
            defined_here = value_id in self.def_blocks and bid in self.def_blocks.get(value_id, set())
        except Exception:
            defined_here = False
        if defined_here:
            existing = vmap.get(value_id)
            if is_phi_debug_enabled():
                existing_type = type(existing).__name__ if existing is not None else "None"
                trace_phi(f"[resolve/def_here] bb{bid} v{value_id} existing={existing_type} in vmap={value_id in vmap}")
            if existing is not None and hasattr(existing, 'type') and isinstance(existing.type, ir.IntType) and existing.type.width == 64:
                trace_values(f"[resolve] local reuse: bb{bid} v{value_id}")
                self.i64_cache[cache_key] = existing
                return existing
            elif existing is not None:
                if is_phi_debug_enabled():
                    existing_llvm_type = str(existing.type) if hasattr(existing, 'type') else "no_type"
                    trace_phi(f"[resolve/def_here] bb{bid} v{value_id} existing has wrong type: {existing_llvm_type}")
        else:
            # Do NOT blindly reuse vmap across blocks: it may reference values defined
            # in non-dominating predecessors (e.g., other branches). Only reuse when
            # defined_here (handled above) or at entry/no-preds (handled below).
            pass
        
        if not pred_ids:
            # Entry block or no predecessors: prefer local vmap value (already dominating)
            base_val = vmap.get(value_id)
            if base_val is None:
                result = ir.Constant(self.i64, 0)
                trace_phi(f"[resolve] bb{bid} v{value_id} entry/no-preds → 0")
            else:
                # If pointer string, box to handle in current block (use local builder)
                if hasattr(base_val, 'type') and isinstance(base_val.type, ir.PointerType) and self.module is not None:
                    pb = ir.IRBuilder(current_block)
                    try:
                        pb.position_at_start(current_block)
                    except Exception:
                        pass
                    i8p = ir.IntType(8).as_pointer()
                    v = base_val
                    try:
                        if hasattr(v.type, 'pointee') and isinstance(v.type.pointee, ir.ArrayType):
                            c0 = ir.Constant(ir.IntType(32), 0)
                            v = pb.gep(v, [c0, c0], name=f"res_gep_{value_id}")
                    except Exception:
                        pass
                    # declare and call boxer
                    for f in self.module.functions:
                        if f.name == 'nyash.box.from_i8_string':
                            box_from = f
                            break
                    else:
                        box_from = ir.Function(self.module, ir.FunctionType(self.i64, [i8p]), name='nyash.box.from_i8_string')
                    result = pb.call(box_from, [v], name=f"res_ptr2h_{value_id}")
                elif hasattr(base_val, 'type') and isinstance(base_val.type, ir.IntType):
                    result = base_val if base_val.type.width == 64 else ir.Constant(self.i64, 0)
                else:
                    result = ir.Constant(self.i64, 0)
        elif len(pred_ids) == 1:
            # Single-predecessor block: take predecessor end-of-block value directly
            coerced = self._value_at_end_i64(value_id, pred_ids[0], preds, block_end_values, vmap, bb_map)
            self.i64_cache[cache_key] = coerced
            return coerced
        else:
            # Multi-pred: if JSON declares a PHI for (current block, value_id),
            # materialize it on-demand via end-of-block resolver. Otherwise,
            # synthesize a localization PHI at the current block head to ensure
            # dominance for downstream uses (MIR13 PHI-off compatibility).
            try:
                cur_bid = int(str(current_block.name).replace('bb',''))
            except Exception:
                cur_bid = -1
            declared = False
            try:
                if isinstance(self.block_phi_incomings, dict):
                    m = self.block_phi_incomings.get(cur_bid)
                    if isinstance(m, dict) and value_id in m:
                        declared = True
            except Exception:
                declared = False
            if declared:
                # Return existing placeholder if present; do not create a new PHI here.
                trace_phi(f"[resolve] use placeholder PHI: bb{cur_bid} v{value_id}")
                placeholder = vmap.get(value_id)
                if (placeholder is None or not hasattr(placeholder, 'add_incoming')) and hasattr(self, 'global_vmap') and isinstance(self.global_vmap, dict):
                    cand = self.global_vmap.get(value_id)
                    if cand is not None and hasattr(cand, 'add_incoming'):
                        placeholder = cand
                result = placeholder if (placeholder is not None and hasattr(placeholder, 'add_incoming')) else ir.Constant(self.i64, 0)
            else:
                # No declared PHI and multi-pred: do not synthesize; fallback to zero
                import os
                if os.environ.get('NYASH_LLVM_STRICT') == '1':
                    # P0-2: STRICT mode - fail fast on undeclared PHI in multi-pred context
                    def_blocks_info = "not_in_def_blocks"
                    try:
                        if value_id in self.def_blocks:
                            def_blocks_info = f"def_blocks={sorted(list(self.def_blocks[value_id]))}"
                    except Exception:
                        pass

                    raise RuntimeError(
                        f"[LLVM_PY/STRICT] Undeclared PHI in multi-pred block:\n"
                        f"  ValueId: {value_id}\n"
                        f"  Block: bb{cur_bid}\n"
                        f"  Predecessors: {pred_ids}\n"
                        f"  {def_blocks_info}\n"
                        f"  Hint: Value needs PHI but not declared in block_phi_incomings"
                    )
                result = ir.Constant(self.i64, 0)
        
        # Cache and return
        self.i64_cache[cache_key] = result
        return result
    
    def resolve_ptr(self, value_id: int, current_block: ir.Block, 
                    preds: Dict, block_end_values: Dict, vmap: Dict) -> ir.Value:
        """Resolve as i8* pointer"""
        cache_key = (current_block.name, value_id)
        if cache_key in self.ptr_cache:
            return self.ptr_cache[cache_key]
        # Coerce current vmap value or GlobalVariable to i8*
        val = vmap.get(value_id)
        if val is None:
            result = ir.Constant(self.i8p, None)
        else:
            if hasattr(val, 'type') and isinstance(val, ir.PointerType):
                # If pointer to array (GlobalVariable), GEP to first element
                ty = val.type.pointee if hasattr(val.type, 'pointee') else None
                if ty is not None and hasattr(ty, 'element'):
                    c0 = ir.Constant(ir.IntType(32), 0)
                    result = self.builder.gep(val, [c0, c0], name=f"res_str_gep_{value_id}")
                else:
                    result = val
            elif hasattr(val, 'type') and isinstance(val.type, ir.IntType):
                use_bridge = False
                try:
                    if hasattr(self, 'is_stringish') and self.is_stringish(int(value_id)):
                        use_bridge = True
                except Exception:
                    use_bridge = False
                if use_bridge and self.builder is not None:
                    bridge = None
                    for f in self.module.functions:
                        if f.name == 'nyash.string.to_i8p_h':
                            bridge = f; break
                    if bridge is None:
                        bridge = ir.Function(self.module, ir.FunctionType(self.i8p, [self.i64]), name='nyash.string.to_i8p_h')
                    result = self.builder.call(bridge, [val], name=f"res_h2p_{value_id}")
                else:
                    result = self.builder.inttoptr(val, self.i8p, name=f"res_i2p_{value_id}")
            else:
                # f64 or others -> zero
                result = ir.Constant(self.i8p, None)
        self.ptr_cache[cache_key] = result
        return result

    def get_end_values(self, block_id: int) -> Dict[int, ir.Value]:
        """P0-1: Get end-of-block snapshot (SSOT).

        Returns the snapshot of values at the end of the specified block.
        This is the single source of truth for PHI incoming resolution.
        """
        return self.block_end_values.get(block_id, {})

    def _value_at_end_i64(self, value_id: int, block_id: int, preds: Dict[int, list],
                          block_end_values: Dict[int, Dict[int, Any]], vmap: Dict[int, Any],
                          bb_map: Optional[Dict[int, ir.Block]] = None,
                          _vis: Optional[set] = None) -> ir.Value:
        """P0-2: Resolve value from block snapshot only (no global search).

        This function now ONLY looks at the block_end_values snapshot for the
        specified block. It does NOT recursively search predecessors. This
        eliminates processing-order dependency bugs.
        """
        trace_phi(f"[resolve] end_i64 enter: bb{block_id} v{value_id}")
        key = (block_id, value_id)
        if key in self._end_i64_cache:
            return self._end_i64_cache[key]

        # P0-2: ONLY use snapshot - no predecessor recursion
        snap = block_end_values.get(block_id, {})
        val = snap.get(value_id)

        if val is not None:
            is_phi_val = False
            try:
                is_phi_val = hasattr(val, 'add_incoming')
            except Exception:
                is_phi_val = False
            try:
                ty = 'phi' if is_phi_val else ('ptr' if hasattr(val, 'type') and isinstance(val.type, ir.PointerType) else ('i'+str(getattr(val.type,'width','?')) if hasattr(val,'type') and isinstance(val.type, ir.IntType) else 'other'))
                trace_phi(f"[resolve]  snap hit: bb{block_id} v{value_id} type={ty}")
            except Exception:
                pass
            if is_phi_val:
                # PHIs are valid SSA values to carry through snapshots: a PHI defined at a
                # dominating block head can be used at the end of successor blocks.
                coerced = self._coerce_in_block_to_i64(val, block_id, bb_map)
                self._end_i64_cache[key] = coerced
                return coerced
            else:
                coerced = self._coerce_in_block_to_i64(val, block_id, bb_map)
                self._end_i64_cache[key] = coerced
                return coerced

        # P0-2: Snapshot miss - STRICT mode error, else fallback to 0
        import os
        if os.environ.get('NYASH_LLVM_STRICT') == '1':
            # Collect diagnostic information
            def_blocks_info = "not_in_def_blocks"
            try:
                if value_id in self.def_blocks:
                    def_blocks_info = f"def_blocks={sorted(list(self.def_blocks[value_id]))}"
            except Exception:
                pass

            snapshot_keys = sorted(list(snap.keys())) if snap else []

            raise RuntimeError(
                f"[LLVM_PY/STRICT] Value not in block snapshot:\n"
                f"  ValueId: v{value_id}\n"
                f"  Block: bb{block_id}\n"
                f"  Available in snapshot: {snapshot_keys}\n"
                f"  {def_blocks_info}\n"
                f"  Hint: Value should be in block_end_values[{block_id}] but is missing"
            )

        # Non-STRICT: fallback to 0 (for diagnostics)
        trace_phi(f"[resolve] end_i64 miss: bb{block_id} v{value_id} → 0")
        z = ir.Constant(self.i64, 0)
        self._end_i64_cache[key] = z
        return z

    def _coerce_in_block_to_i64(self, val: Any, block_id: int, bb_map: Optional[Dict[int, ir.Block]]) -> ir.Value:
        """Ensure a value is available as i64 at the end of the given block by inserting casts/boxing there."""
        if hasattr(val, 'type') and isinstance(val.type, ir.IntType):
            # If already i64, avoid re-materializing in predecessor block.
            # Using a value defined in another block inside pred may violate dominance (e.g., self-referential PHIs).
            if val.type.width == 64:
                return val
            # Otherwise, extend/truncate in predecessor block just before the terminator.
            pred_bb = bb_map.get(block_id) if bb_map is not None else None
            if pred_bb is None:
                return ir.Constant(self.i64, 0)
            pb = ir.IRBuilder(pred_bb)
            try:
                term = pred_bb.terminator
                if term is not None:
                    pb.position_before(term)
                else:
                    pb.position_at_end(pred_bb)
            except Exception:
                pb.position_at_end(pred_bb)
            if val.type.width < 64:
                return pb.zext(val, self.i64, name=f"res_zext_{block_id}")
            else:
                return pb.trunc(val, self.i64, name=f"res_trunc_{block_id}")
        if hasattr(val, 'type') and isinstance(val.type, ir.PointerType):
            pred_bb = bb_map.get(block_id) if bb_map is not None else None
            if pred_bb is None:
                return ir.Constant(self.i64, 0)
            pb = ir.IRBuilder(pred_bb)
            try:
                term = pred_bb.terminator
                if term is not None:
                    pb.position_before(term)
                else:
                    pb.position_at_end(pred_bb)
            except Exception:
                pb.position_at_end(pred_bb)
            i8p = ir.IntType(8).as_pointer()
            v = val
            try:
                if hasattr(v.type, 'pointee') and isinstance(v.type.pointee, ir.ArrayType):
                    c0 = ir.Constant(ir.IntType(32), 0)
                    v = pb.gep(v, [c0, c0], name=f"res_gep_{block_id}_{id(val)}")
            except Exception:
                pass
            # declare boxer
            box_from = None
            for f in self.module.functions:
                if f.name == 'nyash.box.from_i8_string':
                    box_from = f
                    break
            if box_from is None:
                box_from = ir.Function(self.module, ir.FunctionType(self.i64, [i8p]), name='nyash.box.from_i8_string')
            return pb.call(box_from, [v], name=f"res_ptr2h_{block_id}")
        return ir.Constant(self.i64, 0)
    
    def resolve_f64(self, value_id: int, current_block: ir.Block,
                    preds: Dict, block_end_values: Dict, vmap: Dict) -> ir.Value:
        """Resolve as f64"""
        cache_key = (current_block.name, value_id)
        if cache_key in self.f64_cache:
            return self.f64_cache[cache_key]
        val = vmap.get(value_id)
        if val is None:
            result = ir.Constant(self.f64_type, 0.0)
        else:
            if hasattr(val, 'type') and val.type == self.f64_type:
                result = val
            elif hasattr(val, 'type') and isinstance(val.type, ir.IntType):
                result = self.builder.sitofp(val, self.f64_type)
            elif hasattr(val, 'type') and isinstance(val.type, ir.PointerType):
                tmp = self.builder.ptrtoint(val, self.i64, name=f"res_p2i_{value_id}")
                result = self.builder.sitofp(tmp, self.f64_type, name=f"res_i2f_{value_id}")
            else:
                result = ir.Constant(self.f64_type, 0.0)
        self.f64_cache[cache_key] = result
        return result
    
    def _coerce_to_i64(self, val: Any) -> ir.Value:
        """Coerce various types to i64"""
        if isinstance(val, ir.Constant) and val.type == self.i64:
            return val
        elif hasattr(val, 'type') and val.type.is_pointer:
            # ptr to int
            return self.builder.ptrtoint(val, self.i64, name=f"res_p2i_{getattr(val,'name','x')}") if self.builder is not None else ir.Constant(self.i64, 0)
        elif hasattr(val, 'type') and isinstance(val.type, ir.IntType):
            # int to int (extend/trunc)
            if val.type.width < 64:
                return self.builder.zext(val, self.i64) if self.builder is not None else ir.Constant(self.i64, 0)
            elif val.type.width > 64:
                return self.builder.trunc(val, self.i64) if self.builder is not None else ir.Constant(self.i64, 0)
            return val
        else:
            # Default zero
            return ir.Constant(self.i64, 0)
