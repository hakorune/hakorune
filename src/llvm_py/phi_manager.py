"""
Phase 132-Post: PHI Management Box

Box-First principle: Encapsulate PHI lifecycle management
- Track PHI ownership (which block created which PHI)
- Protect PHIs from overwrites (SSOT principle)
- Filter vmap to preserve PHI values
"""

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

    def filter_vmap_preserve_phis(self, vmap: dict, target_bid: int) -> dict:
        """Filter vmap while preserving owned PHIs

        SSOT: PHIs in vmap are the single source of truth
        Phase 132-P0: Also add PHIs from predeclared registry
        """
        result = {}
        for vid, val in vmap.items():
            # PHIs are valid SSA values across dominated blocks; keep them in the per-block view.
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
