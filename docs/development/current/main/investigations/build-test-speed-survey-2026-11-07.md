# build と検証の速さの調査（2026-11-07）

Status: 測定の記録です。authority ではありません。Sonnet が調べ、Opus が監査しました。commit は `9c3999f384`（test の 3 行だけ）。
ParentCurrentCard: `docs/development/current/main/investigations/app-boxtorrent-mini.md`

## 1. 全 lib が遅かった理由

- 全 lib で「60 秒超」と出ていた 8 本のうち、1 本で流しても遅いのは 2 本だけでした。
  - `call_result_tests::owned_call_results_publish_original_child_handle_contracts`: 375 秒。
  - `declared_read_tests::declared_read_result_keeps_local_support_through_field_publication`: 89 秒。入力が 76 個あるだけなので、直していません。
  - 残りの 6 本は、1 本で流すと 0.3〜2 秒でした。
- 環境変数を触る test は、`src/test_support.rs` の process 全体の lock（`PROCESS_STATE_LOCK`、`with_env_var` から取る）を順に待ちます。この lock を使う source は 201 個あります。375 秒の test がこの lock を握っている間、ほかの test は待ちます。だから全 lib は、この 1 本より速く終わりません。
- 375 秒の test は、`issue_lifecycle_compiled_entry_contract()`（1 回 41 秒）を呼んだすぐ後に `issue_lifecycle_physical_abi_input()` も呼んでいました。後者は中で同じ契約をもう一度発行し、その契約を `entry()` で返します。test は前者の呼び出しをやめ、`input.entry()` を読むようにしました。case と assert は 1 つも減っていません。

## 2. 数字（4 thread、`RUST_MIN_STACK=67108864`）

| | 前（`8b3cceacce`） | 後（`9c3999f384`） |
|---|---|---|
| 遅い 1 本 | 375 秒 | 291 秒 |
| 全 lib | 575 秒、赤 231 | 492 秒、赤 231（名前も同じ、新しい赤 0） |
| 全 lib の process の最大メモリ | 測っていない | 506 MB |

- 8 thread で 1 回だけ流すと 488 秒で、ほぼ縮みませんでした。赤は 260 本で、新しい赤 30 本の大半は `mir::compiler::raw_root_*` でした（同時に流すと競合すると見られます。原因は調べていません）。OOM はありませんでした。thread 数を変えるかはユーザーが決めます。この測定からの提案は、4 のままです。
- 意味の変わる 1 行の編集の後の build は、`cargo quick-build --bin hakorune` が 51 秒、`quick-build-fast` が 30 秒、`quick-test-fast --no-run` が 47 秒でした。以前に見た 3〜4.5 分は再現しませんでした。編集した file の違いか、ほかの cargo や test と同時に流していたためと見られますが、測っていません。
- 何もない状態から test を build すると 204 秒かかります。内訳は、型の検査 40 秒、borrow の検査 45 秒、コード生成 65 秒（うち LLVM 28 秒）、link 2 秒です。link が短いので、`mold` に替えても縮みません。

## 3. 残っている物（未着手）

- 遅い 1 本の 291 秒の大半は、production の compile そのものの時間です。test の build（opt-level 0）で `apps/mimalloc-lite` を compile すると 1 回 60 秒、ABI 入力の発行は 1 回 41 秒かかります。`perf` で見ると、`borrowed_formal_*` と `revalidate_*_object_incoming_v1` の再検証に時間が散らばっていて、原因を 1 か所に絞れませんでした。次に調べる時は、frame pointer 付きの build で、再検証の回数が呼び出しの数 × program の大きさで増えていないかを測ります。
- 環境変数の lock を握る区間を狭めると、全 lib はこの 1 本に引きずられなくなります。ただし、compiler が設定を環境変数から毎回読む作り（未着手の「呼び出しごとの env 読み」）とつながっています。
- crate を分けると、型の検査と borrow の検査（合わせて約 85 秒）を並列にできます。ただし大きな構造の変更です。
