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
