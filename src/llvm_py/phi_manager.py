"""
Phase 132-Post: PHI Management Box

Box-First principle: Encapsulate PHI lifecycle management
- Track PHI ownership (which block created which PHI)
- Protect PHIs from overwrites (SSOT principle)
- Filter vmap to preserve PHI values
"""

import llvmlite.ir as ir

class PhiManager:
    """PHI value lifecycle manager (Box pattern)"""

    def __init__(self):
        self.predeclared = {}  # (bid, vid) -> phi_value

    def register_phi(self, bid: int, vid: int, phi_value):
        """Register a PHI as owned by specific block"""
        self.predeclared[(bid, vid)] = phi_value

    def is_phi_owned(self, bid: int, vid: int) -> bool:
        """Check if PHI is owned by block"""
        return (bid, vid) in self.predeclared

    def _is_global_safe_value(self, val) -> bool:
        return isinstance(val, (ir.Argument, ir.Constant))

    def _phi_owner_block_id(self, val):
        try:
            owner = getattr(getattr(val, "basic_block", None), "name", None)
            if owner is None:
                owner = getattr(getattr(val, "parent", None), "name", None)
            if isinstance(owner, bytes):
                owner = owner.decode()
            if isinstance(owner, str) and owner.startswith("bb"):
                return int(owner[2:])
        except Exception:
            return None
        return None

    def _phi_owner_dominates_target(self, val, target_bid: int, context) -> bool:
        if context is None:
            return False
        owner_bid = self._phi_owner_block_id(val)
        if owner_bid is None:
            return False
        try:
            return bool(context.dominates(int(owner_bid), int(target_bid)))
        except Exception:
            return False

    def _single_def_block_id(self, vid, context):
        if context is None:
            return None
        try:
            defs = context.def_blocks.get(int(vid), set())
            if len(defs) != 1:
                return None
            return next(iter(defs))
        except Exception:
            return None

    def _single_def_dominates_target(self, vid, target_bid: int, context) -> bool:
        def_bid = self._single_def_block_id(vid, context)
        if def_bid is None:
            return False
        try:
            return bool(context.dominates(int(def_bid), int(target_bid)))
        except Exception:
            return False

    def _merge_predeclared_for_target(self, result: dict, target_bid: int):
        for (bid, vid), phi_val in self.predeclared.items():
            if bid == target_bid and vid not in result:
                result[vid] = phi_val

    def _is_cross_block_safe(self, vid, val, target_bid: int, context) -> bool:
        """Return True when a global vmap value is safe to reuse in target block.

        Contract:
        - Function arguments and LLVM constants are safe everywhere.
        - A PHI is only safe when its actual owner block dominates the target
          block. MIR-level def_blocks are not enough because later localization
          PHIs reuse the same value-id.
        - Any non-PHI SSA value is only safe when it has a single defining
          block and that block dominates the target block.
        - Multi-def values must be re-resolved via snapshots/PHI, not copied.
        """
        if self._is_global_safe_value(val):
            return True
        if hasattr(val, "add_incoming"):
            return self._phi_owner_dominates_target(val, target_bid, context)
        return self._single_def_dominates_target(vid, target_bid, context)

    def filter_vmap_preserve_phis(self, vmap: dict, target_bid: int, context=None) -> dict:
        """Filter vmap while preserving owned PHIs

        SSOT: PHIs in vmap are the single source of truth
        Phase 132-P0: Also add PHIs from predeclared registry
        """
        result = {}
        for vid, val in vmap.items():
            if self._is_cross_block_safe(vid, val, target_bid, context):
                result[vid] = val

        # Phase 132-P0: Add PHIs from predeclared that aren't in vmap yet
        self._merge_predeclared_for_target(result, target_bid)

        return result

    def sync_protect_phis(self, target_vmap: dict, source_vmap: dict):
        """Sync values but protect existing PHIs (Fail-Fast)

        Never overwrite PHIs - they are SSOT
        """
        for vid, val in source_vmap.items():
            existing = target_vmap.get(vid)
            if existing and hasattr(existing, 'add_incoming'):
                continue  # SSOT: Don't overwrite PHIs
            target_vmap[vid] = val
