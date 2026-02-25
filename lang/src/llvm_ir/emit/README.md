# LLVMEmitBox — MIR(JSON v0) → Object (.o)

Responsibility
- Generate an object file from MIR(JSON v0) via a provider.
- Phase 20.34 starts with a provider‑first stub; connects to a Plugin v2 wrapper for ny-llvmc (or llvmlite) next.

Interface (stable)
- `emit_object(mir_json: String, opts: Map|Null) -> String|Null`
  - Returns output path string on success; returns null with a tagged diagnostic on failure.

Tags (Fail‑Fast, stable)
- `[llvmemit/input/null]` — input is null
- `[llvmemit/input/invalid]` — missing `functions`/`blocks`
- `[llvmemit/provider/missing]` — no provider hint
- `[llvmemit/provider/unsupported] <name>` — unsupported provider name
- `[llvmemit/ny-llvmc/not-found]` — ny-llvmc not found (planned)
- `[llvmemit/ny-llvmc/failed status=N]` — ny-llvmc returned non‑zero (planned)
- `[llvmemit/skip] provider stub; implement Plugin v2 call` — current stub behavior

Toggles (default OFF)
- `HAKO_LLVM_EMIT_PROVIDER=ny-llvmc|llvmlite` — select provider
- `HAKO_LLVM_OPT_LEVEL=0..3` — optimization level (provider‑side)
- `HAKO_LLVM_TIMEOUT_MS=60000` — process timeout (provider‑side)

Notes
- Box‑First: keep provider behind a single Plugin v2 method `LLVMCodegenBox.emit_object/2`.
- Environments must not change defaults silently; Fail‑Fast with stable tags.
