# Phase 277 P1.5: Structured error handling for PHI strict mode

from dataclasses import dataclass
from typing import Optional


@dataclass
class PhiStrictError:
    """Structured error for PHI strict mode violations.

    Phase 277 P1.5: Central error handling for all PHI strict violations.
    """
    message: str
    next_file: str
    block_id: Optional[int] = None
    dst_vid: Optional[int] = None
    context: Optional[str] = None

    def raise_if_strict(self):
        """Raise RuntimeError if strict mode enabled."""
        from .debug_helper import is_phi_strict_enabled
        import sys

        if is_phi_strict_enabled():
            print(f"[CRITICAL] {self.message}", file=sys.stderr)
            if self.context:
                print(f"  Context: {self.context}", file=sys.stderr)
            print(f"  → Next file: {self.next_file}", file=sys.stderr)
            raise RuntimeError(self.message)


@dataclass
class PhiDebugMessage:
    """Structured debug message for PHI operations.

    Phase 277 P1.5: Centralize debug output formatting.
    """
    level: str  # "DEBUG", "WARNING", "INFO"
    component: str  # e.g., "phi_wiring", "ensure_phi"
    message: str
    block_id: Optional[int] = None
    dst_vid: Optional[int] = None

    def print_if_enabled(self):
        """Print if appropriate debug mode enabled."""
        from .debug_helper import is_phi_debug_enabled
        import sys

        if is_phi_debug_enabled():
            prefix = f"[{self.component}]" if self.component else "[phi]"
            full_msg = f"{prefix} {self.message}"
            if self.block_id is not None or self.dst_vid is not None:
                context = ""
                if self.block_id is not None:
                    context += f"bb{self.block_id}"
                if self.dst_vid is not None:
                    if context:
                        context += " "
                    context += f"v{self.dst_vid}"
                full_msg += f" ({context})"
            print(f"{full_msg}", file=sys.stderr)
