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

- 入った物（L1〜L4・L14）: 引数の無い `new MapBox()` は、body shape の層（`is_empty_map_construction_v1`）で「keys が空の `MapLiteral` の行」として 1 回だけ封印する。Home の走査・`MapHomeFlow`・`acquire_map`・lifecycle の undertaking・物理の `Map::New` と End は、既にある `%{}` の道をそのまま使う。
- 入った物（`set`、L5・L7 の一部）: named local の Map への `m.set("<literal>", <i64>)` の文は、同じ `MapHomeFlow` の行の `stores`（`MapLocalStoreV1`）に 1 件足す。受理の条件は、`.hako` の行の法則 `MapTextSetReplaceOrInsert`（method 名ではなく行）・key が文字列 literal・値が I64 の literal か I64 の local・receiver が live な Map の Home。物理は既にある `PrepareKey`・`InstallValue`・`EndOutcome` を、構築の emission と同じ検査（key の順・install の数・値の種類）に載せる。fault の cleanup は live な Home 全部の既存の unwind（`prior_home_unwind_for`）で、Map の End は 1 回。新しい持ち主・台帳・公開の形は無い。
- 拒否の診断: `OwnerTerminalHomesUnavailable` に `cause` を足し、Home の走査を止めた最初の文を指す。
- 入った物（`get`、L5・L6・L8 の一部）: named local の Map への `m.get("<literal>")` は、`local v = ..`・`return`・`print`・`+` の operand のどこでも、共通の式の評価の Core operation として扱う。使う所ごとの判定は足していない。
  - 源の側: `OriginalCoreOperandRequestV1` が、receiver の local の唯一の初期化子が封印済みの Map 構築（`MapLiteral` の行）で、行の op が `MapGet` の時だけ `local_map` を持つ。`checked_map_get_law_v1`（行の法則 `MapTextGetPresentOrFault` と形の検査）が受理の条件。
  - Home の側: `prepare_expression_calls_v1` が、その Map が live な Home である事を確かめ、receiver の read を Core の呼び出しに属させる（Identity）。結果は `ExpressionCoreSourceLoanV1::result_class` が receiver の証拠を見て I64 にする（field の Map の `get` は object を返すので、target だけでは決めない）。
  - 物理: receiver の read は alias と同じ `path_reads` の道で 1 回だけ取り、構築の emission の値と一致する事を ledger（`local_map_receiver_read_v1`）が確かめる。Invoke は既にある `CheckedGetI64` で、Fault は live な Home 全部の unwind（Map の End 1 回）。
- 通る物: `new MapBox()` だけ、`set`（上書きを含む）、`get`（return・local・print・演算）、無い key は Fault 114（e12 は exit 5）。`%{}` の literal にも同じ。
- 通らない物（止めた）:
  - `has`・`size`・`delete`: named local の Map 用の物理操作が無い。`MapInvokeOperation` にあるのは field 用の `FieldHas`・`FieldDeleteIndexed` だけで、`size`・`delete` には runtime の export も無い。新しい MIR の操作・C の emit・runtime の ABI・物理 ABI の JSON の kind が要り、L12 に当たる。
  - 条件の中の `get`（`if m.get("k") == 5`）: 条件の経路（`scalar_invoke_input_order_v1`・`comparison_input_order_v1`・`checked_scalar_base_operation_v1`）が Map の受け手を field に限っている。拒否は `PrefixNotCovered(その文)` を指す。
  - `Text`・`Bool` の値の `set`、非 literal の key、`keys`: L7・L10 のとおり受けず、拒否は文を指す。

## 2 段目の続き: 統合の前の確かめと、named local の `get` を 1 本の道に（2026-11-13）

ユーザーが ChatGPT Pro に相談して決めた（Pro の答えの要約）: この回では `has`・`size`・`delete` の公開の形（MIR の操作・JSON の kind・C の emit・runtime の export）を足さない。先に統合の前の確かめを閉じ、次に、同じ `m.get("k")` が置く場所で別の道を通る状態を終わらせる。

### 経路をたどった結果（実装前、読むだけ。subagent 2 本）

- `return m.get("k")` が共通の Core に入らない理由は、Return の入口の先着順ではなく Home の分類にある。`observe_terminal_statement` は共通 Core の `scalar_terminal` を先に試すが、`home_terminal_scalar_evaluation.rs` の `root_core` の match が CodePointCount の core だけを受け、`requires_fault_frame_v1` の拒否が frame を要る core を `terminal_map_get` に返す。`terminal_map_get` は `else` の中でだけ試される。`return m.get("k") + 1` は `previous_scalar_candidate` で共通の Core に入っている。
- `terminal_map_get` の `OwnedLocal` は「Map の Home の local 全部」で、`%{}`・`new MapBox()` のほかに、関数が返した Map の local も含む。`BorrowedParameter`（`: MapBox` 引数）は Home を持たない。共通の Core の `local_map` は「唯一の初期化子が封印済みの Map 構築」の local だけを受ける。
- 条件の中の `get` が通らない本体は、受け手が field かどうかではなく、`get` の core が比較の入力に置かれる事にある。条件の経路は core が条件の根（`has`）である事を前提にし、比較の入力に置ける core は frame を要らない読み（Array の count、code point）だけ。`comparison_input_order_v1`（frame-free のみ、Identity の受け手を除く）、`home_bool_condition.rs` の余りの core の拒否、`emit_scalar_condition_v1` の `path_reads` が空である事の確認、`Proof::ScalarInvocation` に `path_reads` の欄が無い事、の 4 か所。field の Map の `get` は core ではない（`MapGet` の core は named local だけ）ので、条件に `field_map() || local_map` を足す必要は元々無い。
- 受け手の証拠は 4 種類（field の Map・named local の Map・array alias・keys の結果）で、運び方が別々。`path_reads` に載せる 4 か所（array alias・local の Map・statement field の root・direct array receiver）と、検証側の受け手ごとの分岐が、同じ形の写しになっている。

### 規則（実装より先に書く）

A. 統合の前の確かめ
- M1. production の file は 800 行以下にする。足した hook は責任の名前の file へ移し、詰めて書かない。
- M2. `new MapBox()` を空の Map にするのは、組込みの `MapBox` の時だけにする。利用者が `box MapBox` を宣言した時は、空の Map にしない。判定の持ち主は、利用者の宣言を知る既存の層（`ordinary_new_candidate` の組込み/利用者の判定）に置き、shape の層に名前の分岐を増やさない。
- M3. 「受け手が live な Map の Home である」という証拠と、「`get` の結果が I64 である」根拠は別の物にする。結果の I64 は、checked な読み `CheckedGetI64` の契約（行の法則 `MapTextGetPresentOrFault`: 無い key は Fault 114、値が i64 でなければ Fault 104）で決める。Map へ書く値が今は I64 だけである事を、型の保証にしない。
- M4. `set` の install の Fault（key を消費する attempt の Fault）で、Map の End は 1 回、key と outcome の数は合う。注入できない所は、確認済みと書かず、未確認の範囲に残す。
- M5. 生成の側が新しい使い方を始めたので、C と runtime を変えていなくても、Python v4 の 21 本と Map の物理 test を流す。
- M6. dump の 3 本（`plain/in/a5.hako`・`red-sort/in/mg.hako`・`audit-text-field/b_map.hako`）は、EXE の退行と分けて、名前と理由で並べる。「受理の退行 0」と書かず、ユーザーの承認を聞く。文の後ろに未対応の method がある時だけ古い raw の道へ戻す fallback は作らない。拒否の文言は、原因の所を指す。

B. named local の `get` を 1 本の道に
- M7. 同じ `m.get("k")`（named local の Map、literal key）は、`local` の初期化・`return`・`print`・算術の operand・比較の入力・条件のどこに置いても、共通の式評価の Core operation（元の Map 操作の法則 + その読みの点で有効な受け手の証拠 → 共通の get の計画 → 共通の MIR writer）として判定する。使う所は、結果の行き先だけを渡す。
- M8. 受け手の証拠は、field の Map・named local の Map を 1 つの型で、Core の request から検証まで同じ形で渡す。使う所は受け手の種類を選び直さず、その型の問い（受け手の site・read の取り方・期待する Leaf の種類）に聞く。`field_map() || local_map` を条件の各所に足さない。
- M9. 受け手の read を `path_reads` に載せる 4 か所の写し（array alias と named local の Map）は、同じ型の問いで 1 つにまとめる。まとめられない物（statement field の root・direct array receiver）は、残す理由を書く。
- M10. named local の Map（`%{}` と `new MapBox()` の local）の `return m.get("k")` は、`terminal_map_get` ではなく、共通の Core の道（返却の scalar 評価）で扱う。`: MapBox` 引数と、関数が返した Map の local は Home の証拠が違うので、この回は専用の道のまま残し、残る理由を書く。caller が 0 になった部分は同じ series で消す。
- M11. 条件の中の `get`（`if m.get("k") == 5`）は、比較の入力として同じ Core operation を評価する。受け手の read は式の道と同じ path read で 1 回だけ取り、検証側にも同じ形で渡す。無い key の `get` が条件の中で Fault 114 になる時、Map の End は 1 回（Home の unwind。二重も漏れも無い）。
- M12. 既に通っている field の Map の `has` の条件（根の core。`apps/json-stream-aggregator` など）の MIR と出力は変えない。
- M13. 公開 MIR・opcode・ABI・descriptor・C shim・JSON の形、新しい持ち主・台帳は足さない。`has`・`size`・`delete` の公開の形は、この回では入れない。

### 見積もり（D − A − U。実装の前の数）

- D（確実に消える）: 受け手の `local_map().is_some() && checked_map_get_law_v1()` の判定の写し 4〜5 か所を 1 つの問いに畳む分と、`path_reads` を載せる 2 つの loop を 1 つにする分で、約 −25 行。`terminal_map_get` 本体は、`: MapBox` 引数と関数が返した Map の local が使うので、D に入れない（経路の調査で、caller が 0 になる型・関数は無い。MapLiteral の初期化子の local 向けの test の期待が `MapGet` から共通の評価へ変わる）。
- A（要る追加、production）: Return の受け口（`root_core` の match と frame の免除）約 15 行。条件（`comparison_input_order_v1`・`home_bool_condition`・`emit_scalar_condition_v1`・`ScalarInvocation` の `path_reads` と検証）約 90 行。受け手の証拠の型と問い 約 40 行。`new MapBox()` の衝突の判定 約 15 行。合わせて約 +160 行。
- U（consumer の production の変更。test と文書は別）: 受け手の証拠の型へ置き換える読む側・検証側の約 15 か所の機械的な書き換え 約 +100 行。
- 見込み: production は約 +160 − 25 + 100 ≒ +235 行（機能の回）。test 約 +120、文書約 +60。範囲が変わった時は、その時に見積もりを直す。

### 2 段目の続きの結果（Sonnet 5.5 実装、2026-11-13）

A. 統合の前の確かめ
- M1: 文の hook を `ledger_statement.rs` へ移し、`child_lowering_impl.rs` は 789 行（元は 808）。詰めて書いてはいない。
- M2: 利用者が `box MapBox` を宣言して `new MapBox()` と書く入力は、W16（2 段目の前）では利用者の object として通り、2 段目では空の Map にされて内部の拒否（`MapObligationDescribe`）で止まっていた。shape の層は class の名前だけで封印するため、宣言を知る最初の層 `ordinary_new_candidate` が `BuiltinMapNameShadowed` で名前を指して拒否する。判定の式は `is_builtin_empty_map_new_v1` の 1 か所（shape の層と candidate が同じ式を使う）。W16 で通っていた「利用者の `MapBox` + 引数なしの `new MapBox()`」は今は拒否に変わる（ユーザーの承認が要る）。通すには shape の層へ宣言の情報を渡す必要がある（resolver の入口が約 7 か所、brand catalog の持ち回りが約 118 か所に及ぶため、この回では入れていない）。
- M3: `get` の結果の I64 は `CheckedGetI64` の契約（行の法則 `MapTextGetPresentOrFault`）で決める。`%{"k" => "x"}` の `get` は Fault 104（EXE で確認。read probe にも 1 ケース）。受け手の証拠は「どの Map を借りるか」だけを示す。コメントと型の名前に区別を書いた。
- M4: read probe に `value-install-fault` の注入点を足し（`checked_install_value` の wrap。実 runtime の attempt Fault をそのまま使う）、`set` の install が Fault した時に Map の End が 1 回、key と outcome の数が合う事を確認した（2 ケース）。
- M5: Python v4 の 21 本は、最終の test binary で capture を作り直して流した。17 本が rc=0、4 本（add_view・checked_compare・new_argument・set_view）は Bool の capture が無く実行できない（基準の summary と同じ。PASS に数えない）。Map の物理 test（`--ignored`、17 本）は 7 本が通り、10 本は C の file の wrapper と test の wrap 一覧の食い違いによる link の失敗などで落ちる。落ちる 10 本は、2 段目の前の test binary でも同じで、この系列とは無関係。
- M6: dump の 3 本の名前と理由は報告に並べた。`--dump-mir` の受理に頼る tools は、`tools/checks` の `--dump-mir` を使う 7 本の fixture に `new MapBox()` が無い事を grep で確認した範囲では見つからなかった。文の形による fallback は作っていない。拒否の文言は、`size` のような未対応の method を指すもの（`root-instance-call-map-local-method-uncovered`）に直した。条件の中の `has`（a5）と型の誤り（b_map）は、後段の物理検証が出す文言のままで、原因の文は指さない。

B. named local の `get` を 1 本の道に
- M7〜M9: 受け手の証拠を `MapReceiverEvidenceV1`（field の Map・named local の Map）1 つにし、Core の request から検証まで同じ形で渡す。`local_map()`・`field_map()` は、その型への問い（投影）。「checked な named-local の `get` か」は `is_checked_local_map_get_v1` 1 つの問い（以前は 4 か所の写し）。array alias と named local の受け手の read を `path_reads` に載せる 2 つの loop は、`path_read_receiver_v1` の問いで 1 つにした。statement field の root と direct array receiver の read は、物理の read の取り方が違う（field の root は祖先の field writer の後に束縛する）ので残した。
- M10: named local の Map（`%{}` と `new MapBox()`）の `return m.get("k")` は共通の Core の返却の scalar 評価に移った（`home_terminal_scalar_evaluation.rs` の `root_core` と frame の要否の 2 点。MIR は専用の `MapGet` ingress から共通の writer の形に変わり、値と Fault は同じ）。`terminal_map_get` は、`: MapBox` 引数（Home を持たない）と、関数が返した Map の local（`StoredLocal::Map` の `OwnedLocal`）が使うので、型も関数も caller が 0 にならず、D は 0。`%{}` の local 向けの test 3 本は、共通の道を確かめる形に書き換えた。
- M11: 条件の中の `get` は、比較の入力として同じ Core operation で評価する（`comparison_input_order_v1` が checked な get を計算入力として受け、受け手のノードを入力に含める。`ScalarInvocation` が `path_reads` を持ち、検証側へ渡す）。条件の各所に `field_map() || local_map` は足していない。無い key の `get` は条件の中で Fault 114、Map の End は 1 回（read probe の 3 ケース）。
- M12: `apps/json-stream-aggregator` を含む apps 100 本、1,866 本の MIR、384 組、229 入力は、2 段目と差 0（拒否の文言 1 行を除く）。
- 残り: 条件の比較入力に乗算を含む `m.get("a") * 2 == 6` は、`Mul` が条件の比較入力に置けない既存の制限で拒否のまま（`get` とは無関係）。`has`・`size`・`delete` の公開の形は入れていない。

D・A・U の実際（この回、`45827ec086..HEAD`、production は test と docs を除く）
- production: +230 / −69（正味 +161）。内訳: 800 行の移動 +45（新 file）/ −21（impl）、`MapBox` の判定 +31、受け手の証拠の型・問い +105 / −24、consumer（Return・条件・path read の統合）+36 / −19、診断 +9。
- D（確実に消えた）: path read の 2 つの loop を 1 つに −10、`local_map && law` の写し 4 → 1 で約 −6。`terminal_map_get` は 0。見積もりの −25 のうち約 −16 を達成。
- U（consumer の production）: 見積もり約 +100 に対し +36 / −19（機械的な書き換えは、投影を残したので不要になった）。
- test: +147 / −47（read probe 6 ケース・Home の test 3 本・`MapBox` の衝突・診断の test、`terminal_map_get` の test の書き換え）。docs: この節。

## 2 段目の仕上げ: 使う側の「Map の get か」の問いを、性質の問いに（2026-11-13）

2 段目の続きの監査で見つかった事。`is_checked_local_map_get_v1`（「named local の Map の checked な get か」）を、使う側が 10 か所（Return 2、条件の比較 5、式の値の writer 1、Core の発行側 3）で問っている。動きは正しいが、使う側が「Map の get という操作の名前」で分かれており、`has` や Text の値の `get` を足すたびに 10 か所へ `||` が増える（Pro の「条件用と Return 用の例外を積み増さない」の始まり）。この回は動きと MIR を変えない整理の回。

### 経路をたどった結果（実装前、読むだけ）
- 使う側が本当に要る性質は 2 つ。(P1) 受け手の読み方: その Core の受け手を、普通の受け手の値として読むか、共通の path read で借りるか。(P2) Fault の扱い: その Core の読みが、frame を要らない読み（`frame_free_read_law_v1`）か、法則に Fault がある checked な読み（Fault の後始末を、使う側の Home が持つ）か。
- Return の `root_core` の受理と frame の免除、比較の計算入力の受理、式の値の writer の operation 検査、Core の `result_class`・`matches_plan` は P2。比較の受け手のノードの transfer の除外・eager な Core の投影の除外・受け手のノードの skip・`is_checked_map_receiver_v1` は P1。
- P1 の持ち主は既にある `ExpressionCoreSourceLoanV1::path_read_receiver_v1` と `MapReceiverEvidenceV1`。P2 の持ち主は行の法則（`checked_map_get_law_v1`）で、使う側の frame の問い（`frame_free_read_law_v1`）の対になる。どちらも既にある型の method にする。

### 規則（実装より先に書く）
- N1. 使う側（Return・条件の比較・式の値の writer）は、Core の「受け手の読み方」と「Fault の扱い」という性質だけを問い、操作の名前（Map の get か）を問わない。
- N2. 「これは Map の get だ」と決めるのは、Core を発行する `core_method_source/ordinary.rs` の 1 か所だけ。受け手の証拠と行の法則を合わせて決め、結果を 2 つの性質として Core の method に出す。
- N3. 受け手の読み方（P1）は、Core が発行時に持つ受け手の証拠から答える。受け手が共通の path read で借りられる Core だけが「path read で借りる」と答え、普通の受け手の値として読む Core（Text の `len` など）と array alias の Core の答えは、前と同じにする。
- N4. Fault の扱い（P2）は、行の法則から答える。法則に Fault がある checked な読み（無い key の Fault 114、i64 でない値の Fault 104）は、frame を要る読みとして Home が後始末を持つ。frame を要らない読みは `frame_free_read_law_v1` のまま。
- N5. 性質の問いは、既にある `OrdinaryCoreSourceCallV1` と `ExpressionCoreSourceLoanV1` の method にする。新しい型・台帳・公開の形は足さない。
- N6. 次に `has`・Text の値の `get`・`: MapBox` 引数の受け手を足す時は、Core の発行側が正しい性質を答えるようにするだけで、使う側の問いは変えない。
- N7. 動きと MIR は変えない。`--dump-mir`（1,866 本・229 入力・apps 100 本・probe 61 本・未見の入力 40 本）と EXE の結果は、2 段目の続きの binary と同一にする。
- N8. 使う側の名前の問い `is_checked_local_map_get_v1` は、発行側の 1 か所以外から消す。消した後に呼び出しが 0 の名前の問いを残さない。

### 見積もり（D − A − U。実装の前の数）
- D（確実に消える）: loan の `is_checked_local_map_get_v1` の写しと doc で約 −7、使う側 10 か所の「名前 + 法則」の重複は 1 対 1 で置き換わるので 0。
- A（production）: 性質の問い 2 つ（P1・P2）を `OrdinaryCoreSourceCallV1` と loan に 約 +14（doc を含む）。
- U（consumer の production）: 10 か所の問いの置き換え。行数は 0 に近い（1 行ずつの書き換え）。
- 見込み: production の正味は 0〜+10 行。test と docs は別（性質の問いが前と同じ答えを返す unit test を 1〜2 本）。範囲が変わった時は、その時に見積もりを直す。

### 2 段目の仕上げの結果（Sonnet 5.5 実装、2026-11-13）
- N1〜N5・N8: 使う側の問いを、`OrdinaryCoreSourceCallV1` と `ExpressionCoreSourceLoanV1` の 2 つの性質の問い `checked_fault_read_v1`（Fault の扱い）と `borrows_receiver_by_path_read_v1`（受け手の読み方）に替えた。Fault の扱いは Return の受理と frame の免除・比較の計算入力・式の値の writer の検査・`result_class`・`matches_plan` が、受け手の読み方は比較の 3 か所（transfer の除外・eager な投影の除外・受け手のノードの skip）が問う。Map の get だと決めるのは `core_method_source/ordinary.rs` の `issues_checked_fault_read_v1` 1 つ。使う側から `is_checked_local_map_get_v1` は消えた。新しい型は無い。
- N3: 受け手の読み方は `local_map` の証拠から答える（array alias は前と同じく問いに入らない）。
- N7: `--dump-mir` 1,866 本・229 入力・apps 100 本・384 組・`$P` 61 本・未見の入力 40 本・固定 suite は、2 段目の続きの binary と同一。18 ケースの物理 test は通る。
- 実際の production は +36 / −25（正味 +11、doc を含む。10 か所の書き換えが +16 / −16、発行側が +25 / −14 のうち 2 つの性質の問いと doc）。見積もりの 0〜+10 より 1 行多い。

## 2 段目の続き 2（2d）: 空の `new MapBox()` が組込みの Map かを、宣言を知る所で 1 回決める（2026-11-13）

ユーザーが ChatGPT Pro に聞いた答えで決まった事。利用者の box が組込みと同じ名前を使う事（shadowing）は今の main のとおり許し、`MapBox` を予約しない。利用者の宣言へ解決された `new MapBox()` は利用者の box の構築で、組込みの Map の契約（空の Map の Home・`CheckedGetI64`）は、組込みへ解決された構築か `%{}` からだけ得る。2c の `BuiltinMapNameShadowed` の拒否は main で通る入力の退行なので統合しない。

### 経路をたどった結果（実装前、読むだけ。subagent 3 本）
- 今、名前の文字列だけで決めている所は 3 つ。(1) shape の封印 `body_shape_resolver.rs:48` が `is_empty_map_construction_v1` で `MapLiteral` の行を作る。(2) builder の `child_lowering_impl.rs:766` が同じ名前の問いを繰り返して Map の lowering を選ぶ。(3) `ordinary_new_candidate.rs:72` が、利用者の宣言の行が見つかった後で同じ名前の問いを使って `BuiltinMapNameShadowed` で拒否する。
- 宣言の環境（利用者の box の行）の持ち主は `ParserOrdinaryBoxSourceCoverageV1`（`parser/normal_callable_program_source/model.rs:392`）で、batch の `ordinary_box_coverage()`（`callable_semantic_batch/model.rs:149`）が委譲する。shape の封印（`ShadowResolverV0`）には届かない。shape の行と宣言の環境が最初に同じ所へ集まるのは、batch の発行（`callable_semantic_batch/issuer.rs`）で、ここで `body_shapes.remove(owner)` を行にし、同じ関数が `source.ordinary_box_coverage()` を読んでいる。
- `selected`（`ordinary_new_coseal_issue.rs:304`）は local の初期化子の `new` だけ、`result_sites` は return・Shared の単項・taken の引数だけを見る。Map の要素の `new` などはどちらにも入らない。`resolve_site` の `Ok(None)` は「ordinary の box の対象外（宣言が無く `is_builtin_box`）」で、Map を特定しない。
- `MapLiteral` の行を読む所は production で 20 か所以上ある（Home の `map_literal_keys` の 6 か所、`resolved_control_flow/map_control.rs`、`map_lifecycle_undertaking.rs`、`direct_call_*` など）。宣言の情報を各所へ持ち回す道は選ばない。読む所はそのままにして、行を作る所を 1 か所にする。
- builder の `lower_callable_map_v1` は名前を見ず、claim ledger の `map_flow(site)`（`completion.cleanup().root_flow().maps()`）を読む。入口の判定 `child_lowering_impl.rs:766` だけが名前を見ている。
- 主因は「shape の層が、宣言の環境が届く前に、名前だけで意味を確定している」事。

### 選んだつなぎ方
- shape の resolver は `new MapBox()` を main と同じ中立な行（`Other`）のままにする。
- batch の発行（宣言の環境と shape の行が揃う最初の所）で、`Other` の行のうち「宣言が無く組込みの `MapBox` へ解決され、引数も field の初期化子も無い `new`」の site だけを、`%{}` と同じ空の `MapLiteral` の行にする。決めるのは 1 つの問い（宣言の持ち主である coverage の、「この名前は利用者の宣言が無く組込みの box か」）で、行を書き換える所は shape 側の 1 つの method にする。
- 読む側（Home・control・builder の Map の lowering）は、行を読む点を変えない。builder の入口の名前の問いは、Home が観測した Map の site を ledger に問う形に替える。
- 利用者の `MapBox` の宣言がある時は行を書き換えないので、main と同じ ordinary の `new` として通り、`BuiltinMapNameShadowed` は不要になって消える。

### 規則（実装より先に書く）
- P1. 空の `new MapBox()` が組込みの Map かは、利用者の宣言を知る batch の発行の 1 か所だけで決める。
- P2. shape の resolver は `new` を名前で組込みの Map と封印せず、main と同じ中立な行のままにする。
- P3. 組込みの Map と決める条件は、その名前に利用者の宣言が無く、組込みの box であり、引数が無く、field の初期化子が無い事の全部である。
- P4. 組込みの Map と決めた site だけが、`%{}` と同じ空の `MapLiteral` の行になり、既にある MapHomeFlow・Home・writer を使う。
- P5. 利用者の宣言へ解決された `new MapBox()` は、利用者の box の構築として ordinary の道を通り、引数の数が合わない時は既にある診断で拒否し、組込みへ再挑戦しない。
- P6. 利用者の `MapBox` の method（`get`・`set` を含む）は利用者の method で、組込みの Map の操作（`CheckedGetI64`）の根拠にしない。
- P7. builder は Map の lowering を、名前でなく Home が観測した Map の site で選ぶ。
- P8. 宣言を知る問いは宣言の持ち主（coverage）に 1 つだけ置き、新しい台帳・別の box の一覧・Map 専用の名前解決・公開の MIR・ABI・JSON の形を足さない。
- P9. この回で閉じられない時は、`new MapBox()` の新しい接続だけを series から外して main と同じ動きに戻し、名前の予約・一時の拒否・文の形を見る fallback で補わない。
- P10. 宣言が無い入力の MIR と結果は 2c と同一にする。

### 2c の直し（B）の規則
- Q1. 「受け手を path read で借りるか」は 1 つの問いにするか、MIR が変わるなら変えずに名前と doc を実際に答えている事（named local の Map の受け手）に合わせる。
- Q2. Return と比較の根の Core の受理は、発行側の 1 つの問いに揃える。MIR が変わるなら入れず、変わる入力を報告に並べる。
- Q3. `get` の結果が I64 である根拠は `CheckedGetI64` の契約にし、「Fault がある」事を根拠にしない。決める所は発行側の 1 か所のまま。

### 見積もり（D − A − U。実装の前の数）
- D（消える）: shape の `is_empty_map_construction_v1` の封印の腕（6 行）と同名の関数、`ordinary_new_candidate.rs` の `BuiltinMapNameShadowed` の拒否（7 行）と enum の腕、builder の名前の問い（約 −3）。約 −25。
- A（足す）: coverage の問い 1 つ、shape の行を書き換える method 1 つ、発行側の site の判定 1 つ、ledger の「Map の site があるか」の問い 1 つ。約 +60。
- U（使う側の production）: builder の入口 1 か所の書き換え、batch 発行の 2 か所（root と nested の shape）。約 +10。
- 見込み: production の正味は +30〜+50 行。test と文書は別。範囲が変わった時は、その時に見積もりを直す。

### 2d の結果（Sonnet 5.5 実装、2026-11-13）
- P1〜P8: 空の `new MapBox()` が組込みの Map かは、batch の発行（`callable_semantic_batch/builtin_map_construction.rs` の `is_builtin_empty_map_new_v1`）の 1 か所で決める。shape の resolver は `new` を中立な行のままにし、発行側が「宣言が無く組込みの box（coverage の `resolves_to_builtin_box`）で、引数も field の初期化子も無い」site の行だけを空の `MapLiteral` にする（root・nested・constructor の 3 つの shape の発行点が同じ関数を使う）。名前の文字列で決める所は 3 か所から 1 か所になった。builder の lowering の入口は、同じ 1 つの問い（`is_builtin_empty_map_new_v1`）を ledger の宣言済みの box 名（`declares_ordinary_box`）で問う。名前の文字列は 1 つの関数にだけあり、問う所は発行の 3 点と builder の入口の 2 種類になる。`BuiltinMapNameShadowed` は消え、利用者の `MapBox` は main と同じ ordinary の box として通る。
- P10: 宣言が無い入力の `--dump-mir` は 1,866 本全てが 2c と同一。229 入力・apps 100 本・384 組・927 入力の EXE 608 本も 2c と同一。途中の版は builder の入口を Home の観測で決めたため `audit-text-field/b_map.hako`（`me.name = new MapBox()` の型の誤り）が main と同じ MIR に変わったので、入口が同じ問いを ledger の宣言で問う形に直し、2c と同じ拒否（`map-completion`）に戻した。
- Q1: 「受け手を path read で借りるか」を array alias も含む 1 つの問い（`path_read_receiver_v1().is_some()`）にすると、alias の長さを条件にした入力の拒否の診断が変わる（`bool-condition/transferred-node-emitted` → `mir/invoke/[freeze:contract`）。入れず、条件が問う性質を `borrows_local_map_receiver_v1` に改名して、named local の Map の受け手だけを答えると doc に書いた。
- Q2: Return の根の Core は `CodePointCount` と checked な Fault の読みだけを受け、比較は frame の要らない読み全部（`ArrayCount`・`CodePointCount`・`CodePointSearchFromZero`）と checked な Fault の読みを受ける。発行側の 1 つの問いに揃えると `return s.indexOf("l")`（局所の String）が拒否から exit 2 に変わる（受理の拡大）ので入れていない。2c の結果の「`has` を足す時、使う側は替えなくてよい」は誤り: frame の要らない local の `has` は Return では受理されず、足す時に Return の受理の決定が要る。
- Q3: `get` の結果の I64 は、Fault がある事ではなく法則 `MapTextGetPresentOrFault`（`CheckedGetI64` の契約）から答える（`checked_read_result_class_v1`）。MIR は変わらない。

### 2 段目の Opus の確認と統合（2026-11-14）
- main へ早送りした（`6983bb5c19`）。Sonnet 5.5 が実装した（2 段目・続き・仕上げ・続き 2 の 25 commit）。
- 入った物: 組込みへ解決された引数なしの `new MapBox()` と `%{}` の local は、同じ `MapHomeFlow` の空の Map の行になる。literal の key の I64 の `set` は、同じ行の `stores` に入る。named local の Map の `get` は、Local・Return・Print・算術・条件のどこでも、共通の Core operation `CheckedGetI64` の 1 本の道を通る。使う側は「Map の get か」を問わず、Fault の扱いと受け手の読み方の性質を問う。組込みの Map かは、宣言を知る batch の発行（`callable_semantic_batch/builtin_map_construction.rs`）の 1 つの関数で決める。
- ChatGPT Pro への相談 2 回（ユーザーが聞いてきた）: (1) 2 段目の報告について。公開の操作（`has`・`size`・`delete`）はまだ足さず、統合の前の確かめを閉じ、同じ `m.get("k")` が置く場所で別の道を通る状態を先に終わらせる。(2) 組込みと同じ名前の利用者の box について。名前を予約せず shadowing を許したままにし、名前を指す拒否は統合せず、組込みかを宣言の分かる所で 1 回決める。
- ユーザーの決定（2026-11-14）:
  - 組込みと同じ名前の利用者の box は許す。組込みの契約は受け継がない。`LANGUAGE_REFERENCE_2025.md` の 2.1 に書いた（`static box` の同名と、`new` 以外の位置の名前解決は未決）。
  - dump の 3 本を、限って承認する。どれも EXE は main でも拒否で、`--dump-mir` だけが古い raw の道で通っていた。今は dump も拒否する。文の形で raw に戻す fallback は作らない。
    - `plain/in/a5.hako`（local の Map の `if m.has("k")`）: 拒否の文言は後段の CFG の検証の物で、`has` を指さない。local の Map の `has` の公開の形ができた時に解消する。
    - `red-sort/in/mg.hako`（local の Map の `m.size()`）: 拒否は `size` の呼び出しを指す（`root-instance-call-map-local-method-uncovered`）。`size` の公開の形ができた時に解消する。
    - `audit-text-field/b_map.hako`（`String` の field へ `new MapBox()` を入れる型の誤りの test）: 拒否は `map-completion` で、型の誤りを指さない。field の型の検査が Map の構築より先に効くようになった時に解消する。
- 行数（`src/` の整形後、`5f985dc00b..6983bb5c19`）: production +902、test +379。ほかに C の test（`published_map_fault_probe.c`）+17。
- 実装者の比べる組: 宣言の無い入力は、1,866 本の MIR、229 入力、apps 100 本、384 組、927 入力の EXE 608 本が、仕上げ（2c）の binary と差 0。Map の read probe の 18 ケースが通る。
- Opus の監査: 監査の入力 222 本（ほかの段の 180 本、未見の 40 本、利用者の `MapBox` の 2 本）を、main と同じ source の binary、2b、2c、2d で流した。2d で変わったのは利用者の `MapBox` の 2 本だけで、どちらも main と同じ MIR に戻った。新しい未見の入力 12 本（今の書き方）では、利用者の `MapBox` の入力は main と同じ MIR か正しい値（利用者の method の 11 など）、宣言の無い `new MapBox()` は `print`・算術・条件で前と同じ値だった。持ち主の数の見張り（基点 `5f985dc00b`）は PASS。全 lib の赤の名前は 2c と同じ（2b より 1 本多い `raw_invocation_port_preserves_async_qmark_check_collection_and_print_children` は、main と同じ source の log でも落ちていて、単独では通る）。
- 残り:
  - Return の根の Core の受理（`CodePointCount` と checked な Fault の読み）と、比較の受理（frame の要らない読み全部と checked な Fault の読み）が違う。揃えると局所の String の `return s.indexOf("l")` が拒否から通るへ変わるので、この回では入れていない。local の Map の `has` を足す前に、Return の受理を決める。
  - 比較は array alias の受け手を共通の path read で読まない（`direct_array_receiver_v1` の別の道）。揃えると拒否の診断が変わるので入れていない。
  - builder の Map の lowering の入口は、発行と同じ関数を ledger の宣言の名前（同じ batch の coverage から作った写し）で問い直す。発行の結果そのものは builder へ渡っていない。
  - `static box MapBox` を宣言して `new MapBox()` と書くと、組込みの Map になる（main の古い道も組込みとして作っていた）。仕様では static box はインスタンスを作らない箱なので、扱いは未決。
  - named local の Map の `get` が通らない位置: loop の条件、呼び出しの引数。loop の中の `set` も通らない。`: MapBox` の引数と、関数が返した Map の local は、まだ `terminal_map_get` を通る。
  - production の file `ordinary_new_local_commit/map.rs` は main で 857 行、今は 861 行（800 行の上限を超えたまま）。test の file `map_home_flow_tests.rs` は 1,019 行。
  - Map の物理 test（`--ignored`）の 10 本は、C の file の wrapper と test の wrap 一覧の食い違いで main でも落ちる。Python v4 の 4 本は Bool の capture が無く実行できない。

## 次の段の順番（ChatGPT Pro に相談、2026-11-14）

ユーザーが ChatGPT Pro に聞いてきた（読んだ main は `0e524cb848`）。残りを 1 回でまとめず、証拠の境目ごとに分けて進める。
- A: `static box` と同じ名前の `new` の決まりを決める。builder は構築先を問い直さず、発行した結果を受け取る。shape の行の書き換えは、確かめ付きの限定的な再発行の入口にする。
- B: Core の scalar の受理と結果型を、発行側の 1 つの判断（結果型・Fault の扱い・受け手の読み方）にそろえる。Return・比較・条件は、それぞれの契約（返却の型・型の組・Bool）だけを確かめる。受理が広がる分（`return s.indexOf(...)` など）は、先に法則と結果型をユーザーが承認する。
- C: array alias の受け手を、比較でも共通の path read の観測と検証までつなぐ。
- D: 関数が返した Map の local、次に `: MapBox` の引数を共通の `get` へ移し、整数の `terminal_map_get` を退役する。field の中の object を借りる get は残す。
- E: `get` を呼び出しの引数へつなぐ。F: `get` を loop の条件へつなぐ。loop の中の `set` は別にする。
- その後に、`size`・`has`・`delete` を操作ごとにつなぐ。`delete` は、更新できる受け手と取り外した値の責任が要るので、読みの共通化だけでは完成しない。

ユーザーの決定（2026-11-14）: `static box MapBox { ... }` の宣言は許し、`new MapBox()` は static box を作れないので拒否する（ほかの static box の `new` と同じ）。組込みへやり直さない。`%{}` は組込みの Map のまま。決まりの正本は `docs/reference/language/types.md` の「Builtin box names and user declarations」（`LANGUAGE_REFERENCE_2025.md` は昔の記録なので、参照だけを残した）。

## 3 段目 A: 構築先の決定を宣言の環境で 1 回にし、builder は発行した結果を受け取る（2026-11-14）

ユーザーが ChatGPT Pro に聞いた答えと、ユーザーの決定による。`static box MapBox { ... }` の宣言は許し、`new MapBox()` は static box を作れないので拒否する（ほかの static box の `new` と同じ）。拒否の後に組込みの Map へやり直さない。`%{}` は組込みの Map のまま。決まりの正本は `docs/reference/language/types.md` の「Builtin box names and user declarations」。

### 経路をたどった結果（実装前、読むだけ。subagent 1 本 + 自分で確認）
- 今の判定 `is_builtin_empty_map_new_v1` は、ordinary の box の coverage（`ParserOrdinaryBoxSourceCoverageV1`、`parser/normal_callable_program_source/ordinary_new_source.rs`）の `contains_box` だけを問う。coverage の行は source で封印された ordinary の box だけで、`static`・interface・record の box は `into_source_backed_ordinary_coverage`（`parser/postpass_envelope.rs:79`）で捨てられる（`AstOnlyCompatibility { _final_box_ordinal }`。名前は持たない）。coverage に無い事は、利用者の宣言が無い事を意味しない。
- 全部の box の宣言を知る持ち主は、同じ parser の postpass の行（`ParserBoxPostpassRowV1`）で、`AstOnlyCompatibility` の行は `compatibility_rows` と `source_backed_compatibility_rows`（`postpass_envelope/source_rows.rs`）が AST の statement から作る。ここは statement を持つので名前も持てる。coverage はそこから `into_source_backed_ordinary_coverage` で作られる 1 つの持ち主。別の持ち主は作らない。
- `static box Foo` の `new Foo()` は、`OrdinaryNewCandidate::resolve_site`（`ordinary_new_candidate.rs:47–71`）で、coverage に行が無く、`is_builtin_box("Foo")` も偽なので `OrdinaryBoxCoverageMissing` で拒否される。`static box MapBox` は `is_builtin_box` が真なので `Ok(None)`（組込みの対象外）になって抜け、さらに発行側が coverage に無い事から空の `MapLiteral` の行にしてしまう（2d の `d07`・`d12`）。子の初期化子の側（`ordinary_new_coseal_issue_source_owned_children.rs:400`）は既に `Ok(None)` を `OrdinaryBoxCoverageMissing` にしている。
- builder の入口（`normal_callable_binding_materialization.rs:531`）は、発行と同じ関数を ledger の `ordinary_box_names`（coverage の写し）で問い直す。`declares_ordinary_box` の呼び出しはここ 1 か所だけ。coverage の問いが static を含む形に広がると、写しだけが古くなる。
- builder は、batch の行の `Arc<VerifiedResolvedBodyShapeInventoryV1>` を `ResolvedFunctionLoweringInputV1.body_shape` 経由で既に受け取っている（`RawInvocationChildPortV1.source_input`）。`New` の AST に `MapLiteral` の行が付くのは、発行側が書き換えた時だけである。この発行済みの行を読めば、問い直さずに済む。
- `with_empty_map_constructions_v1`（`resolved_semantics/body_shape.rs:475`）は `self` を取って、callback が受けた `Other` を空の `MapLiteral` にするだけで、Map の関係の検証（`validate_map_relations`、`body_shape_seal.rs`）を書き換えの後にやり直さない。呼び出しは `seal_builtin_empty_map_constructions_v1` の 1 か所で、root・nested・constructor の 3 つの発行点から呼ばれる。
- 主因: 「宣言の環境が coverage の外の宣言（static など）を持たない」事と、「builder が発行の結果を受け取らず問い直す」事。新しい仕組みは要らず、既にある持ち主の情報を広げ、既にある行を使う。

### 規則（実装より先に書く）
- R1. 構築先（組込みの Map・利用者の box・作れない宣言）は、source の宣言の持ち主である parser の coverage だけに問い、Map の code に名前の一覧を作らない。
- R2. coverage は、利用者が宣言した box の名前を種類（ordinary・それ以外）を問わず持ち、`static box`・interface・record の名前も宣言として答える。
- R3. method の無い空の `static box MapBox {}` でも同じに宣言として扱い、callable の行が無い事から宣言が無いと推測しない。
- R4. 利用者が宣言した名前の `new` は、組込みの box の名前でも組込みへ解決しない。
- R5. static box（作れない宣言）の `new MapBox()` は、`static box Foo` の `new Foo()` と同じ道と同じ診断（`OrdinaryBoxCoverageMissing`）で拒否し、組込みの Map へやり直さない。
- R6. 宣言は許す。`static box MapBox` の static method の呼び出しと、同じ program の `%{}` は、今までどおり動く。
- R7. builder は構築先を問い直さず、発行済みの shape の行（`New` の site が空の `MapLiteral`）を読んで Map の lowering を選ぶ。
- R8. builder が区別するのは、組込みの Map（用途の契約があれば Map の lowering、無ければその事で拒否）・利用者の box（ordinary の構築）・source の行の欠落（契約違反として拒否）の 4 つで、Home が無い事を組込みの Map ではなかったと解釈しない。
- R9. 空の Map の構築への書き換えは、同じ owner の元の source の、引数も field の初期化子も無い `new` で、宣言の環境から組込みの Map へ解決された site だけに限る。
- R10. 書き換えの後に Map の関係の不変条件を確かめ、合わなければ `Result` で拒否し、読む側の観測は書き換え後の行を使う。
- R11. `MapLiteral` の行は、`%{}` に加えて、解決済みの空の組込みの `new MapBox()` も表す。producer と consumer の doc にそろえて書く。
- R12. 新しい台帳・持ち主・公開の MIR・ABI・JSON の形は足さず、使われなくなった `declares_ordinary_box` と名前の写しを使う問いは消す。
- R13. 宣言の無い入力の MIR と結果は、`static box` と組込みと同じ名前の入力を除いて 2d と同一にする。

### 見積もり（D − A − U。実装の前の数）
- D（消える）: `OrdinaryNewClaimLedgerV1::declares_ordinary_box`（−4）、builder の入口の名前の問い（約 −8）、`is_builtin_empty_map_new_v1` の closure の引数。約 −15。
- A（足す）: parser の postpass の `AstOnlyCompatibility` に名前を持たせる（+6）、coverage の宣言の名前と `declares_box`（約 +14）、shape の限定的な再発行（`Result`・書き換え後の検証、約 +25）、発行の失敗の variant 2 つ（+6）、builder の発行済みの行の問い（約 +20）。約 +70。
- U（使う側の production）: `resolve_site` の組込みの shortcut の条件（+3）、発行の 3 点で `?`（+6）、builder の入口 1 か所（+5）。約 +15。
- 見込み: production の正味は +60〜+80 行。test と文書は別。範囲が変わった時は、その時に見積もりを直す。

### 3 段目 A の結果（Sonnet 5.5 実装、2026-11-14）
- R1〜R5: parser の postpass の `AstOnlyCompatibility` 行が名前を持つようになり、`ParserOrdinaryBoxSourceCoverageV1` が `static`・interface・record の宣言名も答える（`declares_non_ordinary_box`、`declares_box`）。構築先の決定（`is_builtin_empty_map_new_v1`）は `declares_box` に 1 つだけ問う。`static box MapBox` の `new MapBox()` は、`resolve_site` の組込みの shortcut が `declares_non_ordinary_box` の時は効かず、`static box Foo` と同じ `OrdinaryBoxCoverageMissing` で拒否される。組込みへはやり直さない。
- R6: `static box MapBox` の宣言だけ・`%{}` との同居・static method の呼び出しは、前と同じ結果。method の無い空の `static box MapBox {}` は coverage が宣言として答える（unit test）。ただし method も field も無い static box は、`new` が無くても main の前から別の段（`mir/normal-root/consume` の `IntegrityInvalid`）で拒否されるので、`.hako` の入力では構築先の判定まで届かない。
- R7・R8: builder は `issued_builtin_empty_map_new_v1` で、発行済みの shape の行（`MapLiteral` なら組込みの Map の構築）を読む。他の行は利用者の構築。shape も行も無い時は契約違反の拒否。`declares_ordinary_box` と builder の名前の問いは消えた。builder への届け方は、構築子の semantic scope（`normal_instance_constructor_semantic_scope.rs`）が、持っていた入力を port の `source_input` へ置くようにした（他の source scope と同じ）。これが無いと構築子の本体の `new` で shape が届かず、20 本の `--dump-mir` の結果が変わった。
- R9〜R11: `with_empty_map_constructions_v1` は `Result` を返し、書き換えの後に `validate_map_relations` をやり直す。呼び出しは `seal_builtin_empty_map_constructions_v1` だけで、そこが元の source の `new`（同じ owner、引数・field の初期化子なし、宣言の環境が組込みへ解決）を確かめてから呼ぶ。`MapLiteral` の行が `%{}` と解決済みの `new MapBox()` の両方を表す事を doc に書いた。
- 受理の変化: `static box MapBox` の `new MapBox()` が、成功から拒否（`OrdinaryBoxCoverageMissing`）に変わる。`d07`（exit 2）・`d12`（exit 1）と、新しい `s01`・`s06`。ほかの入力は 2d と差 0（`--dump-mir` 1,866 本、927 入力の EXE 608 本、229 入力、apps 100 本、384 組、固定 suite 8/8、18 ケース）。
- 全 lib の赤は 224 本で、2d の 225 本の部分集合（2d で落ちていた `raw_invocation_port_preserves_async_qmark_check_collection_and_print_children` は、今回は通り、単独でも通る）。

### 3 段目 A の Opus の確認と統合（2026-11-14）
- main へ merge した（`789c0a8ee9`、branch `map-contract-3` の 4 commit、Sonnet 5.5 実装）。
- 行数（`src/` の整形後、`cfc97b3b94..8ea6948b40`）: production +95、test +239。
- Opus の監査:
  - binary: worktree で何も変えずに build し直して、作り直しが無く、実装者の `hakorune.final` と同じ SHA-256 だった。
  - 入力 283 本を、2d の binary とこの回の binary で流した（2 段目の監査の 234 本、static の名前と static 呼び出しの 38 本、新しい未見の 11 本。今の書き方）。変わったのは 5 本で、どれも static box と同じ名前の組込みの box の `new` が `OrdinaryBoxCoverageMissing` の拒否になった物（`d07`・`d12`、新しい `f01`・`f02`・`f06`）。ほかの 278 本は、MIR も実行ファイルの結果も 2d と同じ。
  - 決まりは Map だけの例外ではない（types.md の決まりどおり）。`static box StringBox` の `new StringBox("ab")`（`f01`。main では MIR を作れて、実行ファイルは作れなかった）と、`static box ArrayBox` の `new ArrayBox()`（`f02`。main では別の診断の拒否）も、同じ拒否になる。
  - 構築子の本体の `new` を見る入力 9 本を足した。`static box MapBox` の時に構築子の本体で `new MapBox()` と書く形（`g09`・`g12`）は、`static box Foo` の `new Foo()`（`g15`）と同じ診断（`artifact-source-unavailable`、`NewEmissionUnavailable`）で拒否になった（2d では組込みの Map として `map-completion`）。宣言なし・利用者の `MapBox` の形は 2d と同じ。
  - 持ち主の数の見張り（基点 `cfc97b3b94`）は PASS。`git diff --check` は問題なし。stash は 2 本のまま。全 lib の赤は 2d の部分集合で、新しい赤は 0。
- 残り:
  - 構築子の本体で static box を `new` すると、`--dump-mir` は通って `new Foo()` を出し、実行ファイルの道だけが拒否する。main からある、static box 全部に共通の穴（関数の本体では発行の所で拒否される）。拒否の場所をそろえるのは別の回。
  - 書き換えの層（`with_empty_map_constructions_v1`）は、`Other` の行が `New` の行かを自分では確かめず、呼び出し側の確認に任せている。行の種類は型の無い文字列。B 以降で行の種類を読む所を 1 か所にする時に、一緒に直す。
  - parser の compatibility の道（`compatibility_rows`）では、ordinary の box の名前も `non_ordinary_names` に入る。答え（`declares_box`）は正しいが、doc の「`static`・interface・record」とは合わない。
  - 構築子の本体の Map の local（宣言なしの `new MapBox()` と `get`）は、2d と同じく `map-completion` で拒否。Map を使う helper を `return me.make()` で呼ぶ形は、main から `StaticCallTargetAuthorityUnavailable` で拒否（static 呼び出しの穴と一緒に見る）。
  - `ordinary_new_local_commit/map.rs` は 857 行に戻ったが、800 行の上限は超えたまま。
- 次: 段 B（Core の scalar の受理と結果型を発行の側で決める）は、tmux の Codex に送る（ユーザーの指示、2026-11-14）。受理が広がる法則と結果型（`return s.indexOf` など）は、送る前にユーザーの承認を取る。

## 段 B: scalar Core の結果型を一つの問いから借りる（2026-11-14）

基点 `549de8f790`。承認された拡張は既存の1引数String indexOf（CodePointSearchFromZero、Normal I64）のReturn根への接続だけ。static Call、Array receiver、Bool入口、2引数版は広げない。

### 実装前の経路と責任（read-only worker と照合）

| 基点の場所（src/mir/ 相対） | 分類 | 扱い |
|---|---|---|
| resolved_semantics/home_terminal_scalar_evaluation.rs:173–180 | (a) scalar readの適格性 | Returnの法則名一覧を削除し、loanの共通の結果型の問いへ |
| resolved_semantics/home_expression_comparison_nodes.rs:251–252 | (a) scalar readの適格性 | 同じ問いへ、prefixとmatches_planは保持 |
| resolved_semantics/home_expression_evaluation.rs:33–111 | (b) Fault枠 | frame_free_read_lawによる判定を保持、checked Map getの枠を消さない |
| resolved_semantics/home_bool_condition.rs:215–247 | (c) 入力到達とowner | 比較の入力、field presence、直接Array receiverを保持 |
| normal_callable_semantic_package/ordinary_new_local_commit/root_home.rs:308–316 | (b)(c) root cleanup借用 | shortcutの証拠を保持、一般評価の拒否と混ぜない |
| builder/ordinary_new_admission/selected/arguments.rs:692–697 | (b) 既存testの命令形 | frame-free/Invokeの期待を保持 |
| normal_callable_semantic_package/core_method_source/ordinary.rs:307–354 | (c) 元sourceの対応 | plan/site/receiver/operandの対応を保持 |
| normal_callable_semantic_package/ordinary_new_local_commit/root_call_entry/expression_source_values.rs:604–659 | (c) 物理投影 | 元planとText domain、所有結果、既存writerを保持 |

経路はobserve_scalar_return_expression_v1 → actuals(ExpressionCoreSource) → ExpressionCoreSourceLoanV1 → prepare_value_evaluation_v1（home_expression_value_preparation.rs:4–124、完成したroot class一致）→ TerminalScalarReturnV1 → selected/return_expression.rs:59–144 → 元source/finishing照合 → verification/invoke_string_read.rs:89–121。法則・Fault・receiverの別一覧は作らない。

B4の関門（home_terminal_scalar_evaluation.rs:233–238）は、準備済みevaluationにFault枠が必要な時、元のterminal ownerへ戻す。新しいcleanupではなく、既存のqualificationとReturnのscalar continuationで閉じるCore根を接続する。Call/参照/fieldの証拠が完成しない一時値は未接続として保持する。indexOf名の例外や長さの互換一覧は足さない。長さ一時値も対照として測り、未承認の拡張が出れば止める。

### 規則（実装前）

- R1. loanのscalar_read_result_class_v1は既存frame-free law又はchecked Fault read、非所有結果、既存Normal結果型I64又はBoolを組み合わせて答える。
- R2. Returnと比較のscalar適格性はR1だけを問い、元の法則名の一覧と条件の写しを削除する。
- R3. Core根の結果型はR1からprepare_value_evaluation_v1へ渡し、返却先から逆向きに推測しない。
- R4. 比較・論理根の既存型判定と、完成したevaluation・返却先の照合を保持する。
- R5. Fault枠の要否、元plan/site/operand/receiver、比較入力owner、所有結果の対応はR1へ置き換えない。
- R6. 取得と子の評価はsource順に一度だけ行い、既存のNormal/Fault cleanupと独立MIR検証で責任を閉じる。
- R7. indexOf本体は二つのTextを借りるだけでsource Faultも取得もEndも増やさず、取得・他HomeのEndにだけ実在するFaultを観測する。
- R8. ArrayCountは名指し除外せず、既存証拠でReturnだけ拒否する差が見つかった時は実装を止めて報告する。
- R9. static Callは引数0/1以上、main/通常関数、Normal/Faultの証拠を調査するだけで接続しない。
- R10. 成功同士のMIR/出力/exitを保持し、承認済み拡張・診断差・未承認の拡張を入力ごとに分類する。
- R11. 独立検証は型・生存・Endを、元source照合はreceiver/needleの対応を守り、両者の証明を混同しない。

### D・A・U の見積もり

D（消える）12–20行: Return法則名一覧、比較の写し、B4の不要な拒否条件。A（共通queryと説明）14–20行。U（使う側productionの結果型配線）8–16行。正味+2–24行程度、production3ファイルを想定し、同一rustfmtの増減とGit物理行差を別に報告する。testと文書は別に数える。完成済み評価だけを接続し、新しい持ち主・cleanup・MIR/ABIは作らない。

段Bの実装途中の境界: loanの共通queryとReturn/比較の結果型配線を接続し、元sourceのreceiver/needle交換を拒否する既存moduleのtestを追加した（既存の独立型・生存行列ではsource順序を証明できないため）。B4を一般化する試作では一時receiverのlengthも拒否から成功になったため、承認範囲外のその部分は保留して元の関門を保持する。法則名でlengthだけを除外する腕は作らない。

### 段 B の着地と残る境界

- `scalar_read_result_class_v1`を1か所に置き、Returnと比較が借りる。Return根の法則名の一覧は1か所から0か所。Coreの結果を返却先から逆算せず、既存のFault枠・source対応・独立検証は保持した。
- B4を保持した範囲の見積もりを修正: D=6行（旧法則import/判定の写し）、A=17行（共通query）、U=正味+3行（使う側の置換・結果型配線等）、整形後productionは+14行。testは独立検証では区別できない元sourceのoperand交換の証拠1件で+58行。
- 新しいprobe112本は既存成功39本が同一、拒否維持34本、検索法則のReturn接続により成功へ39本（calleeや枝を含むprogramと既存find別名も含む）。負例2本の診断は宣言Bool/Stringと実際のI64の不一致へ移り、拒否を維持する。既存1866 dump・927入力・229入力・apps100・384組・監査292入力は差0、固定8/8、全libの既知失敗224名を維持、Mapの実行18ケースも成功。
- B4の関門の一般化は未着地: 一時receiverのlengthも成功へ変わるため承認範囲外となり、元の関門を保持した。一時receiver/needle/両方を直接Returnする3形は拒否を維持する。法則名や引数数でこの差を隠さない。共通cleanupに閉じる受理範囲を追加承認した時に再開する。
- static Callの直接Returnは未接続。0/1引数ともmainはroot-completion、通常関数はroot-exit-source-missing。calleeにFault可能な式を置く場合も同じ拒否で、Fault経路の実行成功とは数えない。普通の関数の明示Returnへ、選択CallとNormal結果・出口のsource relationを届ける別作業が要る。

### 段 B の Opus の確認と統合（2026-11-14）

- main `784882224b`（`map-contract-b` を `--no-ff` で統合）。Codex 実装、Opus 監査。
- 中身: Return の根で法則の名前を見る判定（`CodePointCount` か checked Fault read だけを受ける）を消した。Return と比較は `ExpressionCoreSourceLoanV1::scalar_read_result_class_v1` という 1 つの問いを使う。Core の根の結果型はこの問いから決め、返却先から逆算しない。Fault の枠の判定（`requires_fault_frame_v1`）と、入力の到達の確認（`home_bool_condition.rs`）は変えていない。
- 行数: 整形後の production +14 行、test +58 行。`ordinary.rs` の Git 上の差 +85 行の大半は、前から整形されていなかった部分の rustfmt。`ordinary.rs` は 722 行。
- Opus の確認:
  - ビルド: 作業 tree で cargo を空で実行し、binary の SHA が報告と一致した（`e2706e15…`）。
  - 入力: 3A の 283 本、前の probe 16 本（h01–h12、k01–k06）、Opus の新しい 15 本（q01–q15）を、3A の binary と比べた。変わったのは `return s.indexOf(..)` の形の 10 本だけで、全部「拒否 → 成功」、値も正しい。中身は banana の中の "nan"、local の needle、if-else の中、instance method の中、alias、絵文字、print の後、helper 経由。成功同士の MIR は全部同じ。
  - 広げていない物は拒否のまま: `return ("ab" + "cd").length()`、`return s.indexOf("l" + "o")`、2 引数の `indexOf`、String field の `return me.name.indexOf(..)`。同じ式を local に置く形は、前と同じく成功する。
  - `s.find(x)` も Return で通るようになった。`find` は前からある `indexOf` の別名（`core_method_contract_rows.rs`、同じ法則と引数の数）で、新しい許可ではない。
  - 負例 2 本（宣言が Bool・String の関数で `return s.indexOf(..)`）は拒否のまま、診断が「宣言と結果 I64 の不一致」（`DeclaredResultDisagrees`）へ移った。返却の型の確認が正しい場所で効いた結果なので、受け入れる。
  - 持ち主の数の見張り（基点 `549de8f790`）、pointer guard、`git diff --check`、private の走査は PASS。lib の失敗 224 名は 3A と同じ集合。stash は 2 件のまま。
- 残る物:
  - B4（一時値の関門）は保留。関門を一般にすると、承認の外の `return ("he" + "llo").length()` も通るため、Codex は元のままにして止めた。local では通る式が Return でだけ落ちる差は、まだ残っている。再開には、共通の cleanup で閉じる一時値（`length` を含む）を Return で受ける承認が要る。
  - 関門の上のコメントが「承認されるまで」という作業の状態を書いている。B4 の時に直す。
  - static 呼び出しの直接の `return`、String field の直接の Return、formal の Text は未接続（調査の表は上）。

## 段 B4: 一時値を持つReturnの根を共通評価へ接続する（2026-11-14）

基点 `ef911fa5ea`。ユーザー承認: 基準でLocal版が同じ値で通る、一時receiverのlengthと1引数indexOf（receiver/needle/両方）をReturnの根でも受ける。比較は入力どうしで行いASTを書き換えない。

### 実装前の関門の表（read-only workerと照合、src/mir相対の基点行）

| 根 | 関門の現状 | 既存の責任と今回の扱い |
|---|---|---|
| scalar Core＋一時Text | home_terminal_scalar_evaluation.rs:231–238のFault枠要求で戻す | query成功・完成したevaluationを共通のscalar continuationへ通す |
| checked Map get Core | checked_fault_root=trueで既に通る | core_result_classが存在する集合に含まれるので重複boolを削除する |
| 非Coreの算術/比較/論理/copy/scalar field | previous_scalar_candidate=trueで既に通る | 共通qualificationとcleanupを保持、受理追加なし |
| 非Core Call＋既存の複雑な引数評価 | root_argument_evaluation=trueなら既に通る | 選択Call/引数/Normal結果の証拠を保持、static Callを開けない |
| Core loan不成立又はscalar query不適格 | 159–180行で既に戻る | この未接続境界は保持 |
| 単なるliteral/reference | 102–108又は242–251行で既存ownerへ | 新しい評価を作らない境界を保持 |

上流はobserve_scalar_return_expression_v1→ExpressionCoreSourceLoanV1→scalar_read_result_class_v1→prepare_value_evaluation_v1（home_expression_value_preparation.rs:46–121、root class一致）。一時取得はhome_expression_operations.rs:388–392、home_expression_qualification.rs:103–145、home_expression_evaluation.rs:363–375に保持される。root_home.rs:308–316は一時値がある時shortcutを使わず、selected/return_expression.rs:128,144の通常評価→finish_scalar_continuationへ進む。expression_value/cleanup.rs:474–490はNormalの一時値とFaultの一時値＋prior Homesを既存planへ渡す。独立検証はinvoke_string_read.rs:89–121、invoke_string_lifetime.rs:401–419,450–483。新writer・取得モデルは不要。

### 規則（実装前）

- R1. 完成したscalar Core根は既存queryのNormal classを持つ事で関門を通し、法則名・method名・引数数・入力の形で分けない。
- R2. checked_fault_rootはcore_result_classの存在に包含されるので削除し、同じ判断を残さない。
- R3. requires_fault_frame_v1は一切変更せず、枠が要るかとReturnのowner選択を別の責任に保つ。
- R4. 非Core根の既存candidate/引数評価と、未発行のCore source・literal/referenceの境界は保持する。
- R5. receiver→needleの一度の評価と、Normal/Faultの取得・移転・Endのidentity/順序は既存評価とcleanupが保持する。
- R6. 新規成功は同じ入力のLocal版を基準binaryに通した成功・同値で確認し、承認文の外の成功は別表、基準Localが拒否又は値違いの成功は停止する。
- R7. 成功同士のMIR/値/exitを保持し、短絡RHSの一時値・新receiver・static Call等の別責務を開けない。
- R8. Faultは取得Add/Map Endへだけ注入し、検索/数え本体やString Endへ作り物のFaultを入れない。
- R9. 既存の独立変異試験を再利用し、Normal End欠落とEndより先のI64 Returnの不足証拠だけを同じ行列へ足す。

- R10. 前置きのLocalがないReturnも同じ式需要として既存text-expression-planのwalk選択へ接続し、value-flow依存の整数射影はその後に保持する。walk選択は受理権限を与えず、Core queryと完成したevaluationが引き続き受理を決める。

### D・A・U の見積もり

D=4行（checked_fault_rootの定義3行＋旧gate条件1行）、A=1行（class存在の共通条件）、U=0行、production正味−3行を見込む。契約コメントは旧2行と同じ行数で置き換える。新しい型・関数・writer・公開MIR/ABI・cleanup・checkerは0。testは既存行列のNormal End欠落/早いI64 Returnの2腕だけ、約10–15行。変更対象はproduction1ファイルと既存の独立検証test1ファイル。

B4実測補足: Fault関門だけの接続後、前置きのない一時値ReturnはMain root-completion未保持で拒否された。Localの式需要判定をReturnにも再利用する（R10）。新しい取得/cleanupやplain Completionの穴埋めはせず、既存verified walkを選択する。追加3行、seriesのproduction正味0行を見込む。

### B4b: Call結果の読みをLocal/Returnでそろえる（実装前、2026-11-14）

基点ab4076de11。ユーザー承認: literal receiver 3本と、Call結果receiverのlength/1引数indexOfを両destinationで共通ownerへ接続する。直接static Call Returnの8本は別責務のまま。

| 段（src/mir相対・基点行） | Local | Return/既存共有owner |
|---|---|---|
| 需要と観測入口 | resolved_semantics/home_new_prefix_local_statement.rs:339–341がBinary配下Callだけへ限定 | home_new_prefix_scan.rs:92→home_local_call_borrowed_actuals.rs:299–333はReturn計画全体 |
| 欠けたsource | resolved_value_profile/expression_plan.rs:168–172はReceiver配下Callを拾わない | stage_expression_actuals_v1:391–433は元計画の全MethodCallをpostorderで観測 |
| 従来の部分観測 | Local fallback:357–369→observe_borrowed_call_actuals:249–265は引数だけを再帰 | stage_argument_roots_v1:451–475が同siteを重複排除して既存Observeへ渡す |
| loanの拒否 | normal_callable_semantic_package/ordinary_new_borrowed_call_result_source.rs:393–400でpending欠落 | 同:401–416が元source/契約/引数を照合して同じloanを貸す |
| 後段 | home_new_prefix_initializer_expression.rs:104 | home_expression_value_preparation.rs:46。両方がissue_source_expression_call_v1を使用 |

- R11. Localの元式計画も既存stage_expression_actuals_v1へ渡し、Binary配下に限るfilterを削除する。
- R12. 共通ownerのowner/statement照合、Always child条件、postorder、同site重複排除と元Callのsource/契約/引数照合を保持する。
- R13. 条件付き/欠落planの従来fallbackと直接static Call Returnの境界は保持し、名前やreceiver形で例外を足さない。
- R14. Call結果の取得/移転とCore結果class、Normal/Fault cleanup、独立検証は既存ownerへ接続し、新しい仕組みは足さない。
- R15. B4の170組と追加のCall receiver/needle・引数付き/instanceを対で測り、既知8本以外の片側成功は停止する。
- R16. literal calleeも実際のMaterializeを観測し、staticというsource表記からEnd0を仮定しない。Add取得/Fault、各identity EndとMap Endのprimary/suppressedを両destinationで比較する。

D=限定filter 1行、A=0、U=終端semicolon 1箇所と同数3行の契約comment。意味あるproduction純減1行を見込む。has_call_below_operation_v1はrebind consumerが残るため削除しない。新owner/source種別/取得/cleanup/writer/公開MIR/ABIは0。既存の独立変異testを先に再利用し、足りない証拠だけ既存行列へ統合する。

### 段 B4 の Opus の確認と統合（2026-11-14）

- main `e262a81340`（`map-contract-b4` を `--no-ff` で統合）。Codex 実装、Opus 監査。3 回に分かれた（B4、B4b、B4c）。
- ユーザーの決定（2026-11-14）:
  - local で通る、一時値を含む読み（`length` と 1 引数の `indexOf` の、一時の receiver・needle・両方）を、return の根でも共通の後片付けで受ける。
  - literal を receiver にした形も、local と同じ値なので return で受ける。
  - 呼び出しの結果を receiver にした `length`・`indexOf` は、return を塞がず、local と return の両方で同じ持ち主を通して受ける。
  - instance method が literal を返した結果の local の `length`・`indexOf` を受ける。
  - 止める条件: return だけが通る・値が違う・成功が拒否に戻る時は止める。local だけが通り、return が前と同じ理由で拒否のままの形は、止めずに「残っている return の穴」に書く。
- 中身:
  - Return の根の関門は、Core の根が `scalar_read_result_class_v1` に答えたら元の持ち主へ戻さない。重なっていた `checked_fault_root` を消した。`requires_fault_frame_v1` は変えていない。
  - Return の需要を、local と同じ `requires_text_expression_plan_v1` へつないだ。
  - local の初期化の「演算の下の Call だけ」という絞り込みを消し、return と同じ `stage_expression_actuals_v1` へ式の全体を渡す。
- 行数: 整形後の production −1 行、test +34 行（既にある独立の検証の行列に腕を足した）。
- Opus の確認:
  - ビルド: 作業 tree で cargo を空で実行し、binary の SHA が報告と一致した（`2aebaf77…`）。
  - 入力: 段 B の 314 本と、Opus の新しい 17 本（r01–r11）を、段 B の binary と比べた。前からの 314 本で変わったのは 3 本だけで（`"xyz".length()` の直接の return、一時値の `length`、一時の needle）、全部「拒否 → 成功」で値が正しい。新しい 17 本は、一時値・呼び出しの結果・literal・if の中・instance method の中の形が、local と return で同じ値になった。2 引数の `indexOf` は拒否のまま。成功同士の MIR は全部同じ。
  - 持ち主の数の見張り（基点 `ef911fa5ea`）、`git diff --check`、private の走査は PASS。lib の失敗 224 名は段 B と同じ集合。stash は 2 件のまま。
  - End と Fault は、Codex が return と local の両方で実行して数えた（32 回の実行。値・回数・End の identity と順が一致）。Opus は数え直していない。
- 残っている return の穴（local では通る。次の回で閉じる）:
  - static の呼び出しの直接の return が 8 組（main は `root-completion`、普通の関数は `root-exit-source-missing`）。
  - instance method が literal を返した結果を使う return が 2 組（`artifact-source-unavailable`）。
  - これとは別に、前から変わらない local の側の差が 1 組ある（String field の読みを複数の return から local 経由に変えた形）。


## 段 B5: Local と同じ Call 証拠を Return へ接続（実装前）

基点は main `324058be00`。読むだけの worker で既存の入口から確認した。対象は B4c の残り10組（static Main 4・static ordinary 4・instance literal 2）と、同じ原因の束だけ。所有モデルの仮決定とは独立し、Home・移転・End の意味は変更しない。

| 束 | Local が成功する道 | Return が外れる所 | つなぐ既存の持ち主 |
|---|---|---|---|
| static Main | `home_new_prefix_local_statement.rs:337–355` の共通 actual staging → `ordinary_new_coseal_issue_local_route.rs:68–75` の元 Qualified claim | `qualified_static_call_claim.rs:350–377,408–425` は scalar-only の元 Return Call を walk/terminal 需要として保持せず、plain Completion は整数 literal のみ | 同じ source Call の actual・result 証拠と homes-aware Completion/terminal の保持。plain seed の穴埋めはしない |
| static ordinary | 同じ staging と source Call の Local 観測 | `home_static_value_call.rs:86–94,153–160` の terminal 資格に届かず、`ordinary_new_coseal_issue.rs:573–592` で有効な terminal を保持できない → `ordinary_new_normal_return_projection.rs:154–185` の exit source 不在 | source Call の証拠を既存 Return 評価へ渡し、同一 Call・同一 exit の Completion/terminal を保持する |
| instance literal | 共通 staging → `source_ordinary_result_arguments_v1` の元 instance・完成 Text・実引数照合 | `ordinary_new_borrowed_call_result_source.rs:258–268` の purpose 列挙は Local を許し ReturnValue を含まない | 同じ purpose owner へ Return を接続し、元 target/callee/契約/phase 照合は保持。artifact gate は変えない |

### 規則

- R17. Local が受ける元 Call の証拠を Return も同じ持ち主から借り、source 順に一度だけ評価する。
- R18. Return の actual staging は既存 `home_new_prefix_scan.rs:92` → `stage_statement_actuals` → `stage_expression_actuals_v1` を使い、二重に観測しない。
- R19. static Call の元 target・結果・実引数の照合を保持し、既にある Completion と同一 return site の terminal を渡す。新しい Return writer・Completion 補完・source 種類は作らない。
- R20. instance の元 source・完成 Text・retained target・callee・namespace・実引数 phase の照合は一つも外さない。
- R21. 取得・移転・Home・Normal/Fault の End の回数・identity・順序を Local と一致させ、生成後の独立検証を保持する。
- R22. 同じ判定を複製せず、既存の source-purpose / Call 証拠の持ち主へ集約する。builder の artifact gate は緩めない。
- R23. 3束以外の新受理、片側だけの新成功、値/出力差、成功から拒否、新公開契約/cleanup が必要になった束は止めて一覧にする。入力名や形で塞がない。

instance purpose の接続は、直接 Text Return・Text 連結・非literal callee にも影響し得るため、基準と比較する。追加の結果型や nullable Dynamic の Local 限定を広げない。

見積もり D/A/U: 重複した purpose/terminal の判定を増やさず既存ownerの条件と接続を変更する。production は数十行以内の差分を想定（上限800行、単なる圧縮をしない）。新しい持ち主・公開型・writer・cleanup は0。独立 End 欠落/重複の既存行列を再利用し、新しい test/checker は既存証拠で不足する場合のみ理由を記録する。


B5 追跡の確定: static の直接 Return は既存 `QualifiedTerminal` の元 Call 資格へ接続する。共通式側の Core/whole-binding/result の3関門を別々に広げる案は、既存 statement-level scalar owner と二重になるため採らない。

- R24. terminal の元入力契約は、既存 ordinary source cohort に加え、元 Qualified I64 結果かつ全 formal が既存 `is_i64_actual_v1` である契約（0引数を含む）。宣言名・arityの型紙では選ばない。
- R25. R24 と元 ReturnValue site・元 source/cardinality の照合を共有する。既存 tagged-input finish の `is_static_terminal_source_v1` は旧 ordinary cohort のまま保ち、parameter0 の前提を壊さない。広い元 terminal query は actual 観測・Qualified source 引数要求・source 投影の既存3consumerだけが借りる。実行entryは既存の入力/完成/引数検証が決める。
- R26. instance は既存 purpose 列挙へ ReturnValue を接続するだけとし、完成 Text の元結果照合と Dynamic の Local 限定は変えない。束外への影響は基準比較で判定する。
