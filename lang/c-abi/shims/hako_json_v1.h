#pragma once

#ifdef __cplusplus
extern "C" {
#endif

// Minimal v1 JSON validator using yyjson
// Returns 0 on success, non-zero on failure and sets *err_out to a short message (malloc'd).
int hako_json_v1_validate_file(const char* path, char** err_out);

#ifdef __cplusplus
}
#endif

