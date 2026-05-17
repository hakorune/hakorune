# pure-first-same-module-static-helper-global-call-proof

Purpose: prove `PURE-FIRST-GLOBAL-CALL-001`.

This proof keeps the acceptance shape small:

```text
static box Main helper
  -> same-module global_call_routes row
  -> lowering_plan DirectAbi direct_function_call
  -> pure-first EXE
```

It covers both scalar and typed object helper returns without adding allocator
behavior or backend app/name matchers.

