This directory will host public headers for the C ABI kernel line.

Canonical header paths:
- general C ABI surface: `lang/c-abi/include/hako_hostbridge.h`
- AOT compile/link surface: `lang/c-abi/include/hako_aot.h`

Diagnostics helpers (Fail‑Fast):
- `lang/c-abi/include/hako_diag.h` provides `HAKO_FAIL_WITH(err_out, "CODE", "message")`.
  - Including files must define `hako_set_last_error` and `set_err`.
  - Use this to keep short error codes consistent (OK / OOM / FAILED / NOT_FOUND / VALIDATION / UNSUPPORTED).

Compatibility:
- `include/hako_hostbridge.h` may remain as a thin shim that includes `hako_aot.h` for AOT declarations during transition.
