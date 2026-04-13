"""
Function-local state container for LLVM lowering.

Phase 132-P1: Box-First isolation of function-level state.
Prevents cross-function state leakage and collisions.
"""

from typing import Dict, Set, Any, Optional
from llvmlite import ir


class FunctionLowerContext:
    """
    Box containing all function-local lowering state.

    Lifetime: Created at function entry, destroyed at function exit.
    Scope: Single MIR function lowering.

    Benefits:
    - Automatic state isolation (no manual clearing needed)
    - Clear ownership (context belongs to one function)
    - Easy to test (create context, lower, verify)

    Design Principle (Box-First):
    All function-scoped state lives here. This Box is created fresh for each
    function and automatically destroyed when the function lowering completes,
    ensuring zero cross-function contamination.
    """

    def __init__(self, func_name: str):
        """Initialize function-local context.

        Args:
            func_name: Name of the function being lowered (for debugging/tracing)
        """
        self.func_name = func_name

        # Block snapshot state (was: builder.block_end_values with tuple-key)
        # Maps: block_id -> {value_id -> ir.Value}
        # SSOT: End-of-block value snapshots for PHI wiring
        self.block_end_values: Dict[int, Dict[int, ir.Value]] = {}
        # String pointer mirror snapshots for FAST string PHI wiring.
        self.block_end_string_ptrs: Dict[int, Dict[int, ir.Value]] = {}

        # Definition tracking (was: builder.def_blocks)
        # Maps: value_id -> set of block_ids where it's defined
        # Used by resolver to determine if value is defined in current block
        self.def_blocks: Dict[int, Set[int]] = {}

        # Jump-only blocks (was: builder._jump_only_blocks)
        # Maps: jump_only_block_id -> predecessor_block_id
        # Used for snapshot resolution in Pass B
        self.jump_only_blocks: Dict[int, int] = {}

        # PHI management (was: builder.phi_manager)
        # Will be set to PhiManager instance
        self.phi_manager: Any = None  # Type: PhiManager (avoid circular import)

        # Resolver caches (function-local)
        # These caches are keyed by (block_name, value_id) and must be
        # cleared between functions to prevent cross-function collisions
        self.resolver_i64_cache: Dict = {}
        self.resolver_ptr_cache: Dict = {}
        self.resolver_f64_cache: Dict = {}
        self.resolver_end_i64_cache: Dict = {}
        self.resolver_binop_expr_cache: Dict = {}
        self.resolver_compare_expr_cache: Dict = {}

        # String-related caches (function-local)
        self.resolver_string_ids: Set[int] = set()
        self.resolver_array_ids: Set[int] = set()
        self.resolver_string_literals: Dict[int, str] = {}
        self.resolver_string_ptrs: Dict[int, ir.Value] = {}
        self.resolver_length_cache: Dict[int, ir.Value] = {}

        # NewBox→string-arg hints (function-local)
        self.resolver_newbox_string_args: Dict = {}

        # PHI incomings metadata (function-local)
        # Maps: block_id -> {value_id -> [(pred_bid, val_vid), ...]}
        self.block_phi_incomings: Dict[int, Dict[int, Any]] = {}
        # Trivial PHI alias map (function-local)
        # Maps: (block_id, dst_vid) -> src_vid when all incoming source vids are identical.
        # Used to avoid unnecessary PHI placeholders/wiring for copy-like merges.
        self.phi_trivial_aliases: Dict[tuple[int, int], int] = {}

        # FAST compare contract (function-local)
        # ValueIds whose compare result is consumed only by branch cond.
        # Used to keep compare result as i1 in hot loops without changing
        # generic i64 bool behavior for other compare results.
        self.fast_branch_only_compare_dsts: Set[int] = set()

        # FAST const-hoist contract (function-local)
        # Entry metadata for dominance-safe hoist and per-function caches.
        self.entry_block_id: Optional[int] = None
        self.entry_block: Optional[ir.Block] = None
        self.reachable_block_ids: Set[int] = set()
        self.block_dominators: Dict[int, Set[int]] = {}
        self.non_negative_value_ids: Set[int] = set()
        self.integerish_value_ids: Set[int] = set()
        # Loop prepass metadata (function-local)
        # Maps: loop_header_block_id -> annotated loop plan dict
        self.numeric_loop_plans: Dict[int, Dict[str, Any]] = {}
        # LoopSimdContract metadata (function-local)
        # Maps: loop_header_block_id -> proof/policy/lowering/diag contract dict
        self.loop_simd_contracts: Dict[int, Dict[str, Any]] = {}
        self.resolver_hoisted_string_handles: Dict[str, ir.Value] = {}
        self.resolver_hoisted_string_ptrs: Dict[str, ir.Value] = {}

        # LLVM hot-trace counters (function-local)
        # Emitted once per function when NYASH_LLVM_HOT_TRACE=1.
        self.hot_trace_counts: Dict[str, int] = {}

    def get_block_snapshot(self, block_id: int) -> Dict[int, ir.Value]:
        """Get end-of-block value snapshot for a block.

        Args:
            block_id: Block ID to get snapshot for

        Returns:
            Dictionary mapping value_id -> ir.Value (empty dict if not found)
        """
        return self.block_end_values.get(block_id, {})

    def set_block_snapshot(self, block_id: int, snapshot: Dict[int, ir.Value]) -> None:
        """Set end-of-block value snapshot for a block.

        Args:
            block_id: Block ID to set snapshot for
            snapshot: Dictionary mapping value_id -> ir.Value
        """
        self.block_end_values[block_id] = snapshot

    def get_block_string_ptr_snapshot(self, block_id: int) -> Dict[int, ir.Value]:
        """Get end-of-block string pointer mirror snapshot for a block."""
        return self.block_end_string_ptrs.get(block_id, {})

    def set_block_string_ptr_snapshot(self, block_id: int, snapshot: Dict[int, ir.Value]) -> None:
        """Set end-of-block string pointer mirror snapshot for a block."""
        self.block_end_string_ptrs[block_id] = snapshot

    def register_jump_only_block(self, block_id: int, pred_id: int) -> None:
        """Register a block as jump-only (trampoline block).

        Args:
            block_id: ID of jump-only block
            pred_id: ID of predecessor block to copy snapshot from
        """
        self.jump_only_blocks[block_id] = pred_id

    def is_jump_only(self, block_id: int) -> bool:
        """Check if a block is registered as jump-only.

        Args:
            block_id: Block ID to check

        Returns:
            True if block is jump-only, False otherwise
        """
        return block_id in self.jump_only_blocks

    def add_def_block(self, value_id: int, block_id: int) -> None:
        """Record that a value is defined in a block.

        Args:
            value_id: Value ID
            block_id: Block ID where value is defined
        """
        if value_id not in self.def_blocks:
            self.def_blocks[value_id] = set()
        self.def_blocks[value_id].add(block_id)

    def is_defined_in_block(self, value_id: int, block_id: int) -> bool:
        """Check if a value is defined in a specific block.

        Args:
            value_id: Value ID to check
            block_id: Block ID to check

        Returns:
            True if value is defined in the block, False otherwise
        """
        return block_id in self.def_blocks.get(value_id, set())

    def dominates(self, def_bid: int, use_bid: int) -> bool:
        """Best-effort dominance query for lowering-time cache checks."""
        try:
            u = int(use_bid)
            d = int(def_bid)
        except Exception:
            return False
        dom_set = self.block_dominators.get(u)
        if isinstance(dom_set, set):
            return d in dom_set
        # Conservative fallback when dominator metadata is unavailable.
        return d == u

    def __repr__(self) -> str:
        """String representation for debugging."""
        return (
            f"FunctionLowerContext(func_name={self.func_name!r}, "
            f"blocks={len(self.block_end_values)}, "
            f"jump_only={len(self.jump_only_blocks)}, "
            f"defs={len(self.def_blocks)}, "
            f"numeric_loops={len(self.numeric_loop_plans)}, "
            f"simd_contracts={len(self.loop_simd_contracts)})"
        )
