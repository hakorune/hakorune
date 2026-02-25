Normalize Passes (AOT Prep)

Purpose
- Provide modular, opt-in JSON v0 normalizers to keep Rust normalize.rs minimal and move pre-ny-llvmc shaping into .hako.

Passes (initial)
- NormalizePrintBox: rewrite `op: print` to `externcall env.console.log(value)`.
- NormalizeRefBox: rewrite `ref_get/ref_set` to `boxcall getField/setField` with a string field operand (best-effort; no CFG change).

Toggles (default OFF)
- HAKO_MIR_NORMALIZE_PRINT=1
- HAKO_MIR_NORMALIZE_REF=1

Notes
- These operate on MIR(JSON v0) text. They are simple, conservative string transforms built on JsonFrag utilities.
- Keep effects and CFG unchanged. For ref_set, inserting a barrier is not enforced here; AOT will later select the shortest extern hot-path.

