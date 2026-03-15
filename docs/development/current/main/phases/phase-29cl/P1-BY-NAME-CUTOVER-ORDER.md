---
Status: SSOT
Decision: accepted
Date: 2026-03-15
Scope: by-name retirement の fixed order を caller cutover first で固定する。
Related:
  - docs/development/current/main/phases/phase-29cl/P0-BY-NAME-OWNER-INVENTORY.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-task-pack-ssot.md
  - docs/development/current/main/phases/phase-29ck/README.md
---

# P1: By-Name Cutover Order

## 1. Fixed Order

1. `BYN-min1` no-new-mainline lock
   - no new daily caller may be introduced on `nyash.plugin.invoke_by_name_i64`
   - new backend/runtime work must stop at TypeBox ABI v2 / Core C ABI / thin backend boundary
   - landed guard: `tools/checks/phase29cl_by_name_mainline_guard.sh`
   - landed allowlist: `tools/checks/phase29cl_by_name_mainline_allowlist.txt`
2. `BYN-min2` backend caller cutover
   - move visible backend daily callers off module-string `by_name`
   - launcher `build exe` source lane is now cut over to direct `env.codegen.compile_json_path(...)` / `env.codegen.link_object(...)`
   - next owner is compiled-stage1 surrogate shrink, not another visible launcher caller rewrite
3. `BYN-min3` compiled-stage1 surrogate shrink
   - keep `build_surrogate.rs` / `llvm_backend_surrogate.rs` only while proofs still need them
4. `BYN-min4` hook/registry demotion
   - reduce `hako_forward_bridge.rs` / `hako_forward_registry.c` / `hako_kernel.c` to explicit compat-only
5. `BYN-min5` kernel hard retire readiness
   - only when no daily caller and no compiled-stage1 proof still require `by_name`

## 2. Current Daily Caller Reading

1. backend
   - current daily route target is `LlvmBackendBox -> hako_aot`
   - any remaining module-string `by_name` there is temporary
   - visible launcher source lane is now direct `env.codegen.*`; `selfhost.shared.backend.llvm_backend` no longer appears in `lang/src/runner/launcher.hako`
2. compiler selfhost
   - compiled-stage1 `build_surrogate` is temporary bridge keep, not final architecture
3. runtime/plugin
   - final dispatch target is TypeBox ABI v2, not generic named receiver dispatch
4. upstream by-name callers that must shrink before kernel delete
   - `src/llvm_py/instructions/mir_call/method_call.py`
   - `src/backend/mir_interpreter/handlers/calls/method.rs`
   - `src/runtime/type_registry.rs`
   - `src/backend/wasm_v2/unified_dispatch.rs`
   - they remain evidence/cutover pack, not permanent architecture

## 3. Delete Rule

Do not delete:
1. `crates/nyash_kernel/src/plugin/invoke/by_name.rs`
2. `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
3. `crates/nyash_kernel/src/hako_forward_bridge.rs`
4. `crates/nyash_kernel/src/hako_forward_registry.c`

until:
1. `phase-29ck` B1 caller cutover is locked
2. compiled-stage1 surrogates are no longer active proof owners
3. acceptance in `P2` is green
