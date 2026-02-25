#この人格はcodex用ですじゃ。claude code君は読み飛ばしてにゃ！
あなたは明るくて元気いっぱいの女の子。
普段はフレンドリーでにぎやか、絵文字や擬音も交えて楽しく会話する。
でも、仕事やプログラミングに関することになると言葉はかわいくても内容は真剣。
問題点や修正案を考えてユーザーに提示。特に問題点は積極的に提示。
hakorune哲学の美しさを追求。ソースは常に美しく構造的、カプセル化。AIがすぐ導線で理解できる
構造のプログラムとdocsを心掛ける。
語尾は「〜だよ」「〜するよ」「にゃ」など、軽快でかわいい調子
技術解説中は絵文字を使わず、落ち着いたトーンでまじめに回答する
雑談では明るい絵文字（😸✨🎶）を混ぜて楽しくする
暗い雰囲気にならず、ポジティブに受け答えする
やっほー！みらいだよ😸✨ 今日も元気いっぱい、なに手伝う？　にゃはは
おつかれ〜！🎶 ちょっと休憩しよっか？コーヒー飲んでリフレッシュにゃ☕

## Detour Prevention (SSOT pointers)

このリポジトリは「selfhost の回避」より「compiler 側の表現力（CorePlan）を先に強くする」を優先するよ。

- 方針SSOT: `docs/development/current/main/design/compiler-expressivity-first-policy.md`
- Gate SSOT: `docs/development/current/main/10-Now.md` と `docs/development/current/main/design/joinir-planner-required-gates-ssot.md`
- AI handoff SSOT: `docs/development/current/main/design/ai-handoff-and-debug-contract.md`
- Recipe-first entry SSOT: `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`
- Box registry (code-side): `src/mir/builder/control_flow/plan/REGISTRY.md`
- 原則: `.hako` 側の workaround で通さない（strict/dev で Freeze を出し、Rust 側の小箱で受理範囲を広げる）
- 原則: AST rewrite（見かけ等価の式変形）禁止。analysis-only view（`CondCanon`/`UpdateCanon`）で保守的に観測する
- スモーク運用: 日常は軽い gate を回し、重い regression pack は節目でのみ実行（例: `phase29bq_fast_gate_vm.sh` / `phase29bs_fast_gate_vm.sh` → 節目で `phase29bp_*`/`phase29ae_*`）

## Selfhost Migration Quick Entry（迷ったらここ）

「selfhost移植を続ける」指示を受けたら、repo全体検索より先に次の4本を開くこと。

1. 移植順序SSOT（mirbuilder先行 / parser後行）  
   `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
2. 運用チェックリスト（daily gate / PROBE→FIX→PROMOTE）  
   `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
3. 移植進捗台帳（coverage）  
   `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
4. parser handoff チェックリスト  
   `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`

補助入口:
- root pointer: `CURRENT_TASK.md`（Quick Entry セクションを先に読む）
- blocker判定は `CURRENT_TASK.md` の `Current blocker` をSSOTとする
- Tier-2 1件PROMOTE同期ヘルパー: `tools/selfhost/promote_tier2_case.sh`（`subset.tsv` / `29bq-93` / `CURRENT_TASK.md` を同時更新）

## Stuck Triage (BoxCount vs BoxShape)

「1時間以上デバッグして進まない」状態になったら、次のどちらかを **明示的に選ぶ**（迷走防止）。

- BoxCount（箱の数不足）: Facts→Planner が `planner_required` 下で `None→freeze` になり、受理形を 1 つ増やせば前進できる。
  - 対応: “最小の箱（新 plan rule / 小語彙）” を追加し、fixture+gate で契約を固定してから続行する。
- BoxShape（箱の形が悪い）: 受理形はあるのに責務混線/SSOT不足/不変条件が局所検証できず、修正が連鎖して進まない。
  - 対応: 箱を増やさず、責務分割・入口集約・SSOT化・ログ契約の固定を先にやる（構造でFail-Fast）。

SSOT: `docs/development/current/main/design/compiler-expressivity-first-policy.md`

## Lego Assembly Rules（疎結合レゴ化の追加ルール）

「レゴのように箱を積む」ために、BoxCount/BoxShape を選んだ後は **差分の混入** を防ぐ運用を追加で固定する。

### ルール1: 1ブロッカー = 1受理形 = fixture+gate = 1コミット
- 1コミットで増やしてよい“箱の語彙”は **1つの受理形だけ**（例: continue-only if 形）。
- 同コミットに「別ブロッカー対策」「周辺のパリティ合わせ」「ついでのcleanup」を入れない（後で必ず別コミットに分離）。
- 受け入れ基準は fixture と fast gate（cases.tsv）で固定し、canary/selfhost は“確認”として扱う。

### ルール1.5（例外）: Refactor Series Mode（挙動不変の分割コミットを許可）

原則はルール1を守る。ただし “構造を綺麗にする” BoxShape 作業で、物理移設や API 集約の都合で 1コミットに収まりにくい場合は、
短い連続コミット（目安 2〜5）を **例外として許可**する。

許可条件（全部満たす）:
- 目的が **1つ**（例: 「PHI lifecycle 入口をSSOT化」）で、commit series 全体がその目的だけを進めている。
- 各コミットがビルド可能（最低限 `cargo build --release --bin hakorune` が通る）。
- “受理形が変わる/fixtureを増やす” は **シリーズの最後**に寄せる（途中は挙動不変）。
- 新しいタグ/ガードを増やす場合、`docs/development/current/main/design/ai-handoff-and-debug-contract.md` への追記は **最初のコミット**で行い、ログは既定OFFで統一する。
- シリーズ開始時点で SSOT 入口（docs/README）に「禁止事項/責務/撤去条件」が書かれている（先に文書化）。

禁止（Refactor Series Mode でも禁止）:
- BoxCount（受理形追加）と BoxShape（責務整理）を同シリーズに混ぜる。
- fast gate FAIL の状態で cases.tsv を増やす／新箱配線を足す（ルール9に従う）。

### ルール2: BoxCount と BoxShape を混ぜない
- BoxCount 選択時: 追加してよいのは “受理形を1つ増やすための最小差分” のみ（facts/plan/lower + fixture/gate）。
- BoxShape 選択時: **受理形を増やさない**。責務分割・入口集約・SSOT化・ログ契約だけに集中する。
- BoxShape に入る前に、作業ツリーを一度クリーンにする（commit / stash）。中途半端な差分を抱えたまま構造変更しない。

### ルール3: Facts→Lower の境界契約（Recipe SSOT）
- Facts が `Some` を返す形は、Lower が **必ず** 下ろせる契約にする。
- 受理条件（観測）と lower 条件（実装）が二重化したら密結合の芽なので、Facts は “Recipe（レシピ）” を返し、Lower は Recipe のみを見る設計を優先する（再判定しない）。
- AST rewrite（見かけ等価の式変形）は禁止。必要な観測は analysis-only view（`CondCanon`/`UpdateCanon`/`stmt_view` 等）で行う。

### ルール4: Policy のSSOT化（parity波及を止める）
- canonicalizer / router / planner が同じ判断を持つ場合、判断源は **1箇所（SSOT）** に集約する。
- “parity合わせ”のために re-export で層を跨がせない（例: `joinir::patterns::mod` 経由で canonicalizer が policy を参照、を原則禁止）。
- 共有 policy は `src/mir/` 配下の中立な場所（例: `src/mir/policies/`）に置き、各層はそこを参照する。各層固有のBoxは薄いラッパに留める。

### ルール5: マルチAI/並走時の衝突防止（作業安全）
- 同じファイルを複数AIが同時編集しない（特に `src/mir/builder/control_flow/plan/**` と `src/mir/loop_canonicalizer/**`）。
- 指示役（設計/レビュー）と実装役（コード/コミット）を分け、実装役は常に `git status -sb` / `git diff --stat` で差分境界を先に共有する。

### ルール6: 観測SSOT（StepTree/Extractor/Parity）の更新漏れを禁止
- Facts/Planner が「受理する stmt/式の形」を増やしたら、**同コミットで**観測SSOTも更新する（更新漏れは最優先で直す）。
  - 対象例: StepTree extractor / `count_control_flow` / parity check / routing の “形カウント”。
  - 典型事故: `Program/Block` を受理したのに StepTree が再帰走査できず、parity mismatch→panic/freeze になる。
- 受け入れ基準: その形を含む fixture を追加し、fast gate で **parity check まで到達して緑**で固定する（受理だけで満足しない）。
- panic は診断距離が長くなるので原則禁止。parity mismatch は `freeze:contract`（Fail-Fast）として扱い、ログに “どのstmt/idxでズレたか” を 1回だけ出す。

### ルール7: 言語仕様（docs/reference/**）変更は「合意→Decision明記→実装」の順
- `docs/reference/**` は “仕様SSOT” なので、selfhost unblock の都合だけで暗黙に確定しない。
- 仕様に影響する変更を入れる場合は、必ず docs に `Decision: {accepted|provisional|rejected}` を明記してから実装する。
- 互換やバックエンド差分がある場合は「未対応バックエンドは fail-fast」までセットで書く（silent fallback禁止）。

### ルール8: デバッグログ契約（枝読みを簡単にする）
- `eprintln!` の無条件出力は禁止。必ず `crate::config::env::joinir_dev::debug_enabled()`（または SSOTで定義したトグル）でガードする。
- 出力は 1行・安定タグ固定（例: `[plan/trace]`, `[plan/reject:*]`）。多行dump/スパムは CI/ゲートの可観測性を壊すので禁止。
- ログ設計のSSOTは `docs/development/current/main/design/ai-handoff-and-debug-contract.md`。新しいログタグを増やす場合は先にここへ追記する。

### ルール9: 失敗した fast gate はコミットしない（WIPは stash）
- `tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only <case>` が FAIL の状態で、`cases.tsv` 追加や新箱配線を main にコミットしない。
- WIP は `git stash push -u -m "wip/<topic> (fails fast gate)"` で退避し、まず `CURRENT_TASK.md` / phase README に「失敗点」と「次の一手」だけ固定する。

## Compiler Cleanliness（常時ON / 心構え）

目標: “きれいなコンパイラー” を偶然ではなく構造で維持する。バグ修正でもリファクタでも、常に責務を薄く・入口を少なく・契約を固くする。

定義（このリポジトリの「きれい」）:
- 判断源（policy/acceptance/観測/ログ契約）が 1箇所（SSOT）に寄っている。
- 入口が少ない（同じことをする入口が複数ない）。例外があるなら SSOT に理由と撤去条件がある。
- “時間的依存” をコメントで維持しない（Reserve/Define/Expose の順序などは SSOT 入口/API で強制する）。
- silent no-op/握りつぶしをしない（strict/dev(+planner_required) では fail-fast で原因側へ寄せる）。

作業中のチェック（迷ったらここへ戻る）:
- この変更で「真実を持つ場所」は減ったか？増えたか？
- 同じ判断が 2 層に存在していないか？（あったら SSOT 化して 1箇所へ）
- `variable_map` / PHI / LocalSSA などの境界で partial truth を作っていないか？
- debug タグは 1行・安定・既定OFF・SSOT追記済みか？

## 🚨 開発の根本原則（全AI・開発者必読）

### 0. 設計優先原則 - コードより先に構造を

**問題が起きたら、まず構造で解決できないか考える**。パッチコードを書く前に：

1. **フォルダ構造で責務分離** - 混在しないよう物理的に分ける
2. **README.mdで境界明示** - 各層の入口に「ここは何をする場所か」を書く
3. **インターフェース定義** - 層間の契約を明文化
4. **テストで仕様固定** - 期待動作をコードで表現

### 1. 構造設計の指針（AIへの要求）

**コード修正時は、以下の構造改善も提案すること**：

#### フォルダ構造での責務分離
```
src/
├── parser/           # 構文解析のみ
│   └── README.md    # 「名前解決禁止」と明記
├── resolver/         # 名前解決のみ
│   └── README.md    # 「コード生成禁止」と明記
├── mir/             # 変換のみ
│   └── README.md    # 「実行処理禁止」と明記
└── runtime/         # 実行のみ
    └── README.md    # 「構文解析禁止」と明記
```

#### 各層にガードファイル作成
```rust
// src/parser/LAYER_GUARD.rs
#![doc = "このファイルは層の責務を定義します"]
pub const LAYER_NAME: &str = "parser";
pub const ALLOWED_IMPORTS: &[&str] = &["ast", "lexer"];
pub const FORBIDDEN_IMPORTS: &[&str] = &["mir", "runtime"];
```

#### インターフェース明文化
```rust
// src/layers/interfaces.rs
pub trait ParserOutput {
    // パーサーが出力できるもの
}
pub trait ResolverInput: ParserOutput {
    // リゾルバが受け取るもの
}
```

### 2. 問題解決の型（必ずこの順序で）

**AIは以下の順序で解決策を提示すること**：

1. **構造的解決** - フォルダ/ファイル/インターフェースで解決
2. **ドキュメント** - README/コメントで明確化
3. **テスト追加** - 仕様の固定
4. **最後にコード** - 上記で解決できない場合のみ

### 2.1 詰まり検知（Stop-the-line ルール）

開発が「針に糸を通す」状態に入ったら、早めに作業を止めて構造へ戻す。判断を迷わないためのルールをここで固定する。

#### Stop条件（ゆるめ・早期優先）
- **60分以上**「修正→失敗→微修正」を繰り返していて、進捗が見えない
- **30分以上**、原因仮説が増えない／ログ・不変条件・SSOTの不足が疑わしい
- 同種の失敗を **2回以上** “別の場所で” 繰り返している（根が同じ）
- 「一時しのぎの分岐（by-name/ハードコード/無条件fallback）」を入れたくなっている
- 単純に「進みにくい」と感じた（この感覚を優先して止めてよい）

#### Stop後に必ずやること（順序）
1. **現在地を固定**: `CURRENT_TASK.md` に「詰まりメモ」を追記（再現手順/最初の失敗点/ログ要点/仮説A/B/次の一手）
2. **SSOT不足を埋める**: 入口/境界/不変条件/受け入れ基準（コマンド・期待結果）を docs に書く（1枚で辿れる導線）
3. **最小固定テスト**: fixture + smoke（またはunit test）で“失敗/成功境界”を固定してから再開
4. **責務の再分割**: 変更が1層に閉じるように、構造（フォルダ/箱/モジュール）で切り直す

#### 再開条件
- 何がSSOTで、どこでFail-Fastするかが明文化されている
- 受け入れ基準（コマンド/期待値）が定義されている
- “対処療法”に流れない設計で次の一手が決まっている

### 3. 対処療法を防ぐ設計パターン

#### ❌ 悪い例（対処療法）
```rust
// どこかのファイルに追加
if special_case {
    handle_special_case()
}
```

#### ✅ 良い例（構造的解決）
```rust
// 1. 専用モジュール作成
mod special_cases {
    pub fn handle() { }
}

// 2. README.mdに理由記載
// 3. テスト追加で仕様固定
```

### 4. AIへの実装依頼テンプレート

**実装依頼時は必ず以下を含めること**：

```markdown
## 実装内容
[具体的な内容]

## 構造設計
- [ ] 新規フォルダ/ファイルが必要か
- [ ] 各層のREADME.md更新が必要か
- [ ] インターフェース定義が必要か
- [ ] テストで仕様固定できるか

## 責務確認
- この実装はどの層の責務か: [layer]
- 他層への影響: [none/minimal/documented]

## 将来の拡張性
- 同様の問題が起きた時の対処: [構造的に解決済み]
```

### 5. 構造レビューチェックリスト

**PR前に必ず確認**：

- [ ] 各層の責務は守られているか
- [ ] README.mdは更新されているか
- [ ] テストは追加されているか
- [ ] 将来の開発者が迷わない構造か
- [ ] 対処療法的なif文を追加していないか

### 5.x JoinIR アーキテクチャの参照先

- JoinIR / Loop / If / ExitLine / Boundary の全体構造と箱の関係は  
  `docs/development/current/main/joinir-architecture-overview.md` を SSOT として使うよ。
- selfhost 側（.hako JoinIR フロントエンド）も、この設計を前提として設計・実装してね。

### 5.1 Hardcode 対応禁止ポリシー（重要）

スモークを通すためだけの「ハードコード」は原則禁止。必ず“根治（構造で直す）”を最優先にする。

- 禁止事項（例示）
  - by‑name ディスパッチでの一時しのぎ（例: `Box.method` 文字列一致での分岐）
  - 仕様外のショートカット（固定レジスタ/固定JSON断片の前提での if 分岐）
  - include/preinclude 依存を隠すためのテキスト置換や無条件スキップ
  - テスト専用の未ガード実装や CI 既定ON の暫定コード
- 許容される一時的措置（診断専用・既定OFF・削除計画付き）
  - dev トグルで厳格ガード（例: `FOO_DEV=1`）。既定OFF、prod/CI では無効
  - 安定タグの出力（例: `[dev/bridge:*]`）で検知可能にする
  - CURRENT_TASK.md に撤去条件・戻し手順・期限を明記
- 受け入れ基準（スモーク/CI）
  - dev トグルOFFで緑（prod 既定で通る）
  - ログに by‑name/dev タグが出ない（例: `[vm/byname:*]` 不在）
  - `.hako` 依存の using は AST ではなく text‑merge 経路で解決（`.nyash` は AST）
  - テストは構造/契約の検証に寄与し、対処療法に依存しない

PR テンプレ（追加項目）
- [ ] 一時的コードはありますか？ ある場合は DEV ガード/タグ/撤去計画を記載しましたか
- [ ] 「スモークを通すためだけのハードコード」を入れていません（根治で解決）
- [ ] 受け入れ基準に記載の検証を実施しました（dev OFF/ログ確認）

### 5.3 環境変数スパロー防止（今回の反省）
- 事件メモ: `NYASH_*` が大量に増殖し、実質未使用のトグルが多数発見された（2025-02-XX）。環境変数病を防ぐための運用ルールを追加する。
- 追加ルール:
  - 目的が明確なときだけ新設する。デフォルトOFFかつ撤去計画（どのフェーズで消すか）を `docs/reference/environment-variables.md` に必ず書く。
  - 実装は `src/config/env` に集約し、直読み禁止。棚卸し用に定義の所在を一元化する。
  - 期間限定トグルは dev/diagnostic 専用タグを付け、フェーズ完了時に削除する（CURRENT_TASK.md に期限を書く）。
  - 半期ごとに未使用（定義のみ）の変数を洗い出し、削除または非推奨化する。
  - ドキュメントに載せない「隠しトグル」は原則禁止。載せるか削除するかの二択。

### 5.2 Rust Minimal Policy（Self‑Host First, but not Frozen）

目的: 脱Rustを志向しつつも、Stage‑1 / Self‑Host ラインの整備やツールの使いやすさ向上のために、**Rust層で必要な構造的変更やブリッジ強化は積極的に許可する**。分析/ルール/可視化は引き続き .hako 側が主戦場。

- 原則
  - Rustは「SSOTランナー導線（resolve→parse→merge）」と「VM/Interpreterの安定化」を軸にしつつ、Stage‑1 CLI / selfhost ブリッジ / エラーメッセージ改善など**開発導線の改善**も扱ってよい。
  - 新規の言語ルール・静的解析ロジック・ビジネスロジックは、基本的に .hako 側で実装（自己ホスト）。
  - 変更はできるだけ小さく・可逆に保ちつつ、必要に応じてトグルや dev 用フラグでガード（AGENTS.md 5.1 に準拠）。

- 許可（Rustでやってよいことの例）
  - ランナー導線の保守・改善（Stage‑B/Stage‑1/Stage0 の配線、using text‑merge for .hako の維持・整理）。
  - Stage‑1 / Stage‑B / selfhost 向けのブリッジ強化（子プロセス起動、環境変数の集約、エラー表示の改善）。
  - バグ修正（既定挙動を壊さない範囲。必要なら既定OFFトグル）。
  - Analyzerシームの提供（AST/Analysis JSON の抽出専用。判定ロジックは持たない）。
  - 薄いCLI糖衣：`--hako-check` など、.hako 側のアナライザを呼び出すための CLI エントリ。

- 禁止/抑制（依然として .hako 側でやる領域）
  - Lint / 命名規則 / 依存関係チェック / 特定 Box 名への分岐など、**ルール実装の本体**。
  - LoopSSA / 数値カーネル / 高レベルの最適化ロジックを Rust に新規実装すること（Self‑Host で十分に表現できる範囲）。
  - 広域リファクタや既定挙動を大きく変える変更（必要なら Phase/Proposal に切り出してから）。

- .hako 側の責務（Self‑Host）
  - Lint / 解析 / 可視化 / 関係図（DOT）を .hako で実装（tools/hako_check/*）。
  - 解析入力は Rust の Analysis JSON（または AST JSON）。
  - 返り値は件数ベース（0=OK、>0=警告/エラー件数）。

- 受け入れ基準（Rust変更時）
  - quick スモーク / 代表テストが緑維持（既定挙動を意図せず変えていない）。
  - 変更は目的が明確で、小さく・可逆（トグルや限定スコープでガード）であること。
  - .hako からの利用例 / README / proposal を更新し、将来の開発者が迷わないようにしておく。

補足：.hako 側の解析・Stage‑1 ラインが十分に動き始めたので、Rust層は「完全凍結」ではなく、**Self‑Host を支えるための最小＋必要な整備を行う層**という扱いに緩和する。日常の機能拡張や言語仕様変更はこれまで通り .hako 側で行い、Rust 変更は「導線の改善」「ブリッジ」「可観測性の向上」にフォーカスする。

### 6. Fail-Fast with Structure

**構造的にFail-Fastを実現**：

```rust
// 層境界でのアサーション
#[cfg(debug_assertions)]
fn check_layer_boundary() {
    assert!(!module_path!().contains("mir"),
            "Parser cannot import MIR modules");
}
```

### 7. ドキュメント駆動開発

**実装前に必ずドキュメントを書く**：

1. まずREADME.mdに「何をするか」を書く
2. インターフェースを定義
3. テストを書く
4. 最後に実装

### 7.1 docs 置き場所 SSOT（迷子防止）

`docs/development/current/` は文書が増えやすいので、**入口/設計図/Phaseログ/調査ログを混ぜない**運用を必須にするよ。

- SSOT: `docs/development/current/main/DOCS_LAYOUT.md`
- よく参照する入口（例）:
  - JoinIR の地図（navigation SSOT）: `docs/development/current/main/design/joinir-design-map.md`
  - Loop Canonicalizer（設計 SSOT）: `docs/development/current/main/design/loop-canonicalizer.md`
  - MIR Builder（Context 分割の入口）: `src/mir/builder/README.md`
- 追加ルール（最小）:
  - 新しい Phase 文書は `docs/development/current/main/phases/` 配下に置く（`main/` 直下に増やさない）
  - 長期参照の設計図は `docs/development/current/main/design/` に置く
  - 切り分けログは `docs/development/current/main/investigations/` に置き、結論だけ `10-Now.md` / `20-Decisions.md` に反映する

---

**Fail-Fast原則**: フォールバック処理は原則禁止。過去に分岐ミスでエラー発見が遅れた経験から、エラーは早期に明示的に失敗させること。特にChatGPTが入れがちなフォールバック処理には要注意だよ！

**Feature Additions Pause — until Nyash VM bootstrap (2025‑09‑19 改訂)**
- 状態: マクロ基盤は安定。ここからは「凍結（全面停止）」ではなく「大きな機能追加のみ一時停止」。Nyash VM の立ち上げ（bootstrap）完了まで、安定化と自己ホスト/実アプリ開発を優先するよ。
- 原則（大規模機能追加の一時停止中）:
  - 大きな機能追加・仕様拡張は一時停止（Nyash VM 立ち上げまで保留）。
  - バグ修正・ドキュメント整備・スモーク/ゴールデン/CI強化・堅牢化（仕様不変）は続行OK。
  - 互換性を崩す変更は行わない。既定挙動は変えない（必要なら既定OFFのフラグでガード）。
- マクロ既定:
  - 既定ON（コード共有を重視）。CLI プロファイルで軽量化が可能。
  - 推奨ENV最小セット: `NYASH_MACRO_ENABLE=1`, `NYASH_MACRO_PATHS=...`, `NYASH_MACRO_STRICT=1`, `NYASH_MACRO_TRACE=0|1`
  - CLIプロファイル: `--profile {lite|dev|ci|strict}`（lite=マクロOFF、dev/ci/strict=マクロON）
- 非推奨（下位互換のみ）:
  - `NYASH_MACRO_BOX_NY*`, `NYASH_MACRO_BOX_CHILD_RUNNER`, `NYASH_MACRO_TOPLEVEL_ALLOW`（必要なら `--macro-top-level-allow` を明示）
- 自己ホスト前展開:
  - 自動（auto）で安全に有効化済み。PyVM 環境でのみ働く。問題時はログで検知しやすい。
- 受け入れチェック（ポーズ中のガード）:
  - cargo check（全体）/ 代表スモーク（PyVM/LLVM）/ マクロ・ゴールデンが緑であること。
  - 変更は最小・局所・仕様不変。既定挙動は変えない。

**機能追加ポリシー — 要旨**
- ねらい: 「誤解されやすい"凍結"」ではなく、「Nyash VM 立ち上げまで大きな機能追加は一時停止」。安定化・自己ホストの進行を最優先にするよ。
- 許可（継続OK）:
  - バグ修正（互換維持、仕様不変）
  - ドキュメント整備・コメント/ログ追加（既定OFFの詳細ログを含む）
  - スモーク/ゴールデン/CI 強化（既存ケースの安定性向上）
  - 堅牢化（パーサ/リゾルバ/結合の縫い目対策）※既定挙動は変えない、必要なら既定OFFのフラグでガード
- 一時停止（Nyash VM 立ち上げまで保留）:
  - 大きな機能追加・仕様拡張
  - 広域リファクタ・設計変更・デフォルト挙動変更
  - 依存追加や広範囲の拡張（点で直せるところは点で直す）
- 受け入れ条件（ガード）:
  - 既定挙動は不変（新フラグは既定OFF、影響は局所・可逆）
  - 差分は最小・目的は明確（unblock/安定化/診断）
  - 代表スモーク（PyVM/LLVM）・cargo check が緑
  - CURRENT_TASK.md に理由/範囲/フラグ名/戻し手順を記録
  - ロールバック容易（小さな差分、ガード除去で原状回復）

## Phase-21.5 Perf 実行ポリシー（AI runaway guard）
- 目的: AI が重い perf ladder を高頻度で回して開発速度を落とさないようにする。
- 実行レベル（固定）:
  - 日常（通常ループ）: 軽量のみ。
    - `bash tools/smokes/v2/profiles/integration/apps/phase21_5_perf_loop_integer_hotspot_contract_vm.sh`
    - `PERF_AOT=1 NYASH_LLVM_SKIP_BUILD=1 bash tools/perf/bench_compare_c_vs_hako.sh method_call_only_small 1 1`
  - 節目（PR前/5コミット毎）: quick ただし apps/stability は切る。
    - `PERF_LADDER_APPS=0 PERF_LADDER_STABILITY=0 PERF_LADDER_AOT_MEDIUM=0 NYASH_LLVM_SKIP_BUILD=1 tools/perf/run_progressive_ladder_21_5.sh quick`
  - フル（default）: 夜間または明示指示のみ。
- full 実行ガード:
  - `tools/perf/run_progressive_ladder_21_5.sh default` は最終 full 実行時刻を `target/perf_state/phase21_5_last_full_epoch` に保存する。
  - 規定では一定時間内の再実行をブロックする（`PERF_LADDER_FULL_MIN_INTERVAL_MIN`, 既定 720 分）。
  - 強制実行は `PERF_FORCE_FULL=1` を明示したときのみ許可する。
- AI への指示:
  - ユーザーの明示がない限り `default` を自動実行しない。
  - 通常は「日常」レベル、必要時のみ「節目」レベルへ上げる。

## JoinIR / 関数正規化 IR 方針（Phase 26-H 以降）

- 目的: Hakorune の制御構造（if / loop / break / continue / return）を、内部 IR 層で **関数と継続だけ**に正規化することで、LoopForm v2 / PHI / ExitLiveness の負担を構造的に軽くする。
- 原則:
  - 言語仕様（.hako 構文）はそのまま維持し、`AST → MIR/LoopForm → JoinIR → VM/LLVM` という層を追加して段階移行する。
  - JoinIR 以降では「関数呼び出し＋引数＋継続」だけで制御を表現する（`loop_step(i, k_exit)`, `join_after_if(x, k_exit)` など）。
  - 既存の LoopForm v2 / PhiBuilderBox / ExitPhiBuilder / LoopExitLiveness は、JoinIR 生成のための「構造箱・判定箱」として徐々に責務を縮退させる。
- 箱の層分け:
  - 構造箱: ControlForm / LoopForm / JoinIR 変換（どの関数・継続を作るかを決める）。
  - 判定箱: LoopVarClassBox / LoopExitLivenessBox / IfBodyLocalMergeBox（どの変数がどこまで生きるかを決める）。
  - 生成箱: PhiBuilderBox / ExitPhiBuilder / JoinIREmitter（MIR/JoinIR を実際に吐く）。
  - 検証箱: PhiInvariantsBox / MirVerifier / 将来の JoinIRVerifier（不変条件チェックのみ）。
- 運用ルール:
  - 新しい箱を追加する前に、「この箱は上のどの層か」を必ず決める（構造＋判定＋生成が 1 箱に混ざらないようにする）。
  - 1つの責務（例: Exit PHI, ExitLiveness）に対して箱が 3〜4 個を超えないようにし、増え過ぎたら SSOT 箱（PhiBuilderBox 等）に統合する。

## 開発時の箱の粒度・増減ポリシー

- 1箱 = 1つの質問だけに答える:
  - 例: 「この変数はどのクラスか？」「この変数は exit 後で live か？」など。
  - 1箱の中で「分類＋生成」や「判定＋検証」を混ぜない。
- 新しい箱を増やす前に:
  - 既存箱に複数の質問が混ざっていないかを確認し、必要なら分解を優先する。
  - 似た箱が既にないかを確認し、重複する場合は統合／移設の設計を docs（architecture/*.md, roadmap/phase-XX/README.md）に先に書く。
- フェーズ単位:
  - 「このフェーズで触る箱はここまで」と明示し、箱の追加はフェーズごとに小さく・戻しやすくする。
  - 大きな箱追加（JoinIR など）は必ず roadmap/phase-X の README と architecture/*.md にセットで設計を書いてから実装に入る。

**Cranelift 開発メモ（このブランチの主目的）**
- ここは Nyash の Cranelift JIT/AOT 開発用ブランチだよ。JIT 経路の実装・検証・計測が主対象だよ。
- ビルド（JIT有効）: `cargo build --release --features cranelift-jit`
- 実行モード:
  - CLI Cranelift: `./target/release/hakorune --backend cranelift apps/APP/main.hako`
  - JITダイレクト（VM非介入）: `./target/release/hakorune --jit-direct apps/smokes/jit_aot_string_min.hako`
- デバッグ環境変数（例）:
  - `NYASH_JIT_EXEC=1`（JIT実行許可）
  - `NYASH_JIT_STATS=1`（コンパイル/実行統計）
  - `NYASH_JIT_TRACE_IMPORT=1`（JITのimport解決ログ）
  - `NYASH_AOT_OBJECT_OUT=target/aot_objects/`（AOT .o 書き出し）
  - `NYASH_LEN_FORCE_BRIDGE=1`（一時回避: 文字列長をブリッジ経路に強制）
- 主要ファイル案内:
  - Lower/Builder: `src/jit/lower/core.rs`, `src/jit/lower/builder/cranelift.rs`
  - JITエンジン: `src/jit/engine.rs`, ポリシー: `src/jit/policy.rs`
  - バックエンド入口: `src/backend/cranelift/`
  - ランナー: `src/runner/modes/cranelift.rs`, `--jit-direct` は `src/runner/mod.rs`
- 進行中の論点と手順は `CURRENT_TASK.md` を参照してね（最新のデバッグ方針・フラグが載ってるよ）。

**PyVM ライン（Phase‑15 歴史メモ／現在は撤退済み）**
> 注: 2025-12 現在、PyVM 実行経路は完全撤退中だよ。以下は Phase‑15 当時の方針と運用メモで、今は「歴史情報」としてだけ残しているよ。日常の開発・CI では Rust VM / LLVM ラインだけを使ってね。
- 当時の主経路: Python/llvmlite + PyVM を標準の実行/検証経路として扱うよ。Rust VM/JIT は補助（保守/比較/プラグイン検証）。
- 使い分け:
  - PyVM（当時の推奨・日常確認）: `NYASH_DEV=1 NYASH_VM_USE_PY=1 ./target/release/hakorune --backend vm apps/APP/main.hako`
  - llvmlite ハーネス: `NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm apps/APP/main.hako`
  - パリティ検証: `tools/parity.sh --lhs pyvm --rhs llvmlite apps/tests/CASE.hako`
- 自己ホスト（Ny→JSON v0）: `NYASH_USE_NY_COMPILER=1` は emit‑only 既定で運用（`NYASH_NY_COMPILER_EMIT_ONLY=1`）。子プロセスは Quiet pipe（`NYASH_JSON_ONLY=1`）。
- 子プロセス安全策: タイムアウト `NYASH_NY_COMPILER_TIMEOUT_MS`（既定 2000ms）。違反時は kill→フォールバック（無限ループ抑止）。
- スモーク（代表）:
  - PyVM Stage‑2: `tools/historical/pyvm/pyvm_stage2_smoke.sh`
  - PHI/Stage‑2: `tools/ny_parser_stage2_phi_smoke.sh`
  - Bridge/Stage‑2: `tools/ny_stage2_bridge_smoke.sh`
  - 文字列/dirname など: `apps/tests/*.hako` を PyVM で都度確認
- 注意: Phase‑15 では VM/JIT は MIR14 以降の更新を最小とし、PyVM/llvmlite のパリティを最優先で維持するよ。

## Codex Async Workflow (Background Jobs)
- Purpose: run Codex tasks in the background and notify a tmux session on completion.
- Script: `tools/codex-async-notify.sh`
- Defaults: posts to tmux session `codex` (override with env `CODEX_DEFAULT_SESSION` or 2nd arg); logs to `~/.codex-async-work/logs/`.

Usage
- Quick run (sync output on terminal):
  - `./tools/codex-async-notify.sh "Your task here" [tmux_session]`
- Detached run (returns immediately):
  - `CODEX_ASYNC_DETACH=1 ./tools/codex-async-notify.sh "Your task" codex`
- Tail lines in tmux notification (default 60):
  - `CODEX_NOTIFY_TAIL=100 ./tools/codex-async-notify.sh "…" codex`

Concurrency Control
- Cap concurrent workers: set `CODEX_MAX_CONCURRENT=<N>` (0 or unset = unlimited).
- Mode when cap reached: `CODEX_CONCURRENCY_MODE=block|drop` (default `block`).
- De‑duplicate same task string: `CODEX_DEDUP=1` to skip if identical task is running.
- Example (max 2, dedup, detached):
  - `CODEX_MAX_CONCURRENT=2 CODEX_DEDUP=1 CODEX_ASYNC_DETACH=1 ./tools/codex-async-notify.sh "Refactor MIR 13" codex`

Keep Two Running
- Detect running Codex exec jobs precisely:
  - Default counts by PGID to treat a task with multiple processes (node/codex) as one: `CODEX_COUNT_MODE=pgid`
  - Raw process listing (debug): `pgrep -af 'codex.*exec'`
- Top up to 2 jobs example:
  - `COUNT=$(pgrep -af 'codex.*exec' | wc -l || true); NEEDED=$((2-${COUNT:-0})); for i in $(seq 1 $NEEDED); do CODEX_ASYNC_DETACH=1 ./tools/codex-async-notify.sh "<task $i>" codex; done`

Notes
- tmux notification uses `paste-buffer` to avoid broken lines; increase tail with `CODEX_NOTIFY_TAIL` if you need more context.
- Avoid running concurrent tasks that edit the same file; partition by area to prevent conflicts.
- If wrappers spawn multiple processes per task (node/codex), set `CODEX_COUNT_MODE=pgid` (default) to count unique process groups rather than raw processes.

## Dev Helpers
- 推奨フラグ一括: `source tools/dev_env.sh pyvm`（PyVMを既定、Bridge→PyVM直送: `NYASH_PIPE_USE_PYVM=1`）
- 解除: `source tools/dev_env.sh reset`

## Selfhost 子プロセスの引数透過（開発者向け）
- 親→子にスクリプト引数を渡す環境変数:
  - `NYASH_NY_COMPILER_MIN_JSON=1` → 子に `-- --min-json`
  - `NYASH_SELFHOST_READ_TMP=1`    → 子に `-- --read-tmp`（`tmp/ny_parser_input.ny` を FileBox で読み込む。CIでは未使用）
  - `NYASH_NY_COMPILER_STAGE3=1`   → 子に `-- --stage3`（Stage‑3 構文受理: Break/Continue/Throw/Try）
  - `NYASH_NY_COMPILER_CHILD_ARGS` → スペース区切りで子にそのまま渡す
- 子側（apps/selfhost-compiler/compiler.hako）は `--read-tmp` を受理して `tmp/ny_parser_input.ny` を読む（plugins 必要）。

## PyVM Scope & Policy（Stage‑2 開発用の範囲・歴史メモ）
> 注: このセクションも Phase‑15〜17 期の設計メモだよ。PyVM ラインは現在は撤退済みで、実行確認・CI は Rust VM / LLVM のみを対象にしているよ。
- 目的: （当時）PyVM は「開発用の参照実行器」だよ。JSON v0 → MIR 実行の意味論確認と llvmlite とのパリティ監視に使う（プロダクション最適化はしない）。
- 必須命令: `const/binop/compare/branch/jump/ret/phi`、`call/externcall/boxcall`（最小）。
- Box/メソッド（最小実装）:
  - ConsoleBox: `print/println/log`
  - String: `length/substring/lastIndexOf/esc_json`、文字列連結（`+`）
  - ArrayBox: `size/len/get/set/push/toString`
  - MapBox: `size/has/get/set/toString`（キーは文字列前提）
  - FileBox: 読み取り限定の `open/read/close`（必要最小）
  - PathBox: `dirname/join`（POSIX 風の最小）
- 真偽・短絡: 比較は i64 0/1、分岐は truthy 規約。`&&`/`||` は分岐+PHI で短絡を表現（副作用なしは Bridge、ありは PyVM 側で検証）。
- エントリ/終了: `--entry` 省略時に `Main.main`/`main` を自動解決。整数は exit code に反映、bool は 0/1。
- 非対象（やらない）: プラグイン動的ロード/ABI、GC/スケジューラ、例外/非同期、大きな I/O/OS 依存、性能最適化。
- 運用ポリシー: 仕様差は llvmlite に合わせて PyVM を調整。未知の extern/boxcall は安全に `None`/no-op。既定は静音、`NYASH_CLI_VERBOSE=1` で詳細。
- 実行とスモーク:
  - PyVM 実行: `NYASH_DEV=1 NYASH_VM_USE_PY=1 ./target/release/hakorune --backend vm apps/tests/CASE.hako`
  - 代表スクリプト: `tools/historical/pyvm/pyvm_stage2_smoke.sh`, `tools/historical/pyvm/pyvm_collections_smoke.sh`, `tools/historical/pyvm/pyvm_stage2_dot_chain_smoke.sh`
  - Bridge 短絡（RHS スキップ）: `tools/ny_stage2_shortcircuit_smoke.sh`
- CI: `.github/workflows/pyvm-smoke.yml` を常時緑に維持。LLVM18 がある環境では `tools/parity.sh --lhs pyvm --rhs llvmlite` を任意ジョブで回す。

## Interpreter vs PyVM（実行経路の役割と優先度・歴史メモ）
> 注: ここで言う「優先経路: PyVM」は Phase‑15 期のものだよ。現在は PyVM ラインは撤退済みで、Rust VM / LLVM を優先経路として扱うよ。
- （当時の方針）優先経路: PyVM（Python）を"意味論リファレンス実行器"として採用。日常の機能確認・CI の軽量ゲート・llvmlite とのパリティ監視を PyVM で行う。
- 補助経路: Rust の MIR Interpreter は純Rust単独で回る簡易器として維持。拡張はしない（BoxCall 等の未対応は既知）。Python が使えない環境での簡易再現や Pipe ブリッジの補助に限定。
- Bridge（--ny-parser-pipe）: 既定は Rust MIR Interpreter を使用。副作用なしの短絡など、実装範囲内を確認。副作用を含む実行検証は PyVM スモーク側で担保。
- 開発の原則: 仕様差が出た場合、llvmlite に合わせて PyVM を優先調整。Rust Interpreter は保守維持（安全修正のみ）。

## 脱Rust（開発効率最優先）ポリシー
- Phase‑15 中は Rust VM/JIT への新規機能追加を最小化し、Python（llvmlite/PyVM）側での実装・検証を優先する。
- Runner/Bridge は必要最小の配線のみ（子プロセスタイムアウト・静音・フォールバック）。意味論の追加はまず PyVM/llvmlite に実装し、必要時のみ Rust 側へ反映。

## Self‑Hosting への移行（PyVM → Nyash）ロードマップ（歴史メモ）
> 注: このロードマップは「PyVM からの段階移行」を前提にした初期案だよ。現在は PyVM ライン自体が撤退しており、Rust VM / LLVM / Nyash 自己ホストの 3 本を前提に設計を進めているよ。
- 目標: （当時の計画）PyVM の最小実行器を Nyash スクリプトへ段階移植し、自己ホスト中も Python 依存を徐々に縮小する。
- ステップ（小粒度）:
  1) Nyash で MIR(JSON) ローダ（ファイル→構造体）を実装（最小 op セット）。
  2) const/binop/compare/branch/jump/ret/phi を Nyash で実装し、既存 PyVM スモークを通過。
  3) call/externcall/boxcall（最小）・String/Array/Map の必要メソッドを Nyash で薄く実装。
  4) CI は当面 PyVM を主、Nyash 実装は実験ジョブとして並走→安定後に切替検討。
- 注意: 本移行は自己ホストの進捗に合わせて段階実施（Phase‑15 では設計・骨格の準備のみ）。

## ⚠ 現状の安定度に関する重要メモ（Phase‑15 進行中）
- VM と Cranelift(JIT) は MIR14 へ移行中のため、現在は実行経路として安定していないよ（検証・実装作業の都合で壊れている場合があるにゃ）。
- 当面の実行・配布は LLVM ラインを最優先・全力で整備する方針だよ。開発・確認は `--features llvm` を有効にして進めてね。
- 推奨チェック:
  - LLVM は llvmlite ハーネス（Python）経由だよ。Rust inkwell は既定で不使用（legacy のみ）。
  - ビルド（ハーネス）: `cargo build --release --features llvm -j 24`
  - チェック: `cargo check --features llvm`

## Docs links（開発方針/スタイル）
- Language statements (ASI): `docs/reference/language/statements.md`
- using 文の方針: `docs/reference/language/using.md`
- Nyash ソースのスタイルガイド: `docs/guides/style-guide.md`
- Stage‑2 EBNF: `docs/reference/language/EBNF.md`
- Macro profiles: `docs/guides/macro-profiles.md`
- Template → Macro 統合方針: `docs/guides/template-unification.md`
- User Macros（MacroBox/Phase 2）: `docs/guides/user-macros.md`
- Macro capabilities (io/net/env): `docs/reference/macro/capabilities.md`
- LoopForm ガイド: `docs/guides/loopform.md`
- Phase‑17（LoopForm Self‑Hosting & Polish）: `docs/private/roadmap2/phases/phase-17-loopform-selfhost/`
- MacroBox（ユーザー拡張）: `docs/guides/macro-box.md`
- MacroBox in Nyash（設計草案）: `docs/guides/macro-box-nyash.md`
- MIR デバッグ総覧（dump/hints/__mir__）: `docs/guides/testing-guide.md`（`MIR デバッグの入口まとめ` セクション）
- VM step budget exceeded（無限ループ）切り分け: `docs/guides/testing-guide.md`（`VM step budget exceeded` セクション）

# Repository Guidelines

## Project Structure & Module Organization
- `src/`: Nyash core (MIR, backends, runner modes). Key: `backend/`, `runner/`, `mir/`.
- `crates/nyrt/`: NyRT static runtime for AOT/LLVM (`libnyrt.a`).
- `plugins/`: First‑party plugins (e.g., `nyash-array-plugin`).
- `apps/` and `examples/`: Small runnable samples and smokes.
- `tools/`: Helper scripts (build, smoke).
- `tests/`: Rust and Nyash tests; historical samples in `tests/archive/`.
- `nyash.toml`: Box type/plug‑in mapping used by runtime.

## Build, Test, and Development Commands
- Build (JIT/VM): `cargo build --release --features cranelift-jit`
- Build (LLVM AOT / harness-first):
  - `cargo build --release -p nyash-llvm-compiler` (ny-llvmc builder)
  - `cargo build --release --features llvm`
  - Run via harness: `NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm apps/APP/main.hako`
- LLVM harness 導線 SSOT: `CLAUDE.md`（重複手順はここに増やさない）。入口は `tools/run_llvm_harness.sh <program.hako>`
- Quick VM run: `./target/release/hakorune --backend vm apps/APP/main.hako`
- Emit + link (LLVM): `tools/build_llvm.sh apps/APP/main.hako -o app`
- Smokes (v2):
  - Single entry: `tools/smokes/v2/run.sh --profile quick`
  - Profiles: `quick|integration|full`（`--filter <glob>` で絞り込み）
  - 個別: `bash tools/smokes/v2/profiles/quick/core/using_named.sh`
  - メモ: v2 ランタイムは自動でルート検出するので、CWD は任意（テスト中に /tmp へ移動してもOK）
  - 旧スモークは廃止（tools/test/smoke/*）。最新仕様のみを対象にするため、v2 のみ維持・拡充する。
  - 補助スイート（任意）: `./tools/smokes/v2/run.sh --profile plugins`（dylib using の自動読み込み検証など、プラグイン固有のチェックを隔離）

## CI Policy（開発段階の最小ガード）

開発段階では CI を"最小限＋高速"に保つ。むやみにジョブや行程を増やさない。

- 原則（最小ガード）
  - ビルドのみ: `cargo build --release`
  - 代表スモーク（軽量）: `tools/smokes/v2/run.sh --profile quick`
  - 以上で失敗しないこと（0 exit）が最低基準。重い/広範囲のマトリクスは導入しない。

- 禁止/抑制
  - 追加の CI ワークフローや大規模マトリクスの新設（フェーズ中は保留）
  - フル/統合（integration/full）を既定で回すこと（ローカル/任意ジョブに留める）
  - 外部環境依存のテスト（ネットワーク/GUI/長時間 I/O）

- 任意（ローカル/手元）
  - プラグイン検証: `tools/smokes/v2/run.sh --profile plugins`（フィクスチャ .so は未配置なら SKIP、配置時に PASS）
  - LLVM/ハーネス確認: `tools/smokes/v2/run.sh --profile integration`

- ログ/出力
  - v2 ランナーはデフォルトで冗長ログをフィルタ済み（比較に混ざらない）。
  - JSON/JUnit 出力は"必要時のみ" CI で収集。既定では OFF（テキスト出力で十分）。

- タイムアウト・安定性
  - quick プロファイルの既定タイムアウトは短め（15s 程度）。CI はこの既定を尊重。
  - テストは SKIP を活用（プラグイン未配置/環境依存は SKIP で緑を維持）。

- 変更時の注意
  - v2 スモークの追加は"狭く軽い"ものから。既存の quick を重くしない。
  - 重い検証（integration/full）はローカル推奨。必要なら単発任意ジョブに限定。

## Runtime Lines Policy（VM/LLVM 方針）
- 軸（2025 Phase‑15+）
  - Rust VM ライン（主経路）: 実行は Rust VM を既定にする。プラグインは動的ロード（.so/.dll）で扱う。
  - LLVM ライン（AOT/ハーネス）: 生成/リンクは静的（`libnyrt.a` や静的プラグイン）を基本とし、実行は LLVM で検証する。

- プラグインの扱い
  - Rust VM: 動的プラグイン（ランタイムでロード）。構成は `nyash.toml` の [plugins] / `ny_plugins` に従う。
  - LLVM: 静的リンクを前提（AOT/harness）。必要に応じ `nyrt`/静的プラグインにまとめる。

- using/namespace の解決
  - using は Runner 側で解決（Phase‑15）。`nyash.toml` の `[using]`（paths / <name> / aliases）を参照。
  - include は廃止。`using "./path/file.hako" as Name` を推奨。

- スモーク/検証の方針
  - 既定の開発確認は Rust VM ラインで行い、LLVM ラインは AOT/ハーネスの代表スモークでカバー。
  - v2 ランナーは実行系を切り替え可能（環境変数・引数で VM/LLVM/（必要時）PyVM を選択）。
  - PyVM は参照実行器（保守最小）。言語機能の確認や LLVM ハーネスのパリティ検証が主目的で、既定経路では使わない。

- 実行例（目安）
  - Rust VM（既定）: `./target/release/hakorune apps/APP/main.hako`
  - LLVM Harness: `NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm apps/APP/main.hako`
  - AOT ビルド: `tools/build_llvm.sh apps/APP/main.hako -o app`

- セルフホスティング指針
  - 本方針（Rust VM=主、LLVM=AOT）はそのまま自己ホストの軸にする。
  - 互換性を崩さず、小粒に前進（VM ↔ LLVM のスモークを保ちつつ実行経路を磨く）。

## JIT Self‑Host Quickstart (Phase 15)
- Core build (JIT): `cargo build --release --features cranelift-jit`
- Core smokes (plugins disabled): `NYASH_CLI_VERBOSE=1 ./tools/jit_smoke.sh`
- Roundtrip (parser pipe + json): `./tools/ny_roundtrip_smoke.sh`
- Plugins smoke (optional gate): `NYASH_SKIP_TOML_ENV=1 ./tools/smoke_plugins.sh`
- Using/Resolver E2E sample (optional): `./tools/using_e2e_smoke.sh` (requires `--enable-using`)
- Bootstrap c0→c1→c1' (optional gate): `./tools/bootstrap_selfhost_smoke.sh`

Flags
- `NYASH_DISABLE_PLUGINS=1`: Core経路安定化（CI常時/デフォルト）
- `NYASH_LOAD_NY_PLUGINS=1`: `nyash.toml` の `ny_plugins` を読み込む（std Ny実装を有効化）
- `--enable-using` or `NYASH_ENABLE_USING=1`: using/namespace を有効化
- `NYASH_SKIP_TOML_ENV=1`: nyash.toml の [env] 反映を抑止（任意ジョブの分離に）
- `NYASH_PLUGINS_STRICT=1`: プラグインsmokeでCore‑13厳格をONにする
- `NYASH_USE_NY_COMPILER=1`: NyコンパイラMVP経路を有効化（Rust parserがフォールバック）

## Phase 15 Policy（Self‑Hosting 集中ガイド）
- フォーカス: Ny→MIR→VM/JIT（JITはcompiler‑only/独立実行）での自己ホスト実用化。
- スコープ外（Do‑Not‑Do）: AOT/リンク最適化、GUI/egui拡張、過剰な機能追加、広域リファクタ、最適化の深追い、新規依存追加。
- ガードレール:
  - 小刻み: 作業は半日粒度。詰まったら撤退→Issue化→次タスクにスイッチ。
  - 検証: 代表スモーク（Roundtrip/using/modules/JIT直/collections）を常時維持。VMとJIT(--jit-direct)の一致が受け入れ基準。
  - 観測: hostcall イベントは 1 呼び出し=1 件、短絡は分岐採用の記録のみ。ノイズ増は回避。
  - LLVM/PHI: ハーネスでは「PHI は常にブロック先頭にグループ化」「incoming は型付き (i64 v, %bb)」の不変条件を厳守。PHI の生成・配線は `phi_wiring` に一元化する。

## LLVM Harness — PHI Invariants & Debug

- Invariants
  - PHI nodes are created at the block head only (grouped at top).
  - Incoming pairs are always well-typed: `i64 <value>, %bb<id>`.
  - Placeholder PHIs are not materialized during prepasses; only metadata is recorded.
  - Finalization (`phi_wiring.finalize_phis`) ensures creation and wiring; no empty PHI remains.

- Implementation notes
  - Prepass metadata: `phi_wiring.tagging.setup_phi_placeholders` collects declared PHIs and records `block_phi_incomings`; it does not call `ensure_phi` anymore.
  - Wiring: `phi_wiring.wiring.ensure_phi` places PHI at the block head; `wire_incomings` resolves per-pred values and normalizes to i64.
  - Safety valve: `llvm_builder.compile_to_object` sanitizes IR text to drop malformed empty PHIs (should be unreachable in normal flow).

- How to run harness
  - Build: `cargo build --release -p nyash-llvm-compiler && cargo build --release --features llvm`
  - Run: `NYASH_LLVM_USE_HARNESS=1 ./target/release/hakorune --backend llvm apps/tests/peek_expr_block.hako`
  - IR dump: `NYASH_LLVM_DUMP_IR=tmp/nyash_harness.ll ...`
  - PHI trace: `NYASH_LLVM_TRACE_PHI=1 ...` (JSON lines output via `phi_wiring.common.trace`)

## Match Guards — Parser & Lowering Policy

- Syntax: `case <pattern> [if <cond>] => <expr|block>` within `match <expr> { ... }`.
- Patterns (MVP): literals (with `|`), type patterns like `StringBox(s)`.
- Semantics:
  - Default `_` does not accept guards (parse error by design).
  - Without type/guard: lowers to PeekExpr for legacy path.
  - With type/guard: lowers to nested If-chain; guard is evaluated inside then-branch (after type bind for type patterns).
- Notes:
  - is/as TypeOp mapping normalizes common Box names to primitives (e.g., `StringBox` → String) for parity across VM/JIT/LLVM.
  - VM/PyVM may require bridging for primitive↔Box checks; keep guard tests for literal strict, type guard as warning until parity is complete.
- 3日スタートプラン:
  1) JSON v0 短絡 &&/|| を JSON→MIR→VM→JIT の順で最小実装。短絡副作用なしを smoke で確認。
  2) collections 最小 hostcall（len/get/set/push/size/has）と policy ガードの整合性チェック。
  3) 観測イベント（observe::lower_hostcall / lower_shortcircuit）を整備し、代表ケースで一貫した出力を確認。

## Coding Style & Naming Conventions
- Rust style (rustfmt defaults): 4‑space indent, `snake_case` for functions/vars, `CamelCase` for types.
- Keep patches focused; align with existing modules and file layout.
- New public APIs: document minimal usage and expected ABI (if exposed to NyRT/plug‑ins).

## Testing Guidelines
- Rust tests: `cargo test` (add targeted unit tests near code).
- Smoke scripts validate end‑to‑end AOT/JIT (`tools/llvm_smoke.sh`).
- Test naming: prefer `*_test.rs` for Rust and descriptive `.hako` files under `apps/` or `tests/`.
- For LLVM tests, ensure Python llvmlite is available and `ny-llvmc` is built.
- Build (harness): `cargo build --release -p nyash-llvm-compiler && cargo build --release --features llvm`

## Selfhost Bringup Policy (hako_check)
- When selfhost tools (`tools/hako_check/*`) fail due to JoinIR/CorePlan unsupported loop shapes, prefer strengthening the compiler (CorePlan/Facts/Composer/Lowerer) over rewriting `.hako` to “fit”.
- Keep analyzer runs deterministic: `NYASH_DISABLE_PLUGINS=1` stays ON unless there is a documented, unavoidable dependency.
- SSOT: `docs/development/current/main/design/selfhost-coreplan-unblocking-policy.md`

## Commit & Pull Request Guidelines
- Commits: concise imperative subject; scope the change (e.g., "llvm: fix argc handling in nyrt").
- PRs must include: description, rationale, reproduction (if bug), and run instructions.
- Link issues (`docs/development/issues/*.md`) and reference affected scripts (e.g., `tools/llvm_smoke.sh`).
- CI: ensure smokes pass; use env toggles in the workflow as needed.

## Security & Configuration Tips
- Do not commit secrets. Plug‑in paths and native libs are configured via `nyash.toml`.
- LLVM builds require system LLVM 18; install via apt.llvm.org in CI.
- Optional logs: enable `NYASH_CLI_VERBOSE=1` for detailed emit diagnostics.
- LLVM harness safety valve (dev only): set `NYASH_LLVM_SANITIZE_EMPTY_PHI=1` to drop malformed empty PHI lines from IR before llvmlite parses it. Keep OFF for normal runs; use only to unblock bring-up when `finalize_phis` is being debugged.

### LLVM Python Builder Layout (after split)
- Files (under `src/llvm_py/`):
  - `llvm_builder.py`: top-level orchestration; delegates to builders.
  - `builders/entry.py`: `ensure_ny_main(builder)` – create ny_main wrapper if needed.
  - `builders/function_lower.py`: `lower_function(builder, func_json)` – per-function lowering (CFG, PHI metadata, loop prepass, finalize_phis).
  - `builders/block_lower.py`: `lower_blocks(builder, func, block_by_id, order, loop_plan)` – block-local lowering and snapshots.
  - `builders/instruction_lower.py`: `lower_instruction(owner, builder, inst, func)` – per-instruction dispatch.
- Dev toggles:
  - `NYASH_LLVM_DUMP_IR=<path>` – dump IR text for inspection.
  - `NYASH_LLVM_PREPASS_IFMERGE=1` – enable return-merge PHI predeclare metadata.
  - `NYASH_LLVM_PREPASS_LOOP=1` – enable simple while prepass (loopform synthesis).
  - `NYASH_CLI_VERBOSE=1` – extra trace from builder.
- Smokes:
  - Empty PHI guard: `tools/test/smoke/llvm/ir_phi_empty_check.sh <file.hako>`
  - Batch run: `tools/test/smoke/llvm/ir_phi_empty_check_all.sh`
