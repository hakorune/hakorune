# .hakoコンパイラ LoopForm v2化 実現可能性調査レポート

**調査日**: 2025-11-18
**調査者**: Claude Code (AI分析)
**目的**: Rust LoopForm v2 設計を .hako コンパイラに適用する実現可能性を評価

---

## エグゼクティブサマリー

### 結論: **実装可能だが工数大 (Phase C: 16-24時間)**

**推奨**: **Option 2 (Exit PHIのみ実装)** を推奨
**理由**:
- 現在の .hako コンパイラは既に Header PHI 生成機能を持つ (`LoopFormBox.loop_counter`)
- Exit PHI 生成のみが no-op stub (`LoopSSA.stabilize_merges`)
- 最小限の修正で Test 2 (break文付きループ) が動作可能
- 完全移植 (Option 1) は工数対効果が低い

---

## 1. 現状分析レポート

### 1.1 .hakoコンパイラの現在のループ実装

#### 実装箇所
```
lang/src/shared/mir/loop_form_box.hako          # LoopFormBox (PHI生成済み)
lang/src/compiler/builder/ssa/loopssa.hako     # LoopSSA (no-op stub)
lang/src/shared/mir/block_builder_box.hako     # BlockBuilderBox (MIR組立)
lang/src/shared/mir/mir_schema_box.hako        # MirSchemaBox (JSON生成)
```

#### 実装状況
✅ **Header PHI生成**: 実装済み
```hako
// LoopFormBox.loop_counter() より (lines 30-40)
local phi_i_inputs = new ArrayBox()
phi_i_inputs.push(MirSchemaBox.phi_incoming(0, 1))   // from preheader
phi_i_inputs.push(MirSchemaBox.phi_incoming(7, 18))  // from latch
header.push(MirSchemaBox.inst_phi(10, phi_i_inputs)) // r10 = current i
```

✅ **Latch PHI生成**: 実装済み (continue/normal merge)
```hako
// LoopFormBox.loop_counter() より (lines 71-80)
local latch_phi_i = new ArrayBox()
latch_phi_i.push(MirSchemaBox.phi_incoming(5, 15)) // continue path
latch_phi_i.push(MirSchemaBox.phi_incoming(6, 17)) // body path
latch_block.push(MirSchemaBox.inst_phi(18, latch_phi_i))
```

✅ **Exit PHI生成**: 実装済み (単一ループのみ)
```hako
// LoopFormBox.loop_counter() より (lines 83-89)
local exit_vals = new ArrayBox()
exit_vals.push(MirSchemaBox.phi_incoming(1, 11)) // normal completion
exit_vals.push(MirSchemaBox.phi_incoming(3, 11)) // break path
local exit_block = new ArrayBox()
exit_block.push(MirSchemaBox.inst_phi(20, exit_vals))
```

❌ **LoopSSA統合**: no-op stub
```hako
// loopssa.hako (lines 3-7)
static box LoopSSA {
  stabilize_merges(stage1_json) { return stage1_json } // ← 何もしていない
}
```

### 1.2 既存コードの問題点

**Problem 1: LoopFormBox は独立したテスト用コード**
- `LoopFormBox.loop_counter()` は完全なMIRモジュールを生成 (preheader → header → body → latch → exit)
- コンパイラパイプライン (`pipeline_v2/`) からは**使用されていない**
- テスト専用 (`tests/phase33/unit/loopform/test_basic.hako`)

**Problem 2: コンパイラは BlockBuilderBox 経由で個別命令生成**
- `emit_mir_flow.hako` → `BlockBuilderBox.loop_counter()` → 個別MIR命令
- LoopFormBox のような構造化アプローチは未使用

**Problem 3: Exit PHI生成がパイプラインに未統合**
- `LoopSSA.stabilize_merges()` は no-op stub
- break文のスナップショット収集機構が存在しない
- Exit PHI生成のタイミングが未定義

---

## 2. LoopForm v2 適用の技術的課題

### 2.1 データ構造表現の実現可能性

#### Rust実装 (参考)
```rust
struct CarrierVariable {
    name: String,
    init_value: ValueId,      // 初期値
    preheader_copy: ValueId,  // Preheaderでの一意ValueId
    header_phi: ValueId,      // Header PHIでの一意ValueId
    latch_value: ValueId,     // Latchでの更新値
}
```

#### .hako実装 (MapBox/ArrayBox)
**難易度: B (中程度)** - 3-4時間

```hako
// CarrierVariable相当をMapBoxで表現
method create_carrier_var(name, init_value) {
    local carrier = new MapBox()
    carrier.set("name", name)
    carrier.set("init_value", init_value)
    carrier.set("preheader_copy", -1)  // 後で割り当て
    carrier.set("header_phi", -1)
    carrier.set("latch_value", -1)
    return carrier
}
```

**評価**:
- ✅ MapBox/ArrayBox で構造体相当を表現可能
- ✅ MirSchemaBox で ValueId 表現は確立済み (`this.i(value)`)
- ⚠️ 型安全性なし (フィールドアクセスミスがランタイムエラー)
- ⚠️ コード量増加 (Rustの3-4倍)

### 2.2 4パス構成の実装可能性

#### Pass 1: prepare_structure
**難易度: B (中程度)** - 4-6時間

```hako
static box LoopFormV2Builder {
    carriers: ArrayBox    // CarrierVariable配列
    pinned: ArrayBox      // PinnedVariable配列

    prepare_structure(current_vars, params, allocator) {
        me.carriers = new ArrayBox()
        me.pinned = new ArrayBox()

        // current_vars をイテレート
        local keys = current_vars.keys()
        local i = 0
        loop(i < keys.length()) {
            local name = keys.get(i)
            local value = current_vars.get(name)

            if me._is_parameter(name, params) {
                // Pinned変数
                local pinned = new MapBox()
                pinned.set("name", name)
                pinned.set("param_value", value)
                pinned.set("preheader_copy", allocator.new_value())
                pinned.set("header_phi", allocator.new_value())
                me.pinned.push(pinned)
            } else {
                // Carrier変数
                local carrier = new MapBox()
                carrier.set("name", name)
                carrier.set("init_value", value)
                carrier.set("preheader_copy", allocator.new_value())
                carrier.set("header_phi", allocator.new_value())
                carrier.set("latch_value", -1)  // 後でsealing時に設定
                me.carriers.push(carrier)
            }

            i = i + 1
        }
    }

    _is_parameter(name, params) {
        local i = 0
        loop(i < params.length()) {
            if ("" + params.get(i)) == name { return 1 }
            i = i + 1
        }
        return 0
    }
}
```

**課題**:
- .hako に HashMap.keys() メソッドが存在するか不明 → **要確認**
- ValueId allocator が .hako コンパイラに存在するか → **要確認**

#### Pass 2: emit_preheader
**難易度: A (簡単)** - 2-3時間

```hako
emit_preheader(builder) {
    // Pinned変数のCopy生成
    local i = 0
    loop(i < me.pinned.length()) {
        local pin = me.pinned.get(i)
        builder.emit_copy(
            pin.get("preheader_copy"),
            pin.get("param_value")
        )
        i = i + 1
    }

    // Carrier変数のCopy生成
    i = 0
    loop(i < me.carriers.length()) {
        local car = me.carriers.get(i)
        builder.emit_copy(
            car.get("preheader_copy"),
            car.get("init_value")
        )
        i = i + 1
    }

    // Header へジャンプ
    builder.emit_jump(me.header_id)
}
```

**評価**: 既存の `MirSchemaBox.inst_copy()` / `inst_jump()` で実装可能

#### Pass 3: emit_header_phis
**難易度: B (中程度)** - 3-4時間

```hako
emit_header_phis(builder) {
    // Pinned変数のPHI生成 (不完全: preheaderのみ)
    local i = 0
    loop(i < me.pinned.length()) {
        local pin = me.pinned.get(i)
        local inputs = new ArrayBox()
        inputs.push(MirSchemaBox.phi_incoming(
            me.preheader_id,
            pin.get("preheader_copy")
        ))
        builder.emit_phi(
            pin.get("header_phi"),
            inputs
        )
        builder.update_var(pin.get("name"), pin.get("header_phi"))
        i = i + 1
    }

    // Carrier変数のPHI生成 (不完全: preheaderのみ)
    i = 0
    loop(i < me.carriers.length()) {
        local car = me.carriers.get(i)
        local inputs = new ArrayBox()
        inputs.push(MirSchemaBox.phi_incoming(
            me.preheader_id,
            car.get("preheader_copy")
        ))
        builder.emit_phi(
            car.get("header_phi"),
            inputs
        )
        builder.update_var(car.get("name"), car.get("header_phi"))
        i = i + 1
    }
}
```

**課題**:
- `builder.update_var()` が .hako コンパイラに存在するか → **要確認**
- PHI命令の更新機構 (seal時) が必要

#### Pass 4: seal_phis
**難易度: B (中程度)** - 4-5時間

```hako
seal_phis(builder, latch_id) {
    // Pinned変数のPHI完成 (latch入力追加)
    local i = 0
    loop(i < me.pinned.length()) {
        local pin = me.pinned.get(i)
        local latch_value = builder.get_variable_at_block(
            pin.get("name"),
            latch_id
        )
        if latch_value == null {
            latch_value = pin.get("header_phi")  // fallback
        }

        local inputs = new ArrayBox()
        inputs.push(MirSchemaBox.phi_incoming(
            me.preheader_id,
            pin.get("preheader_copy")
        ))
        inputs.push(MirSchemaBox.phi_incoming(
            latch_id,
            latch_value
        ))

        builder.update_phi_inputs(
            me.header_id,
            pin.get("header_phi"),
            inputs
        )
        i = i + 1
    }

    // Carrier変数のPHI完成 (同様の処理)
    // ... (省略)
}
```

**課題**:
- `builder.update_phi_inputs()` メソッドが必要 → **新規実装必要**
- JSONベースのMIR構築で「既存PHI命令の更新」をどう実現するか

### 2.3 Exit PHI生成

**難易度: C (難しい)** - 8-10時間

```hako
build_exit_phis(builder, exit_id, exit_snapshots) {
    // exit_snapshots: [(block_id, { var_name: value_id }), ...]

    local all_vars = new MapBox()  // var_name => [(block_id, value_id), ...]

    // Header fallthrough値を追加
    local i = 0
    loop(i < me.pinned.length()) {
        local pin = me.pinned.get(i)
        local name = pin.get("name")
        local inputs = new ArrayBox()
        inputs.push(new ArrayBox().push(me.header_id).push(pin.get("header_phi")))
        all_vars.set(name, inputs)
        i = i + 1
    }

    // Carrier変数も同様
    // ...

    // break snapshots を統合
    i = 0
    loop(i < exit_snapshots.length()) {
        local snap = exit_snapshots.get(i)
        local block_id = snap.get(0)
        local vars_map = snap.get(1)

        local var_names = vars_map.keys()
        local j = 0
        loop(j < var_names.length()) {
            local var_name = var_names.get(j)
            local value_id = vars_map.get(var_name)

            local existing = all_vars.get(var_name)
            if existing == null {
                existing = new ArrayBox()
                all_vars.set(var_name, existing)
            }
            existing.push(new ArrayBox().push(block_id).push(value_id))

            j = j + 1
        }
        i = i + 1
    }

    // PHI生成
    local var_names = all_vars.keys()
    i = 0
    loop(i < var_names.length()) {
        local var_name = var_names.get(i)
        local inputs = all_vars.get(var_name)

        // 重複除去 (sanitize_phi_inputs相当)
        inputs = me._sanitize_phi_inputs(inputs)

        if inputs.length() == 0 {
            // スキップ
        } else if inputs.length() == 1 {
            // 単一入力: 直接バインド
            builder.update_var(var_name, inputs.get(0).get(1))
        } else {
            // 複数入力: PHI生成
            local phi_id = builder.new_value()
            local phi_inputs = new ArrayBox()
            local j = 0
            loop(j < inputs.length()) {
                local inp = inputs.get(j)
                phi_inputs.push(MirSchemaBox.phi_incoming(
                    inp.get(0),  // block_id
                    inp.get(1)   // value_id
                ))
                j = j + 1
            }
            builder.emit_phi(phi_id, phi_inputs)
            builder.update_var(var_name, phi_id)
        }

        i = i + 1
    }
}

_sanitize_phi_inputs(inputs) {
    // 重複除去: block_idでグループ化し、最後の値を採用
    local map = new MapBox()
    local i = 0
    loop(i < inputs.length()) {
        local inp = inputs.get(i)
        local bb = "" + inp.get(0)  // block_idを文字列化
        map.set(bb, inp)
        i = i + 1
    }

    // MapBox → ArrayBox変換
    local result = new ArrayBox()
    local keys = map.keys()
    i = 0
    loop(i < keys.length()) {
        result.push(map.get(keys.get(i)))
        i = i + 1
    }

    return result
}
```

**最大の課題**:
- **break文検出とスナップショット収集**をどこで行うか？
- コンパイラパイプラインの改修が必要

### 2.4 ValueId事前割り当て

**現状調査結果**:
- .hako コンパイラのValueId生成器が不明
- `MirSchemaBox.i(value)` は既存値をラップするのみ

**必要機能**:
```hako
static box ValueIdAllocator {
    next_id: IntegerBox

    birth() {
        me.next_id = 100  // 初期値
    }

    new_value() {
        local id = me.next_id
        me.next_id = me.next_id + 1
        return id
    }
}
```

**難易度: A (簡単)** - 1-2時間

---

## 3. 実装難易度・工数見積もり

### 3.1 難易度評価

| コンポーネント | 難易度 | 工数 | 理由 |
|---------------|--------|------|------|
| データ構造設計 | B | 3-4h | MapBox/ArrayBox で実装可能だが冗長 |
| Pass 1: prepare_structure | B | 4-6h | HashMap.keys() 存在確認が必要 |
| Pass 2: emit_preheader | A | 2-3h | 既存APIで実装可能 |
| Pass 3: emit_header_phis | B | 3-4h | PHI更新機構が必要 |
| Pass 4: seal_phis | B | 4-5h | update_phi_inputs実装が必要 |
| Exit PHI: build_exit_phis | C | 8-10h | break検出機構の実装が必要 |
| ValueId allocator | A | 1-2h | シンプルなカウンター |
| テスト・デバッグ | B | 4-6h | MIR JSON検証 |

**総工数見積もり**:
- **最小 (MVP)**: 20-26時間
- **推奨 (完全版)**: 30-40時間

### 3.2 段階的実装計画

#### Phase A (MVP): 最小動作版
**目標**: Test 2でheader PHI生成を確認
**工数**: 10-12時間

**タスク**:
1. ValueIdAllocator実装 (1-2h)
2. LoopFormV2Builder骨格作成 (2-3h)
3. Pass 1-3実装 (header PHI生成のみ) (6-7h)
4. テスト実行 (1h)

**成果物**:
- 単純ループ (break/continueなし) で header PHI 生成
- `LoopFormBox.loop_count()` と同等の機能

#### Phase B (Exit PHI): break対応
**目標**: Test 2で単一break文のループが動作
**工数**: 12-16時間

**タスク**:
1. Pass 4 (seal_phis) 実装 (4-5h)
2. break文検出機構 (5-6h)
3. build_exit_phis実装 (8-10h)
4. テスト・デバッグ (3-4h)

**成果物**:
- break文付きループで exit PHI 生成
- `LoopFormBox.loop_counter()` と同等の機能

#### Phase C (完全版): 全機能
**目標**: Test 2完全パス
**工数**: 8-12時間

**タスク**:
1. continue対応 (latch PHI) (3-4h)
2. ネストループ対応 (4-5h)
3. 複数break対応 (1-2h)
4. 総合テスト (2-3h)

**成果物**:
- Rust LoopForm v2 と同等の完全実装

**総工数 (Phase A-C)**: **30-40時間**

---

## 4. 代替案の検討

### Option 1: LoopForm v2 完全移植
**メリット**:
- ✅ Rustと同等の完全性
- ✅ 将来的な拡張性

**デメリット**:
- ❌ 実装工数大 (30-40時間)
- ❌ .hakoコンパイラのメンテナンス負荷増
- ❌ 型安全性なし (ランタイムエラーリスク)

**評価**: **非推奨** (工数対効果が低い)

### Option 2: Exit PHIのみ実装 ⭐推奨
**メリット**:
- ✅ 最小限の修正でTest 2パス
- ✅ 既存の `LoopFormBox` を活用
- ✅ 工数削減 (12-16時間)

**デメリット**:
- ⚠️ Header PHI生成は既存ロジック依存
- ⚠️ LoopForm v2の核心思想 (事前ValueId割り当て) を完全には反映しない

**実装方針**:
1. `LoopSSA.stabilize_merges()` を実装
2. break文検出とスナップショット収集
3. Exit PHI生成 (`build_exit_phis` 相当)

**評価**: **強く推奨** (実用性と工数のバランス)

### Option 3: Rustコンパイラに統一
**メリット**:
- ✅ .hakoコンパイラのメンテナンス不要
- ✅ LoopForm v2の恩恵を直接享受

**デメリット**:
- ❌ Selfhostの意義喪失
- ❌ Phase 15目標に反する

**評価**: **非推奨** (プロジェクト方針に反する)

---

## 5. 推奨事項

### 推奨Option: **Option 2 (Exit PHIのみ実装)**

**理由**:
1. **最小限の修正で目標達成**: Test 2 (break文付きループ) が動作
2. **既存実装を活用**: `LoopFormBox.loop_counter()` で Header/Latch PHI は既に実装済み
3. **工数削減**: 12-16時間 (vs 完全移植30-40時間)
4. **段階的移行**: 将来的に完全移植も可能 (Phase Cへの道筋)

### 実装計画 (Option 2)

**Step 1: break文検出機構 (4-5時間)**
```hako
// lang/src/compiler/pipeline_v2/stmt_break_detector_box.hako (新規)
static box StmtBreakDetectorBox {
    break_snapshots: ArrayBox  // [(block_id, var_snapshot), ...]

    birth() {
        me.break_snapshots = new ArrayBox()
    }

    capture_snapshot(block_id, var_map) {
        local snap = new ArrayBox()
        snap.push(block_id)
        snap.push(var_map)
        me.break_snapshots.push(snap)
    }

    get_snapshots() {
        return me.break_snapshots
    }
}
```

**Step 2: Exit PHI生成 (6-8時間)**
```hako
// lang/src/compiler/builder/ssa/loopssa.hako (修正)
using lang.compiler.pipeline_v2.stmt_break_detector_box as BreakDetector

static box LoopSSA {
    stabilize_merges(stage1_json, break_detector, header_vars) {
        // break文が存在しない場合はスキップ
        local snaps = break_detector.get_snapshots()
        if snaps.length() == 0 { return stage1_json }

        // Exit PHI生成
        local exit_phis = me._build_exit_phis(
            header_vars,
            snaps
        )

        // stage1_jsonに追加
        // ... (JSON操作)

        return stage1_json
    }

    _build_exit_phis(header_vars, exit_snapshots) {
        // Option 2の build_exit_phis 実装
        // ... (上記セクション2.3参照)
    }
}
```

**Step 3: パイプライン統合 (2-3時間)**
- `emit_mir_flow.hako` から `LoopSSA.stabilize_merges()` を呼び出し
- break文処理時に `BreakDetector.capture_snapshot()` を呼び出し

**Step 4: テスト・デバッグ (2-3時間)**
- Test 2 (break文付きループ) で動作確認
- MIR JSON 検証

**総工数**: **14-19時間**

### リスク評価

**技術リスク: 中**
- .hako コンパイラのアーキテクチャが JSON ベースであり、Rust の命令型アプローチと異なる
- PHI命令の更新 (seal時) が JSON 操作で複雑化する可能性

**メンテナンスリスク: 低**
- Option 2 は既存の `LoopFormBox` を活用するため、新規コードは最小限
- `LoopSSA.stabilize_merges()` のみの実装で、他への影響が少ない

**互換性リスク: 低**
- 既存のループ処理は変更せず、exit PHI 生成のみ追加
- 後方互換性が保たれる

---

## 6. 期待される成果物

### 1. 現状分析レポート ✅
**内容**: 本ドキュメント セクション1

### 2. 実装可能性評価 ✅
**内容**: 本ドキュメント セクション2

### 3. 工数見積もり ✅
**内容**: 本ドキュメント セクション3

### 4. 実装計画案 ✅
**内容**: 本ドキュメント セクション5

### 5. 推奨事項 ✅
**結論**: **Option 2 (Exit PHIのみ実装)** を推奨

**根拠**:
- 最小工数 (14-19時間) で目標達成
- 既存実装を活用し、リスク低減
- 段階的移行の道筋が明確

**次のアクション**:
1. ユーザーに Option 2 を提案
2. 承認後、実装開始 (Step 1-4)
3. Test 2 パス確認
4. 必要に応じて Phase C (完全版) への移行を検討

---

## 7. 調査手法

### 実施した調査
1. ✅ コード精読: `lang/src/compiler/` 配下の関連ファイル
   - `loopssa.hako`, `loop_form_box.hako`, `block_builder_box.hako`, `mir_schema_box.hako`
2. ✅ Rust実装との比較: `src/mir/phi_core/loopform_builder.rs` との対比
3. ✅ アーキテクチャ分析: .hakoのMIR生成パイプライン理解
   - JSON ベースのMIR構築方式を確認
4. ✅ 実装シミュレーション: 疑似コードレベルでの実装可能性検証

### 未確認項目 (要調査)
- [ ] .hako の `MapBox.keys()` メソッド存在確認
- [ ] .hako コンパイラの ValueId 生成器の実装箇所
- [ ] `builder.update_var()` / `builder.get_variable_at_block()` の存在確認
- [ ] JSON操作によるPHI命令更新の実装詳細

---

## 付録A: Rust vs .hako データ構造対応表

| Rust型 | .hako表現 | 実装例 |
|--------|-----------|--------|
| `struct CarrierVariable` | `MapBox` | `local car = new MapBox(); car.set("name", "i")` |
| `Vec<CarrierVariable>` | `ArrayBox` | `local carriers = new ArrayBox(); carriers.push(car)` |
| `HashMap<String, ValueId>` | `MapBox` | `local vars = new MapBox(); vars.set("i", 10)` |
| `ValueId` | `IntegerBox` | `local vid = 10` (整数で表現) |
| `BasicBlockId` | `IntegerBox` | `local bid = 5` (整数で表現) |

**注意点**:
- .hako は型安全性がないため、フィールドアクセスミスがランタイムエラーになる
- Rustの3-4倍のコード量が必要

---

## 付録B: 参考実装コード (Option 2)

**完全な実装例は本ドキュメント セクション5参照**

---

**レポート作成日**: 2025-11-18
**次回レビュー**: 実装開始前 (ユーザー承認後)
**ドキュメント状態**: READY FOR REVIEW ✅
