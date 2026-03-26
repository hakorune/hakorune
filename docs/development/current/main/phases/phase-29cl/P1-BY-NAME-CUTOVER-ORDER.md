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
  - docs/development/current/main/phases/phase-29cl/P18-BYN-MIN5-LLVM-BACKEND-SURROGATE-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P19-BYN-MIN5-HAKO-FORWARD-BRIDGE-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P20-BYN-MIN5-HAKO-FORWARD-REGISTRY-SHARED-IMPL-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
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
   - hard-retire readiness judgment bucket
   - current result: positive; no new mainline caller remains, and proof/keep residue is frozen exact rather than live-owner ambiguous
10. `P10-BYN-MIN5-FILEBOX-COMPAT-LEAF-SHRINK.md`
   - narrowest next blocker bucket under the negative `P9` judgment
11. `P11-BYN-MIN5-METHOD-DISPATCH-SHRINK.md`
   - runtime method-dispatch bucket for the next name-resolution dependent migration slice
   - close-sync is landed; return to `P9` readiness re-check before opening another shrink bucket
12. `P12-BYN-MIN5-FILEBOX-WRITE-COMPAT-SHRINK.md`
   - remove only `FileBox.write` from the explicit Python-side compat leaf
   - close-sync is landed; return to `P9` readiness re-check before opening another shrink bucket
13. `P13-BYN-MIN5-COMPILED-STAGE1-PROOF-READINESS-INVENTORY.md`
   - inspect whether the surrogate proof cluster is still live owner or archive-ready
   - current result: archive-only proof residue; move next to compat keep readiness
14. `P14-BYN-MIN5-COMPAT-KEEP-READINESS-INVENTORY.md`
   - inspect whether the compat keep cluster is still live keep surface or archive-ready
   - current result: explicit frozen keep set; move next to the built-in `FileBox` keep residue
15. `P15-BYN-MIN5-FILEBOX-BUILTIN-KEEP-INVENTORY.md`
   - inspect whether the built-in `FileBox` branch in `plugin/invoke/by_name.rs` can shrink further before any broader compat-keep retirement judgment
   - current result: `writeBytes` is the narrowest next shrink bucket
16. `P16-BYN-MIN5-FILEBOX-WRITEBYTES-COMPAT-SHRINK.md`
   - shrink only `FileBox.writeBytes` before reopening any broader `FileBox` family question
   - close-sync is landed; return to `P9` readiness re-check before opening another shrink bucket
17. `P17-BYN-MIN5-BUILD-SURROGATE-READINESS-INVENTORY.md`
   - inspect whether `build_surrogate.rs` is still a live proof owner or archive-ready
   - current result: archive-only proof residue
18. `P18-BYN-MIN5-LLVM-BACKEND-SURROGATE-READINESS-INVENTORY.md`
   - inspect whether the combined backend surrogate route is still a live proof owner or archive-ready
   - current result: archive-only proof residue; keep `compile_obj` and `link_exe` paired until execution says removable
19. `P19-BYN-MIN5-HAKO-FORWARD-BRIDGE-READINESS-INVENTORY.md`
   - inspect whether the Rust-side keep bridge is still a live keep owner or archive-ready
   - current result: frozen exact keep bridge; keep register/try-call/fallback bridge paired until execution says removable
20. `P20-BYN-MIN5-HAKO-FORWARD-REGISTRY-SHARED-IMPL-READINESS-INVENTORY.md`
   - inspect whether the shared C registry body is still a live keep owner or archive-ready
   - current result: frozen exact keep body; keep hook storage/register/try-call body paired until execution says removable
21. `P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md`
   - readiness is now positive
   - hard-retire execution is active
   - `FileBox.open`, `FileBox.read`, `FileBox.close`, and `FileBox.readBytes` slices are landed
   - `FileBox.write` built-in keep retire is landed
   - `InstanceBox.getField/setField` built-in keep retire is landed
   - `P24-BYN-MIN5-KNOWN-BOX-DIRECT-MISS-INVENTORY.md` is landed
   - `P25-BYN-MIN5-CORE-BY-NAME-SURFACE-INVENTORY.md` is landed
   - `P26-BYN-MIN5-MODULE-STRING-DISPATCH-SURFACE-INVENTORY.md` is landed
   - `P27-BYN-MIN5-MIRBUILDER-DIRECT-MISS-RETIRE.md` is landed
   - `P28-BYN-MIN5-MODULE-STRING-DISPATCH-LIVE-ROUTER-INVENTORY.md` is landed
   - `P29-BYN-MIN5-USING-RESOLVER-STUB-INVENTORY.md` is landed with current result `still-live keep`
   - `P30-BYN-MIN5-MIRBUILDER-SOURCE-SEAM-INVENTORY.md` is landed with current result `still-live compat owner`
   - `P31-BYN-MIN5-MIRBUILDER-PROGRAM-JSON-SEAM-INVENTORY.md` is landed with current result `still-live compat owner`
   - `P32-BYN-MIN5-PROGRAM-JSON-LIVE-CALLER-INVENTORY.md` is landed with current result `.hako live/bootstrap callers = monitor-only / near-thin-floor`
   - `P33-BYN-MIN5-PROGRAM-JSON-SHELL-HELPER-INVENTORY.md` is landed with current result `helper trio is heterogeneous; first helper-local bucket = tools/hakorune_emit_mir.sh`
   - `P34-BYN-MIN5-HAKORUNE-EMIT-MIR-HELPER-INVENTORY.md` is landed with current result `tools/hakorune_emit_mir.sh` stays live; first exact seam = generated selfhost builder runner path`
   - `P35-BYN-MIN5-EMIT-MIR-SELFHOST-RUNNER-SEAM-INVENTORY.md` is landed with current result `execution code landed; generated selfhost builder runner seam = near-thin-floor / monitor-only`
   - `P36-BYN-MIN5-SELFHOST-BUILD-HELPER-INVENTORY.md` is landed with current result `selfhost_build helper = monitor-only under accepted phase-29ci proof`
   - helper-local fan-out stops here under the current accepted keep set; reopen only on a fresh exact seam

## 2. Current Daily Caller Reading

1. backend
   - current daily route target is `LlvmBackendBox -> hako_aot`
   - any remaining module-string `by_name` there is temporary
   - visible launcher source lane no longer reaches backend through explicit `invoke_by_name_i64`; a `selfhost.shared.backend.llvm_backend` receiver literal may still appear while direct-known-box lowering keeps the route off the generic by-name tail
2. compiler selfhost
   - compiled-stage1 `build_surrogate` and `llvm_backend_surrogate` are archive-only proof residue, not final architecture
   - execution should not reopen those surrogates while direct caller proof stays green
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
2. remaining FileBox compat execution residue is moved without reviving `by_name`
3. acceptance in `P2` is green
