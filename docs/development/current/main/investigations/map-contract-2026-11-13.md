# Map の契約の回（2026-11-13〜）

順番は `ssot-survey-2026-11-11.md` の「順番」と、`normalize-consult-2026-11-09.md` の「Map の契約を投影する回」による。式の道の一本化を区切った後の、最初の機能の回。

## 最初の調査（Sonnet 5.5、読むだけ、main `f1a5935e9e`）

- 小さな入力 60 本で、今の main の受理を測った（Opus の作業 dir に保存）。
  - `local m = new MapBox()` を使う形は、method を問わず EXE まで通らない。目標の e12（`m.set("k", 5); return m.get("k")`）も同じ。拒否の文言は `main-arguments/root-completion` だが、元は `resolved_semantics/home_new_prefix_local_statement.rs` の local の初期化の連鎖が `PrefixNotCovered` に落ちる所（診断の場所が原因から離れている）。
  - 通るのは、`%{"a" => 1}` の literal の local から `return m.get("a")` する形と、`apps/json-stream-aggregator` の field の Map の形（値が object）くらい。
- 契約の行: `.hako` の `lang/src/runtime/meta/core_method_contract_box.hako` では、`get`・`set`・`delete`・`size` が `Unprojected`。`has`（`MapKeyPresence`）と `keys`（`MapTextKeysSnapshot`）だけ法則がある。Rust の行は `tools/core_method_contract_manifest_codegen.py` が `src/mir/generated/core_method_contract_rows.rs` へ生成する。拒否は `Unprojected` の照合ではなく、`resolved_semantics/core_method_instance_target.rs` に Map の get・set・delete・size の腕が無い事で起きる。
- 持ち主の案（新しい持ち主を作らない）: Map の Home は `resolved_semantics/home_map_flow.rs` の `MapHomeFlow` と、取得の `acquire_map`（`home_ownership_transition.rs`）。`new MapBox()` の local の初期化子をここへ合流させる。`MapReadFactV1`（`normal_callable_semantic_package/map_read_fact.rs`）は入れ子の read だけの証拠なので、持ち主にしない。
- 置き場所（local・field・引数・Call の結果）ごとに、`get`・`has`・`size` の判定を複製しない。

## 仕様で既に決まっていた事（Opus が確かめた）

- `delete` の戻り値は Bool（`docs/reference/language/ownership.md:659`「MapBox deletion keeps its existing Bool result」）。`.hako` の行の Dynamic の方を合わせる。
- `keys` の順番: 仕様は「既にある繰り返しの意味は変えない」（同 `:658`）。今の runtime（`src/boxes/map_box.rs` の `keys`）は、Text の key を並べ替えて新しい Array に入れる。
- `size` の別名 `len`・`length` は、`.hako` の契約に既にある。

## ユーザーの決定（2026-11-13）

1. **無い key の `get` は Fault にする。** 今は、AOT の `runtime_data.get_hh` と checked の scalar の get が `0` を返し（値の 0 と区別できない）、VM の `MapBox.get` は `"[map/missing] Key not found: …"` の String を返していた。null を返す案は、「null になれる整数」という今の仕様に無い型が要るので採らない。key があるかは、先に `has` で確かめる（`apps/json-stream-aggregator` もこの書き方）。
2. **公開の runtime の ABI は、今の v1 を変える。** Fault の理由に `NYRT_FAULT_REASON_MAP_SCALAR_MISSING_V1 = 114` を足し（`include/nyrt_fault_v1.h`。113 までが使われている）、`nyash.map.checked_get_i64_v1` の「無い key は 0 を書いて Normal」（`docs/reference/runtime/runtime-data-dispatch.md:523`）を「Fault 114、`out` は書かない」に変える。v2 の export を足して v1 を残す案は、同じ仕事の export が 2 つになり古い道が残るので採らない。今のプログラムが「無い key は 0」に頼っていない事は、比べる組で確かめる。

## 段の案（調査の報告から。各段の前に D − A − U で数え直す）

1. 契約の行と ABI: `.hako` の `get`・`set`・`delete`・`size` の法則、codegen、`CoreMethodSemanticLawV2`、`core_method_instance_target.rs` の腕と独立の検証。`delete` を Bool に。Fault 114 と checked の get の変更、仕様の文の更新。
2. named local の `new MapBox()` を `MapHomeFlow` へ合流させる。診断を原因の所へ寄せる。
3. local の `set`・`has`・`size`・`delete`・`keys` と、`get` の結果の束縛。field 版と local 版の row の型を 1 つの語彙へ寄せる。
4. 引数と Call の結果。`get` の結果の所有の型を 1 つへ寄せる。
5. 別の回: 古い raw の `call_method MapBox.*` の道の退役（3b の範囲）。
