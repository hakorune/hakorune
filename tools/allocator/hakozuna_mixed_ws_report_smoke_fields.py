"""Replacement-front smoke pack report fields for Hakozuna mixed-ws reports."""

from __future__ import annotations

from replacement_front_support import counter_value


def replacement_front_smoke_pack_fields(
    replacement_front_smokes: dict[str, dict[str, str]],
) -> list[str]:
    if not replacement_front_smokes:
        return []
    malloc_family = replacement_front_smokes["malloc_family"]
    cross_thread_free = replacement_front_smokes["cross_thread_free"]
    abandoned_owner = replacement_front_smokes["abandoned_owner"]
    cross_thread_realloc = replacement_front_smokes["cross_thread_realloc"]
    return [
        "replacement_front_product_smoke_pack_v0=1",
        "replacement_front_product_smoke_pack_non_activating=1",
        "replacement_front_malloc_family_smoke_ok=1",
        "replacement_front_cross_thread_free_smoke_ok=1",
        "replacement_front_abandoned_owner_smoke_ok=1",
        "replacement_front_cross_thread_realloc_smoke_ok=1",
        "replacement_front_malloc_family_null_free_smoke_ok=1",
        "replacement_front_malloc_family_alloc_count="
        f"{counter_value(malloc_family, 'replacement_front_alloc_count')}",
        "replacement_front_malloc_family_calloc_count="
        f"{counter_value(malloc_family, 'replacement_front_calloc_count')}",
        "replacement_front_malloc_family_realloc_count="
        f"{counter_value(malloc_family, 'replacement_front_realloc_count')}",
        "replacement_front_malloc_family_free_count="
        f"{counter_value(malloc_family, 'replacement_front_free_count')}",
        "replacement_front_malloc_family_realloc_inplace_count="
        f"{counter_value(malloc_family, 'replacement_front_realloc_inplace_count')}",
        "replacement_front_malloc_family_calloc_zero_bytes="
        f"{counter_value(malloc_family, 'replacement_front_calloc_zero_bytes')}",
        "replacement_front_malloc_family_host_passthrough_count="
        f"{counter_value(malloc_family, 'replacement_front_host_passthrough_count')}",
        "replacement_front_cross_thread_free_policy=remote_queue",
        "replacement_front_abandoned_owner_policy=mark_abandoned_no_host_free",
        "replacement_front_cross_thread_realloc_policy=unsupported_counted",
        "replacement_front_cross_thread_free_remote_free_push_count="
        f"{counter_value(cross_thread_free, 'replacement_front_remote_free_push_count')}",
        "replacement_front_cross_thread_free_remote_free_drain_count="
        f"{counter_value(cross_thread_free, 'replacement_front_remote_free_drain_count')}",
        "replacement_front_cross_thread_free_arena_registry_overflow_count="
        f"{counter_value(cross_thread_free, 'replacement_front_arena_registry_overflow_count')}",
        "replacement_front_abandoned_owner_abandoned_arena_count="
        f"{counter_value(abandoned_owner, 'replacement_front_abandoned_arena_count')}",
        "replacement_front_abandoned_owner_abandoned_remote_free_count="
        f"{counter_value(abandoned_owner, 'replacement_front_abandoned_remote_free_count')}",
        "replacement_front_abandoned_owner_host_passthrough_count="
        f"{counter_value(abandoned_owner, 'replacement_front_host_passthrough_count')}",
        "replacement_front_cross_thread_realloc_unsupported_count="
        f"{counter_value(cross_thread_realloc, 'replacement_front_cross_thread_realloc_unsupported_count')}",
        "replacement_front_cross_thread_realloc_host_passthrough_count="
        f"{counter_value(cross_thread_realloc, 'replacement_front_host_passthrough_count')}",
    ]
