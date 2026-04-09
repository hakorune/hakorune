from typing import Any, Dict, Optional

import llvmlite.ir as ir

from instructions.sum_ops import _resolve_local_sum_aggregate, materialize_local_sum_aggregate


def materialize_sum_escape_value_if_needed(
    builder: ir.IRBuilder,
    module: ir.Module,
    value_id: Optional[int],
    vmap: Dict[int, Any],
    resolver=None,
    *,
    name_hint: str = "sum_escape",
):
    if not isinstance(value_id, int):
        return None

    local_sum = _resolve_local_sum_aggregate(int(value_id), vmap, resolver)
    if local_sum is None:
        return None

    return materialize_local_sum_aggregate(
        builder,
        module,
        local_sum,
        resolver,
        name_hint=name_hint,
    )
