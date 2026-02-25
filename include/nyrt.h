#ifndef NYRT_H
#define NYRT_H

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Phase 20.36/20.37: C‑ABI scaffold (PoC)
// Minimal kernel API for Rust runtime; subject to evolution.

// Initialize NyRT kernel. Returns 0 on success.
int nyrt_init(void);

// Tear down NyRT kernel.
void nyrt_teardown(void);

// Load MIR(JSON v1) text. Returns a handle (>=1) or 0 on error.
uint64_t nyrt_load_mir_json(const char* json_text);

// Execute main() of the loaded module. Returns process‑like rc (0..255).
int nyrt_exec_main(uint64_t module_handle);

// Verifier gate: validate MIR(JSON v1) contract before execution.
// Returns 0 on success.
int nyrt_verify_mir_json(const char* json_text);

// Safety gate: validate lifecycle/unsafe boundary before execution.
// Returns 0 on success.
int nyrt_safety_check_mir_json(const char* json_text);

// Hostcall bridge (name = "env.*" / provider). Returns 0 on success.
int nyrt_hostcall(const char* name, const char* method,
                  const char* payload_json,
                  /*out*/ char* out_buf, unsigned int out_buf_len);

// Handle lifecycle extensions (Phase 29y ABI shim).
// Contract SSOT:
// - retain_h(0) -> 0
// - release_h(0) -> no-op
int64_t nyrt_handle_retain_h(int64_t handle);
void nyrt_handle_release_h(int64_t handle);

#ifdef __cplusplus
}
#endif

#endif // NYRT_H
