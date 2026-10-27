# APP-boxtorrent-mini

Status: selected; design reviewed, implementation incomplete. Date: 2026-10-27
Scope: unchanged boxtorrent-mini, canonical EXE, expected output and exit.
Refs: [RULES](../../../RULES.md), [CURRENT_STATE](../CURRENT_STATE.toml), [Home contract](../../../../reference/language/ownership.md), [Map contract](../../../../reference/runtime/runtime-data-dispatch.md).

## 現在地
- 最新実行は `HakoAllocHeap.release`: `borrowed-entry/original-input-not-ready`。入力 cohort 未完、MIR 構築で停止し EXE 未生成。
- 上流の `BoxTorrentManifest.releaseFrom` / `materialize` は `ids = me.chunk_ids; n = ids.length(); loop(i < n)` 内で `ids.get(i)` を Text key の受け手へ渡す。元の element SUM は Text だが、範囲外 Null が残る。
- 現行の範囲証明は定数添字と直前の終了 guard 用。Loop の全 backedge、同じ Array、非縮小、支持 Home の生存をまだ結んでいない。
- 固定 suite の前回実測は 9/11、611.474s（実 EXE 8本＋空 marker、timeout/SKIP なし）。JSON と boxtorrent が赤。今回の設計整理では再計測していない。

## 採用設計
- **既存仕様を維持して実装する。null-key の仕様回答はこのアプリの着手条件ではない。** 一般 Dynamic→Map の null 受理は別の未決事項であり、Text-only key の現在の経路へ勝手に stringify / TypeError を導入しない。
- **証明を三つに分ける:** (a) 同じ Array の添字範囲、(b) get 時点までの holder/field/Home 生存、(c) get 後の独立 carrier と OwnedText の取得・使用・End。範囲証明を所有権証明の代用にしない。
- 全 heap / 全 Array の payload graph 完成を先行条件にしない。対象の支持経路と実際の呼び出し区間に必要な effect だけを既存 source owner で閉じる。閉じない境界は未証明として残す。
- 実際の release 連鎖は Store → ContentChunk / Heap → Page。配列操作 get/set は長さを減らさず、set は置換または末尾追加。ただし selector 名だけで許可せず、元 call/target と core 契約を照合する。非縮小でも element の型・値・生存は変わり得る。
- 通常引数の noescape/call-only borrow 契約と既存の root Home を使う。entry loan だけでは完全な call-site ABI も生存期間も証明できない。隠れた retain/share、class 名の違いによる非 alias 推測、`Map.delete` の無条件許可はしない。
- checked ArrayGet は読んだ値を独立 carrier に保持し、Text materialize は独立 OwnedText を取得する。Text key の使用終了まで元 Array slot の借用を延ばさない。generic get 非対応の OwnedSlots にこの経路を流用しない。
- Map の local borrow は **get Normal → 実際の最後の借用 use** に限定する。関数全体の call/write 禁止を source identity と区間検証に分け、use 後の delete を過去の安全な read の拒否理由にしない。use 前・途中の置換/End/escape は拒否する。

```text
original resolver + source value flow + existing Home/call contracts
  -> Array identity / loop range / interval effects and live support
  -> source SUM refinement before first Core receipt issuance
  -> existing ArrayGet carrier -> existing Text snapshot -> existing Map key input
  -> Map local get/nonnull -> chunk.release -> nullable child argument
  -> independent Rust/C final validation -> unchanged EXE
```

## 実装タスク（同じ APP row 内、順番に進める）

1〜3は一本の縦の実装series。1で元sourceの支持条件と後段照合義務を構成し、2はそのsource条件だけを使う。実命令の独立照合は3のlower後に閉じ、物理PASSを2の着手条件へ戻さない。

1. [ ] **支持 Home と呼び出し区間を既存 source に接続する。**
   `home_prefix_field_alias.rs` の `OwnedHome` / `InstanceEntry` 区別と、`ordinary_new_field_array_read_source.rs` の field/alias 正本を再利用。count/get が参照する同じ residence と caller の支持 Home を保持する。
   resolver の全 call observation を対象に、既存 incoming/target 情報を照合して実際の call closure を閉じる。incoming 成功行だけでは全 call の網羅にならない。未解決 direct/method、捕捉、外部効果、支持 root/field の置換・End を無視しない。
   effect は「長さを縮めない」と「支持を無効化しない」を別に判定する。`Map.delete` は payload の破棄を含み、支持経路への影響が不明なら後者は不成立。再帰/SCCは影響の閉鎖を別途確認し、未閉鎖なら未証明。SUMの収束をeffect証明に流用しない。
   完了条件: 元の実 call/引数/receiver と対応する支持が、get までの全経路と caller Normal/Fault で保持され、独立した最終 use 検証にも渡ること。source-only の未使用 receipt を成果にしない。
2. [ ] **Loop 範囲証明を初回 Core 発行前に接続する（1に依存）。**
   `source_value_flow` の LoopEntry/Backedge/Continue 全 predecessor を使い、i の初期0・同一 binding の更新・条件 i<n・n の同じ ArrayLen・再束縛不在を確認。非縮小と長さの非負/i64範囲から overflow も除外する。
   `ordinary_new_field_array_read_bounds.rs` を既存 source proof 内で拡張する。`ordinary_new_borrowed_formal_source_seed_preparation.rs` の初回 Core 発行前に、同じ graph へ証明を反映して必要な SUM/SCC を再解決する。
   証明の前提は OOB を残した元 source 情報で成立させる。自分の Text-only 結論を先に仮定して call/lifetime を証明しない。target identity は元 receiver/宣言/call から照合し、Text-only 引数の受理や完成済み入力・実行可能packetを前提にしない。未確定targetは未証明。既発行 Core receipt を後段で差し替えない。
   完了条件: 実アプリの get で OOB 由来の Null/strict Text だけが不要となり、element 本来の Null は残る。別配列、縮小、失効、未知 call、未完 predecessor では絞り込まない。
3. [ ] **既存 Text snapshot → key の不足接続だけを閉じる（2に依存）。**
   `home_expression_evaluation.rs::text_snapshot_root_v1` → `expression_value.rs::materialize_text_root` → 既存 Map key/ordinary argument consumer を通す。新しい materializer、専用 producer、並列台帳は作らない。
   完了条件: source Text-only と元取得を `root_call_entry/expression_projection.rs` が照合し、receiver-first/引数評価順、carrier と OwnedText 各一回の End、取得失敗時の out 不変と Normal/Fault cleanup が Rust/C の実命令でも一致する。
4. [ ] **Map local borrow を child 引数の最後の use まで接続する（3の key 接続後）。**
   `home_map_value_flow.rs::MapFieldGetLocalV1` の関数全体 veto を、元 get の identity と use-scoped lifetime に分離。`ordinary_new_lexical_instance_call_provenance.rs::borrowed_receiver_interval_v1` の既存 get→chunk.release 証拠を再利用する。
   `chunk.alloc_handle` は親が nonnull でも nullable。元 get/guard/field read/outer-call argument ordinal と支持 Map slot を同じ論理計画で保持し、allocator.release の呼び出し完了まで子の借用を守る。ReturnedField や owned Home へ付け替えない。
   `ordinary_new_local_commit/field_read_argument_interval.rs` と C `lifecycle_v4_field_interval.inc` に同じ区間/実 call 対応を独立検証させる。body index だけで nested argument の終端を決めず、途中の同じ slot の置換/delete/End・未知 call は拒否する。
   完了条件: use 後の `me.chunks.delete(cid)` を許し、use 前/途中なら拒否。Normal/Fault と支配関係を保つ。今回不要な Map-slot 関数外 Return ABI 拡張を前提にしない。
5. [ ] **未変更アプリを EXE/run まで通し、実際に残る依存だけを解く。**
   未完の free_stack/use_counts、integer ArrayGet の source/physical 不一致、NoIncoming formal は記録を再利用する。call closure に不要な全 allocator graph を先に閉じないが、選択 consumer が必要とする境界を省略もしない。
   完了条件: 下記 smoke 全契約と期待 stdout/exit が PASS。続けて固定 suite、必要な shared regression、局所退役を確認して対象変更を commit。JSON と有限ゴール全体の後続条件は残る。

## 接続と同時に行う局所正規化
- 上の1〜3で同じArray identity/支持元を再探索せず、既存source queryの結果をbounds・SUM・lowerへ渡す。site/edgeごとの生存とFault義務は保持する。
- 4では共通get identity＋use intervalへ切替えた関数全体veto/重複探索を退役する。旧caller切替・既存negative境界確認・専有relay削除まで同series。既存定数guardの対応範囲は失わない。
- `ordinary_new_*`形別ファイルを追加しない。広い共有退役は[既存整理queue](mirbuilder-structure-refactor-queue-d0-2026-08-23.md#縮小を伴う正規化2026-10-27)へ集約。着地時は実削除と残る旧callerを報告し、移動を削減に数えない。

## 検証・整理
- 既存 `ordinary_new_borrowed_formal_result_composition_tests.rs` の count guard/OOB/別配列/途中 call、`container_field_uses.rs` の全 writer/capture、既存 Map/field interval のケースを再利用する。
- Map は既存 `direct_map_receiver_numeric_writes_close_the_original_callee_effects`、`field_argument_interval_rejects_early_end_mutation_and_transfer_but_allows_terminating_fault`、C の `published_lifecycle_v4_borrowed_source_execution_test.py` / `published_lifecycle_v4_borrowed_ingress_test.py` を再利用し、実際の slot 境界が未網羅なら同 family で補う。
- kernel `fault_tests.rs::checked_dynamic_invocation_publishes_and_ends_one_opaque_carrier` は非Text materialize の TypeError/out不変と独立 Text/End を既に検証。raw `slot_load_hi` の C get-view 試験を Dynamic/Text の証拠に流用しない。
- 新しい loop/区間境界に既存ケースが足りなければ同 family の最小追加だけ。新規 testファイル・checker・guard・card は増やさない。置換した専有判定/一時adapter は同じ実装seriesで退役する。
- 選択 smoke: `NYASH_BIN="$PWD/target/quick/hakorune" NYASH_EMIT_EXE_NYRT="$PWD/target/quick" bash tools/smokes/v2/profiles/integration/apps/boxtorrent_mini_exe.sh`。
- 既存 targeted check → 同じ binary/runtime の選択 smoke → 着地/広い共有変更時の `real-apps-exe-boundary --timeout 420`。Cargo は一つ、jobs4。今回の docs-only 整理では build/test を起動しない。
- 前回 focused 検証には source-only declared cohort、PageHeap required-integer、JSON callee-exit、physical/Text/terminal 等の未解消赤がある。`borrowed_formal_entry` は63 PASS/3既存FAIL。これらを今回の成功として扱わず、触った境界の差分を確認する。
- ArrayRead OOB の Null/strict Text と v4 raw `slot_load_hi` の0は同じ意味ではない。IntegerBox の unbox や InlineI64 名だけで primitive source を捏造しない。`reallocResult` の NoIncoming は非到達/型の証明ではない。
- 除去済みの未接続実験、旧 trace と詳細履歴は Git に保持。再調査せず上のタスクを接続する。一般 null-key の設計が必要になるのは、その受理を実際に広げる時だけ。
