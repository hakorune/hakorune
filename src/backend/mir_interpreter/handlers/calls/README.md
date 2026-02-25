# handlers/calls — Call Handling Layer

Purpose: isolate call handling by callee kind for the MIR interpreter.

- In scope: routing `handle_call`, per‑callee executors, legacy name resolution.
- Out of scope: arithmetic/box handlers, memory ops, extern provider registry.

Do not import sibling handler modules directly from here (keep boundaries tight).
Use `super::*` only and call through the interpreter methods.

Files
- `mod.rs`: entry point and callee routing
- `global.rs`: global function calls (Callee::Global)
- `method.rs`: instance/static method calls (Callee::Method)
- `externs.rs`: extern calls (Callee::Extern)

Removal status (Phase 2 complete)
- Unified callee path is default。by‑name 旧経路は削除済み（`legacy.rs`/`resolution.rs`）。
- `callee=None` の呼び出しは「厳密一致のモジュール関数名」以外は Fail‑Fast（ビルダーで Callee を付与してね）。

Extern SSOT
- `externs.rs` is the runtime SSOT for provider dispatch. Global calls that are extern-like should delegate here (e.g., `env.get`).
- Arity suffix normalization: names like `env.get/1` are accepted and normalized to `env.get` before dispatch (both in Global and ExternCall paths).

Layer Guard
- Scope: call routing only (Global/Method/Extern/Legacy isolation). Do not import provider registries or runtime plugins directly from here.
- Use helpers under `super::super::utils::*` for shared concerns (e.g., `normalize_arity_suffix`).
