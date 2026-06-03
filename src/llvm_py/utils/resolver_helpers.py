"""Resolver Safe Wrapper Functions

Phase 285LLVM-1.5: Centralized type tag access helpers.
Encapsulates hasattr/isinstance checks for cleaner code.
"""

from typing import Optional, Dict, Any

_SAFE_RESOLVER_HELPERS_EXC = (AttributeError, KeyError, RuntimeError, TypeError, ValueError)


def safe_get_type_tag(resolver, vid: int) -> Optional[Dict[str, Any]]:
    """Safely get type tag from resolver.value_types

    Args:
        resolver: Resolver instance (may be None)
        vid: Value ID

    Returns:
        Type tag dict if exists, None otherwise

    Example:
        tag = safe_get_type_tag(resolver, src)
        if tag and tag.get('kind') == 'handle':
            # ... handle-specific logic
    """
    try:
        if resolver is None:
            return None
        if not hasattr(resolver, 'value_types'):
            return None
        if not isinstance(resolver.value_types, dict):
            return None
        tag = resolver.value_types.get(vid)
        if tag is None or not isinstance(tag, dict):
            return None
        return tag
    except _SAFE_RESOLVER_HELPERS_EXC:
        return None


def safe_set_type_tag(resolver, vid: int, tag: Dict[str, Any]) -> bool:
    """Safely set type tag in resolver.value_types

    Args:
        resolver: Resolver instance (may be None)
        vid: Value ID
        tag: Type tag dict (e.g., {'kind': 'handle'})

    Returns:
        True if successfully set, False otherwise

    Example:
        safe_set_type_tag(resolver, dst, {'kind': 'handle', 'box_type': 'StringBox'})
    """
    try:
        if resolver is None:
            return False
        if not hasattr(resolver, 'value_types'):
            # Initialize if missing
            resolver.value_types = {}
        if not isinstance(resolver.value_types, dict):
            # Reset if wrong type
            resolver.value_types = {}
        resolver.value_types[vid] = tag
        return True
    except _SAFE_RESOLVER_HELPERS_EXC:
        return False


def is_handle_type(resolver, vid: int) -> bool:
    """Check if value is tagged as a handle

    Args:
        resolver: Resolver instance (may be None)
        vid: Value ID

    Returns:
        True if value is tagged with kind='handle', False otherwise
    """
    tag = safe_get_type_tag(resolver, vid)
    if tag is None:
        return False
    return tag.get('kind') == 'handle'


def is_string_handle(resolver, vid: int) -> bool:
    """Check if value is tagged as a StringBox handle

    Args:
        resolver: Resolver instance (may be None)
        vid: Value ID

    Returns:
        True if value is a StringBox handle, False otherwise
    """
    tag = safe_get_type_tag(resolver, vid)
    if tag is None:
        return False
    return (tag.get('kind') == 'handle' and
            tag.get('box_type') == 'StringBox')


def get_box_type(resolver, vid: int) -> Optional[str]:
    """Get box_type from type tag

    Args:
        resolver: Resolver instance (may be None)
        vid: Value ID

    Returns:
        box_type string if exists, None otherwise

    Example:
        box_type = get_box_type(resolver, vid)
        if box_type == 'IntegerBox':
            # ... integer-specific logic
    """
    tag = safe_get_type_tag(resolver, vid)
    if tag is None:
        return None
    return tag.get('box_type')


def mark_as_handle(resolver, vid: int, box_type: Optional[str] = None) -> bool:
    """Mark value as a handle

    Args:
        resolver: Resolver instance (may be None)
        vid: Value ID
        box_type: Optional box type (e.g., 'StringBox', 'IntegerBox')

    Returns:
        True if successfully marked, False otherwise

    Example:
        mark_as_handle(resolver, dst, 'StringBox')
        mark_as_handle(resolver, dst)  # Generic handle
    """
    tag = {'kind': 'handle'}
    if box_type is not None:
        tag['box_type'] = box_type
    return safe_set_type_tag(resolver, vid, tag)


def _arrayrepr_fact_store(resolver):
    """Return the mutable arrayrepr fact store, creating it when needed."""
    try:
        if resolver is None:
            return None
        facts = getattr(resolver, "arrayrepr_facts", None)
        if isinstance(facts, dict):
            return facts
        facts = {}
        setattr(resolver, "arrayrepr_facts", facts)
        return facts
    except _SAFE_RESOLVER_HELPERS_EXC:
        return None


def record_arrayrepr_fact(resolver, vid: int, fact: str) -> bool:
    """Record an explicit ArrayRepr fact for one value id."""
    try:
        if not fact:
            return False
        facts = _arrayrepr_fact_store(resolver)
        if facts is None:
            return False
        facts[int(vid)] = fact
        return True
    except _SAFE_RESOLVER_HELPERS_EXC:
        return False


def mark_arrayrepr_direct_i64(resolver, vid: int) -> bool:
    """Mark a value as carrying the explicit ArrayRepr::DirectI64 fact."""
    return record_arrayrepr_fact(resolver, vid, "ArrayRepr::DirectI64")


def get_arrayrepr_fact(resolver, vid: int) -> Optional[str]:
    """Fetch the recorded ArrayRepr fact for a value id."""
    try:
        facts = _arrayrepr_fact_store(resolver)
        if facts is None:
            return None
        fact = facts.get(int(vid))
        return str(fact) if fact is not None else None
    except _SAFE_RESOLVER_HELPERS_EXC:
        return None


def is_arrayrepr_direct_i64(resolver, vid: int) -> bool:
    """Check whether a value carries the explicit ArrayRepr::DirectI64 fact."""
    return get_arrayrepr_fact(resolver, vid) == "ArrayRepr::DirectI64"


def is_stringish_legacy(resolver, vid: int) -> bool:
    """Legacy is_stringish check (via resolver.is_stringish or value_types)

    Phase 285LLVM-1.5: Transitional helper during migration from string_ids to value_types.
    Checks both old (string_ids set) and new (value_types dict) paths.

    Args:
        resolver: Resolver instance (may be None)
        vid: Value ID

    Returns:
        True if value is marked as stringish, False otherwise
    """
    try:
        # New path: value_types with box_type='StringBox'
        if is_string_handle(resolver, vid):
            return True

        # Legacy path: resolver.is_stringish()
        if resolver is not None and hasattr(resolver, 'is_stringish'):
            return resolver.is_stringish(vid)

        return False
    except _SAFE_RESOLVER_HELPERS_EXC:
        return False
