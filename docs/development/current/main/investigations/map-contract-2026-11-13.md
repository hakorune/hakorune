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

## 2 段目: named local の `new MapBox()` と I64 の値の基本の method（2026-11-13）

この段は、named local の `new MapBox()` を Map の Home として受け入れ、I64 の値の `set`・`get`・`has`・`size`（`len`・`length`）・`delete`（`remove`）を通す。

### 経路をたどった結果（実装前、読むだけ）

- 主因は前の段の証拠が無い事。`ordinary_new_candidate.rs` が builtin の `new` を除外するので、`new MapBox()` は `selected` にも Home の走査の要求（`has_map_v1`、`MapLiteral` の行だけを見る）にも入らず、`home_new_prefix_local_statement.rs` の連鎖の最後で `PrefixNotCovered` になる。`main-arguments/root-completion` はその離れた診断。
- 副の原因は段の仕組みの欠け。named local の受け手を `core_method_source` が受けない（`map = matches!(op, MapHas | MapKeys)` と field の受け手だけ）。`checked_*_law_v1` は `has`・`keys` の分しか無い。`get` は `return m.get("lit")` だけが専用の recognizer（`terminal_map_get`）で通り、`%{}` の local でも `set`・`has`・`size`・`delete`・`local v = m.get(..)`・`+` の operand は通らない。
- `map_lifecycle_undertaking.rs` の doc の「`new MapBox()` は未接続」は正しい。
- つなぐ先の持ち主（新しく作らない）: Home は `MapHomeFlow`（`home_map_flow.rs`）と取得 `acquire_map`（`home_ownership_transition.rs`）、物理の new・End は既にある `LocalCommitV1::Map` と `lower_callable_map_v1`。`new MapBox()` は「空の `%{}`」と同じ流れに乗せる。method は共通の式の評価（`VerifiedResolvedMethodCallSourceV1` → `ClosedOriginalCoreSourceV1` → `OrdinaryCoreSourceCallV1` → `ExpressionCoreSourceLoanV1`）の Core operation として扱う。

### 規則（実装より先に書く）

- L1. 引数の無い `new MapBox()` を named local の初期化子に書いた時は、中身が空の Map として `MapHomeFlow` の 1 つの Home にする。空の `%{}` と同じ扱いにする。
- L2. L1 の Map の責任は、確保が Normal の時だけ 1 つ取る。確保の Fault では、終える Map の責任は無い。
- L3. L1 の Map は、Normal と Fault の全部の出口で End を 1 回だけ行う。End の順は `MapHomeFlow` の既存の出口の順に従う。
- L4. 引数つき・field の初期化子・注釈つきの `new MapBox()` は受けない。拒否の診断は、local の初期化子の文を指す。
- L5. L1 の Home に対する `get`・`set`・`has`・`size`・`delete` の呼び出しは、使う所に依らず、共通の式の評価の Core operation として判定する。
- L6. 使う所（文、`local` の初期化、`return`、`+` の operand、条件）は、Map の method を判定せず、結果の行き先だけを共通の式の評価へ渡す。
- L7. `set` の key は Text、値は I64 に限る。値が I64 以外（Text・Bool・object）は受けず、拒否の文言は値の種類を指す。
- L8. `get` は I64 を返す。無い key は Fault 114（K2）。`has` は Bool、`size`（`len`・`length`）は I64、`delete`（`remove`）は Bool で、K7〜K10 のまま。
- L9. 文として書いた `m.set(..)`・`m.delete(..)` の結果は捨てる。結果を捨てても End の数は変わらない。
- L10. `keys` は受けない。拒否の文言は、結果の Array の所有を指す。
- L11. 受理を広げる時は、`core_method_source/ordinary.rs` の Map の判定と `checked_*_law_v1` を、K17・K18 の行の法則で決める。method 名や入力の形を当てる分岐は書かない。
- L12. 公開 MIR・opcode・ABI・descriptor・C shim・JSON の形は足さない。
- L13. 古い raw の `call_method MapBox.*` の道は、この段では消さない。canonical の道で通る入力は、canonical の道で通る事を確かめる。
- L14. Map の new と End の数は、Normal・`get` の Fault 114・`set` の key 確保の Fault（100・101）の道で、new 1 回に対し End 1 回にする。

### 見積もり（D − A − U。実装の前の数）

- D（確実に消える）: `home_new_prefix_local_statement.rs` で `PrefixNotCovered` になる場合の診断の言い換えで消える行は無い。`terminal_map_get` を共通の Core 経路へ寄せられた時だけ、その recognizer（約 50 行）が D になる。見込めるか、実装の途中で確かめる。D に確実に入れるのは 0〜数行。
- A（要る追加、production）:
  - Home の入口（走査の要求の拡張、`new MapBox()` の枝、物理 lowering の腕）: 約 80 行。
  - named local の受け手の source、`ordinary.rs` と `checked_*_law_v1` の拡張、request の拡張: 約 150 行。
  - 文としての `set`・`delete` の受理: 約 60 行。
  - 合わせて約 290 行（production）。
- U（読む側の差分）: test 約 300 行、文書約 60 行。
- 見込み: production は約 +290 行。機能の回なので増えてよい。報告に実際の数を並べる。

### 2 段目の結果と止めた所（Sonnet 5.5 実装、2026-11-13）

- 入った物: L1〜L4・L14 の Home の部分。引数の無い `new MapBox()` は、body shape の層（`is_empty_map_construction_v1`）で「keys が空の `MapLiteral` の行」として 1 回だけ封印する。Home の走査・`MapHomeFlow`・`acquire_map`・lifecycle の undertaking・物理の `Map::New` と End は、既にある `%{}` の道をそのまま使う。新しい持ち主・型・公開の形は足していない。
- 拒否の診断: `OwnerTerminalHomesUnavailable` に `cause` を足し、Home の走査を止めた最初の文（`PrefixNotCovered(statement)`）を指す。
- 通る物: `new MapBox()` だけ、`return m.get("k")`（無い key は Fault 114）。Normal で Map の new 1・end 1、`get` の Fault 114 でも new 1・end 1。
- 通らない物（L5〜L9、止めた）: `set`・`has`・`size`・`delete` と、それに続く `local v = m.get(..)`・`+` の operand。理由:
  - `has`・`size`・`delete`: named local の Map 用の物理操作が無い。`MapInvokeOperation` は field 用の `FieldHas`・`FieldDeleteIndexed` だけで、`size`・`delete` には runtime の export も無い。通すには新しい MIR の操作・C の emit・runtime の ABI が要り、L12（公開の形を足さない）に当たる。
  - `set`: 物理の操作（`PrepareKey`・`InstallValue`・`EndOutcome`）は既にあるが、文としての install を Home の行・commit・fault の cleanup の packet・独立の検証へ通す道が field 版（`MapFieldInstallV1`・`MapFieldInstallEmissionV1`、約 1,300 行）にしか無い。local 版を並べて書くと、同じ責任の 2 つ目の道になる。3 段目の「field と local の row の語彙を 1 つへ」を先にしてから足すのが筋。
  - 目標のうち `p01`・`p02`・`p03`・`p04`・`p08`・`p10`・`p11`・`p14`・`p15`・`p16` は、この理由で未達。
