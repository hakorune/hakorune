#!/usr/bin/env bash

# Owner-pack selection is a discovery contract, not a runtime profile.
declare -A AGGREGATE_NODE_FATE=()

owner_pack_manifest_path() {
    if [[ "$OWNER_PACK_MODE" = true && ( -z "$SUITE" || "$SUITE" == */* || "$SUITE" == *..* ) ]]; then
        log_error "Owner-pack suite must be one owner-local manifest name: $SUITE"
        return 1
    fi
    printf '%s/suites/%s/%s.txt' "$SCRIPT_DIR" "$OWNER_PROFILE" "$SUITE"
}

aggregate_node_manifest_path() {
    printf '%s/suites/%s/aggregate-nodes.txt' "$SCRIPT_DIR" "$OWNER_PROFILE"
}

load_aggregate_nodes() {
    # Aggregate wrappers are excluded only from the default owner discovery.
    if [ "$OWNER_PROFILE" != "integration" ] || [ -n "$SUITE" ]; then
        return 0
    fi

    local manifest
    manifest="$(aggregate_node_manifest_path)"
    if [ ! -f "$manifest" ]; then
        log_error "Aggregate-node manifest not found: $manifest"
        return 1
    fi

    AGGREGATE_NODE_FATE=()
    local raw_line=""
    local line=""
    local line_no=0
    while IFS= read -r raw_line || [ -n "$raw_line" ]; do
        line_no=$((line_no + 1))
        line="$(trim_manifest_line "$raw_line")"
        case "$line" in
            ""|\#*) continue ;;
        esac

        local aggregate_path=""
        local fate=""
        local child_suite=""
        IFS='|' read -r aggregate_path fate child_suite extra <<< "$line"
        if [ -n "${extra:-}" ] || [ -z "$aggregate_path" ] || [ -z "$fate" ] || [ -z "$child_suite" ]; then
            log_error "Invalid aggregate-node row: $manifest:$line_no"
            return 1
        fi
        if [[ "$aggregate_path" = /* || "$aggregate_path" == *..* || "$child_suite" = */* || "$child_suite" == *..* ]]; then
            log_error "Aggregate-node row escapes its owner: $manifest:$line_no"
            return 1
        fi
        if [ "$fate" != "ExplicitOnlyAggregate" ]; then
            log_error "Unknown aggregate-node fate: $manifest:$line_no -> $fate"
            return 1
        fi
        if [ -n "${AGGREGATE_NODE_FATE[$aggregate_path]+x}" ]; then
            log_error "Duplicate aggregate-node path: $manifest:$line_no -> $aggregate_path"
            return 1
        fi
        AGGREGATE_NODE_FATE["$aggregate_path"]="$fate|$child_suite"
    done < "$manifest"

    if [ ${#AGGREGATE_NODE_FATE[@]} -eq 0 ]; then
        log_error "Aggregate-node manifest is empty: $manifest"
        return 1
    fi

    local profile_dir="$SCRIPT_DIR/profiles/$OWNER_PROFILE"
    local aggregate_path=""
    local child_suite=""
    local suite_path=""
    for aggregate_path in "${!AGGREGATE_NODE_FATE[@]}"; do
        if [ ! -f "$profile_dir/$aggregate_path" ]; then
            log_error "Aggregate-node path is not live: $manifest -> $aggregate_path"
            return 1
        fi
        child_suite="${AGGREGATE_NODE_FATE[$aggregate_path]#*|}"
        suite_path="$SCRIPT_DIR/suites/$OWNER_PROFILE/$child_suite.txt"
        if [ ! -f "$suite_path" ]; then
            log_error "Aggregate-node child suite is missing: $manifest -> $child_suite"
            return 1
        fi
        if grep -Fxq "$aggregate_path" "$suite_path"; then
            log_error "Aggregate-node cannot also be a child leaf: $manifest -> $aggregate_path"
            return 1
        fi
    done
}

is_explicit_only_aggregate() {
    local relative_path="$1"
    [ -n "${AGGREGATE_NODE_FATE[$relative_path]+x}" ]
}

validate_owner_pack_selection() {
    local selected_count="$1"
    if [ "$OWNER_PACK_MODE" != true ]; then
        return 0
    fi

    local entry=""
    local missing=()
    for entry in "${!SUITE_ALLOWLIST[@]}"; do
        if [ "${SUITE_SELECTED[$entry]:-0}" != "1" ]; then
            missing+=("$entry")
        fi
    done
    if [ ${#missing[@]} -ne 0 ] || [ "$selected_count" -ne "${#SUITE_ALLOWLIST[@]}" ]; then
        log_error "Owner pack did not select every manifest entry before execution"
        for entry in "${missing[@]}"; do
            echo "  - $entry" >&2
        done
        return 1
    fi
}
