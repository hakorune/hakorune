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
