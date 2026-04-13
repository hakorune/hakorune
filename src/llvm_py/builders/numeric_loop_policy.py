"""Numeric loop / SIMD LLVM pass policy seam.

Keep the builder thin by centralizing the current vectorization knob policy
here. The first cut only governs `loop_vectorize` / `slp_vectorize`; fast-math
and FMA widening stay out of scope until a concrete numeric-loop proof exists.
"""

from typing import Any, Tuple


def numeric_loop_vectorization_flags(opt_level: int) -> Tuple[bool, bool]:
    enable = int(opt_level) >= 2
    return enable, enable


def apply_numeric_loop_pass_policy(pmb: Any, opt_level: int) -> None:
    loop_vectorize, slp_vectorize = numeric_loop_vectorization_flags(opt_level)
    pmb.loop_vectorize = loop_vectorize
    pmb.slp_vectorize = slp_vectorize
