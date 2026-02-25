#include <stdint.h>

// Design-stage C core probe. Returns 0 on success.
int ny_core_probe_invoke(const char* target, const char* method, int32_t argc) {
    // For now, just return success without doing anything.
    (void)target; (void)method; (void)argc;
    return 0;
}

// Design-stage: MapBox.set stub (no-op). Returns 0 on success.
int ny_core_map_set(int32_t type_id, uint32_t instance_id, const char* key, const char* val) {
    (void)type_id; (void)instance_id; (void)key; (void)val;
    return 0;
}

// Design-stage: ArrayBox.push stub (no-op). Returns 0 on success.
int ny_core_array_push(int32_t type_id, uint32_t instance_id, long long val) {
    (void)type_id; (void)instance_id; (void)val;
    return 0;
}

// Design-stage: ArrayBox.get/size stubs (no-op). Return 0 success.
int ny_core_array_get(int32_t type_id, uint32_t instance_id, long long idx) {
    (void)type_id; (void)instance_id; (void)idx;
    return 0;
}

int ny_core_array_len(int32_t type_id, uint32_t instance_id) {
    (void)type_id; (void)instance_id;
    return 0;
}
