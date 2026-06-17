from typing import List, Optional

LOCAL_FASTPATH_FACT_ROUTE = "local_fastpath.known_receiver_direct_call"
LOCAL_FASTPATH_FACT_KIND = "local_fastpath_fact"
LOCAL_FASTPATH_BACKEND_KIND_KNOWN_RECEIVER = "known_receiver_direct_call"
LOCAL_FASTPATH_ROUTE_MAP_SCALAR_NO_PUBLICATION_GET = "map_repr.generic_hash_runtime"

_SAFE_COLLECTION_METHOD_EXC = (AttributeError, KeyError, RuntimeError, TypeError, ValueError)


def _as_optional_int(value) -> Optional[int]:
    if value is None:
        return None
    try:
        return int(value)
    except _SAFE_COLLECTION_METHOD_EXC:
        return None


def current_local_fastpath_known_receiver_direct_call_fact(
    *,
    resolver,
    box_name,
    method_name: str,
    receiver_vid,
    arg_ids: List[int],
    required_route_plan: str,
):
    if resolver is None or receiver_vid is None:
        return None
    try:
        block_id = int(getattr(resolver, "current_block_id"))
        instruction_index = int(getattr(resolver, "current_instruction_index"))
        receiver_vid = int(receiver_vid)
    except _SAFE_COLLECTION_METHOD_EXC:
        return None

    facts_by_site = getattr(resolver, "local_fastpath_facts_by_site", None)
    if not isinstance(facts_by_site, dict):
        return None

    key_vid = None
    if arg_ids:
        try:
            key_vid = int(arg_ids[0])
        except _SAFE_COLLECTION_METHOD_EXC:
            return None

    for fact in facts_by_site.get((block_id, instruction_index), []):
        if not isinstance(fact, dict):
            continue
        if fact.get("route_id") != LOCAL_FASTPATH_FACT_ROUTE:
            continue
        if fact.get("fact_kind") != LOCAL_FASTPATH_FACT_KIND:
            continue
        if fact.get("backend_kind") != LOCAL_FASTPATH_BACKEND_KIND_KNOWN_RECEIVER:
            continue
        if fact.get("route_plan") != required_route_plan:
            continue
        if fact.get("box_name") not in (None, str(box_name or "")):
            continue
        if fact.get("method_name") not in (None, method_name):
            continue
        if _as_optional_int(fact.get("receiver_value")) not in (None, receiver_vid):
            continue
        if key_vid is not None and _as_optional_int(fact.get("key_value")) not in (None, key_vid):
            continue
        if fact.get("fallback_reason") not in (None, "", "none"):
            continue
        return fact
    return None
