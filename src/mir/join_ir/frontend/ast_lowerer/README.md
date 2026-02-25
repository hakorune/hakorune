AST/CFG → JoinIR frontend lowering layer  

Scope:
- Normalize tiny AST/CFG patterns into JoinIR instructions without touching MIR or runtime concerns.
- Keep pattern-specific lowering isolated (if/return, loop variants, nested-if, read_quoted_from).
- Centralize expression/value extraction and small analysis helpers (if-in-loop var tracking).

Boundaries:
- No code generation beyond JoinIR; MIR/VM concerns belong to the bridge layer.
- Dev-flagged paths stay opt-in (HAKO_JOINIR_NESTED_IF, HAKO_JOINIR_READ_QUOTED*).
- Avoid hard-coded semantics; prefer structural pattern detection and reusable helpers.

Layout:
- `mod.rs`: public surface + entry dispatch + shared counters
- `context.rs`: `ExtractCtx` (var ids) helpers
- `expr.rs`: expression-to-JoinIR value extraction
- `if_return.rs`: simple if→Select lowering
- `loop_patterns.rs`: loop variants (simple/break/continue)
- `read_quoted.rs`: read_quoted_from lowering
- `nested_if.rs`: NestedIfMerge lowering/detection
- `analysis.rs`: loop if-var analysis + metadata helpers
- `tests.rs`: frontend lowering tests gated by dev flags
