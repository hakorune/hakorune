Phase 21.7 — Normalization & Unification (Methodize Static Boxes)

Goal
- Unify user-defined function calls onto a single, consistent representation.
- Move from ad-hoc Global("Box.method") calls toward Method calls with an explicit (singleton) receiver when appropriate.
- Keep defaults stable; introduce dev toggles and canaries; then promote to default after green.

Scope
- Parser/Stage‑B: keep emitting Program(JSON v0). No schema changes required for MVP.
- MirBuilder: add methodization (dev toggle) that:
  - Emits method functions as-is (defs) and/or provides a mapping to Method calls
  - Rewrites Global("Box.method") to Method {receiver=static singleton, box_name, method}
  - Preserves arity and naming as Box.method/N
- VM: handle Method calls uniformly (receiver resolved via ensure_static_box_instance for static boxes).
- LLVM: rely on mir_call(Method) lowering (already supported) and provide the static receiver where needed.

Toggles
- HAKO_MIR_BUILDER_METHODIZE=1 — enable methodization (rewrite Global→Method with static singleton receiver)
- HAKO_STAGEB_FUNC_SCAN=1 — Stage‑B defs scan (already available)
- HAKO_MIR_BUILDER_FUNCS=1 — defs→MIR 関数化（既存）
- HAKO_MIR_BUILDER_CALL_RESOLVE=1 — Global 名解決（既存）

Design Rules
- Naming: canonical "Box.method/N". Arity N must equal params.len (or callsite args len if defs unknown).
- Receiver: for static boxes, provide implicit singleton receiver. For instance boxes, preserve existing Method path.
- Effects: preserve EffectMask semantics; do not broaden side effects.
- Fail‑Fast: when rewrite is ambiguous (multiple candidates), keep original form and WARN under dev toggle; never change defaults silently.

Acceptance
- Canaries:
  - call (global style) passes when methodization ON (rc unchanged)
  - Existing return/binop/loop/call (global) canaries remain green when OFF
  - A dedicated methodization canary asserts presence of callee.type=="Method" in MIR(JSON v1) or correct v0 lowering

Rollout Plan
1) Ship behind HAKO_MIR_BUILDER_METHODIZE=1 with canaries
2) Validate on representative apps (selfhost‑first path)
3) Promote to default ON only after green for a cycle; keep rollback instructions

Rollback
- Disable HAKO_MIR_BUILDER_METHODIZE. Revert to Global("Box.method") resolution path (current 21.6 behavior).

## Phase 21.7++ 実装完了 (2025-11-22)

### Global 名の SSOT ルール

#### 原則
- Global 関数名は **`Box.method/N`** が SSOT
- VM/LLVM で `Box.method` を受け取ったら、arity は `args.len()` から補完
- すべての名前解決は `NamingBox::StaticMethodId` 経由

#### 実装箇所

**NamingBox**: `src/mir/naming.rs`
- `StaticMethodId::parse()`: 名前のパース（"Box.method/N" or "Box.method"）
- `StaticMethodId::format()`: 正規化された名前生成
- `StaticMethodId::with_arity()`: arity 補完
- 13 テストケースで検証済み（src/tests/namingbox_static_method_id.rs）

**VM**: `src/backend/mir_interpreter/handlers/calls/global.rs`
- `StaticMethodId` で名前解決
- arity 無し → `args.len()` で補完
- "Did you mean?" エラーメッセージ実装

**UnifiedCallEmitter**: `src/mir/builder/calls/unified_emitter.rs`
- Methodization で `StaticMethodId` 使用
- TypeRegistry と連携して static box method 判定
- 素手 split 根絶

**Rewrite Known**: `src/mir/builder/rewrite/known.rs`
- split_once → StaticMethodId::parse() に統一

#### デバッグ環境変数
- `NYASH_DEBUG_FUNCTION_LOOKUP=1`: VM 関数ルックアップ詳細（box/method/arity 表示）
- `NYASH_DEBUG_USING=1`: using 解決詳細
- `NYASH_METHODIZE_TRACE=1`: Global→Method 変換ログ

#### 実装フェーズ（全完了）
- ✅ **Phase 0: 観測ライン** (commit 63012932) - Silent Failure 根絶
- ✅ **Phase 1: 基盤整備** (commit 96c1345e) - StaticMethodId SSOT 確立
- ✅ **Phase 2: VM 統一** (commit 1b413da5) - arity バグ根治
- ✅ **Phase 3: 全体統一** (commit c8ad1dae) - Builder 側統一、素手 split 根絶

#### 技術的成果
- Silent Failure 根絶（デバッグ時間: 時間→分）
- arity バグ根治（Hotfix 卒業）
- 素手 split 根絶（全箇所を SSOT 経由に統一）
- 型安全化（構造化表現で誤用防止）
- テスト完全通過（349 passed, 退行なし）

---

## 旧ノート（Phase 25.x bring-up 時点）

### NamingBox / static 名 SSOT
- `src/mir/naming.rs` に NamingBox を実装済み。`Main.main` / `main._nop` などの揺れを `"Main.main/0"` 形式に正規化する経路は Rust VM/LLVM/JSON bridge から既に利用中。
- VM 側は `normalize_static_global_name` を通して static box 名を一元化するよう更新済み。
- **Phase 21.7++ で完全 SSOT 化完了**（2025-11-22）

### 既知のギャップ（解決済み）
- ~~static box 内のローカル呼び出し（例: `Main.main` → `me._nop()`）が Global 呼び出しのまま落ちるケースを確認済み。~~ → Phase 2 で解決
- ~~MeCallPolicy / method_call_handlers 周りで、static box method に対しても一律で receiver（`me`）を先頭に追加してしまうパスがあり、arity 不一致や miss-call の温床になり得る。~~ → Phase 3 で解決
