---
Status: SSOT
Decision: accepted
Date: 2026-03-15
Scope: by-name retirement の fixed order を caller cutover first で固定する。
Related:
  - docs/development/current/main/phases/phase-29cl/P0-BY-NAME-OWNER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P3-BYN-MIN3-COMPILED-STAGE1-SURROGATE-CLOSEOUT.md
  - docs/development/current/main/phases/phase-29cl/P4-BYN-MIN4-HOOK-REGISTRY-CLOSEOUT.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P6-BYN-MIN5-DAILY-CALLER-SHRINK.md
  - docs/development/current/main/phases/phase-29cl/P7-BYN-MIN5-COMPILED-STAGE1-PROOF-FREEZE.md
  - docs/development/current/main/phases/phase-29cl/P8-BYN-MIN5-COMPAT-KEEP-ARCHIVE-ONLY.md
  - docs/development/current/main/design/de-rust-full-rust-zero-remaining-rust-task-pack-ssot.md
  - docs/development/current/main/phases/phase-29ck/README.md
---

# P1: By-Name Cutover Order

## 1. Fixed Order

1. `BYN-min1` no-new-mainline lock
   - no new daily caller may be introduced on `nyash.plugin.invoke_by_name_i64`
   - current compat-only keep owner set is allowlisted exactly; expanding that set is a regression
   - new backend/runtime work must stop at TypeBox ABI v2 / Core C ABI / thin backend boundary
   - landed guard: `tools/checks/phase29cl_by_name_mainline_guard.sh`
   - landed allowlist: `tools/checks/phase29cl_by_name_mainline_allowlist.txt`
2. `BYN-min2` backend caller cutover
   - move visible backend daily callers off module-string `by_name`
   - launcher `build exe` source lane is now cut over off explicit `invoke_by_name_i64`; module-string backend literals are still acceptable while direct-known-box lowering keeps them off the generic by-name tail
   - next owner is compiled-stage1 surrogate shrink, not another visible launcher caller rewrite
3. `BYN-min3` compiled-stage1 surrogate closeout
   - keep `build_surrogate.rs` / `llvm_backend_surrogate.rs` only while proofs still need them
   - close-sync is landed; reopen only on fresh caller-proof
4. `BYN-min4` hook/registry demotion
   - close-sync is landed; `hako_forward_bridge.rs` / `hako_forward.rs` / `hako_forward_registry.c` / `hako_forward_registry_shared_impl.inc` / `hako_kernel.c` stay explicit compat-only keep owners until fresh live caller proof says otherwise
5. `BYN-min5` kernel hard retire readiness
   - only when no daily caller and no compiled-stage1 proof still require `by_name`
6. `P6-BYN-MIN5-DAILY-CALLER-SHRINK.md`
   - first blocker bucket for `BYN-min5` readiness
7. `P7-BYN-MIN5-COMPILED-STAGE1-PROOF-FREEZE.md`
   - frozen proof owners bucket; keep surrogate cluster explicit until caller-proof says removable
8. `P8-BYN-MIN5-COMPAT-KEEP-ARCHIVE-ONLY.md`
   - compat keep bucket; decide whether the keep cluster can be archived-only
9. `P9-BYN-MIN5-READINESS-JUDGMENT.md`
   - hard-retire readiness judgment bucket; negative today because caller/proof residue still remains
10. `P10-BYN-MIN5-FILEBOX-COMPAT-LEAF-SHRINK.md`
   - narrowest next blocker bucket under the negative `P9` judgment

## 2. Current Daily Caller Reading

1. backend
   - current daily route target is `LlvmBackendBox -> hako_aot`
   - any remaining module-string `by_name` there is temporary
   - visible launcher source lane no longer reaches backend through explicit `invoke_by_name_i64`; a `selfhost.shared.backend.llvm_backend` receiver literal may still appear while direct-known-box lowering keeps the route off the generic by-name tail
2. compiler selfhost
   - compiled-stage1 `build_surrogate` is temporary bridge keep, not final architecture
3. runtime/plugin
   - final dispatch target is TypeBox ABI v2, not generic named receiver dispatch
4. upstream by-name callers that must shrink before kernel delete
   - `src/llvm_py/instructions/direct_box_method.py`
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
