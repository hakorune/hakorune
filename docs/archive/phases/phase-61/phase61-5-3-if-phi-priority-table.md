# Phase 61-5.3: If PHI 関数優先度表

## 目的
Phase 61-5.2 で作成した If PHI 関数表に削減優先度（P1/P2/P3）を追加する。

## 優先度の定義

### P1（最優先削減候補）
- すでに JoinIR でほぼ構造が決まっていて、削るだけの関数（薄いラッパー）
- 条件：
  - `joinir_coverage=in_loop` or `toplevel_return`（完全カバー済み）
  - 既存経路を削除するだけで JoinIR に完全移行可能
  - 実装コストが低い（数十行レベル）

### P2（次期削減候補）
- JoinIR パターンを少し拡張すれば寄せられそうな関数
- 条件：
  - `joinir_coverage=partial`（部分カバー）
  - IfMerge や PhiSpec を拡張すれば統合可能
  - 実装コストが中程度（数百行レベル）

### P3（将来の削減候補）
- 型情報やより強い If パターン（nested/複雑）を待つべき関数
- 条件：
  - `joinir_coverage=none`（未カバー）
  - 型システムや高度な解析が必要
  - 実装コストが高い（Phase 63+ で対応）

---

## If PHI 関数優先度表（Phase 61-5.3）

| 関数名 | ファイル | JoinIR Coverage | Priority | 削減方針（1行） |
|--------|----------|-----------------|----------|----------------|
| **PhiBuilderBox系** | | | | |
| `set_if_context` | phi_builder_box.rs | in_loop | **P1** | IfPhiContext 生成を直接呼び出しに置き換え、ラッパー削除 |
| `generate_if_phis` | phi_builder_box.rs | partial | **P2** | IfInLoopPhiEmitter で代替、統一後に薄い箱として残すかインライン化 |
| `compute_modified_names_if` | phi_builder_box.rs | partial | **P2** | JoinIR の modified 変数集合解析に統合、ヘルパー削除 |
| `get_conservative_if_values` | phi_builder_box.rs | partial | **P2** | JoinIR の incoming 値解決に統合、void fallback を PhiSpec に移行 |
| **JoinIR Lowering系** | | | | |
| `try_lower_if_to_joinir` | if_select.rs / if_merge.rs | in_loop | **P1** | If パターンマッチング完了後、デフォルト有効化して薄いルーティング関数に |
| `find_if_pattern` | if_select.rs | in_loop | **P1** | Simple/Local パターン検出は完成、ラッパー層削減可能 |
| `find_if_merge_pattern` | if_merge.rs | in_loop | **P1** | IfMerge 検出は完成、ラッパー層削減可能 |
| **JoinIR Context系** | | | | |
| `IfPhiContext::for_loop_body` | if_phi_context.rs | in_loop | **P1** | コンストラクタは軽量、loop_builder.rs から直接呼び出し継続 |
| `IfPhiContext::pure_if` | if_phi_context.rs | none | **P3** | Pure If 経路は Phase 63+ で統合、現状維持 |
| `is_carrier` | if_phi_context.rs | in_loop | **P1** | carrier 判定ロジックは完成、ヘルパーとして残す |
| **PhiSpec系** | | | | |
| `compute_phi_spec_from_joinir` | if_phi_spec.rs | in_loop | **P1** | JoinInst から PhiSpec を計算、Phase 61-3 で variable_name 逆引き完成後そのまま使用 |
| `extract_phi_spec_from_builder` | if_phi_spec.rs | partial | **P2** | PhiBuilderBox 経路観察用、JoinIR 移行完了後は削除候補 |
| `compare_and_log_phi_specs` | if_phi_spec.rs | partial | **P2** | A/B テスト用ロギング、移行完了後は削除候補 |
| **If-in-Loop Emitter系** | | | | |
| `IfInLoopPhiEmitter::emit` | if_in_loop/mod.rs | in_loop | **P1** | PHI 生成の SSOT、既に動作済み、デフォルト有効化で完成 |
| `emit_single_var_phi` | if_in_loop/lowering/single_var_*.rs | in_loop | **P1** | SingleVarThen/Both パターンは実装完了、薄いヘルパーとして残す |
| `emit_conditional_effect_phi` | if_in_loop/lowering/conditional_effect.rs | in_loop | **P1** | ConditionalEffect パターンは実装完了、薄いヘルパーとして残す |
| **Conservative系** | | | | |
| `ConservativeMerge::analyze` | conservative.rs | none | **P3** | 型推論（infer_type_from_phi）依存、Phase 63+ で型システム統合 |
| `infer_type_from_phi` | conservative.rs | none | **P3** | レガシー型推論箱、JoinIR 型情報導入後の削減候補（Phase 63+） |
| **IfMerge系** | | | | |
| `lower_if_to_if_merge` | if_merge.rs | in_loop | **P1** | IfMerge 変換ロジックは完成、デフォルト有効化で完成 |
| `extract_written_vars` | if_merge.rs | in_loop | **P1** | 変数書き込み検出は完成、薄いヘルパーとして残す |
| `find_written_value` | if_merge.rs | in_loop | **P1** | 値検出は完成、薄いヘルパーとして残す |
| **IfSelect系** | | | | |
| `lower_if_to_select` | if_select.rs | in_loop | **P1** | Select 変換ロジックは完成、デフォルト有効化で完成 |
| `try_match_simple_pattern` | if_select.rs | in_loop | **P1** | Simple パターンは完成、薄いヘルパーとして残す |
| `try_match_local_pattern` | if_select.rs | in_loop | **P1** | Local パターンは完成、薄いヘルパーとして残す |
| `is_side_effect_free` | if_select.rs | in_loop | **P1** | 副作用なし判定は完成、薄いヘルパーとして残す |

---

## 優先度別サマリー

### P1（最優先削減候補）: 18関数
- **JoinIR Lowering系**: `try_lower_if_to_joinir`, `find_if_pattern`, `find_if_merge_pattern`, `lower_if_to_if_merge`, `lower_if_to_select`
- **PhiBuilderBox系**: `set_if_context`（薄いラッパー削除候補）
- **Context系**: `IfPhiContext::for_loop_body`, `is_carrier`
- **PhiSpec系**: `compute_phi_spec_from_joinir`
- **Emitter系**: `IfInLoopPhiEmitter::emit`, `emit_single_var_phi`, `emit_conditional_effect_phi`
- **IfMerge/IfSelect ヘルパー**: `extract_written_vars`, `find_written_value`, `try_match_simple_pattern`, `try_match_local_pattern`, `is_side_effect_free`

### P2（次期削減候補）: 5関数
- **PhiBuilderBox系**: `generate_if_phis`, `compute_modified_names_if`, `get_conservative_if_values`
- **PhiSpec系**: `extract_phi_spec_from_builder`, `compare_and_log_phi_specs`

### P3（将来の削減候補）: 3関数
- **Conservative系**: `ConservativeMerge::analyze`, `infer_type_from_phi`
- **Context系**: `IfPhiContext::pure_if`

---

## Phase 61-6 実装推奨順序

### Step 1: P1 薄いラッパー削除（~50行削減）
1. `set_if_context`: loop_builder.rs から直接 `IfPhiContext::for_loop_body` 呼び出しに置き換え
2. If Lowering ルーティング関数のデフォルト有効化（dev フラグ削除）

### Step 2: P2 統合（~200行削減）
1. `compute_modified_names_if`: JoinIR の modified 変数集合解析に統合
2. `get_conservative_if_values`: incoming 値解決を PhiSpec に移行
3. `extract_phi_spec_from_builder`, `compare_and_log_phi_specs`: A/B テスト完了後削除

### Step 3: P3 延期（Phase 63+）
1. `ConservativeMerge::analyze`: 型推論システム統合待ち
2. `infer_type_from_phi`: JoinIR 型情報導入待ち
3. `IfPhiContext::pure_if`: Pure If 経路統合待ち

---

## 期待される削減効果

| Phase | 削減対象 | 見込み削減行数 | 累積削減 |
|-------|---------|--------------|---------|
| 61-6.1 | P1 ラッパー削除 | ~50 | 50 |
| 61-6.2 | P2 統合 | ~200 | 250 |
| 63+ | P3 型システム統合 | ~150 | 400 |

---

## 実装時の注意事項

1. **Fail-Fast原則**: フォールバックなし、エラーは即座に失敗
2. **決定的順序**: BTreeSet/BTreeMap で ValueId 割り当ての決定性保証
3. **箱理論**: 境界を守りながら、SSOT を JoinIR 側に寄せる
4. **テスト**: 各ステップで全 JoinIR tests PASS を確認
5. **ドキュメント**: PHI_BOX_INVENTORY.md と CURRENT_TASK.md を更新

---

## 関連ドキュメント

- Phase 61-1: [if_phi_context.rs](../../../src/mir/join_ir/lowering/if_phi_context.rs)
- Phase 61-2: [if_phi_spec.rs](../../../src/mir/join_ir/lowering/if_phi_spec.rs)
- Phase 61-3: [if_in_loop/mod.rs](../../../src/mir/join_ir/frontend/ast_lowerer/if_in_loop/mod.rs)
- PHI Inventory: [PHI_BOX_INVENTORY.md](../../../docs/private/roadmap2/phases/phase-30-final-joinir-world/PHI_BOX_INVENTORY.md)
Status: Historical
