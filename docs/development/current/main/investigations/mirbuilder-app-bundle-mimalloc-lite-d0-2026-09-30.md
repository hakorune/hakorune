# APP-mimalloc-lite

Status: selected; smoke未達、直接EXEの既存検証は成功。
Date: 2026-10-19
Scope: 未変更mimalloc-liteの既存smokeを全契約込みで通す。
Related: [RULES](../../../RULES.md)、[現在地](../CURRENT_STATE.toml)、
[全体完了条件](../workstreams/mirbuilder-inplace-replacement-current.md)。

## 現在のstop

- 選択row: `APP-mimalloc-lite`。実装goalはpause継続。今回は文書とWIPの整理のみ。
- 直近suiteではgeneric MIR JSONのnegative precheckが
  `ordinary-new/borrowed-static/required-integer-source-unproved`で停止。
  smokeが要求する`unsupported terminator Invoke`より前で落ちている。
- 一方、最後に検証済みのsourceでは正規直接EXE生成・実行・期待出力/exit0が成功。
  smoke失敗を直接EXE失敗と同一視せず、現HEADのsmoke結果とも混同しない。

## 再開後の手順

1. 現HEADから作ったbinaryとlifecycle runtimeを明示する。
2. `bash tools/smokes/v2/profiles/integration/apps/mimalloc_lite_exe.sh`を実行する。
3. 最初の失敗をsource行とgeneric/physical経路へ結び付け、既存のsource/actual ownerを直す。
4. 関連既存testと同じsmokeで確認する。precheckと直接EXEの両方を満たす。
5. 一つのアプリが通るまで追い、内部commitごとのrow/card/証拠連番は増やさない。

入口候補は`ordinary_new_borrowed_static_source_actuals.rs`。
計測のbinaryはsmoke-facing `HAKO_BIN`と`NYASH_BIN`を同じ絶対pathへ設定する。
runtimeは`HAKO_LIFECYCLE_NYRT`、既存FFIが必要なら`HAKO_AOT_FFI_LIB`を明示する。
既存環境のgrammar/stack/route条件を揃え、timeoutはbuild/link時間に合わせる。
直近の直接EXE生成は約144秒なので、smoke既定120秒のtimeoutを意味論の失敗に数えない。

## 正しさと終了条件

- 元source・parameter/incoming・result・Completion・Home/Fault証拠を保持する。
- source→Facts/Recipe/Verify→既存physical owner→独立した両final→JSON/Cを通す。
- `.hako`回避、AST rewrite、legacy retry、表現型やruntime名からのauthority推測は禁止。
- negativeは「意図したexport境界で拒否」の意味を守る。現在のエラーへ機械的に期待値を変えない。
- EXEはpure-first/no compat replay。既存smokeの全出力とexitを変更せず照合する。
- runtime不足SKIP・空marker・timeoutはEXE成功の証拠にしない。
- 置換済み専有edge/adapterは切替・caller-zero・必要検証が揃えば同じ変更seriesで削除する。

## 未完と次の順序

- このsmoke成功まで本rowは未完。固定suiteの成功と全MirBuilder/Gate1完了は別。
- 旧ObjectForward/field Birth/LOCAL初期化/receiver/cleanup/publicationの未完契約は維持する。
  既存成功を全field形・全失敗経路の成功へ一般化しない。
- 次のアプリはuntyped-field、binary-trees、boxtorrent、allocator-stress、JSONの順。
  共有の不足は必要な共通ownerで直し、別アプリの実装を並行しない。
- [JSON/Textの既存契約](mirbuilder-gate1-static-text-input-s0-2026-10-17.md)は未受理のまま後続へ保持。
  未接続ReturnのWIPは専用branchに保存し、本線に混ぜない。
- 過去の完全な契約・履歴・検証条件は、このpathの`0311400375`時点のGit履歴に保存済み。
  必要な意味契約だけを参照し、日常の再開前提へ全履歴を戻さない。
