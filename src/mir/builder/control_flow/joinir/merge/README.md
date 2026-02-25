# JoinIR Merge Coordinator（箱の地図）

責務: JoinIR 生成物をホスト MIR に統合するフェーズ（block/ValueId remap → rewrite → PHI/ExitLine 再接続）。

主な箱と入口:
- `instruction_rewriter.rs` — JoinIR→MIR 命令を書き換えつつブロックを組み立てるメイン導線
- `phi_block_remapper.rs` — PHI の block-id だけを再マップする専用箱（ValueId は remapper 済みを前提）
- `loop_header_phi_builder.rs` / `exit_phi_builder.rs` — ヘッダ/出口 PHI の組み立て
- `inline_boundary_injector.rs` — Host↔JoinIR の ValueId 接続（Boundary 注入）
- `value_collector.rs` / `block_allocator.rs` — 再マップ前の収集と ID 割り当て
- `tail_call_classifier.rs` — tail call 判定（loop_step の末尾呼び出し検出）

Fail-Fast の基本:
- block/ValueId 衝突や PHI 契約違反は握りつぶさず `Err` で止める
- remap は「ValueId remap」→「block-id remap」の順で一貫させる（PHI 二度 remap は禁止）

拡張時のチェックリスト:
1. 新しい JoinInst → MIR 変換を追加する場合、`instruction_rewriter` に閉じて追加し、PHI/ExitLine 契約が壊れないか確認。
2. PHI の入力ブロックを触るときは `phi_block_remapper` 経由に寄せて二重 remap を防ぐ。
3. 増やした box/契約はここ（README）に一言追記して入口を明示。

---

## JoinIR Merge Contracts (SSOT)

### Phase 132-R0: Continuation Contract

#### Continuation Functions

- **Source**: continuation funcs は `JoinInlineBoundary.continuation_func_ids` から来る
- **Responsibility**: router/lowerer が責務（merge は推測しない）
- **Forbidden**: merge は by-name/by-id で continuation 判定しない

#### Skip Conditions

Merge は構造条件のみで continuation のスキップを決定する：

- **Structural only**: 構造条件のみで決定
  - 1 block
  - instruction なし
  - Return のみ
- **Skippable continuation**: 上記条件を満たす continuation
- **Non-skippable continuation**: TailCall(post_k) など他関数への呼び出しを含む

**判定関数**: `is_skippable_continuation(func: &MirFunction) -> bool`

#### Input Contracts

- `JoinInlineBoundary.continuation_func_ids`: Set<JoinFuncId>
- Merge は受け取った ID のみを continuation として扱う
- SSOT: `JoinInlineBoundary::default_continuations()` を使用

#### Prohibited Behaviors

- ❌ By-name classification (例: "join_func_2" という名前で判定)
- ❌ By-id heuristics (例: id == 2 だから continuation)
- ❌ Implicit inference (continuation 候補を merge が推測)

#### Verification

Continuation 契約のテストは `tests/continuation_contract.rs` に配置する。

#### Design Rationale

**なぜ merge は推測してはいけないのか？**

1. **責任分離**: Router/lowerer が JoinIR 構造を知っている。Merge は受け取った指示に従う。
2. **拡張性**: 将来的に複数の continuation パターン（k_exit, k_continue など）が増える可能性がある。
3. **デバッグ性**: Continuation 判定ロジックが router/lowerer に集約されているため、トラブル時の追跡が容易。
4. **Fail-Fast**: Merge が勝手に推測して間違った挙動をするより、明示的な契約違反でエラーを出す方が安全。

**構造判定は許可される理由**：

- 1-block + empty instructions + Return は「何もしない関数」の普遍的な構造的特徴。
- この判定に名前や ID は不要（構造のみで決定可能）。
- スキップは最適化であり、スキップしなくても正しさは保たれる。

---

### Phase 29ae: Header PHI Entry/Latch Contract

- **Entry preds**: `entry_incoming` に含まれるブロック + host entry block（bb0）だけを entry として扱う。
- **Latch preds**: header の predecessor から entry preds を引いた残り。
- **PHI inputs**:
  - entry preds → `entry_incoming` の値
  - latch preds → `latch_incoming` の値
- **Forbidden**: by-name/by-id で entry/latch を推測しない。
- **Fail-Fast**: entry preds が空、または latch が未設定なら `Err`。
