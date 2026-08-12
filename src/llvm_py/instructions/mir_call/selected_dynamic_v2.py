"""Non-emitting early seam for selected Dynamic V2 metadata.

The seam is intentionally not imported by the production dispatcher yet.  It
is a direct test/canary consumer for the I0-D1 transport contract; execution,
provider lookup, and generic-method fallback belong to the later activation.
"""

from typing import Any, Dict, Optional

from builders.dynamic_v2_aot_admission import (
    DynamicV2AotCallView,
    DynamicV2AotAdmissionView,
    load_selected_dynamic_v2_aot_admission,
)


def inspect_selected_dynamic_v2_call(
    func_data: Dict[str, Any], block: int, instruction_index: int
) -> Optional[DynamicV2AotCallView]:
    """Return the sealed call metadata at a site, without emitting LLVM."""

    admission: Optional[DynamicV2AotAdmissionView] = load_selected_dynamic_v2_aot_admission(
        func_data
    )
    if admission is None:
        return None
    return admission.require_call_site(block, instruction_index)
