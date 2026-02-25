# Phase 33‑16: Loop Header PHI as Exit SSOT — Design

日付: 2025‑12‑07  
状態: 設計フェーズ完了（実装前の設計メモ）

このフェーズでは、JoinIR → MIR マージ時の「ループ出口値の扱い」を、

- expr 結果ライン（`loop` を式として使うケース）
- carrier ライン（`start/end/sum` など状態更新だけを行うケース）

で構造的に整理し直すよ。

目的は:

1. SSA‑undef を根本的に防ぐ（継続関数パラメータを PHI 入力に使わない）
2. ループヘッダ PHI を「ループ変数の真の現在値」の SSOT にする
3. ExitLine（ExitMeta/ExitBinding/ExitLineReconnector）と expr PHI をきれいに分離する

---

## 1. 現状の問題整理

### 1.1 何が壊れているか

- Pattern2（`joinir_min_loop.hako`）などで:
  - 以前は `Return { value: i_param }` をそのまま exit PHI の入力に使っていた
  - JoinIR のパラメータ `i_param` は、MIR に inline された時点では **SSA 定義を持たない**
  - そのため PHI 入力に remap された ValueId が未定義になり、SSA‑undef が発生していた

- Phase 33‑15 では:
  - `value_collector` から JoinIR パラメータを除外
  - `instruction_rewriter` の `exit_phi_inputs` / `carrier_inputs` 収集を一時停止
  - → SSA‑undef は解消したが、「ループ出口値」が初期値のままになっている

### 1.2 何が足りていないか

本来必要だったものは:

1. ループヘッダに置かれる PHI（`i_phi`, `sum_phi` など）を、
   「ループの現在値」として扱う仕組み（構造上の SSOT）。
2. expr 結果ライン:
   - 「どの ValueId が `loop` 式の最終結果か」を Loop ヘッダ PHI を通じて知ること。
3. carrier ライン:
   - ExitLine が、ヘッダ PHI の `dst` をキャリア出口として利用できるようにすること。

今まではこの部分が暗黙のまま進んでいたため、JoinIR パラメータに依存した不安定な出口線になっていた。

---

## 2. マージパイプラインと Phase 3.5 追加

現在の JoinIR → MIR マージパイプラインは概念的にこうなっている：

1. Phase 1: block_allocator  
   JoinIR 関数群をホスト MIR 関数に inline するためのブロック ID の「型」を決める。

2. Phase 2: value_collector  
   JoinIR 内で必要となる ValueId を収集し、remap のための準備をする。

3. Phase 3: remap_values  
   JoinIR の ValueId → MIR 側の ValueId にマップする。

4. Phase 4: instruction_rewriter  
   Return → Jump, Call → Jump など命令を書き換える。

5. Phase 5: exit_phi_builder  
   expr 結果用の exit PHI を生成する。

6. Phase 6: exit_line  
   ExitMetaCollector + ExitLineReconnector で carrier を variable_map に反映。

Phase 33‑16 ではここに **Phase 3.5: LoopHeaderPhiBuilder** を追加する：

```text
Phase 1: block_allocator
Phase 2: value_collector
Phase 3: remap_values
Phase 3.5: loop_header_phi_builder   ← NEW
Phase 4: instruction_rewriter
Phase 5: exit_phi_builder
Phase 6: exit_line
```

LoopHeaderPhiBuilder の責務は:

- ループヘッダブロックを特定し
- そのブロックに PHI を生成して
  - 各キャリアの「現在値」の `dst` を決める
  - expr 結果として使うべき値（あれば）を決める
- それらを構造体で返す

---

## 3. LoopHeaderPhiInfo 箱

### 3.1 構造

```rust
/// ループヘッダ PHI の SSOT
pub struct LoopHeaderPhiInfo {
    /// ヘッダブロック（PHI が置かれるブロック）
    pub header_block: BasicBlockId,

    /// 各キャリアごとに「ヘッダ PHI の dst」を持つ
    /// 例: [("i", v_i_phi), ("sum", v_sum_phi)]
    pub carrier_phis: Vec<CarrierPhiEntry>,

    /// ループを式として使う場合の expr 結果 PHI
    /// ない場合は None（キャリアのみのループ）
    pub expr_result_phi: Option<ValueId>,
}

pub struct CarrierPhiEntry {
    pub name: String,   // carrier 名 ("i", "sum" 等)
    pub dst: ValueId,   // その carrier のヘッダ PHI の dst
}
```

### 3.2 入口 / 出口

LoopHeaderPhiBuilder は次の入力を取るイメージだよ：

- `loop_header: BasicBlockId`  
  Pattern lowerer / ルーティングで「ここがヘッダ」と決めたブロック。
- `carrier_names: &[String]`  
  `CarrierInfo` や `LoopFeatures` から得たキャリア名一覧。
- `join_fragment_meta: &JoinFragmentMeta`  
  expr 結果があるかどうかのフラグ（後述）。

そして `LoopHeaderPhiInfo` を返す：

```rust
fn build_loop_header_phis(
    func: &mut MirFunction,
    loop_header: BasicBlockId,
    carrier_names: &[String],
    fragment_meta: &JoinFragmentMeta,
) -> LoopHeaderPhiInfo;
```

---

## 4. JoinFragmentMeta / ExitMeta / Boundary との接続

### 4.1 JoinFragmentMeta

すでに次のような形になっている：

```rust
pub struct JoinFragmentMeta {
    pub expr_result: Option<ValueId>, // JoinIR ローカルの expr 結果
    pub exit_meta: ExitMeta,          // carrier 用メタ
}
```

Phase 33‑16 では:

- lowerer 側（Pattern2 等）は「Loop header PHI を前提にした expr_result」を設定する方向に揃える。
  - もしくは LoopHeaderPhiBuilder の結果から `expr_result_phi` を JoinFragmentMeta に反映する。
- 重要なのは、「expr_result に JoinIR パラメータ（`i_param` 等）を直接入れない」こと。

### 4.2 ExitMeta / ExitLine 側

ExitMeta / ExitLine は既に carrier 専用ラインになっているので、方針は：

- ExitMeta 内の `join_exit_value` は「ヘッダ PHI の dst」を指すようにする。
  - LoopHeaderPhiInfo の `carrier_phis` から情報を取る形にリファクタリングする。
- これにより、ExitLineReconnector は単に

```rust
variable_map[carrier_name] = remapper.remap(phi_dst);
```

と書けるようになる。

### 4.3 JoinInlineBoundary

Boundary 構造体自体には新フィールドを足さず、

- join_inputs / host_inputs（ループパラメータ）
- condition_bindings（条件専用変数）
- exit_bindings（ExitMetaCollector が作るキャリア出口）

の契約はそのまま維持する。LoopHeaderPhiInfo は merge フェーズ内部のメタとして扱う。

---

## 5. 変更範囲（Module boundaries）

### 5.1 触る予定のファイル

| ファイル                                          | 予定される変更                                  |
|---------------------------------------------------|-----------------------------------------------|
| `src/mir/builder/control_flow/joinir/merge/mod.rs`      | merge パイプラインに Phase 3.5 呼び出しを追加        |
| `merge/loop_header_phi_builder.rs` (NEW)          | LoopHeaderPhiBuilder 実装                       |
| `merge/exit_phi_builder.rs`                       | LoopHeaderPhiInfo を受け取って expr PHI を構築      |
| `merge/instruction_rewriter.rs`                   | 33‑15 の暫定 skip ロジックを削除し、LoopHeaderPhiInfo を前提に整理 |
| `merge/exit_line/reconnector.rs`                  | carrier_phis を入口として使うように変更（ExitMeta と連携）   |
| `join_ir/lowering/loop_with_break_minimal.rs` 等 | LoopHeaderPhiBuilder に必要なメタの受け渡し         |

### 5.2 触らない場所

- `CarrierInfo` / `LoopUpdateAnalyzer`
- `ExitMetaCollector`
- `JoinInlineBoundary` 構造体（フィールド追加なし）
- BoolExprLowerer / condition_to_joinir

これらは既に箱化されており、Loop ヘッダ PHI だけを中継する小箱を増やせば整合性を保てる設計になっている。

---

## 6. テスト戦略（完了条件）

### 6.1 expr 結果ライン

- `apps/tests/joinir_min_loop.hako`
  - 期待: RC が「ループ終了時の i」の値になること（現在は 0）。
  - SSA トレース: `NYASH_SSA_UNDEF_DEBUG=1` で undefined が出ないこと。

### 6.2 carrier ライン

- `local_tests/test_trim_main_pattern.hako` など trim 系:
  - 期待: start/end が正しく更新され、既存の期待出力から変わらないこと。
  - ExitLineReconnector が LoopHeaderPhiInfo 経由でも正常に variable_map を更新すること。

### 6.3 回帰

- 既存 Pattern1–4 のループテスト:
  - 結果・RC・SSA すべて元と同じであること。

---

## 7. 次のフェーズ

この設計フェーズ（33‑16）はここまで。

次の 33‑16 実装フェーズでは:

1. `loop_header_phi_builder.rs` を実装し、LoopHeaderPhiInfo を実際に構築。
2. `merge/mod.rs` に Phase 3.5 を組み込み。
3. `exit_phi_builder.rs` / `exit_line/reconnector.rs` / `instruction_rewriter.rs` を LoopHeaderPhiInfo 前提で整理。
4. 上記テスト戦略に沿って回帰テスト・SSA 検証を行う。

実装時に設計が変わった場合は、このファイルと `joinir-architecture-overview.md` を SSOT として必ず更新すること。
Status: Historical

