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

## 1 段目: 契約の行と、無い key の get の Fault（2026-11-13）

この段は、`.hako` の契約の行に `get`・`set`・`delete`・`size` の法則を付け、生成物・`CoreMethodSemanticLawV2`・`core_method_instance_target` の腕・独立の検証へ投影する。あわせて、決定した Fault 114 を足して `checked_get_i64_v1` を変える。受理は広げない。

### 規則（実装より先に書く）

- K1. `get` の法則は `MapTextGetPresentOrFault`。key は Text 1 つで、あれば保存してある値を読む。
- K2. K1 で key が無い時は Fault 114（`NYRT_FAULT_REASON_MAP_SCALAR_MISSING_V1`、details は `{0,0}`）にする。Normal にしない。
- K3. 無い key の `get` は、`out` を書かず、Map を変えず、Map の lease も変えない。
- K4. あるが i64 ではない値の `get` は、今のとおり Fault 104 のまま変えない。
- K5. `set` の法則は `MapTextSetReplaceOrInsert`。key は Text 1 つで、あれば値を置き換え、無ければ足す。
- K6. `set` の結果の種類は Dynamic のまま変えない（結果の使い道は、この段では決めない）。
- K7. `delete`（別名 `remove`）の法則は `MapTextDeleteReportsPresence`。結果は Bool で、key があって外した時は true、無い時は false にする。
- K8. 無い key の `delete` は Fault にしない。
- K9. `size`（別名 `len`・`length`）の法則は `MapEntryCount`。結果は I64 で、入っている entry の数を返す。
- K10. `size` は Fault しない。
- K11. `has` の法則 `MapKeyPresence` は変えない。無い key の Fault を避けるには、先に `has` で確かめる。
- K12. 公開 ABI は `nyash.map.checked_get_i64_v1` の 1 つだけを変える。symbol 名、引数、Normal の意味は変えない。
- K13. `runtime_data.get_hh`・`slot_load_hh` は Fault の枠を持たないので、この段では変えない。無い key で 0 を返す所は、報告に残りとして書く。
- K14. `checked_borrow_indexed_v1` の「無い key は 0」（null になれる借用）と、`checked_array_length_v1` の Fault 111 は変えない。
- K15. VM の `MapBox.get`（文字列の返り値）と `MapBox.delete`（文字列の返り値）は、この段では変えない。VM は別の回にする。
- K16. 法則の正本は `.hako` の行だけにする。Rust 側の法則の名前は、生成物から写す。
- K17. 腕を足す所の target の側（`core_method_instance_target.rs`）は、行の法則・結果の種類・arity・receiver・effect を確かめて発行する。
- K18. 独立の検証の側（`resolver_core_method_callable_contract.rs`）は、K17 と別の関数で、行から直接、法則・結果の種類・引数の種類を確かめる。K17 の関数を呼ばない。
- K19. 新しい腕が発行する target は、この段では受理を広げない。`get`・`set`・`delete`・`size` の source 受理は、2 段目以降で足す。
- K20. 新しい持ち主・型・公開 MIR・opcode・descriptor・JSON の形は足さない。Home の持ち主は `MapHomeFlow` のままにする。
- K21. 「無い key の get は 0」に頼る test の期待は、K2 に合わせて変える。変えた test は名前と理由を報告に書く。
- K22. 今のプログラムが「無い key の 0」に頼っていないことは、前と後の比べる組で確かめる。出力や exit が変わった物は全部報告する。

### 見積もり（D − A − U。実装の前の数）

- D（確実に消える）: `get_i64` の「無い key は 0 を書いて Normal」の腕（約 4 行）、`delete` の `result_dynamic()`（1 行）、4 つの行の `semantic_law_unprojected()`（4 行）。約 10 行。
- A（要る追加、production）:
  - `.hako`: 法則の getter 4 つと行の変更、約 10 行。
  - codegen の法則の表 4 行、Rust の enum と名前 8 行。
  - `core_method_instance_target.rs`: schema 4、abi profile 4、入口 4、表 4、腕 4 つ、パラメータの種類 1。約 90 行。
  - `resolver_core_method_callable_contract.rs`: 表、腕、パラメータの期待。約 60 行。
  - `ordinary.rs` の入口の選択 4 行、Fault 114 の定義と arm で約 12 行。
  - 合わせて 約 190 行（production）。
- U（読む側の差分）: 生成物（Rust と JSON）約 20 行。test と文書は別に数える。test は約 120 行の追加と、期待の変更 数行。
- 見込み: production は約 +180 行。機能の回なので増えてよいが、報告に実際の数を並べる。

### 1 段目の Opus の確認と統合（2026-11-13）
- main へ早送りした（`8767989512`）。Sonnet 5.5 が実装した。
- 法則: `get` は `MapTextGetPresentOrFault`（無い key は Fault 114、i64 でない値は今のまま Fault 104）、`set` は `MapTextSetReplaceOrInsert`、`delete`（別名 `remove`）は `MapTextDeleteReportsPresence`（Bool、Fault しない）、`size`（別名 `len`・`length`）は `MapEntryCount`（I64、Fault しない）。生成物、`CoreMethodSemanticLawV2`、`core_method_instance_target.rs` の腕、独立の検証（`resolver_core_method_callable_contract.rs`）を足した。受理は広げていない（この段の新しい腕は、target の発行と検証だけ）。
- ABI: `NYRT_FAULT_REASON_MAP_SCALAR_MISSING_V1 = 114` を足し、`nyash.map.checked_get_i64_v1` の無い key を Fault 114 にした。仕様の文（`runtime-data-dispatch.md`、`nyrt_c_abi_v0.md`、`language/ownership.md`）も同じ変更で直した。
- 行数（`src/` の整形後）: production +207、test +351。ほかに kernel の実装 4 行と test 50 行。
- 出力が変わったのは、無い key の `get` の 1 本だけ（exit 0 → Fault 114、exit 70）。決めた意味の変化。実装者の比べる組（1,866 本の MIR、927 入力、229 入力、apps 100 本、384 組、固定 suite）は差 0。今のプログラムは「無い key は 0」に頼っていなかった。
- Opus の監査: 新しい runtime で、監査の入力 118 本、Map の調査の入力 61 本、`apps/json-stream-aggregator` を流し、変わったのは同じ 1 本だけ。持ち主の数の見張り（基点 `cd4dcbab12`）も PASS。
- 残り:
  - unchecked の `runtime_data.get_hh` と `slot_load_hh` は、まだ無い key で 0 を返す（Fault の枠が無い道）。VM の `MapBox.get` と `delete` は文字列を返し、契約と食い違う。どちらも古い道の退役の時に扱う。
  - `tools/checks/core_method_contract_manifest_guard.sh` は、この回の前から main でも失敗する（`core_method_source/ordinary.rs` が「予期しない resolver の consumer」とされる）。Opus が main `cd4dcbab12` で確かめた。直すのは別の小さな仕事。
  - 2 段目への注意（実装者）: `core_method_source/ordinary.rs` の `map = matches!(op, MapHas | MapKeys)` と `checked_*_law_v1` を、受理と一緒に広げる必要がある。named local の `new MapBox()` は、`home_new_prefix_local_statement.rs` の `PrefixNotCovered` で止まる。
