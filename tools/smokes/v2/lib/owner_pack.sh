#!/usr/bin/env bash

# Owner-pack selection is a discovery contract, not a runtime profile.
owner_pack_manifest_path() {
    if [[ "$OWNER_PACK_MODE" = true && ( -z "$SUITE" || "$SUITE" == */* || "$SUITE" == *..* ) ]]; then
        log_error "Owner-pack suite must be one owner-local manifest name: $SUITE"
        return 1
    fi
    printf '%s/suites/%s/%s.txt' "$SCRIPT_DIR" "$OWNER_PROFILE" "$SUITE"
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
