# Join-Explicit CFG Construction

Status: SSOT（design goal）  
Scope: JoinIR→MIR の「暗黙 ABI」を消し、Join を第一級に扱う CFG へ収束させる北極星（north star）。  
Related:
- Navigation SSOT: `docs/development/current/main/design/joinir-design-map.md`
- Investigation (Phase 256): `docs/development/current/main/investigations/phase-256-joinir-contract-questions.md`
- Structured→CFG lowering SSOT（Frag/ExitKind）: `docs/development/current/main/design/edgecfg-fragments.md`
- Decisions: `docs/development/current/main/20-Decisions.md`
- Future features (catch/cleanup, cleanup/defer, async): `docs/development/current/main/design/exception-cleanup-async.md`

## Goal（最終形）

“Join-Explicit CFG Construction” を目指す。

- `Jump/continuation/params/edge-args` を **第一級（explicit）**として扱う
- JoinIR↔MIR 間の **暗黙 ABI（順序/長さ/名前/役割）** をなくす（SSOT を 1 箇所に封印）
- 変換は「意味の解釈」ではなく「写像（mapping）」に縮退する

## Non-Goals（いまはやらない）

- JoinIR を即座に削除する（まずは ABI/Contract で SSOT を固める）
- PHI を全面廃止して block params に置換する一括リファクタ（段階導入）

## 現状の問題（Phase 256 で露出した型）

- `jump_args` / `exit_bindings` / `entry.params` / `boundary.join_inputs` が “だいたい同じ順序” を前提にしており、ズレると SSA/dominance が破綻する
- `expr_result` と LoopState carrier が同一 ValueId になり得るが、legacy “expr_result slot” 推測で offset がずれて誤配線になる
- `jump_args` が IR の外側メタ扱いだと、DCE/最適化が “use” を見落としやすい
- spans が並行 Vec だと、パスが 1 箇所でも取りこぼすと SPAN MISMATCH になる

## 方針の核（Phase 259+）

“正規化（normalized）” を 2 つに分けて SSOT を縮退させる：

1. **Semantic Normalization（意味SSOT）**
   - terminator 語彙を固定し、意味の揺れを禁止する
   - 例: `cond 付き Jump` を **正規形から禁止**し、`Branch` に落とす
2. **Plumbing Normalization（配線SSOT）**
   - edge-args / CFG successor / spans など「壊れやすい配線」を IR 構造に閉じ込める
   - 目標: “忘れると壊れるメタ” を減らし、変換を写像に縮退させる

これにより、パターン追加が「意味SSOTに従う局所変更」になり、merge/optimizer 側の推測や補正が増殖しにくくなる。

### 具体例: Pattern8 契約（Phase 259 P0）

Pattern8（BoolPredicateScan）の実装で明示した契約要素（"pattern増でも推測増にしない"の実例）:

- **`loop_var_name`**: merge_entry_block 選択に使用（`Some(parts.loop_var.clone())`）
  - 未設定だと誤った entry block が選ばれる
- **`loop_invariants`**: PHI-free 不変量パラメータ（`[(me, me_host), (s, s_host)]`）
  - `loop_var_name` 設定時、BoundaryInjector が ALL join_inputs Copy をスキップするため必要
  - 不変量は header PHI で持つ（Pattern6 と同じ設計）
- **`expr_result`**: k_exit からの返り値を明示（`Some(join_exit_value)`）
  - Pattern7 style（推測ではなく明示設定）
- **`jump_args_layout`**: ExprResultPlusCarriers（carriers=0）
  - Pattern8 は carriers なし、expr_result のみ
- **`exit_bindings`**: Empty
  - carriers なしなので binding も不要

これらを「boundary builder で明示」することで、merge 側の推測を完全に排除。

## 最小の箱（Box）構成（小さく強く）

- `NormalizeBox`（意味SSOT）: Structured → Normalized、terminator 語彙の固定、Fail-Fast verify
- `AbiBox`（役割/順序SSOT）: `JoinAbi`（sig/roles/special/alias）で暗黙 ABI を封印し、pack/unpack を一箇所に集約
- `EdgeArgsPlumbingBox`（配線SSOT）: edge-args を terminator operand に寄せる段階導入、CFG/spans の同期点を一本化

増やす基準: 同じ不変条件を 2 箇所以上で守り始めたら箱を追加し、参照点を 1 箇所に縮退させる。

## 移行戦略（段階導入 / Strangler）

原則:
- **移行を先に固定**し、機能追加は「新契約に乗るものだけ」併走する（旧経路に新機能を足さない）
- 既定挙動を変えない（必要なら dev-only の診断ガードで観測）

### Stage 1（短期）: JoinIR を “ABI/Contract 付き Normalized SSOT” にする

狙い: 推測をなくし、順序/役割の SSOT を 1 箇所へ寄せる。

- boundary に `jump_args_layout` のような **layout SSOT** を持たせ、collector/rewriter が推測しない
- terminator 語彙を固定し、`cond 付き Jump` を `Branch` へ寄せる（正規形から禁止）
- continuation の識別は **ID SSOT**（String は debug/serialize のみに縮退）

受け入れ:
- `--verify` が PASS（SSA/dominance/PHI/ExitLine の契約違反が消える）
- 直撃回帰テスト（`expr_result == carrier` 等）が固定される

### Stage 2（中期）: MIR を “edge-args を terminator operand に持つ CFG” に寄せる

狙い: `jump_args` を “意味データ” として IR に埋め込み、DCE/CFG が自然に追える形へ収束する。

- `jump_args` を BasicBlock メタから terminator operand へ寄せる（段階導入: 互換フィールド併存→移行→削除）
- “参照 API” は Branch を含むので **複数 edge を前提**にする（単発 `edge_args()` は曖昧になりやすい）
  - 例: `block.out_edges()` / `block.edge_args_to(target)`
- terminator operand 側は `Vec<ValueId>` だけでなく **意味（layout）**も同梱する
  - 例: `EdgeArgs { layout: JumpArgsLayout, values: Vec<ValueId> }`
- spans は `Vec<Spanned<_>>` へ（API で不変条件を守る）

受け入れ:
- `jump_args` 由来の use が最適化で消えない（テストで固定）
- SPAN MISMATCH が構造的に起きない

### Stage 3（長期）: JoinIR と MIR の境界を薄くし、必要なら JoinIR を builder へ降格

狙い: “bridge/merge が意味解釈する余地” を最小化し、一本の CFG 語彙に収束させる。

- JoinIR を SSOT IR として残すか、builder DSL として降格するかは、この段階で再判断する

## 実務ルール（Phase 中の運用）

- 新パターン/新機能は「新しい Contract で記述できる場合のみ」追加する
- Contract の導入中は “機能追加より SSOT 固め” を優先する（泥沼デバッグの再発防止）

## API の作り方（迷子防止）

Strangler 期間は “読む側だけ寄せる” と取りこぼしが起きやすい。読む側/書く側を両方とも API に閉じ込める。

- **読む側（参照点の一本化）**
  - `out_edges()` のように edge を列挙できる API を SSOT にする（`Jump`/`Branch` を同じ形で扱える）
  - 旧メタ（`jump_args`）は API 内部でのみ参照し、外部は見ない
- **書く側（terminator 更新の一本化）**
  - `set_terminator(...)` のような入口に寄せ、successors/preds の同期漏れを構造で潰す
- **verify（Fail-Fast）**
  - terminator から計算した successors と、キャッシュ `block.successors` の一致をチェックして “同期漏れ” を即死させる

## pattern（番号）の位置づけ（収束のさせ方）

重要: “pattern番号で分岐する” こと自体は長期的には消したい（臭い）。ただし **IR上の形（terminator/exit種別）で分岐する**のは普通で、むしろ正しい。

収束方針:

- EdgeCFG の基盤が固まった後、Structured→CFG lowering の中心概念を **pattern番号**ではなく **ExitKind と Frag（fragment）**に移す
- pattern は “Extractor（形の認識）/ Plan（最小要件の抽出）” までに縮退し、merge/配線層へ逆流させない
- 最終的に残る実装は `seq/if/loop/cleanup` 等の **合成則**と、`join(block params)` だけになる

設計SSOT: `docs/development/current/main/design/edgecfg-fragments.md`
