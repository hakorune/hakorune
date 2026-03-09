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
        if isinstance(val, (ir.Argument, ir.Constant)):
            return True
        if context is None:
            return False
        if hasattr(val, "add_incoming"):
            try:
                owner = getattr(getattr(val, "basic_block", None), "name", None)
                if owner is None:
                    owner = getattr(getattr(val, "parent", None), "name", None)
                if owner is None:
                    return False
                if isinstance(owner, bytes):
                    owner = owner.decode()
                if isinstance(owner, str) and owner.startswith("bb"):
                    return bool(context.dominates(int(owner[2:]), int(target_bid)))
            except Exception:
                return False
            return False
        try:
            defs = context.def_blocks.get(int(vid), set())
            if len(defs) != 1:
                return False
            def_bid = next(iter(defs))
            return bool(context.dominates(int(def_bid), int(target_bid)))
        except Exception:
            return False

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
        for (bid, vid), phi_val in self.predeclared.items():
            if bid == target_bid and vid not in result:
                result[vid] = phi_val

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
