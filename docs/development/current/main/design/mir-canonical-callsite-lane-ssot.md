---
Status: SSOT
Scope: MirInstruction の call-site 正規化（BoxShape lane, no behavior expansion）
Decision: accepted (phase29y-safe lane)
Updated: 2026-02-12 (RCL-3 synced)
Related:
- docs/reference/mir/INSTRUCTION_SET.md
- docs/development/current/main/design/mir-instruction-diet-ledger-ssot.md
- src/mir/instruction.rs
- src/mir/definitions/call_unified.rs
- src/mir/contracts/backend_core_ops.rs
---

# MIR Canonical Callsite Lane (SSOT)

## 目的

MIR 命令数そのものを急いで減らすのではなく、まず「call-site 表現の入口」を 1 本化して診断距離を短くする。

- ゴール: backend 手前で call 系表現を canonical へ寄せる
- 方針: BoxShape（責務整理）を先行、BoxCount（受理拡張）はしない
- 契約: fail-fast。曖昧 fallback を増やさない

## 非目標

- 新しい受理形の追加
- AST rewrite
- NewClosure retire（NCL-2 で `Call(callee=Closure...)` の shape 契約は固定済み。`NewClosure` 自体の retire はこの lane の非目標）
- NewBox の即時統合（`Call` の legacy `func` 必須を先に整理する必要がある）

## 実装上の前提（現状）

- `MirInstruction::Call` は `func: ValueId` + `callee: Option<Callee>` の過渡形
- `used_values()` は `callee=None` でのみ `func` を使用し、`callee=Some(Method{receiver})` は receiver を使用
- `ValueId::INVALID` が sentinel として利用可能

参照:
- `src/mir/instruction.rs`
- `src/mir/instruction/methods.rs`
- `src/mir/value_id.rs`

## 最終像（この lane で到達する形）

- backend 入口では call-site を `MirInstruction::Call` で観測できる
- `BoxCall` / `ExternCall` は canonicalization 後に backend へ流れない
- `Call { callee: None }` は backend 入口で freeze（ただし `func=<const-string>` 形は MCL-5 で `Call(callee=Global)` へ正規化）
- docs SSOT と実装 ledger は常に同期（既存テストを維持）

注:
- DebugLog→Debug 統合と Nop retire は別 lane に分離する（本 lane では扱わない）



## 実行手順（1タスク=1コミット）

### MCL-0: Canonicalization pass 入口を作る（挙動不変）

- 追加:
  - `src/mir/passes/callsite_canonicalize.rs`（新規）
  - `src/mir/passes/mod.rs` へ module 追加
- ルール:
  - pass は MIR module を走査し、命令差し替えのみを担当
  - backend 判定・reject はこのコミットで入れない（次コミットに分離）
- 受け入れ:
  - ビルド緑
  - 既存 test 緑

### MCL-1: `BoxCall -> Call(callee=Method)` 変換を追加

- 変換契約:
  - `BoxCall { dst, box_val, method, args, effects }`
  - `=> Call { dst, func: ValueId::INVALID, callee: Some(Callee::Method{ box_name, method, receiver: Some(box_val), certainty, box_kind }), args, effects }`
- 注意:
  - `box_name/certainty/box_kind` が不明な場合は conservative 値を使う（runtime data + union など）
  - 変換不能なら即 freeze（silent keep しない）
- 受け入れ:
  - `used_values()` が receiver + args を維持
  - parity break を起こさない

### MCL-2: `ExternCall -> Call(callee=Extern)` 変換を追加

- 変換契約:
  - `ExternCall { dst, iface_name, method_name, args, effects }`
  - `=> Call { dst, func: ValueId::INVALID, callee: Some(Callee::Extern("<iface>.<method>")), args, effects }`
- 受け入れ:
  - extern route の結果/副作用が既存と一致
  - legacy `ExternCall` が backend 入口まで残存しない

### MCL-3: backend 入口の fail-fast 契約を固定

- 追加契約:
  - backend 入口で `Call { callee: None }` を reject（freeze）
  - backend 入口で `BoxCall` / `ExternCall` 残存を reject（freeze）
- 受け入れ:
  - reject tag が安定
  - 既存 green ケースは維持

### MCL-4: docs / tests 同期

### MCL-5: `Call(callee=None, func=<const-string>)` を `Call(callee=Global)` へ正規化

- 変換契約:
  - `Call { callee: None, func: <const-string value-id>, args, ... }`
  - `=> Call { callee: Some(Callee::Global(<string>)), func: ValueId::INVALID, args, ... }`
- 目的:
  - Program(JSON v0) runtime route で `call-missing-callee` freeze を回避し、MCL lane の backend 契約へ合流させる。
- 受け入れ:
  - `mcl5_rewrites_legacy_call_with_const_string_func_to_global_callee` が green。


- 更新:
  - `docs/reference/mir/INSTRUCTION_SET.md`（必要なら運用注記のみ）
  - `docs/development/current/main/design/mir-instruction-diet-ledger-ssot.md`（cohort 説明）
- テスト:
  - `instruction_diet_ledger_counts_match_docs_ssot` を維持
  - canonicalization 後に legacy call-site 命令が 0 であることを確認する unit test を追加

## post-canonical retire queue（次レーン）

詳細SSOT: `docs/development/current/main/design/mir-callsite-retire-lane-ssot.md`

- 固定順序:
  - 1) Rust 側で legacy call-site を canonical call に吸収（MCL-0..5 完了）
  - 2) `.hako` mirbuilder の新規出力を canonical call-site へ移行
  - 3) `BoxCall/ExternCall` を enum から retire
- キュー:
  - RCL-0 (docs-only): done（`mir-callsite-retire-lane-ssot.md` で契約固定）
  - RCL-1 (BoxCount): done（`.hako` mirbuilder の emit を `Call(callee=Method/Extern)` へ統一）
  - RCL-2 (BoxShape): done（strict/dev の stage1 selfhost MIR 受け口で legacy emit を fail-fast reject）
  - RCL-3 (BoxShape): done（min1/min2/min3 完了。`BoxCall/ExternCall` enum retire 済み）
  - RDN-0 (separate lane): `DebugLog/Nop` retire は callsite lane と分離

## NewClosure clean path（2段階 + 契約固定）

- NCL-0: done（`Call(callee=Closure)` は canonicalization pass で `NewClosure` へ正規化し、backend 境界では `call-closure-not-canonical` を fail-fast）
- NCL-1: done（`NewClosure.body` は `body_id -> module.metadata.closure_bodies` へ外出しし、canonical 形は `body=[]` を維持）
- NCL-2: done（shape 判定を SSOT 化。`dst=Some + args=[]` のみ canonicalize、それ以外は shape-specific fail-fast）

## 作業分離ルール

- BoxShape lane と BoxCount lane を混ぜない
- DebugLog/Nop/NewBox/NewClosure はこの lane で触らない
- 1コミットに 1変換（または 1契約）だけを入れる
- fast gate FAIL 状態で新しい fixture/case を増やさない

## 受け入れコマンド（最小）

```bash
cargo test instruction_diet_ledger_counts_match_docs_ssot -- --nocapture
cargo test mir14_shape_is_fixed --test mir_instruction_set_sync -- --nocapture
cargo check --bin hakorune
```

必要に応じて:

```bash
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq
```

## 作業役へのハンドオフ文（そのまま貼れる版）

「MIR call-site canonicalization lane を MCL-0 から順に実施してください。  
各コミットは 1タスクのみ。BoxShape 専用で、受理拡張（BoxCount）は禁止。  
`BoxCall/ExternCall -> Call(callee=...)` を backend 手前で正規化し、backend 入口で legacy 残存を fail-fast 化してください。  
NewBox/NewClosure/DebugLog/Nop の整理はこの lane の対象外です。」
