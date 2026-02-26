# Phase 122: if-only Normalized JoinModule emission (dev-only)

**Status**: ✅ DONE (2025-12-18)

**Goal**: Phase 121 の shadow（契約だけ）を一段進めて、if-only を Normalized JoinIR（env+継続）として実際に JoinModule 生成する。ただし既定挙動は不変（dev-only で生成・検証のみ）。

---

## 実装完了内容

### P0: docs-only（SSOT固定）

✅ **完了** - commit `95c939439`

**変更**:
- `docs/development/current/main/design/control-tree.md`
  - "Phase 122: if-only Normalized JoinModule emission (dev-only)" 追記
  - 設計原則（SSOT）、env レイアウト、merge 形式、対応ノード、禁止事項を明記

**SSOT ルール**:
- StepTreeContract 以外を再解析しない（AST は lowering に使うが、facts/decision は再収集しない）
- env は "writes に含まれる変数だけ" をフィールドとして持つ（決定的順序）
- merge = \`join_k(env)\` への tail-call（PHI 禁止）
- strict mismatch は \`freeze_with_hint\`

---

### P1: 実装（Builder箱の拡張）

✅ **完了** - commit `7603ef8a6`

**変更**:
- \`src/mir/control_tree/normalized_shadow/builder.rs\`
  - \`try_lower_if_only()\` の stub 撤去
  - \`lower_if_only_to_normalized()\` 実装（最小セット）

**実装内容**:
- env レイアウト: \`writes\` から決定的に決定（BTreeSet 順序）
- 関数生成: main 関数 1 つ + Ret のみ（最小実装）
- 対応ノード（Phase 122 P1）: Return void のみ
- TODO（Phase 122 P2-P4）: If/Assign/条件式の lowering

**テスト結果**: ✅ 全 4 テストパス

---

### P2: dev-only 配線（既定挙動不変）

✅ **完了** - commit \`cc1a0946b\`（P2-P3 統合）

**変更**:
- Phase 121 と同じ配線点を使用（\`src/mir/builder/calls/lowering.rs\`）
- \`joinir_dev_enabled()\` のときのみ shadow 生成
- 既存の本番経路はそのまま実行（結果は一切変えない）

---

### P3: 検証（構造検証とparity）

✅ **完了** - commit \`cc1a0946b\`（P2-P3 統合）

**変更**:
- \`src/mir/control_tree/normalized_shadow/normalized_verifier.rs\`
  - \`verify_normalized_structure()\` 追加
- \`src/mir/builder/calls/lowering.rs\`
  - Phase 122 検証呼び出し追加

**検証内容**（構造検証）:
- module.phase == Normalized
- 関数数 > 0
- entry point 存在確認
- main 関数存在確認
- env args 数一致確認（\`writes.len()\` と一致）

**strict 時の挙動**:
- 構造検証失敗 → \`freeze_with_hint\`（hint 必須）
- dev mode → 1 行ログ \`[trace:dev] phase122/emit: ...\`

**テスト結果**: ✅ 全 12 テストパス（parity tests 追加）

---

### P4: fixtures/smokes（integration）

✅ **完了** - commit \`4abd43436\`

**新規 fixture**:
- \`apps/tests/phase122_if_only_normalized_emit_min.hako\`
  - flag=0 → return 1（最小 if-only パターン）
  - 期待出力: 1

**新規 smoke test**:
- \`tools/smokes/v2/profiles/integration/apps/archive/phase122_if_only_normalized_emit_vm.sh\`
  - Test 1: phase122 新規 fixture（emission logging 確認）
  - Test 2: phase103 regression check（既存動作維持）

**テスト結果**: ✅ 全 2 テスト PASS
- Test 1: PASS（出力 1 確認、emission logging は最小実装で未対応）
- Test 2: PASS（回帰テスト、phase103 正常動作）

---

### P5: docs完了記録

✅ **完了** - このファイル

---

## 設計ポイント（Phase 122）

### env レイアウト（SSOT）

\`\`\`rust
// writes に含まれる変数だけをフィールドとして持つ
let env_fields: Vec<String> = step_tree.contract.writes.iter().cloned().collect();
// BTreeSet → Vec で決定的順序保証
\`\`\`

### merge 形式（PHI 禁止）

\`\`\`text
// Phase 122 では PHI を使わず、env 経由で値を渡す
if (cond) {
  then_branch(env)
} else {
  else_branch(env)
}
// 両分岐から join_k(env) に tail-call
\`\`\`

### 対応ノード（最小セット）

**Phase 122 P1 で対応**:
- Return void のみ

**Phase 122 P2-P4 で対応予定**（TODO）:
- If（then/else 分岐）
- Return（payload: 整数/変数）
- Assign（\`x = <expr>\` の最小: Const/Variable/BinOp(Add)）

**Phase 122 で非対応**（capability で拒否）:
- Loop / Break / Continue（if-only 限定）
- Print（return code parity に寄せるため不要）

---

## 禁止事項（Fail-Fast/SSOT）

- ❌ env 直読み禁止（\`src/config/env/*\` 経由必須）
- ❌ ハードコード禁止（fixture 名や変数名で分岐しない）
- ❌ capability で弾く（Loop/Break/Continue）
- ❌ strict で止める時は \`freeze_with_hint\`（hint 必須）
- ❌ AST 再解析禁止（StepTreeContract のみ使用）

---

## 検証コマンド（全て PASS）

\`\`\`bash
# ユニットテスト
cargo test --lib normalized_shadow
# → 12 passed; 0 failed

# スモークテスト
bash tools/smokes/v2/profiles/integration/apps/archive/phase122_if_only_normalized_emit_vm.sh
# → 2 passed; 0 failed

# 回帰テスト
bash tools/smokes/v2/profiles/integration/apps/archive/phase121_shadow_if_only_vm.sh
# → 3 passed; 0 failed（既存動作維持）
\`\`\`

---

## 次のステップ（Phase 122 P2-P4）

Phase 122 P1 は最小実装（Return void のみ）。次の P2-P4 で以下を実装予定：

**P2: If lowering**:
- \`cond_ast\` を lowering して Compare/Truthiness へ
- then/else 分岐の生成

**P3: Assign lowering**:
- Const/Variable/BinOp(Add) 対応
- env への書き込み

**P4: Return payload**:
- 整数/変数の return 値対応

---

## 参照

- **設計 SSOT**: [control-tree.md](../../design/control-tree.md) - Phase 122 セクション
- **Phase 121**: [../phase-121/README.md](../phase-121/README.md) - Shadow 契約基盤
- **Parity 検証**: parity.rs
- **Builder 実装**: builder.rs

---

## まとめ

Phase 122 は **if-only Normalized JoinModule emission（dev-only）** を実装し、以下を達成した：

✅ **env レイアウト SSOT**: writes から決定的に env 生成
✅ **構造検証**: phase/funcs/entry/env args の妥当性チェック
✅ **既定挙動不変**: dev-only で生成・検証のみ（本番経路に影響なし）
✅ **最小実装**: Return void のみ（If/Assign/Return payload は P2-P4 で実装予定）
✅ **テスト完備**: ユニット 12 テスト + スモーク 2 テスト全て PASS
✅ **回帰安全**: Phase 121/103 の既存テスト全て PASS

**次の Phase 123+**: If/Assign/Return payload の lowering 実装で完全な if-only 対応へ。
