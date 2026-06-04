"""Coverage source-observation helpers for mimalloc algorithm coverage reports."""

from __future__ import annotations

from hako_mimalloc_algorithm_coverage_support import (
    REPLACEMENT_FRONT,
    REPLACEMENT_TEMPLATES,
    CoverageRow,
    count_member_calls,
    has_all,
    has_file,
    hako_file,
    read_text,
)


def derive_source_state(rows: list[CoverageRow]) -> dict[str, object]:
    page_box = read_text(hako_file("page_box.hako"))
    hot_core = read_text(hako_file("object_lifecycle_hot_core_box.hako"))
    page_map = read_text(hako_file("page_map_box.hako"))
    page_map_release = read_text(hako_file("page_map_release_box.hako"))
    realloc_same = read_text(hako_file("page_map_realloc_same_class_box.hako"))
    realloc_grow = read_text(hako_file("page_map_realloc_alloc_copy_release_box.hako"))
    huge_model = read_text(hako_file("huge_page_model_box.hako"))
    osvm_source = read_text(hako_file("osvm_page_source_pilot_box.hako"))
    replacement = read_text(REPLACEMENT_FRONT) + "\n" + read_text(REPLACEMENT_TEMPLATES)
    hot_array_fields = ["free", "local_free", "block_used"]
    hot_array_ops = {
        name: {
            "get": count_member_calls(page_box, name, "get"),
            "set": count_member_calls(page_box, name, "set"),
            "push": count_member_calls(page_box, name, "push"),
        }
        for name in hot_array_fields
    }
    hot_array_get_count = sum(ops["get"] for ops in hot_array_ops.values())
    hot_array_set_count = sum(ops["set"] for ops in hot_array_ops.values())
    hot_array_push_count = sum(ops["push"] for ops in hot_array_ops.values())
    hot_array_arraybox_fields = [
        name for name in hot_array_fields if f"{name}: ArrayBox" in page_box
    ]
    hot_array_direct_fields = [
        name for name in hot_array_fields if f"{name}: DirectArrayI64" in page_box
    ]
    hot_array_source_type_ready = int(
        not hot_array_arraybox_fields and len(hot_array_direct_fields) == len(hot_array_fields)
    )
    hot_array_birth_contract_ready = int(
        hot_array_source_type_ready
        and has_all(page_box, ["new DirectArrayI64", ".set("])
        and hot_array_push_count == 0
    )
    hot_array_source_migration_selected = int(
        hot_array_source_type_ready and hot_array_birth_contract_ready
    )
    if hot_array_source_type_ready:
        migration_blocker = "none" if hot_array_birth_contract_ready else "directarray_i64_birth_contract_unverified"
    elif hot_array_push_count:
        migration_blocker = "push_or_initialized_len_contract"
    else:
        migration_blocker = "field_type_and_birth_contract_unverified"
    hotcore_methods = [
        method
        for method in ("objectLifecycleSmallAlloc", "objectLifecycleReleaseBlock")
        if method in hot_core
    ]
    hotcore_small_alloc_calls_acquire_fresh_small = int(
        "page.acquireFreshSmall(" in hot_core
    )
    hotcore_release_calls_release_local_known_live = int(
        "page.releaseLocalKnownLive(" in hot_core
    )
    page_model_hot_methods_ready = int(
        has_all(page_box, ["acquireFreshSmall", "releaseLocalKnownLive"])
    )
    page_map_source_ready = int(
        has_all(page_map, ["findIndex", "register", "lookup", "unregister"])
    )
    page_map_release_source_ready = int(
        has_all(page_map_release, ["releasePtr", "page_map.lookup", "page.releaseLocal", "page_map.unregister"])
    )
    realloc_same_class_source_ready = int(
        has_all(realloc_same, ["tryReallocSameClass", "page_map.lookup", "blockIsLive", "requested_size > page.block_size"])
    )
    realloc_grow_copy_release_source_ready = int(
        has_all(realloc_grow, ["page_map.lookup", "copy", "page_map.register"])
    )
    huge_page_source_ready = int(
        has_all(huge_model, ["register", "lookup", "huge"])
        or has_all(huge_model, ["allocateHuge", "markReleased", "requestedSizeFor"])
    )
    osvm_page_source_pilot_ready = int(
        has_all(osvm_source, ["osvm", "page"]) and has_file(hako_file("osvm_page_source_pilot_box.hako"))
    )
    size_class_single_bridge_supported = has_all(
        replacement,
        [
            "--replacement-front-match-hako-size-class",
            "hako_good_size",
            "hako_good_size_request_ceiling",
        ],
    )
    page_bins_bridge_supported = has_all(
        replacement,
        [
            "--replacement-front-page-bins-mode",
            "page_shaped",
            "HakoReplacement",
            "Page",
            "benchmark_page_bins",
        ],
    )
    locked_front = has_all(
        replacement,
        [
            "HAKO_REPLACEMENT_FRONT_LOCKED",
            "lock_arena",
            "pthread_mutex_lock(&arena_lock)",
        ],
    )
    tls_front = has_all(
        replacement,
        [
            "HAKO_REPLACEMENT_FRONT_THREAD_LOCAL",
            "remote_free_to_owner",
            "arena_registry",
        ],
    )
    replacement_full_hako = int(
        all(row.replacement_front for row in rows if row.area in {
            "size_class_policy",
            "page_local_free_stack",
            "same_thread_local_free",
            "object_lifecycle_hot_core",
            "page_map_lookup",
        })
    )

    return {
        "page_box": page_box,
        "hot_core": hot_core,
        "page_map": page_map,
        "page_map_release": page_map_release,
        "realloc_same": realloc_same,
        "realloc_grow": realloc_grow,
        "huge_model": huge_model,
        "osvm_source": osvm_source,
        "replacement": replacement,
        "hot_array_fields": hot_array_fields,
        "hot_array_ops": hot_array_ops,
        "hot_array_get_count": hot_array_get_count,
        "hot_array_set_count": hot_array_set_count,
        "hot_array_push_count": hot_array_push_count,
        "hot_array_arraybox_fields": hot_array_arraybox_fields,
        "hot_array_direct_fields": hot_array_direct_fields,
        "hot_array_source_type_ready": hot_array_source_type_ready,
        "hot_array_birth_contract_ready": hot_array_birth_contract_ready,
        "hot_array_source_migration_selected": hot_array_source_migration_selected,
        "migration_blocker": migration_blocker,
        "hotcore_methods": hotcore_methods,
        "hotcore_small_alloc_calls_acquire_fresh_small": hotcore_small_alloc_calls_acquire_fresh_small,
        "hotcore_release_calls_release_local_known_live": hotcore_release_calls_release_local_known_live,
        "page_model_hot_methods_ready": page_model_hot_methods_ready,
        "page_map_source_ready": page_map_source_ready,
        "page_map_release_source_ready": page_map_release_source_ready,
        "realloc_same_class_source_ready": realloc_same_class_source_ready,
        "realloc_grow_copy_release_source_ready": realloc_grow_copy_release_source_ready,
        "huge_page_source_ready": huge_page_source_ready,
        "osvm_page_source_pilot_ready": osvm_page_source_pilot_ready,
        "size_class_single_bridge_supported": size_class_single_bridge_supported,
        "page_bins_bridge_supported": page_bins_bridge_supported,
        "locked_front": locked_front,
        "tls_front": tls_front,
        "replacement_full_hako": replacement_full_hako,
    }
