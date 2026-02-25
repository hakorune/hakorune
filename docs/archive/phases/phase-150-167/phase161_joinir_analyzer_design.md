# Phase 161: Rust JoinIR/MIR を .hako から読む Analyzer 実装

## Task 1 結果: 入力フォーマット仕様完全調査

実施日: 2025-12-04

---

## 📋 調査概要

本ドキュメントは、Phase 161 で実装する .hako 側 Analyzer Box が読むべき Rust JoinIR/MIR JSON フォーマットの完全なインベントリです。

### 調査対象ファイル
- `src/mir/join_ir/json.rs` - JoinIR JSON シリアライザ
- `src/runner/mir_json_emit.rs` - MIR JSON v0/v1 エミット
- `src/runner/mir_json_v0.rs` - MIR JSON v0 パーサー
- `src/tests/joinir_json_min.rs` - テスト例
- `tests/fixtures/joinir/v0_*.jsonir` - 実際の JoinIR JSON サンプル
- `build/doctor/mir/*.mir.json` - 実際の MIR JSON サンプル

---

## 🎯 推奨アプローチ: **MIR JSON v1 を優先**

### 理由

1. **統一Call命令対応** (Phase 15.5):
   - MIR JSON v1 は統一Call (`mir_call`) を完全サポート
   - Callee型（Global/Method/Constructor/Closure/Value/Extern）を型安全に表現
   - Phase 15.5 で確立された設計と完全一致

2. **スキーマバージョン管理**:
   - v1 には `schema_version`, `capabilities`, `metadata` が含まれる
   - 将来の拡張性を保証

3. **CFG情報の統合**:
   - MIR JSON v1 には `cfg` (Control Flow Graph) 情報が含まれる
   - hako_check で使用する制御フロー解析データが利用可能

4. **JoinIR → MIR 変換経路の安定性**:
   - JoinIR → MIR 変換は Phase 32 で完成済み
   - JoinIR は MIR の前段階であり、MIR のほうがより最適化・安定している

---

## 📊 1. MIR JSON v1 最小構造（推奨）

### 1.1 ルートレベル

```json
{
  "schema_version": "1.0",
  "capabilities": [
    "unified_call",      // 統一Call命令サポート
    "phi",              // SSA PHI関数
    "effects",          // エフェクト追跡
    "callee_typing"     // 型安全なCallee解決
  ],
  "metadata": {
    "generator": "nyash-rust",
    "phase": "15.5",
    "build_time": "Phase 15.5 Development",
    "features": ["mir_call_unification", "json_v1_schema"]
  },
  "functions": [...],  // 関数配列（下記参照）
  "cfg": {             // 制御フロー情報（Phase 155追加）
    "functions": {...}  // 関数ごとのCFG情報
  }
}
```

**重要キー**:
- `schema_version`: "1.0"（固定）
- `capabilities`: 機能リスト（.hako側で対応機能を判定可能）
- `functions`: 関数定義配列
- `cfg`: 制御フロー情報（hako_check用）

---

### 1.2 関数レベル

```json
{
  "name": "main",
  "params": [0, 1, 2],        // ValueId配列
  "entry": 0,                  // エントリブロックID
  "blocks": [...]              // BasicBlock配列（下記参照）
}
```

**重要キー**:
- `name`: 関数名（`"main"`, `"Main.main"`, `"Main.equals/1"` 等）
- `params`: パラメータのValueId配列
- `entry`: エントリBasicBlockのID
- `blocks`: BasicBlock配列

---

### 1.3 BasicBlockレベル

```json
{
  "id": 0,
  "instructions": [...]  // Instruction配列（下記参照）
}
```

**重要キー**:
- `id`: BasicBlockのID（u32）
- `instructions`: 命令配列

---

### 1.4 命令レベル（全命令タイプ）

#### 1.4.1 基本命令

**Const命令**:
```json
{
  "op": "const",
  "dst": 0,
  "value": {
    "type": "i64",       // "i64" | "f64" | "void" | {"kind":"handle","box_type":"StringBox"}
    "value": 42          // 実際の値
  }
}
```

**Copy命令**:
```json
{
  "op": "copy",
  "dst": 1,
  "src": 0
}
```

**BinOp命令**:
```json
{
  "op": "binop",
  "operation": "+",    // "+"|"-"|"*"|"/"|"%"|"&"|"|"|"^"|"<<"|">>"|"&&"|"||"
  "lhs": 0,
  "rhs": 1,
  "dst": 2,
  "dst_type": {...}    // オプション: StringBox等の型ヒント
}
```

**UnaryOp命令**:
```json
{
  "op": "unop",
  "kind": "neg",       // "neg" | "not" | "bitnot"
  "src": 0,
  "dst": 1
}
```

**Compare命令**:
```json
{
  "op": "compare",
  "operation": "==",   // "==" | "!=" | "<" | "<=" | ">" | ">="
  "lhs": 0,
  "rhs": 1,
  "dst": 2,
  "cmp_kind": "string" // オプション: 文字列比較ヒント
}
```

#### 1.4.2 制御フロー命令

**Branch命令**:
```json
{
  "op": "branch",
  "cond": 0,
  "then": 1,
  "else": 2
}
```

**Jump命令**:
```json
{
  "op": "jump",
  "target": 3
}
```

**Return命令**:
```json
{
  "op": "ret",
  "value": 0  // null の場合は void return
}
```

**PHI命令**（SSA合流点）:
```json
{
  "op": "phi",
  "dst": 10,
  "incoming": [
    [5, 1],  // [ValueId, BasicBlockId] のペア
    [7, 2]
  ],
  "dst_type": {...}  // オプション: 型ヒント
}
```

#### 1.4.3 Box操作命令

**NewBox命令**:
```json
{
  "op": "newbox",
  "type": "StringBox",
  "args": [0, 1],
  "dst": 2
}
```

**BoxCall命令（v0形式）**:
```json
{
  "op": "boxcall",
  "box": 0,
  "method": "length",
  "args": [1],
  "dst": 2,
  "dst_type": {...}  // オプション: 戻り値型ヒント
}
```

#### 1.4.4 統一Call命令（v1形式、推奨）

**MirCall命令（統一Call）**:
```json
{
  "op": "mir_call",
  "dst": 10,
  "mir_call": {
    "callee": {
      "type": "Method",        // "Global"|"Method"|"Constructor"|"Closure"|"Value"|"Extern"
      "box_name": "StringBox",
      "method": "substring",
      "receiver": 0,
      "certainty": "Known"     // "Known" | "Union"
    },
    "args": [1, 2],
    "effects": ["IO"],         // エフェクトリスト
    "flags": {}
  }
}
```

**Calleeタイプ別構造**:

1. **Global Call**:
```json
{
  "type": "Global",
  "name": "nyash.builtin.print"
}
```

2. **Method Call**:
```json
{
  "type": "Method",
  "box_name": "StringBox",
  "method": "substring",
  "receiver": 0,
  "certainty": "Known"
}
```

3. **Constructor Call**:
```json
{
  "type": "Constructor",
  "box_type": "StringBox"
}
```

4. **Closure Call**:
```json
{
  "type": "Closure",
  "params": ["x", "y"],
  "captures": [["env", 10], ["state", 11]],
  "me_capture": 12
}
```

5. **Value Call**（第一級関数）:
```json
{
  "type": "Value",
  "function_value": 5
}
```

6. **Extern Call**:
```json
{
  "type": "Extern",
  "name": "nyash.console.log"
}
```

#### 1.4.5 その他命令

**TypeOp命令**:
```json
{
  "op": "typeop",
  "operation": "check",  // "check" | "cast"
  "src": 0,
  "dst": 1,
  "target_type": "StringBox"
}
```

**ExternCall命令（v0形式）**:
```json
{
  "op": "externcall",
  "func": "nyash.console.log",
  "args": [0],
  "dst": null,
  "dst_type": "i64"
}
```

---

## 📊 2. JoinIR JSON v0 構造（参考）

JoinIR は MIR の前段階であり、継続渡しスタイル (CPS) で表現されます。

### 2.1 ルートレベル

```json
{
  "version": 0,
  "entry": 0,          // エントリ関数のID
  "functions": [...]   // JoinFunction配列
}
```

### 2.2 関数レベル

```json
{
  "id": 0,
  "name": "skip",
  "params": [3000],
  "exit_cont": null,   // 終了継続（オプション）
  "body": [...]        // JoinInst配列
}
```

### 2.3 命令レベル

**Compute命令**（MIRライクな計算）:
```json
{
  "type": "compute",
  "op": {
    "kind": "const",
    "dst": 3001,
    "value_type": "integer",
    "value": 0
  }
}
```

**Call命令**（継続渡し）:
```json
{
  "type": "call",
  "func": 1,
  "args": [3000, 3001, 3002],
  "k_next": null,      // 継続先（オプション）
  "dst": null          // 戻り値先（オプション）
}
```

**Jump命令**（継続へのジャンプ）:
```json
{
  "type": "jump",
  "cont": 0,
  "args": [4001],
  "cond": 4003         // 条件（オプション）
}
```

**Select命令**（Phase 33追加）:
```json
{
  "type": "select",
  "dst": 10,
  "cond": 1,
  "then_val": 20,
  "else_val": 30,
  "type_hint": "..."   // オプション
}
```

**IfMerge命令**（Phase 33-6追加）:
```json
{
  "type": "if_merge",
  "cond": 1,
  "merges": [
    {"dst": 10, "then_val": 20, "else_val": 30},
    {"dst": 11, "then_val": 21, "else_val": 31}
  ],
  "k_next": null
}
```

---

## 🎯 3. .hako Analyzer Box 実装推奨事項

### 3.1 優先実装順序

1. **Phase 161-1**: MIR JSON v1 基本構造読み込み
   - ルート構造（schema_version, capabilities, functions）
   - 関数構造（name, params, entry, blocks）
   - BasicBlock構造（id, instructions）

2. **Phase 161-2**: 基本命令対応
   - Const, Copy, BinOp, Compare
   - Branch, Jump, Return
   - PHI（SSA合流点）

3. **Phase 161-3**: 統一Call命令対応
   - MirCall（6種のCalleeタイプ）
   - エフェクト追跡
   - 型ヒント処理

4. **Phase 161-4**: Box操作命令対応
   - NewBox, BoxCall
   - TypeOp

5. **Phase 161-5**: CFG情報活用
   - 制御フロー解析
   - デッドコード検出
   - 到達可能性解析

### 3.2 必須読み取りキー一覧

**最小限の .hako Analyzer が読むべきキー**:

#### ルートレベル
- ✅ `schema_version` - バージョン確認
- ✅ `capabilities` - 機能対応チェック
- ✅ `functions` - 関数配列

#### 関数レベル
- ✅ `name` - 関数名
- ✅ `params` - パラメータValueId配列
- ✅ `entry` - エントリブロックID
- ✅ `blocks` - BasicBlock配列

#### BasicBlockレベル
- ✅ `id` - ブロックID
- ✅ `instructions` - 命令配列

#### 命令レベル（最小セット）
- ✅ `op` - 命令タイプ識別子
- ✅ `dst` - 出力先ValueId
- ✅ `value` - Constの値
- ✅ `operation` - BinOp/Compareの演算子
- ✅ `lhs`, `rhs` - 二項演算のオペランド
- ✅ `cond`, `then`, `else` - Branch分岐先
- ✅ `target` - Jump先
- ✅ `incoming` - PHIの入力ペア

#### 統一Call命令レベル（Phase 161-3以降）
- ✅ `mir_call` - 統一Call構造
- ✅ `callee.type` - Calleeタイプ
- ✅ `args` - 引数ValueId配列
- ✅ `effects` - エフェクトリスト

---

## 🔍 4. 型情報の読み取り方法

### 4.1 MirType表現

MIR JSON では型情報が以下の形式で表現されます:

**プリミティブ型**:
- `"i64"` - 整数
- `"f64"` - 浮動小数点
- `"void"` - void型

**Box型**:
```json
{
  "kind": "handle",
  "box_type": "StringBox"
}
```

### 4.2 型ヒント活用

以下の命令で型ヒントが提供されます:

1. **Const命令**: `value.type` で定数の型
2. **BinOp命令**: `dst_type` で結果型（文字列連結等）
3. **Compare命令**: `cmp_kind` で比較種別（文字列比較等）
4. **PHI命令**: `dst_type` で合流後の型
5. **BoxCall命令**: `dst_type` で戻り値型

### 4.3 型伝播アルゴリズム

MIR JSON v0 パーサー (`mir_json_emit.rs`) では以下の型伝播を実施:

1. **PHI型伝播**: 全incoming値がStringBoxなら結果もStringBox
2. **BinOp型伝播**: 左辺または右辺がStringBoxで演算子が`+`なら結果もStringBox
3. **Compare型伝播**: 両辺がStringBoxなら`cmp_kind: "string"`

---

## 🔄 5. PHI, Loop, If の識別方法

### 5.1 PHI命令の識別

**MIR JSON での PHI**:
```json
{
  "op": "phi",
  "dst": 10,
  "incoming": [[5, 1], [7, 2]]
}
```

**識別ポイント**:
- `op == "phi"` で確実に識別
- `incoming` が複数BasicBlockからの入力を持つ
- SSA形式の合流点を表す

### 5.2 Loopの識別

**制御フロー解析**:
1. BasicBlockの `instructions` に `branch` または `jump` がある
2. `jump.target` または `branch.then/else` が自分より前のBlockを指す（後方エッジ）
3. 後方エッジがあればループ構造

**CFG情報活用** (MIR JSON v1):
```json
{
  "cfg": {
    "functions": {
      "main": {
        "loops": [
          {
            "header": 2,
            "body": [2, 3, 4],
            "exits": [5]
          }
        ]
      }
    }
  }
}
```

### 5.3 Ifの識別

**Branch命令**:
```json
{
  "op": "branch",
  "cond": 0,
  "then": 1,
  "else": 2
}
```

**識別ポイント**:
- `op == "branch"` で確実に識別
- `then` と `else` が異なるBasicBlockを指す
- 条件分岐構造

---

## 📊 6. 関数選定基準

.hako Analyzer が解析する関数の選定基準:

### 6.1 エントリポイント

**エントリ関数の検出**:
1. ルートレベルの `entry` フィールド（JoinIR）
2. 関数名が `"main"` または `"Main.main"`（MIR）
3. スタティックBox内の `main()` メソッド

### 6.2 解析対象関数

**優先順位**:
1. ✅ **エントリ関数**: 実行起点
2. ✅ **呼び出し元がある関数**: Call命令でreachable
3. ⚠️ **デッドコード関数**: 到達不可能（hako_check検出対象）

**フィルタリング**:
- Phase 161-1: 全関数を読み込み
- Phase 161-4: CFG解析で到達可能性判定
- Phase 161-5: デッドコード検出・警告

---

## 🎯 7. 代表的な関数スニペット

### 7.1 Simple If（MIR JSON v1）

**ソース**: `local_tests/phase123_simple_if.hako`

```json
{
  "schema_version": "1.0",
  "capabilities": ["unified_call", "phi", "effects", "callee_typing"],
  "functions": [
    {
      "name": "main",
      "params": [],
      "entry": 0,
      "blocks": [
        {
          "id": 0,
          "instructions": [
            {"op": "const", "dst": 0, "value": {"type": "i64", "value": 1}},
            {"op": "const", "dst": 1, "value": {"type": "i64", "value": 0}},
            {"op": "compare", "operation": "==", "lhs": 0, "rhs": 1, "dst": 2},
            {"op": "branch", "cond": 2, "then": 1, "else": 2}
          ]
        },
        {
          "id": 1,
          "instructions": [
            {"op": "const", "dst": 3, "value": {"type": "i64", "value": 10}},
            {"op": "jump", "target": 3}
          ]
        },
        {
          "id": 2,
          "instructions": [
            {"op": "const", "dst": 4, "value": {"type": "i64", "value": 20}},
            {"op": "jump", "target": 3}
          ]
        },
        {
          "id": 3,
          "instructions": [
            {"op": "phi", "dst": 5, "incoming": [[3, 1], [4, 2]]},
            {"op": "ret", "value": 5}
          ]
        }
      ]
    }
  ]
}
```

**構造の特徴**:
- Block 0: エントリ、条件評価、branch
- Block 1, 2: then/else分岐
- Block 3: PHI合流 + return
- PHI命令が複数ブロックからの値を合流

### 7.2 Minimum Loop（MIR JSON v1）

**ソース**: `apps/tests/loop_min_while.hako`

```json
{
  "schema_version": "1.0",
  "functions": [
    {
      "name": "main",
      "params": [],
      "entry": 0,
      "blocks": [
        {
          "id": 0,
          "instructions": [
            {"op": "const", "dst": 0, "value": {"type": "i64", "value": 0}},
            {"op": "jump", "target": 1}
          ]
        },
        {
          "id": 1,
          "instructions": [
            {"op": "phi", "dst": 1, "incoming": [[0, 0], [2, 2]]},
            {"op": "const", "dst": 3, "value": {"type": "i64", "value": 3}},
            {"op": "compare", "operation": "<", "lhs": 1, "rhs": 3, "dst": 4},
            {"op": "branch", "cond": 4, "then": 2, "else": 3}
          ]
        },
        {
          "id": 2,
          "instructions": [
            {"op": "const", "dst": 5, "value": {"type": "i64", "value": 1}},
            {"op": "binop", "operation": "+", "lhs": 1, "rhs": 5, "dst": 2},
            {"op": "jump", "target": 1}
          ]
        },
        {
          "id": 3,
          "instructions": [
            {"op": "const", "dst": 6, "value": {"type": "i64", "value": 0}},
            {"op": "ret", "value": 6}
          ]
        }
      ]
    }
  ]
}
```

**構造の特徴**:
- Block 0: ループ初期化
- Block 1: ループヘッダ（PHI、条件評価、branch）
- Block 2: ループボディ（インクリメント、後方ジャンプ）
- Block 3: ループ脱出、return
- PHI命令がループ変数を管理

### 7.3 JoinIR Skip_WS（JoinIR JSON v0）

**ソース**: `tests/fixtures/joinir/v0_skip_ws_min.jsonir`

```json
{
  "version": 0,
  "entry": 0,
  "functions": [
    {
      "id": 0,
      "name": "skip",
      "params": [3000],
      "exit_cont": null,
      "body": [
        {
          "type": "compute",
          "op": {
            "kind": "const",
            "dst": 3001,
            "value_type": "integer",
            "value": 0
          }
        },
        {
          "type": "compute",
          "op": {
            "kind": "boxcall",
            "dst": 3002,
            "box": "StringBox",
            "method": "length",
            "args": [3000]
          }
        },
        {
          "type": "call",
          "func": 1,
          "args": [3000, 3001, 3002],
          "k_next": null,
          "dst": null
        }
      ]
    },
    {
      "id": 1,
      "name": "loop_step",
      "params": [4000, 4001, 4002],
      "exit_cont": null,
      "body": [
        {
          "type": "compute",
          "op": {
            "kind": "compare",
            "dst": 4003,
            "op": "ge",
            "lhs": 4001,
            "rhs": 4002
          }
        },
        {
          "type": "jump",
          "cont": 0,
          "args": [4001],
          "cond": 4003
        },
        {"type": "compute", "op": {"kind": "const", "dst": 4007, "value_type": "integer", "value": 1}},
        {"type": "compute", "op": {"kind": "binop", "dst": 4006, "op": "add", "lhs": 4001, "rhs": 4007}},
        {
          "type": "call",
          "func": 1,
          "args": [4000, 4006, 4002],
          "k_next": null,
          "dst": null
        }
      ]
    }
  ]
}
```

**構造の特徴**:
- 継続渡しスタイル (CPS)
- 関数呼び出しが末尾再帰形式
- `jump` 命令で継続選択
- ループが関数呼び出しで表現

---

## 📋 8. Phase 161 実装チェックリスト

### Phase 161-1: 基本構造読み込み

- [ ] `AnalyzerBox` または `MirAnalyzer` Box を .hako で実装
- [ ] JSON読み込み（`JsonBox.parse()` または `FileBox.read_all()` + parse）
- [ ] ルート構造パース（`schema_version`, `capabilities`, `functions`）
- [ ] 関数構造パース（`name`, `params`, `entry`, `blocks`）
- [ ] BasicBlock構造パース（`id`, `instructions`）

### Phase 161-2: 基本命令対応

- [ ] Const命令パース（`op: "const"`, `dst`, `value`）
- [ ] Copy命令パース（`op: "copy"`, `dst`, `src`）
- [ ] BinOp命令パース（`op: "binop"`, `operation`, `lhs`, `rhs`, `dst`）
- [ ] Compare命令パース（`op: "compare"`, `operation`, `lhs`, `rhs`, `dst`）
- [ ] Branch命令パース（`op: "branch"`, `cond`, `then`, `else`）
- [ ] Jump命令パース（`op: "jump"`, `target`）
- [ ] Return命令パース（`op: "ret"`, `value`）
- [ ] PHI命令パース（`op: "phi"`, `dst`, `incoming`）

### Phase 161-3: 統一Call命令対応

- [ ] MirCall命令パース（`op: "mir_call"`, `mir_call`）
- [ ] Callee型判定（`callee.type`）
- [ ] Global Call パース
- [ ] Method Call パース
- [ ] Constructor Call パース
- [ ] Closure Call パース
- [ ] Value Call パース
- [ ] Extern Call パース

### Phase 161-4: Box操作命令対応

- [ ] NewBox命令パース（`op: "newbox"`, `type`, `args`, `dst`）
- [ ] BoxCall命令パース（`op: "boxcall"`, `box`, `method`, `args`, `dst`）
- [ ] TypeOp命令パース（`op: "typeop"`, `operation`, `src`, `dst`, `target_type`）

### Phase 161-5: 解析機能実装

- [ ] CFG解析（`cfg` フィールド読み込み）
- [ ] 到達可能性解析（エントリ関数からのreachability）
- [ ] デッドコード検出（unreachable関数・ブロック）
- [ ] PHI検証（incoming BlockIdの存在確認）
- [ ] Loop検出（後方エッジ検出）
- [ ] If検出（Branch命令からの分岐構造）

---

## 🎯 9. まとめ: .hako Analyzer が読むべき最小セット

### 最優先実装（Phase 161-1）

**ルートレベル**:
- `schema_version`: バージョン確認
- `capabilities`: 機能対応チェック
- `functions`: 関数配列

**関数レベル**:
- `name`: 関数名
- `params`: パラメータValueId配列
- `entry`: エントリブロックID
- `blocks`: BasicBlock配列

**BasicBlockレベル**:
- `id`: ブロックID
- `instructions`: 命令配列

**基本命令**:
- `op`: 命令タイプ
- `dst`, `src`, `value`, `lhs`, `rhs`: 基本フィールド
- `cond`, `then`, `else`, `target`: 制御フロー

### 次優先実装（Phase 161-2/3）

**統一Call命令**:
- `mir_call.callee.type`: Calleeタイプ
- `mir_call.args`: 引数配列
- `mir_call.effects`: エフェクトリスト

**PHI命令**:
- `incoming`: 入力ペア配列

**型情報**:
- `dst_type`: 型ヒント
- `value.type`: 定数型

---

## 📚 10. 参考資料

### 実装ファイル（Rust側）
- `src/mir/join_ir/json.rs` - JoinIR JSON シリアライザ
- `src/runner/mir_json_emit.rs` - MIR JSON エミット
- `src/runner/mir_json_v0.rs` - MIR JSON パーサー
- `src/mir/types.rs` - MIR型定義
- `src/mir/definitions/call_unified.rs` - 統一Call定義

### テストケース
- `src/tests/joinir_json_min.rs` - JoinIR JSONテスト
- `tests/fixtures/joinir/v0_*.jsonir` - JoinIR固定データ
- `build/doctor/mir/*.mir.json` - MIR実例

### ドキュメント
- `docs/development/current/main/phase130_joinir_llvm_baseline.md` - Phase 130ベースライン
- `docs/reference/mir/INSTRUCTION_SET.md` - MIR命令セット仕様
- `CLAUDE.md` - 開発ガイド（MIRデバッグ手法）

---

## 🔍 11. 重要な発見・推奨事項

### 11.1 JoinIR vs MIR の選択

**推奨: MIR JSON v1を優先**

理由:
1. ✅ **安定性**: JoinIR → MIR 変換経路は Phase 32 で確立済み
2. ✅ **最適化**: MIR はより最適化・正規化されている
3. ✅ **統一Call**: Phase 15.5 の統一Call命令が完全サポート
4. ✅ **CFG情報**: Phase 155 でCFG情報が統合済み
5. ✅ **型情報**: 型ヒントが充実（PHI, BinOp, Compare等）

JoinIRの用途:
- JoinIR固有の最適化研究（継続渡しスタイルの解析等）
- MIR変換前の中間表現検証

### 11.2 PHI処理の重要性

**PHI命令は .hako Analyzer の核心**:
- SSA形式の合流点を表す
- 複数BasicBlockからの値を統合
- ループ変数、if合流後の変数管理に必須
- `incoming` フィールドで [ValueId, BasicBlockId] ペアを処理

**実装のポイント**:
1. `incoming` 配列を順次処理
2. 各BasicBlockIdの存在を確認（不正なPHIの検出）
3. 型ヒント (`dst_type`) を活用して型伝播

### 11.3 統一Call命令の威力

**Phase 15.5 統一Call命令の利点**:
- 6種のCalleeタイプを統一的に処理
- 型安全な関数解決（コンパイル時）
- シャドウイング問題の根本解決
- エフェクト追跡による最適化

**.hako Analyzer での活用**:
1. `callee.type` でCall種別判定
2. `certainty` フィールドで型確定度判定
3. `effects` でIO/副作用解析
4. デッドコード検出（未使用関数の検出）

### 11.4 型情報活用のベストプラクティス

**型ヒント活用箇所**:
1. **Const命令**: `value.type` で定数の型を確定
2. **BinOp命令**: `dst_type` で文字列連結等を判定
3. **Compare命令**: `cmp_kind: "string"` で文字列比較判定
4. **PHI命令**: `dst_type` で合流後の型を確定
5. **BoxCall命令**: `dst_type` でメソッド戻り値型を確定

**型伝播アルゴリズム**:
- Phase 25: MIR JSON エミット時に型伝播を実施
- 4回反復で `copy → phi → copy` チェーン完全対応
- .hako Analyzer は型ヒントを信頼してよい

---

## ✅ Task 1 完了判定

- [x] JoinIR JSON 最小構造を特定
- [x] MIR JSON v0/v1 最小構造を特定
- [x] 代表的なJSONスニペットを3件抽出（simple if, min loop, skip_ws）
- [x] .hako Analyzer が読むべきキー一覧を作成
- [x] JoinIR vs MIR 優先順位の判定（MIR v1推奨）
- [x] PHI/Loop/If の識別方法を明確化
- [x] 型情報の読み取り方法を文書化
- [x] 関数選定基準を確立
- [x] Phase 161 実装チェックリストを作成

---

**次のステップ**: Phase 161-2 (Task 2) - .hako 側 AnalyzerBox の基本実装
Status: Historical
