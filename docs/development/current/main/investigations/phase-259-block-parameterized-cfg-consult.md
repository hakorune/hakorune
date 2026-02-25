# Phase 259: Block-Parameterized CFG / Join-Explicit CFG — ChatGPT Pro 相談パケット

目的: ChatGPT Pro に「正規化（normalized）をどう設計し、小さい強い箱（Box）で結び、block-parameterized CFG へ段階移行するか」を相談するためのコピペ用 SSOT を用意する。

更新日: 2025-12-20

---

## 1) ChatGPT Pro へ投げる文章（コピペ用）

以下の条件で、段階移行できる設計を提案してください。

### コンテキスト

Nyash/Hakorune の JoinIR→MIR 経路で、`Jump/continuation/params/edge-args` が「暗黙 ABI（順序/長さ/役割/識別）」として散在し、パターン追加のたびに merge/optimizer/verify が連鎖で壊れました。最近は SSOT 化と Fail-Fast を進め、`JumpArgsLayout` を boundary に持たせて推測を排除しましたが、最終的には **block-parameterized CFG**（edge-args を第一級に持つ CFG）に収束させたいです。

北極星（north star）: “Join-Explicit CFG Construction”

### ゴール（最終形）

- `Jump/continuation/params/edge-args` を第一級（explicit）として扱う
- JoinIR↔MIR 間の暗黙 ABI を消し、変換を「意味解釈」ではなく「写像（mapping）」に縮退する
- 長期的には JoinIR を builder DSL に降格できる状態にする（削除は急がない）

### 制約 / 方針

- フォールバックより Fail-Fast（prod/CI 既定で安全）
- by-name ハードコード分岐は禁止（特定 Box 名での条件分岐など）
- 新しい環境変数の増殖は禁止（既存 debug gate の範囲で）
- 段階導入（Strangler）で、差分を小さく・可逆に

### 質問（回答してほしいこと）

1. **Normalized JoinIR の最小語彙**は何がよいか？
   - `Jump(args)` / `Branch(then_args, else_args)` / `Return` / `Call` のセット
   - `cond 付き Jump` を禁止し、`Branch` に寄せるべきか
2. **ABI（役割・順序）SSOT をどこに置くべきか？**
   - `JoinAbi`（sig/roles/special conts/alias）を新設するか、`JoinModule` に持たせるか
   - boundary の `join_inputs/host_inputs` を Vec 順序のまま段階移行する案（最小差分）と、`ParamId→ValueId` 束縛にする案（最終形）
3. **block-parameterized CFG 移行の最小ステップ**を、Phase で刻んで提示してほしい
   - `jump_args` を BasicBlock メタから terminator operand に埋め込む手順（併存→移行→削除）
   - verify / optimizer / printer / builder への影響の最小化
4. **箱（Box）分割**は最小でどう切るべきか？
   - “推測禁止” と “Fail-Fast” をどの箱に閉じ込めるか
   - 例: `AbiBox`, `BoundaryContractCheckBox`, `ExitArgsPlumbingBox`, `CfgSuccessorSyncBox`
5. **不変条件（invariants）**と検証地点の設計
   - Normalizer直後 / merge直前 / `--verify` の各地点で、何を verify するのが最短で強いか

### 期待する出力（形式）

- 「いきなり最終形」ではなく、**段階移行（1フェーズ=小差分）**のマイルストーンを提示
- 各フェーズの **受け入れ基準**（smoke/verify/contract）を明文化
- 新規箱は最小（2〜3個から開始）で、増やす判断基準も書く

---

## 2) リポジトリ現状（SSOTメモ）

### 最近の到達点（コミット）

- `73ddc5f58` feat(joinir): Phase 257 P1.1/P1.2/P1.3（Pattern6 SSOT / PHI predecessor verify / LoopHeaderPhi 修正）
- `23531bf64` feat(joinir): Phase 258 P0（index_of_string dynamic needle window scan）
- `e4f57ea83` docs: Phase 257-259 SSOT 更新

### 現在の “意味データが IR 外” の例

- `jump_args` が `BasicBlock` のメタとして存在（DCE/verify が追う必要がある）
- spans が `instructions` と並行 Vec（同期漏れで SPAN MISMATCH が起きる）

---

## 3) 最低限のソースコード断片（Pro に見せる用）

### 3.1 `BasicBlock`（jump_args と span の現状）

`src/mir/basic_block.rs:45`

```rust
pub struct BasicBlock {
    pub id: BasicBlockId,
    pub instructions: Vec<MirInstruction>,
    pub instruction_spans: Vec<Span>,
    pub terminator: Option<MirInstruction>,
    pub terminator_span: Option<Span>,
    pub predecessors: BTreeSet<BasicBlockId>,
    pub successors: BTreeSet<BasicBlockId>,
    // ...
    pub jump_args: Option<Vec<ValueId>>, // Phase 246-EX: Jump args metadata
}
```

### 3.2 `JumpArgsLayout`（推測排除の SSOT）

`src/mir/join_ir/lowering/inline_boundary.rs:107`

```rust
pub enum JumpArgsLayout {
    CarriersOnly,
    ExprResultPlusCarriers,
}
```

### 3.3 Exit args collection（layout に従うだけ）

`src/mir/builder/control_flow/joinir/merge/exit_args_collector.rs:94`

```rust
pub fn collect(
    &self,
    exit_bindings: &[LoopExitBinding],
    remapped_args: &[ValueId],
    block_id: BasicBlockId,
    strict_exit: bool,
    layout: JumpArgsLayout,
) -> Result<ExitArgsCollectionResult, String>
```

### 3.4 DCE が jump_args を use として数える（現状の暫定対応）

`src/mir/passes/dce.rs:40`

```rust
if let Some(args) = &block.jump_args {
    for &u in args {
        used_values.insert(u);
    }
}
```

---

## 4) 相談したい設計上の痛点（要約）

- `jump_args` がメタなので、最適化/検証/表示/CFG更新が「忘れると壊れる」になりやすい
- continuation の識別が ID/名前/legacy alias で揺れると merge が壊れる
- “順序” が暗黙だと、expr_result と carrier が同一 ValueId のときにズレて誤配線になりやすい
- spans が並行 Vec だと、パスが1箇所でも同期を忘れると壊れる
- `this`/`me` の表面名と内部 `variable_map` キーがズレると、Pattern 側で receiver を取り違えやすい（SSOT不足）
- `Branch` が入ると “edge-args の参照点” が曖昧になりやすい（then/else のどちらかだけ見て事故る）

---

## 5) 移行の北極星（既存 SSOT へのリンク）

- North Star: `docs/development/current/main/design/join-explicit-cfg-construction.md`
- Phase 256 で露出した契約論点: `docs/development/current/main/investigations/phase-256-joinir-contract-questions.md`

## 6) 追加質問（receiver SSOT）

`me` receiver の host ValueId を Pattern 側が直接 `"me"`/`"this"` で参照しないように、API/Box として封印したい。

- どの層に置くべきか？（例: `joinir/api/receiver.rs`）
- Fail-Fast の位置（builder で未登録なら即死？pattern detect の時点で弾く？）
- `this`/`me` の将来拡張（Stage-3/4）に耐える最小設計は？

---

## 7) ChatGPT Pro 追記（設計レビュー観点）

段階移行ロードマップの細部で、次の2点を優先して “迷子防止” を強化したい。

1. **edge-args の参照 API は Branch 前提にする**
   - `edge_args()` 単発は曖昧になりやすい
   - 推奨: `out_edges()` / `edge_args_to(target)` のように「edge を列挙」できる形を SSOT にする
2. **terminator operand の edge-args は “意味付き” にする**
   - `Vec<ValueId>` だけだと layout（`CarriersOnly` / `ExprResultPlusCarriers`）の推測が残る
   - 最小: `EdgeArgs { layout: JumpArgsLayout, values: Vec<ValueId> }` を同梱（将来は `ContSigId` に置換）
