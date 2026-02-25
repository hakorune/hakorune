# Phase 132: LLVM finalize_phis の順序バグ修正

## 🎯 ゴール

LLVM backend における **「PHI 命令の配置順序バグ」を構造的に修正** する。

目的：
- JoinIR→MIR→LLVM の意味論・制御構造は一切変えず、**「PHI はブロック先頭に並ぶ」という LLVM 規約** を満たす
- 修正範囲は LLVM backend の finalize_phis 周辺に限定し、JoinIR/Ring0 には触らない
- 代表ケース 6/7 で LLVM 実行が Rust VM と一致することを確認

```
Phase 131: LLVM backend re-enable（1/7成功、PHI問題発見）✅
        ↓
Phase 132: PHI順序バグ構造的修正 ← ← ここ！
        ↓
Phase 133: 残りのLLVM統合タスク
```

---

## 📋 スコープ（やること・やらないこと）

### ✅ やること
- LLVM の PHI 命令生成・配置ロジック（finalize_phis 相当）を箱として整理
- 常に **「ブロック先頭に PHI が並ぶ」** 順序にする
- 代表ケース（Phase 130/131 で失敗していた 6/7）で LLVM 実行確認
- docs に「PHI 順序の設計と仕様」を明記

### ❌ やらないこと
- 新しい opcode の追加や、JoinIR/MIR の意味論変更
- FileBox/Ring0 や logging まわりには触れない
- LLVM backend 全体の最適化や再設計（PHI 順序に限定）

---

## 🏗️ 6 つのタスク

### Task 1: 設計ドキュメント作成

**ファイル**: `docs/development/current/main/phase132_llvm_phi_ordering.md`（このファイル）

**書く内容**:

#### LLVM の PHI 規則（簡潔版）
- 各 BasicBlock では **「ラベル直後の連続した PHI 群」** が必須
- PHI は同一ブロック内の他の命令より **前** に来なければならない
- return や branch の **後** に PHI を置くのは仕様違反

#### 現状の問題点
Phase 131 で観測した問題：

```llvm
bb5:
  ret i64 %"ret_phi_16"
  %"ret_phi_16" = phi i64 [0, %"bb3"], [0, %"bb4"]  ; ❌ エラー！PHIはretの前に必要
}
```

**原因**: `finalize_phis()` が「既存の末尾命令（terminator）の後ろに PHI を突っ込んでいる」

#### 目指す構造
**PhiPlacement 責務箱** で「1ブロック内の (phi, non-phi) を再配置」する：

1. **分離**: 既存命令列を「仮想 PHI 命令」と「それ以外」に分離
2. **クリア**: ブロックを一旦クリア
3. **再構築**: 先に PHI 群を順に挿入、その後に非 PHI 命令を挿入
4. **terminator 維持**: terminator（ret/br）は必ず非 PHI 群の末尾に維持

#### アルゴリズム方針（2案）

**案A**: 命令順序の後処理（推奨）
- 各 LLVM BasicBlock について、生成後に命令順序を再配置
- PHI 群 → 非 PHI 群 → terminator の順に並べ替え

**案B**: 生成時点での順序保証
- PHI を生成する時点で「必ず insert_at_beginning 的な API」を使う
- 生成タイミングを制御して順序を保証

**Phase 132 では案A を採用**（実装が明確で、既存コードへの影響が小さい）

---

### Task 2: 既存 finalize_phis 実装の棚卸し

**対象ファイル**: `src/llvm_py/llvm_builder.py`（Phase 131 で特定済み）

**やること**:

1. **finalize_phis 関数の詳細確認**:
   ```bash
   rg "def finalize_phis" src/llvm_py/
   ```
   - 約100行の関数
   - どのタイミングで LLVM IR に PHI を挿入しているか確認
   - どのブロックに対して、どの API で PHI を追加しているか記録

2. **PHI 情報の元データ確認**:
   - MIR JSON から PHI 情報を取得している部分を特定
   - Builder の一時バッファ（pending phi list 等）の所在を確認

3. **呼び出し経路の確認**:
   - どのフェーズで finalize_phis が呼ばれているか
   - 複数回呼ばれていないか（2重呼び出しの危険性）

---

### Task 3: PhiPlacement 責務箱の実装

**方針**: finalize_phis の「命令順序入れ替え責務」を専用の関数/クラスに閉じ込める

**実装箇所**: `src/llvm_py/phi_placement.py`（新規）または `llvm_builder.py` 内の関数

**責務**:
- 引数: 単一の LLVM BasicBlock（または block + pending phi list）
- 動作:
  - そのブロック内の命令列を走査し、「PHI 相当」と「それ以外」を分類
  - 新しい命令順序（PHI 群 → 非 PHI 群）に組み直す
- **PHI の内容は一切触らない。順序だけ直す。**

**実装パターン**:

```python
def reorder_phi_instructions(block, phi_nodes):
    """
    LLVM BasicBlock 内の命令順序を PHI-first に並べ替え

    Args:
        block: LLVM BasicBlock
        phi_nodes: このblockに属するPHI命令のリスト

    Returns:
        再配置されたblock
    """
    # 1. 既存命令を分類
    non_phi_instructions = []
    terminator = None

    for instr in block.instructions:
        if is_terminator(instr):
            terminator = instr
        elif not is_phi(instr):
            non_phi_instructions.append(instr)

    # 2. Block をクリア（または新しいblockを作成）
    new_block = create_empty_block(block.name)

    # 3. PHI群を先頭に挿入
    for phi in phi_nodes:
        new_block.append(phi)

    # 4. 非PHI命令を挿入
    for instr in non_phi_instructions:
        new_block.append(instr)

    # 5. Terminator を最後に挿入
    if terminator:
        new_block.append(terminator)

    return new_block
```

**注意点**:
- llvmlite の API に応じて実装を調整
- 既存の finalize_phis から「PHI 生成ロジック」と「配置ロジック」を分離
- 生成ロジックはそのまま、配置ロジックだけを新関数に移す

---

### Task 4: finalize_phis 呼び出し経路の整理

**やること**:

1. **呼び出し位置の確認**:
   ```bash
   rg "finalize_phis" src/llvm_py/
   ```
   - LLVM lowering のどのフェーズで呼ばれているか
   - 関数生成の最後か、ブロック生成の途中か

2. **ルール設定**:
   - 各関数（または各 BasicBlock）で、PHI 生成が完了した後、**必ず PhiPlacement を通す**
   - 2重に呼ばない／途中で呼びかけて順序を壊さないように、呼び出し位置を一箇所にまとめる

3. **位置付け**:
   - JoinIR / MIR 側には一切触れず、**「LLVM 出力直前の最後の整形処理」** として位置付ける

---

### Task 5: テスト追加（LLVM 専用）

**代表ケース**:

Phase 130/131 で「Rust VM OK, LLVM NG」だったケース：
- `local_tests/phase123_simple_if.hako`（シンプルな if）
- `local_tests/phase123_while_loop.hako`（while loop）
- `apps/tests/loop_min_while.hako`（最小ループ）
- `apps/tests/joinir_if_select_simple.hako`（IfSelect）

**テスト戦略**:

1. **LLVM IR 検証**（可能なら）:
   - MIR → LLVM IR 生成時に、各ブロックの PHI が先頭に並んでいるかチェック
   - LLVM IR の text dump をパースして簡易検証

2. **実行結果検証**（必須）:
   ```bash
   # 各テストケースで Rust VM と LLVM の両方を実行
   ./target/release/nyash --backend vm local_tests/phase123_simple_if.hako
   LLVM_SYS_180_PREFIX=$(llvm-config-18 --prefix) \
   NYASH_LLVM_USE_HARNESS=1 \
     ./target/release/nyash --backend llvm local_tests/phase123_simple_if.hako

   # 結果が一致することを確認
   ```

3. **複雑なケース**:
   - loop + if など、PHI 構造が複雑なケース 1 本を確認
   - 例: `apps/tests/joinir_min_loop.hako`

---

### Task 6: ドキュメント & CURRENT_TASK 更新

**やること**:

1. **このファイル（phase132_llvm_phi_ordering.md）の末尾に追記**:
   ```markdown
   ## Phase 132 実装結果

   ### 修正ファイル
   - `src/llvm_py/llvm_builder.py`: finalize_phis() 修正
   - `src/llvm_py/phi_placement.py`: 新規作成（または llvm_builder.py 内の関数）

   ### テスト結果
   | ケース | Rust VM | LLVM (Phase 131) | LLVM (Phase 132) |
   |--------|---------|------------------|------------------|
   | phase123_simple_if.hako | ✅ | ❌ | ✅ |
   | phase123_while_loop.hako | ✅ | ❌ | ✅ |
   | loop_min_while.hako | ✅ | ❌ | ✅ |
   | joinir_if_select_simple.hako | ✅ | ❌ | ✅ |

   ### 成果
   - PHI 順序バグの構造的修正完了
   - LLVM backend の基本整合が取れた
   - 6/7 テストが LLVM で実行成功（残り1つは ConsoleBox 問題）
   ```

2. **CURRENT_TASK.md 更新**:
   ```markdown
   ### Phase 132: LLVM PHI 命令順序バグ修正 ✅

   **完了内容**:
   - finalize_phis() の構造的リファクタリング
   - PHI 命令をブロック先頭に配置する PhiPlacement 実装
   - 代表ケース 6/7 で LLVM 実行成功

   **修正箇所**:
   - src/llvm_py/llvm_builder.py: PHI生成・配置ロジック分離
   - src/llvm_py/phi_placement.py: 新規作成（または関数追加）

   **テスト結果**:
   - 修正前: LLVM 1/7 実行可能
   - 修正後: LLVM 6/7 実行可能（PHI順序問題解決）

   **成果**:
   - JoinIR → LLVM 経路の基本動作確認完了
   - Phase 133 で ConsoleBox 統合を完了すれば 7/7 達成

   **次フェーズ**: Phase 133 - ConsoleBox LLVM 統合 & 残りタスク
   ```

3. **30-Backlog.md 更新**:
   ```markdown
   ### Phase 133: ConsoleBox LLVM 統合 & JoinIR→LLVM 完成

   Phase 132 で PHI 順序問題解決、残りタスク：
   - ConsoleBox の Rust VM 登録確認
   - ConsoleBox の LLVM 統合（println/log 外部関数）
   - 全7テストケースで LLVM 実行成功確認

   完了条件：
   - ✅ 7/7 テストが Rust VM と LLVM で実行成功
   - ✅ JoinIR → LLVM 第3章クローズ
   ```

---

## ✅ 完成チェックリスト（Phase 132）

- [ ] LLVM PHI 規則の設計ドキュメント作成
- [ ] finalize_phis() 実装の詳細確認（約100行）
- [ ] PhiPlacement 責務箱の実装（新関数 or 新ファイル）
- [ ] PHI 命令をブロック先頭に配置するロジック実装
- [ ] finalize_phis 呼び出し経路の整理
- [ ] 代表ケース 4-6 本で LLVM 実行成功確認
- [ ] phase132_llvm_phi_ordering.md に実装結果追記
- [ ] CURRENT_TASK.md & Backlog 更新
- [ ] git commit で記録

---

## 所要時間

**3〜4 時間程度**

- Task 1-2 (設計ドキュメント & 棚卸し): 1時間
- Task 3 (PhiPlacement 実装): 1.5時間
- Task 4-5 (呼び出し整理 & テスト): 1時間
- Task 6 (ドキュメント更新): 30分

---

## 次のステップ

**Phase 133: ConsoleBox LLVM 統合 & JoinIR→LLVM 完成**
- Phase 132 で PHI 問題解決後、残りの ConsoleBox 統合
- 全7テストケースで LLVM 実行成功
- JoinIR → LLVM 第3章クローズ

---

## 進捗

- ✅ Phase 131: LLVM backend re-enable & PHI 問題発見（完了）
- 🎯 Phase 132: LLVM PHI 命令順序バグ修正（← **現在のフェーズ**）
- 📋 Phase 133: ConsoleBox LLVM 統合 & 完成（予定）

---

## Phase 132 実装結果

### 修正ファイル
- `src/llvm_py/phi_wiring/wiring.py`: ensure_phi() 関数の修正
  - PHI instruction を block の絶対先頭に配置する処理を追加
  - terminator が既に存在する場合の警告機能追加
  - `position_before(instrs[0])` を使用して既存命令より前に配置
- `src/llvm_py/phi_wiring/tagging.py`: setup_phi_placeholders() の強化
  - デバッグモード追加（`NYASH_PHI_ORDERING_DEBUG=1`）
  - PHI 生成時の terminator チェック追加
  - エラーハンドリングの改善
- `src/llvm_py/phi_placement.py`: 新規作成（検証ユーティリティ）
  - PHI ordering 検証機能
  - 命令分類機能（PHI / 非PHI / terminator）

### 技術的解決策

**根本原因**: llvmlite では命令を作成後に移動できないため、PHI は必ず最初に作成する必要がある。

**実装アプローチ**:
1. **早期 PHI 生成**: `setup_phi_placeholders` で全 PHI を block lowering 前に生成
2. **配置位置の明示的制御**: `position_before(instrs[0])` で既存命令より前に配置
3. **デバッグ機能**: 環境変数 `NYASH_PHI_ORDERING_DEBUG=1` で詳細ログ出力

**キーポイント**:
- llvmlite は命令の事後移動をサポートしない
- PHI は block が空の状態で作成するのが最も確実
- `finalize_phis` は新規 PHI 作成ではなく、既存 PHI への incoming 配線のみ行う

### デバッグ方法

```bash
# PHI ordering デバッグモード有効化
export NYASH_PHI_ORDERING_DEBUG=1

# LLVM backend でテスト実行
NYASH_LLVM_USE_HARNESS=1 NYASH_LLVM_OBJ_OUT=/tmp/test.o \
  ./target/release/hakorune --backend llvm test.hako
```

### 期待される動作

**修正前**:
```llvm
bb5:
  ret i64 %"ret_phi_16"
  %"ret_phi_16" = phi i64 [0, %"bb3"], [0, %"bb4"]  ; ❌ PHI が terminator の後
```

**修正後**:
```llvm
bb5:
  %"ret_phi_16" = phi i64 [0, %"bb3"], [0, %"bb4"]  ; ✅ PHI が block 先頭
  ret i64 %"ret_phi_16"
```

### 実装完了チェックリスト

- ✅ LLVM PHI 規則の設計ドキュメント作成
- ✅ finalize_phis() 実装の詳細確認（約100行）
- ✅ PhiPlacement 責務箱の実装（phi_placement.py 新規作成）
- ✅ PHI 命令をブロック先頭に配置するロジック実装（ensure_phi 修正）
- ✅ setup_phi_placeholders のデバッグ機能強化
- 📋 代表ケース 4-6 本で LLVM 実行成功確認（テスト実行必要）
- 📋 phase132_llvm_phi_ordering.md に実装結果追記（この項目）
- 📋 CURRENT_TASK.md & Backlog 更新

### テスト戦略

**代表ケース**（Phase 130/131 で失敗していたケース）:
1. `local_tests/phase123_simple_if.hako` - シンプルな if
2. `local_tests/phase123_while_loop.hako` - while loop
3. `apps/tests/loop_min_while.hako` - 最小ループ
4. `apps/tests/joinir_if_select_simple.hako` - IfSelect

**テスト実行**:
```bash
# 自動テストスクリプト
./tools/test_phase132_phi_ordering.sh

# 個別テスト
NYASH_PHI_ORDERING_DEBUG=1 NYASH_LLVM_USE_HARNESS=1 NYASH_LLVM_OBJ_OUT=/tmp/test.o \
  ./target/release/hakorune --backend llvm local_tests/phase123_simple_if.hako
```

### 成果

**構造的修正完了**:
- PHI 生成タイミングの制御強化
- llvmlite API の制約に対応した実装
- デバッグ機能の充実

**設計原則確立**:
- PHI は必ず block lowering 前に生成
- finalize_phis は配線のみ、新規生成はしない
- position_before を使用した明示的配置

**次のステップ**:
- Phase 133: ConsoleBox LLVM 統合で 7/7 テスト完全成功を目指す
Status: Historical
