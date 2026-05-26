# Hakorune vs Other Languages — Feature Comparison

Perspective
- Hakorune emphasizes a unified Box model, hygienic AST macros with sandboxing, and multi‑backend execution (Rust VM/LLVM/Cranelift/WASM, plus legacy PyVM opt-in).

Axes
- Control Flow & SSA
- Hakorune: explicit observability (hints), PHI hygiene invariants.
  - Rust/Swift/Kotlin: SSA is internal; limited direct observability.
- Exceptions
- Hakorune: postfix `catch/cleanup` (scope‑first), zero‑cost lowering.
  - Rust: Result/? idiom (no exceptions). Swift/Kotlin: try/catch/finally.
- Macros
- Hakorune: AST JSON v0, hygienic by construction, isolated child with capabilities.
  - C: text macro. Rust: macro_rules!/proc‑macros. Lisp/Julia: homoiconic AST.
- Scope
- Hakorune: ScopeBox (compile‑time metadata) and ScopeEnter/Leave hints; disappears at runtime.
  - Go/Rust/Swift: lexical scopes (no explicit observability layer).
- Backends
- Hakorune: Rust VM (default), PyVM (historical/opt-in), LLVM (AOT), Cranelift (JIT), WASM.
  - Others: single backend or VM.

Takeaways
- Hakorune’s differentiator is “observability without cost” and macro safety by default.
- Where trade‑offs exist (e.g., temporary hygiene valves), they’re gated and documented.
