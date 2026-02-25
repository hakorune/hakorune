# Current Task (ARCHIVE)

このファイルは「いま何に集中しているか」と「次にやり得る候補」だけを書く軽量ビューだよ。  
詳細なログや過去フェーズの記録は `docs/development/current/main/` 以下の各 `phase-*.md` と  
`docs/development/current/main/joinir-architecture-overview.md` を真実のソースとして扱うよ。

---

## 🎯 今フォーカスしているテーマ（2025-12-12 時点のスナップショット）

### 0. ✅ Phase 43/245B Normalized JoinIR Infrastructure COMPLETE (Phase 26-45)

**完全サマリ**: [PHASE_43_245B_NORMALIZED_COMPLETION.md](docs/development/current/main/PHASE_43_245B_NORMALIZED_COMPLETION.md)

- **達成内容**:
  - Structured→Normalized→MIR(direct) パイプライン確立
  - P1/P2 + JsonParser (skip_ws/atoi/parse_number) 全対応
  - Mode system (Phase 45), Capability system (Phase 44) 完成
  - 937/937 tests PASS
- **主要コンポーネント**:
  - JoinIrMode enum (StructuredOnly/NormalizedDev/NormalizedCanonical)
  - ShapeCapabilityKind (P2CoreSimple/P2CoreSkipWs/P2CoreAtoi/P2MidParseNumber)
  - CarrierRole (LoopState/ConditionOnly), CarrierInit
  - DigitPos dual-value (is_digit_pos + digit_value)
  - NumberAccumulation, Step scheduling, Exit PHI & Jump args
- **今後の拡張候補**（計画のみ）:
  - Phase 46+: Canonical set 拡張 (capability-based)
  - Pattern3/4 Normalized 適用
  - Selfhost loops 対応

### 0-B. JoinIR / ExprLowerer / Pattern2–4 + JsonParser `_parse_number` / DigitPos ライン（Phase 230–247-EX 完了）✅

- Pattern1–4（while / break / if‑PHI / continue）＋ P5(Trim) でループ lowering を JoinIR 経路に一本化。
- Phase 231/236/240-EX:
  - Pattern2 の header / break 条件を ExprLowerer/ScopeManager 経由（対応パターンのみ）で本番導線化。
  - ConditionEnv ベースの意味論と RC/JoinIR 構造が一致することをテストとカタログで確認済み。
- Phase 237–238-EX:
  - JsonParser/selfhost の条件パターンをカタログ化し、ExprLowerer/ScopeManager/ConditionEnv の責務境界を docs/README で固定。
- Phase 241–242-EX:
  - array_filter 系 3 FAIL を「構造で」解消（hardcode `"sum"` チェック削除、CarrierInfo/ExitMeta 経由に統一）。
  - Pattern3 if‑sum / Pattern4 continue から legacy lowerer と by-name `"sum"`/`"count"` を完全削除。
  - 複雑条件（`i % 2 == 1`）を ConditionPattern/if-sum lowerer で安全に扱えるよう整理（Fail-Fast + テスト付き）。
- Phase 243–244-EX:
  - Pattern3/4 の公開 API を `can_lower + lower` の最小セットに整理し、内部 helper を箱の中に閉じた。
  - `loop_pattern_detection` の classify() が代表ループを P1〜P4 に分類することをユニットテストで固定。
- Phase 245-EX / 245C:
  - `_parse_number` のループについて、ループ変数 `p` の header/break 条件と `p = p + 1` 更新を Pattern2 + ExprLowerer 経路に載せて本番化。
  - FunctionScopeCapture / CapturedEnv を拡張し、関数パラメータ（例: `s`, `len`）をループ条件/本体で読み取り専用なら CapturedEnv 経由で ConditionEnv に載せるようにした。
  - これにより、`p < s.length()` のような header 条件や JsonParser 系ループでのパラメータ参照が、ExprLowerer/ScopeManager から安全に解決される。
- Phase 245B-IMPL:
  - `_parse_number` 本体で `num_str = num_str + ch` を LoopState キャリアとして扱い、Program(JSON) フィクスチャ `jsonparser_parse_number_real` を Structured→Normalized→MIR(direct) で dev テスト固定（出力は num_str 文字列）。
- Phase 247-EX:
  - DigitPos promotion を二重値化し、`digit_pos` から boolean carrier `is_digit_pos`（ConditionOnly）と integer carrier `digit_value`（LoopState）を生成。
  - UpdateEnv で `digit_pos` 解決時に `digit_value` を優先し、NumberAccumulation（`result = result * 10 + digit_pos`）と break 条件の両方で DigitPos パターンが安全に利用可能に。
- 現在: `cargo test --release --lib` で 931/931 テスト PASS（既知 FAIL なし）。
- Phase 28-NORM-P2（dev-only）:
  - Normalized JoinIR のミニ実装を Pattern1 に続き Pattern2 最小ケースまで拡張（Structured→Normalized→Structured を比較）。
  - 対応外の Structured JoinModule では normalize_pattern2_minimal が Fail-Fast するようガードを追加し、normalized_dev テストで固定。
- Phase 29-NORM-P2-APPLY（dev-only）:
  - Phase 34 の break fixture（i/acc/n の単純 break ループ）を Structured→Normalized→Structured の往復に通し、VM 実行結果が Structured 直経路と一致することを dev テストで固定。
  - ガードは 3 パラメータまで緩和しつつ、DigitPos/Trim などの重いキャリアはまだ非対応のまま。
- Phase 30-NORM-P2-DEV-RUN（dev-only）:
  - JoinIR runner に `NYASH_JOINIR_NORMALIZED_DEV_RUN=1` を追加し、Pattern1/2 ミニケースだけ Structured→Normalized→Structured を挟んで dev 実行できるようにした（`normalized_dev` + debug 限定）。通常経路（Structured→MIR）は不変。
- Phase 31-NORM-JP-MINI（dev-only）:
  - JsonParser 系のシンプルな P2 ループ（skip_whitespace ミニ fixture）を Structured→Normalized→Structured 経由でも実行し、runner dev スイッチの比較テストで Structured 直経路と一致することを固定。
- Phase 32-NORM-CANON-PREP（dev-only）:
  - JoinIR→MIR ブリッジの入口を `bridge_joinir_to_mir` に一本化し、normalized_dev スイッチ（feature + env）で Structured→Normalized→Structured の dev roundtrip を切り替える準備を整えた。P1/P2/JP mini の比較テストも VM ブリッジ経路で追加。
- Phase 33-NORM-CANON-TEST（dev-only）:
  - P1/P2（Phase 34 break fixture）/JsonParser skip_ws mini について、normalized_dev ON 時は shape_guard 経由で必ず Normalized roundtrip を通すようブリッジと runner を固めた。normalized_joinir_min.rs の runner/VM 比較テストを拡張し、Normalized が壊れたら dev スイートが必ず赤になるようにした（本番 CLI は従来どおり Structured→MIR）。
- Phase 34-NORM-ATOI-DEV（dev-only）:
  - JsonParser `_atoi` ミニループ（digit_pos→digit_value + NumberAccumulation）を normalized_dev 経路に載せ、Structured↔Normalized↔Structured の VM 実行結果が一致することをフィクスチャテストで固定。`jsonparser_atoi_mini` を shape_guard で認識し、既定経路は引き続き Structured→MIR のまま。
- Phase 35-NORM-BRIDGE-MINI（dev-only）:
  - P1/P2 ミニ + JsonParser skip_ws/atoi ミニを Normalized→MIR 直ブリッジで実行できるようにし、normalized_dev ON 時は Structured→Normalized→MIR（復元なし）経路との比較テストで結果一致を固定。既定経路（Structured→MIR）は不変。
- Phase 36-NORM-BRIDGE-DIRECT（dev-only）:
  - Normalized ブリッジを direct 実装（Normalized→MIR）と Structured 再構成に分離し、shape_guard で P1/P2 ミニ + JsonParser skip_ws/atoi ミニだけ direct 経路に通すよう整理。非対応は `[joinir/normalized-bridge/fallback]` ログ付きで再構成に落とし、テストで direct/従来経路の VM 出力一致を固定。
- Phase 37-NORM-JP-REAL（dev-only）:
  - JsonParser `_skip_whitespace` 本体の P2 ループを Program(JSON) フィクスチャで Structured→Normalized→MIR(direct) に通し、Structured 直経路との VM 出力一致を比較するテストを追加。`extract_value` が `&&`/`||` を BinOp として扱えるようにし、Break パターンの param 推定を柔軟化して real 形状でも panic しないようにした。
- Phase 38-NORM-OBS（dev-only）:
  - Normalized/JoinIR dev 経路のログカテゴリを `[joinir/normalized-bridge/*]` / `[joinir/normalized-dev/shape]` に統一し、`JOINIR_TEST_DEBUG` 下だけ詳細を出すよう静音化。Verifier/Fail‑Fast メッセージも shape/役割付きに整え、デバッグ観測性を上げつつ通常実行のノイズを減らした。
- Phase 43-A（dev-only）:
  - JsonParser `_atoi` 本体の Program(JSON) フィクスチャを normalized_dev に追加し、Structured→Normalized→MIR(direct) と Structured→MIR の VM 出力を比較するテストで一致を固定（符号あり/なしの簡易パス対応。canonical 切替は後続フェーズ）。
- Phase 43-C（dev-only）:
  - JsonParser `_parse_number` 本体の Program(JSON) フィクスチャを normalized_dev に追加し、Structured→Normalized→MIR(direct) と Structured→MIR の VM 出力を比較するテストで一致を固定（num_str は現状仕様のまま据え置き、P2-Mid の足慣らし）。
- **Phase 46-NORM-CANON-P2-MID（実装済み✅ 2025-12-12）**:
  - P2-Mid パターン（_atoi real, _parse_number real）を canonical Normalized に昇格。
  - Canonical set 拡張: P2-Core（mini + skip_ws + atoi mini）+ P2-Mid（atoi real + parse_number real）。
  - JsonParser P2 ライン（_skip_whitespace/_atoi/_parse_number）全て canonical Normalized 化完了。
  - P3/P4 Normalized 対応は NORM-P3/NORM-P4 フェーズで実施（今回スコープ外）。
  - 937/937 tests PASS。
- **Phase 47-B/C（P3 if-sum 拡張 + canonical 化 ✅ 2025-12-21）**:
  - フィクスチャ追加: `pattern3_if_sum_multi_min`（sum+count）/ `jsonparser_if_sum_min`（JsonParser 簡約形）
  - ShapeGuard: `Pattern3IfSumMulti` / `Pattern3IfSumJson` 追加、capability=P3IfSum、canonical set に P3 minimal/multi/json を追加
  - Normalizer/Bridge: P3 if-sum minimal/multi/json を Structured→Normalized→MIR(direct) で dev A/B、canonical ルートでも常時 Normalized 経由
  - テスト: normalized_joinir_min.rs に P3 if-sum multi/json の VM ブリッジ比較テスト追加（Structured と一致）
- **Phase 45-NORM-MODE（実装済み✅ 2025-12-12）**:
  - JoinIR モード一本化: バラバラだったフラグ/feature を `JoinIrMode` enum に集約（StructuredOnly / NormalizedDev / NormalizedCanonical）。
  - `current_joinir_mode()` でモード取得、bridge/runner で `normalized_dev_enabled()` → mode pattern matching に移行。
  - Canonical P2-Core は mode 無視で常に Normalized→MIR(direct)、それ以外は mode に従う統一ルーティング。
  - 937/937 tests PASS（既存挙動完全保持のリファクタ）。
- **Phase 44-SHAPE-CAP（実装済み✅ 2025-12-12）**:
  - Shape検出を capability-based に変更: `NormalizedDevShape` → `ShapeCapability` 抽象化層導入。
  - `ShapeCapabilityKind` 4種: P2CoreSimple / P2CoreSkipWs / P2CoreAtoi / P2MidParseNumber。
  - Shape-level (`is_canonical_shape`) と Capability-level (`is_p2_core_capability`) の二層 API でパターン拡張性を確保。
  - 既存挙動完全保持（canonical set: Pattern2Mini, skip_ws mini/real, atoi mini のまま）、937/937 tests PASS。

### 1. いまコード側で意識しておきたいフォーカス

- **Phase 43/245B Normalized 完了** により JoinIR ループ基盤（Pattern1–4 + ExprLowerer + ScopeManager + CapturedEnv + Normalized layer）は一応の完成状態に入ったので、当面は:
  - 既存パターン/箱の範囲内での **バグ修正・Fail‑Fast/invariant 追加・テスト強化** を優先する。
  - JsonParser/selfhost への新しい適用や大きな仕様拡張は、docs 側で Phase 設計が固まってからコード側に持ち込む。
- 直近のコード側フォーカス候補:
  - ~~Phase 246-EX（コード）~~: ✅ 完了（_atoi Integration, Phase 43/245B の一部）
  - Pattern1–4 / ExprLowerer / ScopeManager まわりで、by-name ハードコードやサイレントフォールバックが見つかった場合は、
    CarrierInfo / ConditionEnv / Scope 情報を使って「構造で」直す。

### 2. 直近で意識しておきたい設計の芯

- **Loop パターン空間**は有限で整理済み:
  - P1: Simple While
  - P2: Break
  - P3: If‑PHI（単一/複数キャリア）
  - P4: Continue（then/else‑continue 正規化込み）
  - P5: LoopBodyLocal 昇格（Trim/JsonParser 用の部分適用）
- 「増やすべき」は新しい Pattern ではなく、既存 Pattern の前処理箱:
  - BoolExprLowerer / ConditionEnv / LoopConditionScopeBox / LoopBodyCarrierPromoter /
    TrimLoopHelper / ComplexAddendNormalizer / LoopBodyLocalEnv / UpdateEnv などで  
    条件式とキャリア更新を吸収し、Pattern1–4 は「ループ骨格」に専念させる方針。
- Fail‑Fast 原則:
  - JoinIR 以外のループ lowering パスは存在しない（LoopBuilder は削除済み）。
  - 「わからないパターン」は必ず `[joinir/freeze]` 系の明示エラーにして、サイレントフォールバックはしない。

---

## ✅ 最近まとまった大きな塊（超要約）

ここ半年くらいで終わっている主な塊だけをざっくり書くね。  
細かいタスク・バグ票・議論は `docs/development/current/main/phase-*.md` と  
`docs/development/current/main/joinir-architecture-overview.md` に全部残っているので、必要になったときだけそちらを読む想定。

- **LoopBuilder 削除ライン（Phase 180 前後）**
  - LoopBuilder を dev‑only → hard freeze → 物理削除まで完了。
  - Loop lowering の SSOT を JoinIR に一本化。
- **LoopPattern / Router ライン（Phase 170–179, 244-EX）**
  - LoopFeatures / LoopPatternKind / PatternRouter / PatternPipelineContext を整備。
  - Pattern1–4 の検出・ルーティングを「構造ベース＋AST features」で統一（関数名ベタ書き依存を除去）。
  - Phase 244-EX で代表ループ（P1〜P4）の classify() 結果をユニットテストで固定。
- **Exit / Boundary / ValueId ライン（Phase 172–205）**
  - ExitMeta / ExitLineReconnector / JoinInlineBoundary(+Builder) / LoopHeaderPhiBuilder を箱化。
  - JoinValueSpace（Param/Local 領域）＋ PHI 契約 Verifier で ValueId 衝突と PHI 破損を根治。
- **P5(Trim/JsonParser) ライン（Phase 171–176, 173–175, 190–193）**
  - LoopBodyLocal 昇格パイプライン（Trim, `_skip_whitespace`, `_parse_string` 簡易版）を構築。
  - StringAppend / NumberAccumulation / Complex addend 正規化など、更新式まわりの箱を揃えた。
- **P3/P4 (If‑PHI / Continue) 汎用化ライン（Phase 195–196, 212–215, 220–242-EX）**
  - multi‑carrier P3 の JoinIR 生成を整理し、if‑sum 最小パターンを AST ベースで一般化（sum+count まで無改造対応）。
  - Pattern3/4 if‑sum/continue lowerer を分離箱にして、legacy PoC lowerer と by-name ハードコード（`"sum"`, `"count"`）を削除。
  - Pattern4CarrierAnalyzer を純粋な「キャリア解析箱」として仕上げ、continue 正規化・更新式解析をユニットテストで固定。

このあたりが「JoinIR ループ基盤の芯」で、以降の Phase は JsonParser/selfhost の各ループへの適用フェーズ、という位置づけだよ。

- **Phase 43/245B Normalized JoinIR（Phase 26–45 完了）** ✅
  - Structured→Normalized→MIR(direct) パイプライン確立
  - Mode system (JoinIrMode) + Capability system (ShapeCapabilityKind)
  - Pattern1/2 + JsonParser (_skip_whitespace, _atoi, _parse_number) 全対応
  - 詳細: [PHASE_43_245B_NORMALIZED_COMPLETION.md](docs/development/current/main/PHASE_43_245B_NORMALIZED_COMPLETION.md)

---

## 🧭 これからの候補（まだ「やる」とは決めていないメモ）

ここは「やることリスト」ではなく「今後やるとしたらこの辺」というメモだよ。
実際に着手するタイミングで、別途 Phase/タスクを切る想定。

1. ~~Phase 245B（コード）~~: ✅ 完了（Phase 43/245B の一部）
2. ~~Phase 246-EX（コード）~~: ✅ 完了（Phase 43/245B の一部）
3. **Phase 47-NORM-P3（設計完了＋最小dev＋direct✅ 2025-12-12）**: Pattern3 Normalized 設計
   - 設計詳細: [phase47-norm-p3-design.md](docs/development/current/main/phase47-norm-p3-design.md)
   - P3 if-sum を Normalized JoinIR に載せる設計。P2 と同じ ConditionEnv/CarrierInfo/ExitLine インフラを再利用。
   - Phase 47-A: Minimal sum_count（dev-only）として、`phase212_if_sum_min.hako` 相当の最小 if-sum ループを AST ベース lowerer + Structured→Normalized→Structured roundtrip（Runner 経路）＋ Normalized→MIR(direct) で検証済み。
   - Phase 47-B 以降: array_filter など body-local/MethodCall を含む P3 ループや canonical 昇格は今後の実装フェーズで扱う。
4. **Phase 48-NORM-P4（設計完了✅＋48-A/B/C canon 完了✅ 2025-12-12→2026-01-XX）**: Pattern4 (continue) Normalized 設計＋実装
   - 設計詳細: [phase48-norm-p4-design.md](docs/development/current/main/phase48-norm-p4-design.md)
   - ターゲットループ決定: _parse_array skip whitespace（◎ PRIMARY）、_parse_object（○）、_unescape_string/parse_string（△）
   - 設計骨格: `continue` = 即座の `TailCallFn(loop_step, ...)` (新命令不要)
   - P1/P2/P3 と同じ `loop_step(env, k_exit)` 骨格に載せる
   - インフラ再利用率: 95%+ (StepKind の ContinueCheck のみ追加)
   - **Phase 48-A実装（minimal dev-only）完了✅** / **Phase 48-B dev（JsonParser skip_ws continue）完了✅**:
     - P4 minimal フィクスチャ追加（skip i==2 パターン、単一 carrier `acc`）＋ JsonParser continue skip_ws (array/object) フィクスチャを追加
     - ShapeGuard: Pattern4ContinueMinimal + JsonParser continue 形状を検出
     - StepScheduleBox: ContinueCheck step 追加（評価順序: HeaderCond → ContinueCheck → Updates → Tail）
     - normalize_pattern4_continue_minimal()/jsonparser_*continue* を dev 正規化に配線（P2 インフラを再利用）
     - テスト完備: minimal + JsonParser continue の VM bridge 比較を normalized_dev スイートで固定
   - **Phase 48-C（canonical 昇格）完了✅**:
     - P4 minimal + JsonParser skip_ws array/object を canonical set に追加（env OFF でも Normalized→MIR(direct) を強制）
     - Bridge/runner で Structured fallback を禁止、fail-fast 契約に統一
     - canonical ルートと Structured 直経路の stdout 一致を比較するテストを追加
5. JsonParser 残りループへの JoinIR 展開
   - `_parse_array` / `_parse_object` / `_unescape_string` / 本体 `_parse_string` など。
   - 既存の P2/P3/P4＋P5 パイプラインをどこまで延ばせるかを docs 側で設計 → コード側はその設計に沿って小さく実装。
6. **Phase 49-SELFHOST-NORM-DEPTH2（設計・コードなし）**: selfhost depth2 Normalized 設計フェーズ
   - 設計詳細: [phase49-selfhost-joinir-depth2-design.md](docs/development/current/main/phase49-selfhost-joinir-depth2-design.md)
7. **Phase 50-SELFHOST-NORM-DEV（dev-only）完了✅ 2025-12-12**: selfhost 軽量 P2/P3 を dev Normalized パイプラインに載せる足慣らし
   - 対象: `selfhost_token_scan_p2` / `selfhost_if_sum_p3`
   - fixtures / ShapeGuard(Selfhost* 系) / VM bridge 比較テストまで整備し、Structured 直経路と一致を固定。
8. **Phase 51-SELFHOST-NORM-DEV-EXTEND（dev-only）完了✅ 2025-12-12**: selfhost 実戦寄り P2/P3 を dev Normalized に追加
   - 対象: `selfhost_token_scan_p2_accum` / `selfhost_if_sum_p3_ext`
   - Phase 50 と同導線で fixtures / shape / 比較テストを追加し、selfhost 断面で緑を維持。
9. **Phase 52-SELFHOST-SHAPE-STRUCT-SIGNATURE（dev-only）完了✅ 2025-12-12**: selfhost shape の構造シグネチャ育成
   - selfhost P2/P3 を「構造一次判定→dev-only name ガード最終確定」の二段 detector に移行。
   - 構造シグネチャの安定テストを追加し、name ガード撤去の足場を SSOT に固定。
10. **Phase 53-SELFHOST-NORM-DEV-EXPAND（dev-only）完了✅ 2025-12-12**: selfhost P2/P3 の実戦寄り形状を追加
   - 対象追加: P2 `args_parse_p2` / P3 `stmt_count_p3`
   - 構造一次判定（carrier 数/型/Compare/branch）→ dev-only name 最終確定の二段 detector を拡張。
   - P3 carrier 上限を 2–10 に拡大し、複雑 if-else 形状を selfhost 群として取り込んだ。
   - `normalized_dev` selfhost 断面/回帰テストが緑、既定挙動は不変。
11. **Phase 54-SELFHOST-SHAPE-GROWTH（dev-only）完了✅ 2025-12-12**: 構造軸の育成と偽陽性観測フェーズ
   - Phase 53 で実戦ループ追加済みのため、追加投入より先に構造判定精度の測定に集中。
   - 構造シグネチャ軸を 5+ に拡張（Compare op 分布など）し、P2/P3 の偽陽性観測テストを追加。
   - 結果: selfhost 群の構造判定だけでは分離が不十分（偽陽性率 ~50%）。dev-only name ガードは当面必須と判断。
12. **Phase 55-SELFHOST-SHAPE-AXIS-EXPAND（dev-only / 保留）**: 構造軸を可変 feature として拡張し誤判定を下げる足場
   - Phase 56–61 の Ownership-Relay ライン優先のため、selfhost shape 軸拡張は一旦保留。
   - OwnershipAnalyzer 導入後に、scope 署名（owned/carriers/captures/relay）を新しい構造軸として合流させる。
13. **Phase 56-OWNERSHIP-RELAY-DESIGN（完了✅ 2025-12-12）**: Ownership-Relay アーキテクチャ設計 + インターフェース skeleton
   - 設計詳細: [phase56-ownership-relay-design.md](docs/development/current/main/phase56-ownership-relay-design.md)
   - コア定義: owned / carriers / captures / relay の 4 分類を明確化
   - 不変条件: Ownership Uniqueness / Carrier Locality / Relay Propagation / Capture Read-Only
   - Module 作成: `src/mir/join_ir/ownership/` - 責務は「解析のみ」
   - 型定義: `ScopeId`, `ScopeOwnedVar`, `RelayVar`, `CapturedVar`, `OwnershipPlan`
   - テスト: 3 つのユニットテスト追加（empty plan / carriers filter / invariant verification）
   - 次: Phase 57 で OwnershipAnalyzer 実装（dev-only）
14. **Phase 57-OWNERSHIP-ANALYZER-DEV（完了✅ 2025-12-12）**: OwnershipPlan を生成する解析箱の実装
   - `OwnershipAnalyzer` を追加し、ネスト含む reads/writes/owned を集計→ carriers/relay/captures を plan 化。
   - 既存 fixtures（pattern2/3, jsonparser, selfhost）で plan の回帰テストを追加。
   - 設計詳細: [PHASE_57_SUMMARY.md](docs/development/current/main/PHASE_57_SUMMARY.md)
15. **Phase 58-OWNERSHIP-PLUMB-P2-DEV（完了✅ 2025-12-12）**: P2 conversion helper (dev-only)
   - `plan_to_p2_inputs()` でOwnershipPlan→P2LoweringInputs変換
   - Fail-Fast: relay_writes 未対応（Phase 60で対応予定）
   - 5つのユニットテスト + 1つのintegrationテスト
   - 設計詳細: [PHASE_58_SUMMARY.md](docs/development/current/main/PHASE_58_SUMMARY.md)
16. **Phase 59-OWNERSHIP-PLUMB-P3-DEV（完了✅ 2025-12-12）**: P3 conversion helper (dev-only)
   - `plan_to_p3_inputs()` でOwnershipPlan→P3LoweringInputs変換（P2と同構造）
   - Multi-carrier対応（sum, count, 5+ carriers）
   - Fail-Fast: relay_writes 未対応（Phase 60で対応予定）
   - 4つのユニットテスト + 2つのintegrationテスト
   - 設計詳細: [PHASE_59_SUMMARY.md](docs/development/current/main/PHASE_59_SUMMARY.md)
17. **Phase 60-OWNERSHIP-RELAY-IMPL（完了✅ 2025-12-12）**: Relay support for P2/P3 (dev-only)
   - `plan_to_p2_inputs_with_relay()` / `plan_to_p3_inputs_with_relay()` を追加（単一hopのみ許可、multi-hopはFail-Fast）
   - P2 Break lowering を dev-only で ownership-with-relay に接続し、legacy 経路との VM 出力一致を比較テストで固定。
   - shape_guard の selfhost family 分離を最小更新（selfhost shapes 優先時の混線を遮断）。
18. **Phase 61-IFSUM-BREAK-STRUCTURAL（完了✅ 2025-12-12）**: if-sum + break を別箱で構造的に導入（dev-only）
   - Break(P2) から P3 固有ロジック（by-name）を撤去し、責務混線を解消。
   - 新箱 `if_sum_break_pattern` を追加し、`return Var+Var` を含む if-sum+break を構造判定→Fail-Fast で lowering。
   - OwnershipPlan を param order/carriers の SSOT に使い、carriers!=return vars の混線を遮断。
   - 詳細: [PHASE_61_SUMMARY.md](docs/development/current/main/PHASE_61_SUMMARY.md)
19. **Phase 62-OWNERSHIP-P3-ROUTE-DESIGN（完了✅ 2025-12-12）**: P3 本番ルートへ OwnershipPlan を渡す設計
   - MIR→JoinIR の `pattern3_with_if_phi.rs` は OwnershipPlan を受け取らないため、AST-based ownership 解析の接続点を設計する。
   - dev-only で段階接続し、legacy と stdout/exit 一致の比較で回帰を固定（既定挙動は不変）。
   - 設計詳細: [phase62-ownership-p3-route-design.md](docs/development/current/main/phase62-ownership-p3-route-design.md)
20. **Phase 63-OWNERSHIP-AST-ANALYZER（完了✅ 2025-12-12）**: 本番 AST から OwnershipPlan を生成（dev-only）
   - `AstOwnershipAnalyzer` を追加し、ASTNode から owned/relay/capture を plan 化（analysis-only）。
   - JSON v0 の "Local=rebind" ハックを排除（fixture 専用のまま）。
   - 詳細: [PHASE_63_SUMMARY.md](docs/development/current/main/PHASE_63_SUMMARY.md)
21. **Phase 64-OWNERSHIP-P3-PROD-PLUMB（完了✅ 2025-12-12）**: 本番 P3(if-sum) ルートへ段階接続（dev-only）
   - ✅ `analyze_loop()` helper API を追加（`ast_analyzer.rs`）
   - ✅ `pattern3_with_if_phi.rs` で OwnershipPlan を導入し、整合チェック実行
   - ✅ Fail-Fast: multi-hop relay (`relay_path.len() > 1`)
   - ✅ Warn-only: carrier set mismatch（order SSOT は Phase 65+）
   - ✅ 回帰テスト追加（`test_phase64_p3_ownership_prod_integration`, `test_phase64_p3_multihop_relay_detection`）
   - ✅ テスト結果: 49/49 tests passing, 0 regressions
   - 詳細: [PHASE_64_SUMMARY.md](docs/development/current/main/PHASE_64_SUMMARY.md), [phase64-implementation-report.md](docs/development/current/main/phase64-implementation-report.md)
22. **Phase 65-REFACTORING-AUDIT（完了✅ 2025-12-12）**: コード品質監査 + スタブドキュメント化
   - ✅ Explore agent による包括的リファクタリング監査（10 opportunities identified）
   - ✅ [REFACTORING_OPPORTUNITIES.md](docs/development/current/main/REFACTORING_OPPORTUNITIES.md) ドキュメント作成
   - ✅ BID-codegen stubs へ deprecation notice + 代替パス文書化
   - ✅ quick wins (< 2時間) 実装可能な内容をドキュメント化
   - 詳細: [REFACTORING_OPPORTUNITIES.md](docs/development/current/main/REFACTORING_OPPORTUNITIES.md)
23. **Phase 65-OWNERSHIP-RELAY-MULTIHOP-DESIGN（完了✅ 2025-12-12）**: Multihop relay 設計（実装はPhase 66以降）
   - ✅ Multihop relay 意味論の定義（relay_path: 内→外の順、段階的carrier化）
   - ✅ Merge relay 意味論の定義（PERMIT with owner merge、複数inner loopが同一祖先owned変数を更新）
   - ✅ Fail-Fast 解除条件の明文化（Phase 66実装時の受け入れ基準）
   - ✅ 実装箇所の特定（Analyzer変更不要、plan_to_lowering/Pattern lowering変更点）
   - ✅ 禁止事項の明文化（by-name分岐排除、dev-only name guard対象外）
   - ✅ 代表ケース（3階層multihop AST例、Merge relay JSON fixture例）
   - 詳細: [phase65-ownership-relay-multihop-design.md](docs/development/current/main/phase65-ownership-relay-multihop-design.md)
24. **Phase 66-OWNERSHIP-RELAY-MULTIHOP-IMPL（完了✅ 2025-12-12）**: Multihop relay 実装（analysis/plan層）
   - ✅ `plan_to_lowering.rs` の relay_path.len() > 1 制限撤去
   - ✅ 構造的 Fail-Fast ガード実装（empty path, scope mismatch, owner=scope, name conflict）
   - ✅ ユニットテスト追加（5件: multihop accepted, empty rejected, path mismatch, owner same, name conflict）
   - ✅ `ast_analyzer.rs` に 3階層 multihop テスト追加
   - ✅ テスト結果: normalized_dev 49/49, lib 947/947 PASS
   - 次フェーズ: Phase 70-B+（merge relay / 本番multihop 実行対応）
   - 詳細: [phase65-ownership-relay-multihop-design.md](docs/development/current/main/phase65-ownership-relay-multihop-design.md)
25. **Phase 67-MIR-VAR-IDENTITY-SURVEY（完了✅ 2025-12-13）**: MIR の束縛モデルを観測して SSOT 化
   - 現状: `variable_map(name→ValueId)` 1枚でブロックスコープ/シャドウイング無し、未宣言代入の挙動が doc と不一致。
   - プローブ（vm smokes）を追加して観測可能化し、Phase 68 の修正方針（MIR 側で lexical scope を実装）を決定。
   - 詳細: [phase67-mir-var-identity-survey.md](docs/development/current/main/phase67-mir-var-identity-survey.md)
26. **Phase 68-MIR-LEXICAL-SCOPE（完了✅ 2025-12-13）**: MIR ビルダーに lexical scope を導入し、仕様に整合
   - `{...}`（Program）/ `ScopeBox` を lexical scope として扱い、`local` の shadowing を正しく復元。
   - “未宣言名への代入はエラー” を SSOT（quick-reference/LANGUAGE_REFERENCE）に揃えて Fail-Fast 化。
   - free-vars 解析も lexical scope に追随（AST walker 重複の整理を含む）。
   - 実装コミット: `1fae4f16`, `0913ee8b`
27. **Phase 69-OWNERSHIP-AST-SHADOWING（完了✅ 2025-12-13）**: AST ownership 解析を shadowing-aware にする（dev-only）
   - `AstOwnershipAnalyzer` を内部 `BindingId` で分離し、ネスト block の local が loop carriers/relay に混ざらないように修正。
   - 代表テストで固定（shadowing あり/なし、outer update の relay_write、ネスト block local の非混入）。
   - 実装コミット: `795d68ec`
28. **Phase 70-A-RELAY-RUNTIME-GUARD（完了✅ 2025-12-13）**: “plan OK / runtime NG” をタグで固定（dev-only）
   - `[ownership/relay:runtime_unsupported]` を標準タグとして文書化し、P3 runtime 経路のエラーを統一。
   - 回帰テストで “タグを含む Err” を固定し、段階解除（Phase 70-B+）の足場にする。
   - 詳細: [phase70-relay-runtime-guard.md](docs/development/current/main/phase70-relay-runtime-guard.md)
   - 実装コミット: `7b56a7c0`
29. **Phase 71-Pre-OWNERSHIP-PLAN-VALIDATOR（完了✅ 2025-12-13）**: OwnershipPlanValidator 箱を導入（dev-only）
   - OwnershipPlan の整合チェックを箱に隔離し、P3 本番導線のガードを再利用可能にする。
   - 実装コミット: `1424aac9`
30. **Phase 70-B-MULTIHOP-PASSTHROUGH（完了✅ 2025-12-13）**: Multihop relay passthrough 対応（dev-only）
   - Runtime guard を「常時エラー」から「未対応ケースのみエラー」に縮め、passthrough パターン受理。
   - 構造判定: `is_supported_multihop_pattern()` で self-conflict 検出。
   - 実装コミット: `c2df1cac`
   - テスト: normalized_dev 52/52, lib 950/950 PASS
31. **Phase 70-C-MERGE-RELAY（完了✅ 2025-12-13）**: Merge relay 検出（dev-only）
   - 複数の inner loop が同一 owner の変数を更新するパターンを検出・受理。
   - Validator は個別の relay をチェック（cross-plan consistency は Phase 70-D+ へ先送り）。
   - 実装コミット: `24cc948f`
   - テスト: normalized_dev 54/54, lib 950/950 PASS
32. **Phase 72-PHI-RESERVED-OBSERVATION（完了✅ 2025-12-13）**: PHI Reserved Region 検証観測
   - PHI dst ValueId の分布を観測し、reserved region (0-99) への適合性を確認。
   - 結論: PHI dst は builder.next_value_id() から割り当てられ、JoinValueSpace の reserved region とは独立。
   - 決定: verifier 強化は **非推奨**（アーキテクチャ的根拠なし）。
   - 現状: 偶発的な非衝突（MirBuilder=0-50, JoinValueSpace=100+）で安定動作中。
   - 実装コミット: `253eb59b`
   - 詳細: [PHASE_72_SUMMARY.md](docs/development/current/main/PHASE_72_SUMMARY.md), [phase72-phi-reserved-observation.md](docs/development/current/main/phase72-phi-reserved-observation.md)
33. **Phase 73-SCOPE-MANAGER-DESIGN（完了✅ 2025-12-13）**: ScopeManager BindingId 設計 + PoC（dev-only）
   - JoinIR 側の name-based lookup を BindingId-based に段階移行する設計を確定。
   - Option A (Parallel BindingId Layer) で段階移行可能性を実証（PoC 6/6 PASS）。
   - 移行ロードマップ: Phase 74-77（合計 8-12時間、本番影響ゼロ）。
	  - 実装コミット: `851bf4f8`
	  - SSOT: [phase73-scope-manager-design.md](docs/development/current/main/phase73-scope-manager-design.md), [phase73-completion-summary.md](docs/development/current/main/phase73-completion-summary.md)
34. **Phase 74-INFRASTRUCTURE（コミット済み✅ `e1574af7` 2025-12-13）**: BindingId infrastructure (dev-only)
	   - `binding_id.rs`: BindingId type + 5 unit tests
	   - `builder.rs`: binding_map + allocate_binding_id() + 4 integration tests
	   - `lexical_scope.rs`: Parallel binding_map restoration
	   - 9/9 new tests PASS, lib 958/958 PASS

35. **Phase 75-PILOT（コミット済み✅ `c18dde23` 2025-12-13）**: BindingId pilot lookup (dev-only)
	   - `scope_manager.rs`: lookup_with_binding() trait method
	   - `condition_env.rs`: resolve_var_with_binding() 3-tier fallback
	   - 3/3 pilot tests PASS, lib 958/958 PASS

36. **Phase 76-PROMOTION（コミット済み✅ `11e68203` 2025-12-13）**: promoted_bindings map (dev-only)
	   - `carrier_info.rs`: promoted_bindings field + resolve/record methods
	   - `scope_manager.rs`: promoted BindingId lookup (direct → promoted → name fallback)
	   - 5/5 promotion tests PASS, lib 958/958 PASS

37. **Phase 77-EXPANSION（コミット済み✅ `72173c1a` 2025-12-13）**: promoted_bindings populate + legacy deprecate（dev-only）
	   - DigitPosPromoter/TrimLoopHelper で promoted_bindings を populate（binding_map を thread して record）
	   - legacy name-based promoted lookup を `#[deprecated]` 化（削除は Phase 78+）
	   - 注: Pattern3/4 の “binding_id を必ず供給する” までの拡張と、E2E tests 4本は Phase 78 に先送り

38. **Follow-up（コミット済み✅ `0aad016b` 2025-12-13）**: legacy promoted lookup の deprecation warning を局所化
	   - `ScopeManager::lookup` 内の legacy 呼び出しを `#[allow(deprecated)]` で包み、全ビルドでの警告を抑制

39. **Phase 78-A（コミット済み✅ `10e78fa3` 2025-12-13）**: PromotedBindingRecorder の整理（dev-only）
   - binding_map の受け渡しを Box 化し、検出器（Detector）と記録（Recorder）の責務を分離。
   - 以後の “BindingId を供給する導線” の議論を単純化するための整地。

40. **Phase 79（コミット済み✅ `48bdf2fb` 2025-12-13）**: Detector/Recorder separation + BindingMapProvider
   - `BindingMapProvider` を導入し、`#[cfg(feature="normalized_dev")]` の散在を抑制する方向へ。

---

## 🚀 次フェーズ候補（Phase 78+）

### Phase 78-B（dev-only）: “promoted carriers” を BindingId で接続する（進行中）
- SSOT: `docs/development/current/main/phase78-bindingid-promoted-carriers.md`
- 目的: `BindingId(original)` → `BindingId(promoted)` → `ValueId(join)` の鎖を作り、name-hack 依存を減らす。
- 残タスク（優先順）:
  1) ExprLowerer/ConditionLoweringBox の call-site で `lookup_with_binding()` を実際に使う（BindingId が効く状態にする）
  2) Trim/Pattern4 側の “BindingId → join” を E2E で固定（DigitPos 以外の回帰ガード）
  3) Pattern3/4 の本番導線へ拡張（P3 if-sum / P4 continue の binding 供給）
  4) legacy name-based promoted lookup の縮退 → 撤去計画を docs に固定（削除自体は Phase 80+ でも可）

---

## 📎 このファイルの運用ルール（自分向けメモ）

- 過去フェーズの詳細な ToDo/Done リストは **CURRENT_TASK には書かない**。  
  代わりに `docs/development/current/main/phase-*.md` と `joinir-architecture-overview.md` を SSOT として維持する。
- CURRENT_TASK は「あくまで最新のフォーカスと次の候補だけ」に絞る。  
  目安として **このファイル自体は 2〜3画面程度（〜300行以内）** に収める。
- 新しい大フェーズを始めたら：
  1. まず docs 配下に `phase-XXX-*.md` を書く。
  2. CURRENT_TASK には「そのフェーズの一行要約」と「今のフォーカスかどうか」だけを書く。
