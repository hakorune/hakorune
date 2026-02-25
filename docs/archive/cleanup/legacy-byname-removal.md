Status: Historical

Legacy By‑Name Call Removal Plan

Context
- Unified call system is default. VM routes by `Callee` kind (Global/Method/Extern/…)
- Legacy by‑name resolution (callee=None → string name lookup) is gated by env and OFF by default.
- Goal: remove legacy code path once parity is proven at integration/full levels.

Guards / Current Behavior
- 旧 by‑name 経路は削除済み。`NYASH_VM_ALLOW_LEGACY` は廃止。
- 既定: `callee=None` 生成時は Fail‑Fast（Builder で Callee を必ず付与）。

Scope of Work
1) Identify callers that could still rely on legacy resolution
   - Builder emissions that may omit `callee` (should be none after Phase 1 refactor)
   - Any VM entry points or helpers constructing by‑name calls dynamically
   - Older tests or tools expecting by‑name lookups

2) Ensure unified path covers them
   - For Global user functions: attach `Callee::Global("Module.func")`
   - For dynamic/indirect calls: mark `Callee::Value` and keep VM restrictions explicit
   - For extern‑like names: delegate via handlers/calls/externs.rs (SSOT)
   - Normalize arity (`foo/1` → `foo`) via `utils::normalize_arity_suffix`

3) Strengthen tests
   - Smokes/quick: unified path（by‑name 不可）を既に強制
   - Integration/full: ユーザー関数呼び出し/extern 名正規化のカナリアを追加
   - 旧ENV依存がないことを確認（`NYASH_VM_ALLOW_LEGACY` は削除済）

4) Remove legacy code path
   - Delete `src/backend/mir_interpreter/handlers/calls/legacy.rs` and its resolver module
   - Drop env guard checks referring to legacy path (keep error message clarity)
   - Update READMEs to remove temporary notes

Readiness Criteria (before removal)
- quick/integration/full all green with legacy OFF
- No `callee=None` emission in builder (verified by grepping MIR JSON / code review)
- Extern SSOT accepts arity‑suffixed names; Global extern‑like names delegate properly

Rollback Plan
- Keep a small revertable commit boundary for legacy deletion
- If issues appear, reintroduce legacy.rs with the same path under a dev‑guard until fixed

Cross‑References
- handlers/calls/README.md (SSOT and boundaries)
- docs/ENV_VARS.md (env guards and policies)
