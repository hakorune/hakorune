# Phase 33-13: trim Pattern Observation

## 33-13-1: 代表ケースの再観測結果

### テストケース

```hako
// local_tests/test_trim_simple_pattern.hako
method trim_string_simple(s) {
    if s == null { return "" }

    local start = 0
    local end = s.length()

    // Loop 1: Trim leading spaces (Pattern2: loop with break)
    loop(start < end) {
        local ch = s.substring(start, start + 1)
        if ch == " " {
            start = start + 1
        } else {
            break
        }
    }

    // Loop 2: Trim trailing spaces (Pattern2: loop with break)
    loop(end > start) {
        local ch = s.substring(end - 1, end)
        if ch == " " {
            end = end - 1
        } else {
            break
        }
    }

    return s.substring(start, end)
}
```

### 観測結果

#### ルーティング
- ✅ `Main.trim_string_simple/1` がホワイトリストに追加済み
- ✅ Pattern2_WithBreak として正しく検出

#### Loop 1 (start キャリア)
```
[trace:varmap] pattern2_start: ... end→r22, ... start→r19
[cf_loop/pattern2] Phase 171-fix: ConditionEnv contains 2 variables:
  Loop param 'start' → JoinIR ValueId(0)
  1 condition-only bindings:
    'end': HOST ValueId(22) → JoinIR ValueId(1)
[joinir/pattern2] Phase 172-3: ExitMeta { start → ValueId(9) }
```

- キャリア: `start`
- ExitMeta: `start → ValueId(9)`
- 条件バインディング: `end` (read-only)

#### Loop 2 (end キャリア)
```
[trace:varmap] pattern2_start: ... end→r22, ... start→r32
[cf_loop/pattern2] Phase 171-fix: ConditionEnv contains 2 variables:
  Loop param 'end' → JoinIR ValueId(0)
  1 condition-only bindings:
    'start': HOST ValueId(32) → JoinIR ValueId(1)
[joinir/pattern2] Phase 172-3: ExitMeta { end → ValueId(9) }
```

- キャリア: `end`
- ExitMeta: `end → ValueId(9)`
- 条件バインディング: `start` (should use Loop 1's exit value!)

### 問題発見

**SSA-undef エラー**:
```
[ssa-undef-debug] fn=Main.trim_string_simple/1 bb=BasicBlockId(16)
  used=ValueId(32)
  inst=Copy { dst: ValueId(37), src: ValueId(32) }
```

**エラーメッセージ**:
```
[ERROR] ❌ [rust-vm] VM error: Invalid value:
  use of undefined value ValueId(32)
  (fn=Main.trim_string_simple/1, last_block=Some(BasicBlockId(16)),
   last_inst=Some(Copy { dst: ValueId(37), src: ValueId(32) }))
```

### 根本原因分析

**問題**: 連続ループの SSA 接続

1. **ループ1終了**: `start` の exit value が variable_map に更新される
2. **ループ2開始**: `start` を condition_binding として読む
   - **BUG**: `HOST ValueId(32)` を参照（ループ1の古い start）
   - **期待**: `variable_map["start"]` の更新後の値を参照すべき

### 仮説

**ExitLineReconnector** が `variable_map` を更新しているはずだが、
Pattern2 lowerer が `condition_bindings` を作成する時点で、
その更新後の値を参照していない。

```rust
// Pattern2 lowerer の問題箇所
let host_id = self.variable_map.get(var_name)
    .copied()
    .ok_or_else(|| ...)?;
```

この `variable_map.get()` 呼び出し時点で、
前のループの ExitLineReconnector がまだ実行されていない可能性がある。

### 解決方向性

**Option A**: ExitLineReconnector の即時実行保証
- merge_joinir_mir_blocks() 完了後すぐに variable_map が更新されていることを保証

**Option B**: condition_bindings の遅延解決
- condition_bindings を作成する時点ではなく、
  JoinIR merge 時に variable_map から最新値を取得

**Option C**: PHI 接続の修正
- ループ2 の PHI 入力が ループ1 の exit block から正しく接続されていることを確認

### 次のステップ (33-13-2)

1. ExitLineReconnector の呼び出しタイミングを確認
2. variable_map の更新フローを追跡
3. 連続ループの SSA 接続を設計

---

## 33-13-2: LoopExitContract 設計

### 現在の ExitMeta 構造

```rust
pub struct ExitMeta {
    pub exit_values: BTreeMap<String, ValueId>,
}
```

### trim の理想的な ExitMeta

**ループ1**:
```
ExitMeta {
    exit_values: {
        "start": ValueId(X)  // ループ出口での start の値
    }
}
```

**ループ2**:
```
ExitMeta {
    exit_values: {
        "end": ValueId(Y)  // ループ出口での end の値
    }
}
```

### 問題: 連続ループの variable_map 更新

```
初期状態:
  variable_map["start"] = r19
  variable_map["end"] = r22

ループ1 JoinIR merge 後:
  variable_map["start"] = r35 (remapped exit value)
  variable_map["end"] = r22 (unchanged)

ループ2 condition_bindings 構築:
  start: HOST r??? → JoinIR ValueId(1)  // r35 or r32?

ループ2 JoinIR merge 後:
  variable_map["end"] = r48 (remapped exit value)
```

### 契約 (Contract)

**ExitLineReconnector の契約**:
1. merge_joinir_mir_blocks() 完了時点で variable_map が更新されている
2. 後続のコードは variable_map["carrier"] で最新の出口値を取得できる

**Pattern2 lowerer の契約**:
1. condition_bindings は variable_map の **現在の値** を使用する
2. ループ開始時点での variable_map が正しく更新されていることを前提とする

### 検証ポイント

1. `merge_joinir_mir_blocks()` の最後で ExitLineOrchestrator が呼ばれているか?
2. ExitLineReconnector が variable_map を正しく更新しているか?
3. Pattern2 lowerer が condition_bindings を構築するタイミングは正しいか?

---

## 🎯 33-13-2: 根本原因特定！

### 問題のフロー

```
1. ExitMetaCollector: exit_bindings 作成
   - start → JoinIR ValueId(9)

2. merge_joinir_mir_blocks:
   - JoinIR ValueId(9) → HOST ValueId(32) (remap)

3. exit_phi_builder: PHI 作成
   - phi_dst = builder.value_gen.next() → ValueId(0) ← NEW VALUE!
   - PHI { dst: ValueId(0), inputs: [..., ValueId(32)] }

4. ExitLineReconnector: variable_map 更新
   - variable_map["start"] = remapper.get_value(JoinIR ValueId(9)) = ValueId(32)

5. 問題！
   - variable_map["start"] = ValueId(32)
   - BUT PHI が定義しているのは ValueId(0)
   - → ValueId(32) は未定義！
```

### 根本原因

**exit_phi_builder と ExitLineReconnector の不整合**:
- `exit_phi_builder` は新しい `phi_dst` を割り当て
- `ExitLineReconnector` は `remapped_exit` を variable_map に設定
- **PHI が定義している ValueId と variable_map が指す ValueId が異なる**

### 設計上の制限

**単一 PHI 問題**:
- 現在の `exit_phi_builder` は **1つの PHI** しか作らない
- しかし trim は **2つのキャリア**（start, end）を持つ
- 複数キャリアには **複数の exit PHI** が必要

### 解決方向性

**Option A**: ExitLineReconnector を exit_phi_result を使うように変更
- シンプルだが、複数キャリアには対応できない

**Option B**: exit_phi_builder を複数キャリア対応に拡張
- 各 exit_binding ごとに PHI を作成
- ExitLineReconnector はその PHI の dst を variable_map に設定

**Option C**: exit_bindings から直接 PHI を作成する新 Box
- ExitLinePHIBuilder Box を新設
- 責任分離: PHI 作成と variable_map 更新を統合

**推奨**: Option B + C のハイブリッド
- exit_phi_builder を拡張して exit_bindings を受け取る
- 各キャリアごとに PHI を作成し、その dst を返す
- ExitLineReconnector はその結果を variable_map に設定

---

## 次のステップ

### Phase 33-13-3: exit_phi_builder の複数キャリア対応

1. exit_bindings を exit_phi_builder に渡す
2. 各キャリアごとに PHI を作成
3. 各 PHI の dst を carrier → phi_dst マップとして返す
4. ExitLineReconnector がそのマップを使って variable_map を更新

### Phase 33-13-4: 統合テスト

1. 単一キャリア（Pattern 2 simple）が動作確認
2. 複数キャリア（trim）が動作確認

---

## Phase 33-13-2: 「式結果 PHI」と「キャリア PHI」の責務分離設計

### アーキテクチャ分析

現在のフロー:
```
1. instruction_rewriter: Return の戻り値を exit_phi_inputs に収集
   - exit_phi_inputs.push((new_block_id, remapped_val))
   - これは「式としての戻り値」（例: loop_min_while() の結果）

2. exit_phi_builder: exit_phi_inputs から式結果 PHI を1つ作成
   - PHI { dst: NEW_ID, inputs: exit_phi_inputs }
   - 「式としての戻り値」用

3. ExitLineReconnector: exit_bindings の remapped_exit を variable_map に設定
   - variable_map[carrier] = remapper.get_value(join_exit_value)
   - これは「キャリア更新」用
```

### 根本問題の再確認

**問題**: ExitLineReconnector が設定する `remapped_exit` は、
**exit_block に到達する前のブロック**で定義されている。

しかし SSA では、exit_block 以降から参照するには、
**exit_block 内で PHI で合流させる必要がある**！

```
# 問題のあるコード                  # 正しいコード
k_exit:                             k_exit:
  // 何もない                         %start_final = phi [(%bb_A, %32), (%bb_B, %35)]
  // exit_block 以降で %32 参照       // exit_block 以降で %start_final 参照
  // → %32 は未定義！                 // → OK！
```

### 責務分離設計

#### 式結果 PHI (exit_phi_builder 担当)

**用途**: ループが「値を返す式」として使われる場合
```hako
local result = loop_min_while(...)  // ← 式として使用
```

**実装**:
- `instruction_rewriter` が Return の戻り値を収集
- `exit_phi_builder` がそれらを合流させる PHI を生成
- 返り値: `Option<ValueId>` (PHI の dst、または None)

#### キャリア PHI (新設: ExitCarrierPHIBuilder 担当)

**用途**: ループが「状態を更新するだけ」の場合
```hako
// trim パターン: start, end を更新
loop(start < end) { ... }  // start キャリア
loop(end > start) { ... }  // end キャリア
```

**実装案**:
1. `exit_bindings` から各キャリアの exit value を取得
2. 各キャリアごとに **PHI を生成**
3. PHI の dst を返す `BTreeMap<String, ValueId>`
4. ExitLineReconnector がその phi_dst を variable_map に設定

### 修正計画

#### Phase 33-13-3: exit_phi_builder をキャリア PHI 対応に拡張

**変更前**:
```rust
pub fn build_exit_phi(
    builder: &mut MirBuilder,
    exit_block_id: BasicBlockId,
    exit_phi_inputs: &[(BasicBlockId, ValueId)],  // 式結果のみ
    debug: bool,
) -> Result<Option<ValueId>, String>
```

**変更後**:
```rust
pub struct ExitPhiResult {
    pub expr_result: Option<ValueId>,              // 式結果 PHI (従来)
    pub carrier_phis: BTreeMap<String, ValueId>,   // キャリア PHI (新設)
}

pub fn build_exit_phi(
    builder: &mut MirBuilder,
    exit_block_id: BasicBlockId,
    exit_phi_inputs: &[(BasicBlockId, ValueId)],
    carrier_inputs: &BTreeMap<String, Vec<(BasicBlockId, ValueId)>>,  // NEW
    debug: bool,
) -> Result<ExitPhiResult, String>
```

#### Phase 33-13-4: ExitLineReconnector を phi_dst を使うように修正

**変更前** (reconnector.rs 99-107行):
```rust
if let Some(remapped_value) = remapper.get_value(binding.join_exit_value) {
    if let Some(var_vid) = builder.variable_map.get_mut(&binding.carrier_name) {
        *var_vid = remapped_value;  // ← remapped_exit を直接使用
    }
}
```

**変更後**:
```rust
if let Some(phi_dst) = carrier_phis.get(&binding.carrier_name) {
    if let Some(var_vid) = builder.variable_map.get_mut(&binding.carrier_name) {
        *var_vid = *phi_dst;  // ← PHI の dst を使用
    }
}
```

### 設計上の決定事項

1. **式結果 PHI は exit_phi_builder が担当** (変更なし)
2. **キャリア PHI は exit_phi_builder が追加で担当** (拡張)
3. **ExitLineReconnector は PHI の dst を variable_map に設定** (修正)
4. **exit_bindings は SSOT として維持** (変更なし)

---

## Date: 2025-12-07
## Status: In Progress - Design Phase Complete!
Status: Historical
