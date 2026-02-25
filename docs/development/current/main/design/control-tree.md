# ControlTree / StepTree（構造SSOT）

Status: **SSOT（design / vocabulary）**  
Scope: **AST の“構造”だけ**を表現し、JoinIR/MIR の値・PHI・ブロックを一切持たない。  

目的:
- ループ/if の **ネスト構造**を SSOT として持ち、pattern/policy の増殖先を「構造」側へ寄せる。
- LoopSkeleton（loop_canonicalizer の骨格）を壊さずに、“次の一般化”の受け皿を追加する。

## 基本方針（禁止事項）

StepTree は「制御構造の形」だけを表す。

禁止（混ぜない）:
- `ValueId` / `BlockId` / `PHI` / `JoinInst` / `MirInstruction` など、値やCFGの概念
- backend 依存の最適化・型情報
- “動くための lowering” ロジック（生成・変換）

許可（持ってよいもの）:
- AST の最小要約（条件の形 / 文の種類 / span など）
- Capability / Feature（例: has_loop, has_if, has_return などの分類）

## 用語

### ControlTree
StepTree（後述）を含む、構造SSOTの総称。将来 “BlockTree / BoundaryTree” 等を追加しても、値は持たない。

### StepTree
AST を「構造ノード」に落とした木（または木＋Block列）で、次のノード種別を持つ:

- `Block`: 文の列（順序を保持）
- `If`: `cond` と then/else の `Block`
- `Loop`: `cond` と body の `Block`
- `Stmt`: 構造化していない文（Local/Assign/Return/Break/Continue/Expr などを tag 化）

## StepTreeContract（構造契約SSOT）

StepTreeContract は「この構造が何を含み、何を要求するか」を最小の契約として宣言する。
lowering/PHI/CFG の判断にはまだ使わない（dev-only）だが、再解析の増殖を防ぐための SSOT になる。

最小フィールド案（P1）:
- `exits`: `return` / `break` / `continue` の存在（構造だけ）
- `writes`: 変数への書き込み（最小は `Assignment target=Variable(name)` と `Local` 宣言の集合）
- `required_caps`: capability 宣言（例: `NestedLoop`, `TryCatch`, `Throw`, `Lambda` など）
- `cond_sig`: if/loop 条件式の要約（下記）

### Facts→Decision→Emit 設計（Phase 120）

**責務分離（Box-First原則）**:
1. **StepTreeBuilderBox**: 構造 + facts 抽出まで
   - AST を StepNode 木に変換
   - `StepTreeFacts` を収集（exits/writes/required_caps/cond_sig の生データ）
   - 意思決定・整形・署名生成はしない

2. **StepTreeContractBox**: facts → contract の整形のみ
   - `StepTreeFacts` を受け取り `StepTreeContract` に整形
   - BTreeSet で安定性保証（順序決定性）
   - 意思決定はしない（facts をそのまま contract に移す）

3. **StepTree→Normalized**: contract を読むだけ
   - AST 再解析禁止
   - contract に含まれる情報のみで lowering 判断
   - 将来実装時の契約

**不変条件**:
- `signature_basis_string()` の決定性維持（既定挙動不変）
- facts は順序に依存しない（BTreeSet 使用）
- contract 生成は冪等（同じ facts から同じ contract）

### cond の SSOT（Phase 119）

**SSOT**: `cond` は **AST 参照（ID/ハンドル）** を保持する。
- `StepNode::If` / `StepNode::Loop` に `cond_ast: Option<AstNodeHandle>` を追加。
- `AstNodeHandle` は AST 参照の軽量表現（将来的に `AstExprId` 等に移行可能）。
- **Phase 119**: `&ASTNode` 直接参照（ライフタイム制約あり）として実装。
  - dev-only 用途なので、将来の ID 化は別 Phase で対応可能。

**派生**: `cond_sig` は署名/ログ/差分検知用の派生表現。
- `AstSummary` から計算される要約文字列（比較・統計・ログ用）。
- `StepTreeSignature` の `signature_basis_string()` に含まれる。
- **Span は含めない**（決定性保証）。

**不変条件**:
- `cond_ast` を `signature_basis_string()` に混ぜない（既存の署名安定性を維持）。
- `AstSummary` は `cond_ast` から計算され、構造分類/契約固定の責務を持つ。

**将来計画**（Phase 119 以降）:
- StepTree→Normalized 変換箱を実装する際は、`cond_ast` を lowering 入力として活用する。
- `cond_sig` は表示/署名用途として維持される。

## StepTreeSignature（構造署名）

StepTreeSignature は StepTreeContract + node kinds の “安定な基底文字列” を hash した識別子。

用途:
- dev-only ログの検索キー
- “同型ループ/同型if” の増殖検知（再解析の増殖防止）

注意:
- `Span` 等の位置情報は signature に含めない（入力差でブレるため）。

## Capability（段階投入のSSOT）

StepTree は capability を“宣言”し、未対応は **Fail-Fast（dev-only / strict）** で止める。

想定する段階（例）:
1. **if-only**（ネストifまで）: Phase 110 P1
2. loop-in-if / if-in-loop: Phase 111+（予定）
3. nested loop: capability guard のまま（別Phaseで解禁）

## 位置づけ（LoopSkeleton との関係）

- **LoopSkeleton**（`loop_canonicalizer`）: loop 1個の骨格を正規化して、JoinIR に渡せる形へ整える。
- **StepTree**（control_tree）: 関数/ブロック全体の“構造”をSSOT化し、ネスト対応の入口を提供する。

両者は競合しない:
- LoopSkeleton は “loop単体の正規化” が責務
- StepTree は “構造の観測と分類” が責務（値やCFGを持たない）

## デバッグ出力（dev-only）

- 既定では出さない（既定挙動不変）。
- `NYASH_JOINIR_DEV=1` のときのみ StepTree をダンプする（prefix は `[trace:dev] control_tree/step_tree`）。
- StepTree は routing の入力にしない（当面は parity/観測のみ。routing SSOT は feature extractor + analyzer）。

## Phase 121: StepTree→Normalized Shadow Lowering

**目的**: StepTree（構造SSOT）から Normalized 形式への最小ルートを確立し、if-only パターンで VM/LLVM との parity を検証する。

**スコープ**: if-only（loop無し）のみ。loop は capability guard で拒否。

### 設計ルール

**入力SSOT**:
- `StepTree` + `StepTreeContract`（facts 再解析禁止）
- contract に含まれる情報のみで lowering 判断

**出力**:
- `JoinModule`（Normalized 方言）
- または "Normalized 相当の中間" を JoinIR 既存型で表現

**実行条件**:
- `joinir_dev_enabled()` のときのみ shadow 変換を実行（dev-only）
- `joinir_strict_enabled()` のときのみ mismatch を Fail-Fast

**禁止事項**:
- fallback 禁止: shadow 変換失敗時は "disabled 扱い" ではなく dev-only で理由ログ、strict で Fail-Fast
- env 直読み禁止（`src/config/env/*` 経由必須）
- ハードコード禁止（fixture 名や変数名で分岐しない）

### 責務分離（Box化）

**`StepTreeNormalizedShadowLowererBox`**:
- 責務: StepTree→JoinModule 変換（if-only限定）
- 入力: `&StepTree`
- 出力: `Result<Option<(JoinModule, JoinFragmentMeta)>, String>`
  - `Ok(None)` = if-only 対象外（例: loop 含む）
  - `Ok(Some(...))` = shadow 生成成功
  - `Err(...)` = 生成できるはずなのに壊れている

**`normalized_shadow/contracts.rs`**:
- 責務: "if-only に限定" チェック、capability 拒否理由の SSOT
- Unsupported capability の明示的列挙（Loop / Break / Continue 等）

**`normalized_shadow/parity_contract.rs`**:
- 責務: router/既存経路との契約比較（dev ログ / strict fail-fast）
- 比較対象: 出口契約（`exits`）と writes の一致（最小で壊れにくい）
- strict mode では `freeze_with_hint` でエラー（hint 必須）

**`normalized_shadow/normalized_verifier.rs`**:
- 責務: 生成された Normalized `JoinModule` の構造検証（strict で Fail-Fast）
- 例: env 引数個数、JoinFunction の形、tail-call 形式など

**`normalized_shadow/dev_pipeline.rs`**:
- 責務: dev/strict の入口を一本化（capability guard → shadow lowering → parity/verify）

### Parity検証（最小セット）

**比較対象**（値の一致まではやらない）:
- `StepTreeContract.exits` / `writes`
- 既存ルータ・既存抽出から得られる "exit/writes" 相当

**不一致時の挙動**:
- dev mode: 1行ログ `[trace:dev] phase121/shadow/parity_mismatch: ...`
- strict mode: `error_tags::freeze_with_hint("phase121/shadow/parity_mismatch", msg, hint)`

### デバッグ出力（dev-only）

**1行ログ形式**:
```
[trace:dev] phase121/shadow: step_tree_sig=... shadow_lowered=true/false reason=... exits=... writes=...
```

**strict fail-fast**:
- "if-only なのに shadow が作れない" は即座に `freeze_with_hint`
- hint 空禁止（必ず具体的な理由を1行で記述）

### 配線場所

**`src/mir/builder/calls/lowering.rs`**:
- `lower_function_body` 内の StepTree capability guard 近辺
- `joinir_dev_enabled()` のときのみ shadow lowerer を呼ぶ
- 既存の本番経路はそのまま実行（結果は一切変えない）

### テスト戦略

**Smoke tests**:
- 既存 fixture を流用（`phase103_if_only_merge_min.hako` 等）
- VM ラインと LLVM ライン両方で実行
- `HAKO_JOINIR_STRICT=1` で strict mode 検証
- `NYASH_JOINIR_DEV=1` で dev mode ログ確認

**期待値**:
- 既存と同じ数値出力（output_validator.sh で比較）
- strict mode で落ちない（= shadow parity mismatch が無い）

## Phase 122: if-only Normalized JoinModule emission (dev-only)

**目的**: Phase 121 の shadow（契約だけ）を一段進めて、if-only を Normalized JoinIR（env+継続）として実際に JoinModule 生成する。

**スコープ**: if-only のみ（loop 無し）。既定挙動は不変（dev-only で生成・検証のみ）。

### 設計原則（SSOT）

**入力SSOT**:
- `StepTree` + `StepTreeContract`（facts 再解析禁止）
- `cond_ast` を lowering 入力として使用（条件式の AST 参照）
- contract 以外の AST 再解析は禁止（構造は StepTree に固定）

**env レイアウト（SSOT）**:
- `writes` に含まれる変数だけをフィールドとして持つ
- 順序は決定的（BTreeSet で安定性保証）
- env 構造体は関数間の値の橋渡しに使用

**merge 形式（PHI 禁止）**:
- merge = `join_k(env)` への tail-call
- PHI ノードは使用しない（env が PHI の役割を果たす）
- if の両分岐から同じ継続関数に tail-call

**strict mismatch 処理**:
- 生成できるはずが失敗 → `freeze_with_hint`（hint 必須）
- capability 外は `Ok(None)`（正常な範囲外）

### 対応ノード（最小セット）

**Phase 122 で対応**:
- **If**（then/else 分岐）
- **Return**（payload なし/整数/変数の最小）
- **Assign**（`x = <expr>` の最小: Const/Variable/BinOp(Add)/Call の範囲）
  - Phase 115/116/117 で既に対応済みの範囲に限定

**Phase 122 で非対応**（capability で拒否）:
- Loop / Break / Continue（capability guard で明示的に拒否）
- Print（return code parity に寄せるため不要）

### 実装責務（Box-First）

**`StepTreeNormalizedShadowLowererBox::try_lower_if_only`**:
- 責務: StepTree → JoinModule（Normalized 方言）変換
- env レイアウトは `writes` を SSOT として決定
- `cond_ast` を lowering して Compare/Truthiness へ変換
- if-only 限定（Loop/Break/Continue は capability で拒否）

**`normalized_shadow/contracts.rs`**:
- 責務: capability チェック（変更なし）
- Unsupported capability の SSOT（Loop/Break/Continue）

**`normalized_shadow/normalized_verifier.rs`**:
- 責務: 構造検証（関数数/継続数/tail-call 形式/env 引数）
- strict 時: 生成失敗で `freeze_with_hint`
- 実行器があれば RC 比較も可能（オプション）

### 禁止事項（Fail-Fast）

- env 直読み禁止（`src/config/env/*` 経由必須）
- ハードコード禁止（fixture 名や変数名で分岐しない）
- capability で弾く（Loop/Break/Continue）
- strict で止める時は `freeze_with_hint`（hint 必須）

### 配線（dev-only）

**既定挙動不変**:
- Phase 121 と同じ配線点を使用
- dev-only のとき: JoinModule を生成 → 検証 → 本経路の結果は変えない

### テスト戦略

**新規 fixture**:
- `apps/tests/phase122_if_only_normalized_emit_min.hako`
- flag=0/1 で return 値が変わる（Phase 114/115 系）
- 期待出力: 数値 1-2 行

**Smoke test**:
- `tools/smokes/v2/profiles/integration/apps/phase122_if_only_normalized_emit_vm.sh`
- 期待: 既存実行は PASS + dev-only strict でも落ちない

### デバッグ出力（dev-only）

```
[trace:dev] phase122/emit: step_tree_sig=... module_emitted=true/false funcs=... conts=... env_fields=...
```
