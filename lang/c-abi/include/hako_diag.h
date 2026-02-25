// hako_diag.h — Short diagnostics helpers for C-ABI shims (Fail-Fast)
// This header defines minimal macros for emitting a short error and setting
// the thread-local last error string, without hiding failures.

#ifndef HAKO_DIAG_H
#define HAKO_DIAG_H

// The including file must provide:
//   void hako_set_last_error(const char* short_msg);
//   static int set_err(char** err_out, const char* msg);

// Fail with a short code and detailed message (single-line).
#define HAKO_FAIL_WITH(ERRPTR, SHORT, MSG) do { \
  hako_set_last_error(SHORT); \
  return set_err((ERRPTR), (MSG)); \
} while(0)

#endif // HAKO_DIAG_H

