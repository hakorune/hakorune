# Phase 71-SSA: trim系 SSA undef 完全解消レポート

**実施日**: 2025-12-02
**担当**: Claude + Task先生

---

## 🎉 エグゼクティブサマリー

**SSA undef 完全解消達成！**
- **修正前**: 4件の SSA undef
- **修正後**: **0件** (100%解消！)

**修正内容**:
1. ParserStringUtilsBox.trim - `StringHelpers.skip_ws` → `ParserStringUtilsBox.skip_ws` (Quick Win)
2. ParserCommonUtilsBox.trim - 委譲を廃止、直接実装に変更
3. ParserBox.trim - 委譲を廃止、直接実装に変更

**所要時間**: 約2時間 (調査 + 実装 + 検証)

---

## 📊 SSA undef 推移

| フェーズ | SSA undef件数 | 詳細 |
|---------|--------------|------|
| Phase 71初回観測 | **4件** | ParserCommonUtilsBox.trim/1, ParserBox.trim/1, Main._parse_number/1, ParserBox.parse_block2/2 |
| Quick Win修正後 | **2件** | ParserCommonUtilsBox.trim/1, ParserBox.trim/1 (予想外の効果: _parse_number/parse_block2 消滅) |
| 修正案A実装後 | **0件** | 🎉 完全解消！ |

---

## 🔍 根本原因の特定 (Task先生による分析)

### ValueId(272)の正体
- **`StringHelpers.starts_with_kw/3`の戻り値**
- ParserCommonUtilsBox.trim/1 の**引数として誤って参照**されていた

### 問題の本質
**static boxの委譲でValueIdマッピング失敗**:
```hako
// ParserCommonUtilsBox.trim (問題あり)
trim(s) {
    return ParserStringUtilsBox.trim(s)  // ← 引数sのValueIdマッピングが失敗
}
```

**ログ証拠**:
```
[ssa-undef-debug] fn=ParserCommonUtilsBox.trim/1 bb=BasicBlockId(800) inst_idx=0
                  used=ValueId(272) inst=Copy { dst: ValueId(2), src: ValueId(272) }
```

**注目点**: 引数パラメータ設定ログが一切出力されていない

---

## 🛠️ 修正内容の詳細

### 修正1: ParserStringUtilsBox.trim (Quick Win)

**ファイル**: `lang/src/compiler/parser/scan/parser_string_utils_box.hako`

**変更箇所** (L76):
```hako
// 修正前
local b = StringHelpers.skip_ws(str, 0)  // インスタンスメソッド呼び出し

// 修正後
local b = ParserStringUtilsBox.skip_ws(str, 0)  // 静的呼び出し
```

**効果**:
- SSA undef 4件 → 2件
- **予想外の副次効果**: Main._parse_number と ParserBox.parse_block2 が消滅

### 修正2: ParserCommonUtilsBox.trim (修正案A)

**ファイル**: `lang/src/compiler/parser/scan/parser_common_utils_box.hako`

**変更内容** (L50-69):
```hako
// 修正前 (委譲パターン)
trim(s) {
    // Delegate to string_utils to keep SSA/semantic consistency.
    return ParserStringUtilsBox.trim(s)
}

// 修正後 (直接実装)
trim(s) {
    // Phase 71-SSA: Direct implementation to avoid ValueId mapping issue in static box delegation
    if s == null { return "" }
    local str = "" + s
    local n = str.length()

    // Leading whitespace: use static method to avoid SSA undef
    local b = ParserStringUtilsBox.skip_ws(str, 0)
    if b >= n { return "" }

    // Trailing whitespace: walk backwards until a non-space is found.
    local e = n
    loop(e > b) {
      local ch = str.substring(e - 1, e)
      if ch == " " || ch == "\t" || ch == "\n" || ch == "\r" || ch == ";" { e = e - 1 } else { break }
    }

    if e > b { return str.substring(b, e) }
    return ""
}
```

### 修正3: ParserBox.trim (修正案A)

**ファイル**: `lang/src/compiler/parser/parser_box.hako`

**変更内容** (L81-98):
- ParserCommonUtilsBox.trim と同じパターンで直接実装に変更

---

## ✅ 成功パターンとの比較

### FuncScannerBox.trim (成功パターン)
- **box宣言** (通常のbox)
- **methodキーワード**使用
- **同じbox内のメソッド**呼び出し
- **二重委譲なし**
- ✅ SSA undef なし

### ParserStringUtilsBox.trim (失敗パターン→成功)
- **static box宣言**
- **methodキーワードなし**
- **別boxの関数**呼び出し (修正後は同じbox)
- **二重委譲あり** (skip_ws → StringHelpers.skip_ws)
- ✅ 修正後 SSA undef なし

### ParserCommonUtilsBox.trim (失敗パターン→成功)
- **static box宣言**
- **methodキーワードなし**
- **委譲パターン** (修正前)
- ✅ 修正後: 直接実装で SSA undef 解消

---

## 📈 検証結果

### 最終確認
```bash
NYASH_FEATURES=stage3 NYASH_USE_NY_COMPILER=1 NYASH_NY_COMPILER_EMIT_ONLY=1 \
NYASH_SELFHOST_KEEP_RAW=1 ./tools/selfhost/selfhost_build.sh \
--in apps/tests/stage1_run_min.hako --run
```

**結果**:
```
[warn] dev verify: NewBox StageBDriverBox at v%366 not followed by birth() call
[warn] dev verify: NewBox→birth invariant warnings: 1
[selfhost/debug] RAW log: .../stageb_20251202_111409_2674670.log

SSA undef 件数: 0 ← 🎉 完全解消！
```

**RAWログ確認**:
```bash
grep -c 'ssa-undef-debug' logs/selfhost/stageb_20251202_111409_2674670.log
# 出力: 0
```

---

## 🎯 残存課題

### 1. dev verify警告 (1件)
```
[warn] dev verify: NewBox StageBDriverBox at v%366 not followed by birth() call
```
- **対象**: StageBDriverBox
- **対応**: Phase 71-SSA次フェーズで実施

### 2. Program JSON未出力
```
[selfhost/raw] rc_stageb=0 extract_ok=0
```
- **状況**: Stage-B rc=0 (実行成功) だが Program JSON 行なし
- **原因**: SSA undef 以外の問題 (dev verify? または別の要因)
- **対応**: 次フェーズでトレース追加して調査

---

## 💡 重要な教訓

### 1. static boxの委譲は危険
- **ValueIdマッピング**が正しく行われない可能性
- 引数パラメータ設定ログが出ないケースあり
- **推奨**: 委譲を避け、直接実装を選択

### 2. 静的呼び出しの重要性
- `BoxName.method()` → SSA-friendly
- `using経由のBox.method()` → インスタンスメソッド呼び出しのリスク
- **推奨**: 同じbox内のメソッドを静的呼び出し

### 3. 成功パターンの活用
- FuncScannerBox.trim の実装パターンを参考
- 既存の成功事例を積極的に再利用

---

## 📝 次のアクション

### Phase 71-SSA次フェーズ
1. **StageBDriverBox birth警告解消** (1件 → 0件)
2. **Program JSON emit調査**
   - StageBDriverBox.main にトレース追加
   - RAWログで出口まで到達しているか確認
3. **最終目標**: `selfhost_build + stage1_run_min.hako` で Program JSON emit成功

---

## 🤖 AI協働成果

### Task先生の貢献
- ✅ ValueId(272) の正体特定
- ✅ static box委譲問題の発見
- ✅ 修正案A-C の提案
- ✅ FuncScannerBox.trim との比較分析

### Claude実装
- ✅ Quick Win (30分で2件削減)
- ✅ 修正案A実装 (委譲廃止で残り2件解消)
- ✅ 検証・ドキュメント化

**合計所要時間**: 約2時間 (Task先生調査含む)

---

## 📚 関連ドキュメント

- **Phase 71初回観測**: `phase71-findings-20251202.md`
- **Phase 71-SSA README**: `docs/private/roadmap2/phases/phase-71-ssa-debug/README.md`
- **Phase 71-SSA TASKS**: `docs/private/roadmap2/phases/phase-71-ssa-debug/TASKS.md`
- **CURRENT_TASK.md**: Phase 71-SSA進捗記録

---

**Phase 71-SSA SSA undef 削減完了判定**: ✅ **100%達成！**

**次のマイルストーン**: dev verify警告解消 + Program JSON emit復活
Status: Historical
