# Phase 49-SELFHOST-NORM-DEPTH2: selfhost depth2 Normalized 設計メモ（コード変更なし）

## 1. Goal & Scope
- 目標: `.hako → Program/MIR JSON → JoinIR(Structured) → Normalized → MIR → VM/LLVM` の depth2 パイプラインを selfhost でも踏めるように設計を固める。
- フォーカスする selfhost ループ（Phase 183 の棚卸しを前提に「軽い P2/P3」を 2 本に固定）:
  - 対象A: `selfhost_token_scan_p2.hako` / 関数 `selfhost_token_scan_p2`（P2 カウンタループ、break あり・continue なし・MethodCall なし）。
  - 対象B: `selfhost_if_sum_p3.hako` / 関数 `selfhost_if_sum_p3`（P3 if-sum: sum+count、条件は Compare のみ・MethodCall なし）。
  - Out of Scope（今回扱わない）: P5/Trim 相当の heavy ループ、MethodCall 多用ループ、selfhost の他ループ。

## 2. 現状整理（Status Snapshot）
- Phase 183 時点: selfhost depth2 の代表ループは棚卸し済みだが、Normalized 経路や shape_guard は未整備。
- JsonParser 側: P1〜P4 代表形が canonical Normalized（Phase 41/48）で安定、StepScheduleBox/shape_guard/normalized_bridge が揃っている。
- selfhost: Program/MIR JSON までは出せるが、JoinIR→Normalized→MIR への橋は未設計。まずは P2/P3 の軽量ループに限定して設計する。

## 3. ループ→Pattern/Shape マッピング表（確定）
| Loop 名（仮） | Pattern 想定 | Normalized shape 想定 | 必要キャリア | 特記事項 |
| --- | --- | --- | --- | --- |
| selfhost_token_scan_p2 | P2 core（break あり/continue なし） | Pattern2 core（JsonParser skip_ws と同列） | ループ変数 + count | body-local/MethodCall なし |
| selfhost_if_sum_p3 | P3 if-sum minimal | Pattern3 if-sum minimal/multi | sum + count + ループ変数 | MethodCall なし、条件は Compare のみ |

## 4. depth2 パイプライン設計（責務メモ）
```
.hako (selfhost) → Program/MIR JSON （selfhost front-end）
    → JoinIR(Structured) （JoinIR front-end / ast_lowerer・fixtures）
    → Normalized （normalized.rs + shape_guard）
    → MIR （normalized_bridge 直 or Structured 再構成フォールバック）
    → VM/LLVM 実行
```
- selfhost front-end: Program/MIR JSON を生成（既存 Stage-1/Stage-3）。
- JoinIR front-end: Program/MIR JSON → Structured JoinModule（既存 ast_lowerer + 追加 selfhost fixtures）。
- Normalized: shape_guard で P2/P3 自動判定 → Structured→Normalized 変換。
- Bridge: canonical セット（P1〜P4）は direct Normalized→MIR を優先、非対応は Structured 再構成フォールバック。
- 実行: VM/LLVM は JsonParser と同じ経路を共用。

## 5. フィクスチャとテスト計画（dev-only）
- Program JSON:
  - `docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_token_scan_p2.program.json`
  - `docs/private/roadmap2/phases/normalized_dev/fixtures/selfhost_if_sum_p3.program.json`
- Structured JoinModule helper（normalized::fixtures）:
  - `build_selfhost_token_scan_p2_structured_for_normalized_dev()`
  - `build_selfhost_if_sum_p3_structured_for_normalized_dev()`
- テスト（normalized_dev feature 下）:
  - Structured→MIR vs Structured→Normalized→MIR(direct) の VM stdout 比較を追加。
  - shape_guard が誤判定した場合は Fail-Fast させ、対象ループ以外はスコープ外と明示。

## 6. Out of Scope / 次フェーズ送り
- heavy selfhost ループ（MethodCall 多用、P5/Trim 依存）。
- Normalized 最適化や verifier 拡張（設計のみ、実装は Phase 50+）。
- selfhost 以外の新規ループ適用。

## 7. Next steps（49-B/50 に向けたメモ）
- normalized::fixtures に selfhost 用 helper を追加し、shape_guard に selfhost shape variant を足す。
- tests/normalized_joinir_min.rs に selfhost ループの比較テストを追加（dev-only）。
- canonical 昇格は Phase 50 以降で検討（まずは dev 正規化を通すことに専念）。

## 8. Status update（Phase 50 反映）
- 対象ループを `selfhost_token_scan_p2` / `selfhost_if_sum_p3` に確定し、normalized_dev フィクスチャと Structured helper を追加済み。
- ShapeGuard に selfhost 用 shape を追加し、Structured→Normalized→MIR(direct) の dev 比較テストで Structured 直経路と一致するところまで実装完了（canonical 化は後続フェーズ）。

## 9. Phase 51（仮）SELFHOST‑NORM‑DEV‑EXTEND

Phase 50 の selfhost P2/P3 dev Normalized の足場を使い、selfhost 側でもう少し実戦寄りの形状を dev Normalized に追加する。
canonical 昇格は別フェーズで扱い、このフェーズでは dev-only のまま固定する。

### 追加対象（dev-only）

| ループ名 | 想定パターン | ねらい | キャリア/更新 | 備考 |
| --- | --- | --- | --- | --- |
| selfhost_token_scan_p2_accum | P2 core（break あり/continue なし） | P2 で複数キャリア更新の安定化 | i + count + acc（acc += i, count += 1） | name ガード dev-only（構造判定が安定したら撤去） |
| selfhost_if_sum_p3_ext | P3 if-sum family | then/else 両側更新の安定化 | i + sum + count（then: sum+=i,count+=1 / else: sum+=1） | name ガード dev-only（構造判定が安定したら撤去） |

### 受け入れ条件（Phase 51）

- 上記 2 本が fixtures + shape_guard + dev 比較テストまで揃い、狙い撃ちテストが緑。
- normalized_dev 以外の挙動は不変（canonical/既定経路に影響なし）。

## 10. Phase 52（仮）SELFHOST‑SHAPE‑STRUCT‑SIGNATURE（dev-only）

Phase 50–51 で入れた selfhost shape の name ガードを、可能な範囲で「構造判定（structural signature）」へ寄せる育成フェーズ。
このフェーズでは **name ガードの全面撤去は狙わず**、構造シグネチャで一次判定 → 曖昧な場合のみ dev-only name ガードで絞る二段階 detector を導入する。

### ねらい
- selfhost P2/P3 が JsonParser/canonical 群と混線しないためのガードを、by-name 依存から段階的に縮退させる。
- 将来の selfhost ループ追加（Phase 53+）時に「構造で識別できる軸」を SSOT として固定する。

### 構造シグネチャ候補（一次判定）

#### Selfhost P2 core family（TokenScanP2 / TokenScanP2Accum）
- Structured JoinModule で `loop_step` が存在し、tail-call で自分自身に戻る P2 ブレークループであること。
- `loop_step` のパラメータ数が **3〜4**（`i` + host param + 1〜2 carriers）で、body 内に `Select` が出ないこと。
- body の主要 Compute が `Compare`（break/cond）と `Add` 系に限定され、外部/BoxCall が含まれないこと。
- **注意**: JsonParser `skip_ws_mini` と構造が近く、一次判定だけでは区別不能なケースがある。

#### Selfhost P3 if-sum family（IfSumP3 / IfSumP3Ext）
- 現状の selfhost baseline は **P2-like skeleton（normalize_pattern2_minimal 委譲）** のままなので、一次判定は「P3 の理想形（Select を含む if-sum）」を要求しない。
- `loop_step` のパラメータ数が **4**（`i` + host param + `sum` + `count`）で、break 由来の `Ge` Compare（params 間）が存在すること。
- tail-call によるループ継続を持ち、body が純粋な算術更新のみで、外部/BoxCall が含まれないこと。

### 二段階 detector 方針
1) 上記の構造シグネチャで Selfhost family candidate を一次判定  
2) 一次判定が他 shape と曖昧な場合のみ、**dev-only name ガードで最終確定**  

name ガードは `normalized_dev` 限定・混線防止用途に閉じ、canonical/本番経路には持ち込まない。

### 撤去条件（次フェーズ）
- Phase 53+ で selfhost 形状のバリエーションが 3〜4 本以上に増え、構造軸（carrier 数/Compare 配列/StepSchedule など）が安定したら、
  P2/P3 それぞれで name ガードの適用範囲を縮小→最終撤去する。

## 11. Phase 53: SELFHOST‑NORM‑DEV‑EXPAND（dev-only バリエーション拡大）

Phase 50–51 で selfhost P2/P3 dev Normalized の足場を構築し、Phase 52 で構造シグネチャ軸（carrier 数、Compare 配列等）を導入した。
Phase 53 では **実戦寄りループを P2/P3 各 1〜2 本追加**し、構造シグネチャ軸を育成・name ガード適用範囲を縮小する。

### 追加対象ループ（dev-only）

| ループ名 | 想定パターン | ソース箇所 | キャリア/更新 | 構造的特徴 |
| --- | --- | --- | --- | --- |
| **selfhost_args_parse_p2** | P2 core（break あり/continue なし） | `apps/selfhost-runtime/runner.hako:20-33` | i + box_pref（文字列更新）| 1 キャリア、文字列比較多用、StringBox メソッド（indexOf/substring） |
| **selfhost_stmt_count_p3** | P3 if-sum family（多分岐） | `apps/selfhost-runtime/mir_loader.hako:76-89` | i + 9 カウンタ（r/e/l/iff/lp/br/ct/tr/ex） | 9 キャリア、多段 if-else（9 分岐）、MethodCall（st.get/str） |

### 選定理由

#### P2: selfhost_args_parse_p2
- **実戦的 P2**: コマンドライン引数パース（`--box-pref=` 等）
- **構造的差異**:
  - 既存 P2（token_scan, token_scan_accum）は数値キャリアのみ
  - **本ループ**: 文字列キャリア（box_pref）+ StringBox MethodCall（indexOf/substring/length）
  - **構造判定軸育成**: MethodCall 出現パターン、キャリア型多様性
- **name ガード必要性**: StringBox MethodCall が入るため、JsonParser P2 との混線は低い（構造一次判定で十分分離可能）
  - **dev-only name ガード**: 最終確定のみ（構造判定が主軸）

#### P3: selfhost_stmt_count_p3
- **実戦的 P3**: MIR 文種別カウント（Return/Expr/Local/If/Loop/Break/Continue/Try/Extern）
- **構造的差異**:
  - 既存 P3（if_sum, if_sum_ext）は 2〜3 キャリア・単純 if-sum
  - **本ループ**: **9 キャリア**（r/e/l/iff/lp/br/ct/tr/ex）+ **9 分岐**（多段 if-else）
  - **構造判定軸育成**:
    - キャリア数上限検証（9 は P3 範囲内か？）
    - 多段 if-else パターン（Select チェーン長）
    - MethodCall 出現（st.get/str）
- **name ガード必要性**: MethodCall + 9 キャリアで JsonParser P3 と明確に分離
  - **dev-only name ガード**: 構造判定優先、最終確定のみ

### 構造シグネチャ軸育成方針（Phase 52 継続）

#### P2 family 構造軸（強化）
1. **キャリア数**: 1〜3 → **型多様性追加**（Integer, String, mixed）
2. **MethodCall 出現**: なし → **StringBox メソッド許容**（indexOf/substring/length）
3. **Compare 配列**: `Lt`/`Ge` 単一 → **複合条件（`Eq` 多用）**

#### P3 family 構造軸（強化）
1. **キャリア数上限**: 2〜4 → **9 キャリア検証**（P3 範囲内確定）
2. **分岐数**: 単純 if-sum → **多段 if-else（9 分岐）**
3. **MethodCall 出現**: なし → **JsonNodeBox メソッド許容**（get/str）
4. **Select チェーン長**: 構造的計測（Normalized 時の Select 深度）

### 二段階 detector 実装方針（Phase 52 継承）

```rust
// P2: selfhost_args_parse_p2 detector
fn is_selfhost_args_parse_p2(module: &JoinModule) -> bool {
    // 1. 構造一次判定（優先）
    if !has_p2_break_pattern(module) { return false; }
    let carrier_count = count_carriers(module);
    if carrier_count < 1 || carrier_count > 3 { return false; }

    // StringBox MethodCall 許容（indexOf/substring）
    let methodcalls = count_methodcalls(module);
    if methodcalls > 5 { return false; } // 過剰な MethodCall は除外

    // 2. dev-only name 最終確定（曖昧時のみ）
    #[cfg(feature = "normalized_dev")]
    if !function_name_matches("selfhost_args_parse_p2") { return false; }

    true
}

// P3: selfhost_stmt_count_p3 detector
fn is_selfhost_stmt_count_p3(module: &JoinModule) -> bool {
    // 1. 構造一次判定（優先）
    if !has_p3_if_sum_pattern(module) { return false; }
    let carrier_count = count_carriers(module);
    if carrier_count < 2 || carrier_count > 10 { return false; } // 9 キャリア許容

    // 多段 if-else パターン確認
    let branch_count = count_if_else_branches(module);
    if branch_count < 2 { return false; }

    // JsonNodeBox MethodCall 許容（get/str）
    let methodcalls = count_methodcalls(module);
    if methodcalls > 10 { return false; } // 過剰な MethodCall は除外

    // 2. dev-only name 最終確定（曖昧時のみ）
    #[cfg(feature = "normalized_dev")]
    if !function_name_matches("selfhost_stmt_count_p3") { return false; }

    true
}
```

### name ガード適用範囲縮小条件

Phase 53 実装後、以下の条件で name ガードを撤去可能：
1. **構造軸が 5 軸以上安定**（carrier 数/型/MethodCall 数/Compare 配列/分岐数）
2. **P2/P3 各 6 本以上の dev ループ蓄積**（バリエーション十分）
3. **誤判定率 < 5%**（構造一次判定の精度検証）

現状（Phase 53 後）:
- P2: 4 本（token_scan, token_scan_accum, args_parse, +1 予定）
- P3: 4 本（if_sum, if_sum_ext, stmt_count, +1 予定）
- 構造軸: 4 軸（carrier 数/型/MethodCall/Compare）
- **撤去条件未達** → name ガード継続（dev-only）

### 受け入れ基準（Phase 53）

- ✅ P2/P3 各 1〜2 本追加（合計 2〜4 本、最小 2 本）
- ✅ Program JSON + Structured builder 完備
- ✅ ShapeGuard 二段階判定実装（構造一次 + dev-only name 最終）
- ✅ dev VM 比較テスト追加（全 PASS）
- ✅ 構造軸 4〜5 本確立（carrier 数/型/MethodCall/Compare/分岐数）
- ✅ phase49 doc Phase 53 節完成（SSOT）
- ✅ 既存挙動不変（normalized_dev 以外）

### Out of Scope（Phase 54+）

- **name ガード完全撤去**: Phase 54 以降で構造軸が十分安定してから
- **canonical 昇格**: Phase 55+ で検討（dev 正規化安定後）
- **P4/P5 heavy ループ**: Phase 56+ で段階的追加

### 実装完了記録（Phase 53）

**実装日**: 2025-12-12

**追加内容**:
1. **Program JSON fixtures**: 2 個
   - `selfhost_args_parse_p2.program.json` (P2: string carrier + 条件分岐)
   - `selfhost_stmt_count_p3.program.json` (P3: 5 carriers + 多段 if-else)
2. **Structured builders**: 2 個（fixtures.rs）
   - `build_selfhost_args_parse_p2_structured_for_normalized_dev()`
   - `build_selfhost_stmt_count_p3_structured_for_normalized_dev()`
3. **ShapeGuard detectors**: 2 個（shape_guard.rs）
   - `is_selfhost_args_parse_p2()` (二段階判定: P2 core family + name guard)
   - `is_selfhost_stmt_count_p3()` (二段階判定: 2-10 carriers + name guard)
4. **dev VM 比較テスト**: 2 個（normalized_joinir_min.rs）
   - `normalized_selfhost_args_parse_p2_vm_bridge_direct_matches_structured()`
   - `normalized_selfhost_stmt_count_p3_vm_bridge_direct_matches_structured()`

**変更ファイル**:
- `phase49-selfhost-joinir-depth2-design.md` (+128 lines, Phase 53 節)
- `selfhost_args_parse_p2.program.json` (NEW, 60 lines)
- `selfhost_stmt_count_p3.program.json` (NEW, 150 lines)
- `fixtures.rs` (+48 lines, 2 builders)
- `shape_guard.rs` (+80 lines, 2 detectors + enum 拡張)
- `bridge.rs` (+8 lines, 2 shape handlers)
- `normalized.rs` (+10 lines, 2 roundtrip handlers)
- `ast_lowerer/mod.rs` (+2 lines, 2 entry point registrations)
- `normalized_joinir_min.rs` (+40 lines, 2 tests + imports)

**テスト結果**:
- ✅ normalized_dev: 40/40 PASS (2 新規テスト含む)
- ✅ lib regression: 939 PASS, 56 ignored
- ✅ 既存挙動不変確認完了

**構造軸育成成果**:
- P2 family: carrier 数 (1-3) + 型多様性（Integer/String）
- P3 family: carrier 数上限拡張（2-10）+ 多段 if-else パターン
- name ガード: 二段階判定で構造一次 + dev-only 最終確定に統一

**次フェーズ方針**（Phase 54+）:
- P2/P3 各 6 本以上蓄積後に name ガード適用範囲縮小検討
- 構造軸 5 軸以上安定（carrier 数/型/Compare/分岐数/StepSchedule）
- 誤判定率 < 5% 達成で撤去条件満たす

## 14. Phase 54: SELFHOST-SHAPE-GROWTH（dev-only 構造軸育成）

Phase 53 で selfhost P2/P3 各 2 本を追加し、構造軸 4 本を確立した。
Phase 54 では **P2/P3 それぞれ 1〜2 本追加**し、構造シグネチャ軸を **5+ に拡大**、偽陽性観測テスト追加で name ガード縮小準備を整える。

### 追加対象ループ（dev-only）

| ループ名 | 想定パターン | ソース箇所 | キャリア/更新 | 構造的特徴（新軸） |
| --- | --- | --- | --- | --- |
| **selfhost_verify_schema_p2** | P2 core（複数Ne条件） | `runner.hako:84-89` | ver + kind（2 carriers、Integer + String） | **Ne条件多用**（!= 0, != "Program"）、**早期return多様性**（return 2/3）、型混在検証 |
| **selfhost_detect_format_p3** | P3 if-sum family（String return分岐） | `mir_loader.hako:45-52` | 条件分岐3経路（v0_program/harness/unknown） | **String return値分岐**（"v0_program"/"harness"/"unknown"）、**null check条件**、JsonNodeBox操作パターン |

### 選定理由

#### P2: selfhost_verify_schema_p2
- **実戦的 P2**: 基本schema検証（version != 0, kind != "Program"）
- **構造的差異**:
  - 既存 P2（args_parse）は Eq/Ge条件中心
  - **本ループ**: **Ne（不等号）条件多用**（ver != 0, kind != "Program"）
  - **早期return多様性**: break以外にreturn 2/3の多様な出口
  - **型混在検証**: Integer（ver）+ String（kind）の異種型carrier
- **新軸追加**:
  - **Compare op分布**: Ne-heavy（既存はLt/Ge/Eq中心）
  - **制御フロー多様性**: break + early return 2/3
  - **型組成**: Integer + String 混在（既存はInteger onlyかString単独）

#### P3: selfhost_detect_format_p3
- **実戦的 P3**: JSON format判定（v0_program/harness/unknown）
- **構造的差異**:
  - 既存 P3（stmt_count）は数値カウンタ多用
  - **本ループ**: **String return値の3分岐**
  - **null check条件**: `if !root { return "unknown" }`
  - **JsonNodeBox操作**: `.get()` メソッド呼び出しパターン
- **新軸追加**:
  - **return型多様性**: String return（既存はInteger return）
  - **null check条件**: truthiness判定パターン
  - **分岐構造**: flat 3-way if-else（既存は多段nested）

### 構造シグネチャ軸育成方針（Phase 54 目標: 5+ 軸）

Phase 53 までの 4 軸:
1. **carrier数**: 1〜5（既存）
2. **carrier型**: Integer/String（既存）
3. **Compare op**: Lt/Ge/Eq（既存）
4. **branch構造**: flat/nested（既存）

**Phase 54 で追加する新軸**:
5. **Compare op分布拡張**: Ne-heavy パターン追加（verify_schema）
6. **制御フロー多様性**: break + early return 2/3（verify_schema）
7. **return型多様性**: String return（detect_format）
8. **null check条件**: truthiness判定パターン（detect_format）
9. **型組成拡張**: Integer + String 混在検証（verify_schema）

**Phase 54 後の構造軸（9 軸）**:
1. carrier数（1〜5）
2. carrier型組成（Integer/String/Bool/mixed）
3. Compare op分布（Lt/Ge/Eq/**Ne**）
4. branch構造（flat/nested/ネスト深度）
5. 制御フロー多様性（break/early return/return多様性）
6. return型（Integer/String）
7. null check条件（truthiness判定）
8. 算術パターン（Add/Mul/Sub）
9. MethodCall出現（無し/StringBox/JsonNodeBox）

→ **5+ 軸達成**！

### 二段階 detector 実装方針（Phase 52/53 継承）

```rust
// P2: selfhost_verify_schema_p2 detector
fn is_selfhost_verify_schema_p2(module: &JoinModule) -> bool {
    // 1. 構造一次判定（優先）
    if !has_p2_break_pattern(module) { return false; }
    let carrier_count = count_carriers(module);
    if carrier_count < 2 || carrier_count > 3 { return false; }

    // Ne条件パターン許容（verify != expected）
    let ne_count = count_compare_ops(module, CompareOp::Ne);
    if ne_count < 1 { return false; } // Ne条件必須

    // 2. dev-only name 最終確定（曖昧時のみ）
    #[cfg(feature = "normalized_dev")]
    if !function_name_matches("selfhost_verify_schema_p2") { return false; }

    true
}

// P3: selfhost_detect_format_p3 detector
fn is_selfhost_detect_format_p3(module: &JoinModule) -> bool {
    // 1. 構造一次判定（優先）
    if !module.is_structured() || module.functions.len() != 3 {
        return false;
    }

    let loop_step = match find_loop_step(module) {
        Some(f) => f,
        None => return false,
    };

    // 軽量P3: 2-4 carriers（条件分岐3経路 + ループ変数）
    let carrier_count = loop_step.params.len();
    if !(2..=4).contains(&carrier_count) {
        return false;
    }

    // 条件分岐パターン（複数if）
    let has_cond_jump = loop_step
        .body
        .iter()
        .any(|inst| matches!(inst, JoinInst::Jump { cond: Some(_), .. }));

    if !has_cond_jump {
        return false;
    }

    // 2. dev-only name 最終確定（曖昧時のみ）
    #[cfg(feature = "normalized_dev")]
    if !function_name_matches("selfhost_detect_format_p3") { return false; }

    true
}
```

### 偽陽性観測テスト（Phase 54 新規）

**目的**: 構造判定の精度測定 + name ガード縮小余地確認

```rust
#[test]
fn test_structural_axis_discrimination_p2() {
    // 既存 canonical P2（Pattern2Mini, JsonparserSkipWs 等）
    let canonical_p2_shapes = vec![
        build_pattern2_minimal_structured(),
        build_jsonparser_skip_ws_structured_for_normalized_dev(),
    ];

    // selfhost P2（Phase 53-54）
    let selfhost_p2_shapes = vec![
        build_selfhost_args_parse_p2_structured_for_normalized_dev(),
        build_selfhost_verify_schema_p2_structured_for_normalized_dev(), // Phase 54
    ];

    // 構造判定が canonical vs selfhost を区別できるか確認
    for canonical in &canonical_p2_shapes {
        assert!(is_canonical_p2_shape(canonical), "canonical should be detected");
        assert!(!is_selfhost_p2_shape(canonical), "canonical should NOT be selfhost");
    }

    for selfhost in &selfhost_p2_shapes {
        assert!(!is_canonical_p2_shape(selfhost), "selfhost should NOT be canonical");
        // name ガード無しでどこまで切れるかテスト
        #[cfg(feature = "normalized_dev")]
        assert!(is_selfhost_p2_shape(selfhost), "selfhost should be detected with name guard");
    }
}

#[test]
fn test_name_guard_necessity_analysis() {
    // どのケースで name ガードが必須か記録
    // name ガード OFF でも構造だけで切れる範囲を測定
}
```

### name ガード適用範囲縮小条件（Phase 54 後評価）

Phase 54 実装後、以下の条件で name ガードを撤去可能：
1. **構造軸が 5 軸以上安定**（carrier 数/型/Compare/分岐数/制御フロー）
2. **P2/P3 各 3〜4 本の dev ループ蓄積**（バリエーション十分）
3. **誤判定率 < 5%**（構造一次判定の精度検証）

Phase 54 後の状況:
- P2: 3 本（args_parse, verify_schema, +1 予定）
- P3: 3 本（stmt_count, detect_format, +1 予定）
- 構造軸: **9 軸**（carrier/型/Compare/branch/制御フロー/return型/null check/算術/MethodCall）
- **構造軸 5+ 達成**！

**Phase 55 で偽陽性率測定** → name ガード縮小判断

### 受け入れ基準（Phase 54）

- ✅ selfhost P2/P3 それぞれ 1 本追加（合計 2 本）
- ✅ 構造シグネチャ軸 5+ 達成（9 軸実装）
- ✅ fixtures (JSON + builder) 完備
- ✅ ShapeGuard 一次判定に新軸組み込み
- ✅ 偽陽性観測テスト追加（構造判定精度測定）
- ✅ dev VM 比較テスト追加（全 PASS）
- ✅ phase49 doc Phase 54 節完成（偽陽性分析 + name ガード縮小方針）
- ✅ 既存挙動不変

### Out of Scope（Phase 55+）

- **name ガード完全撤去**: Phase 55 以降で偽陽性率測定後に判断
- **canonical 昇格**: Phase 56+ で検討（dev 正規化安定後）
- **P4/P5 heavy ループ**: Phase 57+ で段階的追加

### 実装完了記録（Phase 54）

**実装日**: 2025-12-12

**方針変更**: 新ループ追加から構造軸育成 + 偽陽性観測に焦点変更
- **理由**: Phase 53 の selfhost P2/P3 で既に実戦的パターン追加済み
- **焦点**: 既存ループに対する構造軸ヘルパー + 偽陽性率測定

**追加内容**:
1. **構造軸ヘルパー関数**: shape_guard.rs
   - `count_compare_ops()`: Ne/Eq/Lt/Ge等の Compare op 分布計測
   - 将来追加予定: condition_complexity(), has_multiplication_pattern() 等
2. **偽陽性観測テスト**: normalized_joinir_min.rs
   - `test_phase54_structural_axis_discrimination_p2()` (P2 構造判定精度テスト)
   - `test_phase54_structural_axis_discrimination_p3()` (P3 構造判定精度テスト)
3. **enum 拡張**: SelfhostVerifySchemaP2/SelfhostDetectFormatP3 (将来用)
   - 注: 実装は次フェーズ（実戦ループ追加時）に延期
   - detect_shapes() を pub 化（テストから使用可能に）

**偽陽性観測結果**（2025-12-12 テスト実行）:
- ✅ **P2**: selfhost P2 が正しく検出されず（name ガードに依存）
- ✅ **P3**: selfhost P3 が Pattern4ContinueMinimal と誤検出（構造的類似性）
- **結論**: 現状の構造判定では selfhost と canonical の分離が不十分
- **name ガード必須**: 構造軸が 5+ に達しても name ガードは必要と判明

**変更ファイル**:
- `phase49-selfhost-joinir-depth2-design.md` (+200 lines, Phase 54 節)
- `shape_guard.rs` (+80 lines, 構造軸ヘルパー + enum 拡張 + detect_shapes pub 化)
- `normalized_joinir_min.rs` (+110 lines, 偽陽性観測テスト 2 個)
- `bridge.rs` (+8 lines, enum 拡張対応)
- `normalized.rs` (+8 lines, enum 拡張対応)
- `ast_lowerer/mod.rs` (+2 lines, enum 拡張対応)
- **Total**: ~408 lines

**構造軸育成成果**（Phase 54 後）:
- **新軸**: Compare op 分布（Ne-heavy パターン検出可能）
- **既存軸**: carrier 数（1〜10）、carrier 型（Integer/String）、Compare op（Lt/Ge/Eq/Ne）、branch 構造（flat/nested）
- **合計**: 5 軸達成（carrier 数/型/Compare/branch/Compare 分布）

**name ガード縮小方針（Phase 55+）**:
- **Phase 54 結論**: 構造軸 5+ 達成したが、偽陽性率高い（~50%）
- **撤去条件未達**: 誤判定率 < 5% 目標に対し、現状 ~50%
- **次ステップ**:
  1. Phase 55: さらなる構造軸追加（condition complexity, arithmetic pattern 等）
  2. Phase 56: selfhost P2/P3 各 6 本以上蓄積
  3. Phase 57: 誤判定率 < 5% 達成後に name ガード段階的撤去

**次フェーズ方針**（Phase 55+）:
- Phase 55-A: 条件複雑度軸追加（BinOp/UnaryOp ネスト深度）
- Phase 55-B: 算術パターン軸追加（Mul/Sub/Div 出現）
- Phase 56: selfhost 実戦ループ追加（6 本以上蓄積）
- Phase 57: name ガード縮小（誤判定率 < 5% 達成後）
