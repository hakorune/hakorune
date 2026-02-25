# Self Current Task — Decisions (main)

2026-02-24
- phase-21.5 perf lane は monitor-only に切替え、mainline priority を selfhost/de-rust（phase-29cc M4）へ移す（status: `accepted`）。
  - 条件:
    - `kilo_kernel_small` の `ratio_c_aot >= 0.95` を満たす実測が継続して得られている
    - `phase21_5_perf_gate_vm.sh` が緑
  - 運用:
    - perf は劣化検知時のみ failure-driven で再起動
    - proactive な HOT-20 深掘りは一時停止
  - SSOT:
    - `CURRENT_TASK.md`
    - `docs/development/current/main/10-Now.md`

2026-02-24
- LLVM-HOT-20 以降の最適化は portability-first（no-regret）で運用する（status: `accepted`）。
  - 優先順位は `Class A -> Class B -> Class C` に固定し、Class B は Temporary Bridge と撤去条件を必須にする。
  - Class A は「移植後も残る契約/IR/ABI/gate」を対象とし、`.hako` 移植資産として扱う。
  - Class B は Rust runtime 内部最適化に限定し、`.hako` 相当戦略を 1 行で記録する。
  - commit 前に `class / survives-after-migration / retirement condition` の 3 点を記録する。
  - SSOT:
    - `docs/development/current/main/design/optimization-portability-classification-ssot.md`
    - `docs/development/current/main/design/optimization-ssot-string-helper-density.md`

2026-02-16
- Phase 29y Y3（optional GC implementation queue docs-first）を Decision として固定する（status: `provisional`）。
  - Queue order は `min1 -> min2 -> min3` に固定し、`1 min task = 1 commit = fixture/gate pin` を必須化。
  - Non-negotiable は次の 3 点を維持する:
    - semantics invariance（`NYASH_GC_MODE=rc+cycle|off` で意味論不変）
    - ABI fixed（`args borrowed / return owned`）
    - RC insertion single-source（retain/release/weak_drop は 1 箇所）
  - Gate contract:
    - `phase29y_optional_gc_lane_entry_vm.sh`
    - `phase29y_lane_gate_vm.sh`
    - `rc_gc_alignment_g2_fast_milestone_gate.sh`
  - Rollback note:
    - 上記ゲートが FAIL した状態で queue を次へ進めない。
    - 60 分以内に復旧できない場合は `CURRENT_TASK.md` に詰まりメモを固定して docs-first へ戻す。
  - SSOT:
    - `docs/development/current/main/phases/phase-29y/40-OPTIONAL-GC-LANE-ENTRY-SSOT.md`（Section 8）
    - `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
- GC mode の受理値を `auto|rc+cycle|off` に縮小し、`minorgen/stw/rc` は fail-fast とする（CLI + env validation）。
  - 目的: Phase 29y の運用契約（ON/OFF semantics invariance）と実装入力面の齟齬を解消する。
  - Status: `min1` として 2026-02-16 実装完了（guard + gate PASS）。
  - Follow-up: `min2` として optional GC observability pin を 2026-02-16 実装完了（metrics ON 時のみ stable tag）。
  - Follow-up: `min3` として optional GC pilot execution を 2026-02-16 実装完了（`G-RC-2` + `phase29y_lane_gate_vm` PASS）。
  - Follow-up (2026-02-18): lane gate 先頭に compiler pipeline parity（`phase29y_hako_using_resolver_parity_vm.sh`）を昇格し、固定順序を4段へ更新。
  - SSOT:
    - `docs/reference/runtime/gc.md`
    - `docs/tools/cli-options.md`
    - `docs/reference/environment-variables.md`

2026-02-13
- Lifecycle/RC responsibility split を Decision として固定する（status: `accepted`）。
  - MIR は lifecycle intent のみを表現し、数値 refcount の真実は持たない。
  - Runtime/Kernel が retain/release と最終解放の唯一の責務を持つ。
  - LLVM/VM backend は count policy を独自実装せず、runtime ABI 契約を実行する。
  - 互換名 `ny_release_strong` は残すが、新規実装は `nyrt_handle_release_h` を優先する。
- RC retire（削除/縮退）は現時点で実施しない。G-RC-1..3 lock 後に Decision を `accepted` へ昇格した。
- Progress note (2026-02-13):
  - G-RC-1 は guard + parity gate で lock 済み。
  - SSOT: `docs/development/current/main/design/rc-gc-alignment-g1-lifecycle-parity-ssot.md`
  - G-RC-2 は fast/milestone matrix gate で lock 済み。
  - SSOT: `docs/development/current/main/design/rc-gc-alignment-g2-fast-milestone-gate-ssot.md`
  - G-RC-3 は cycle + explicit-drop timing matrix gate で lock 済み。
  - SSOT: `docs/development/current/main/design/rc-gc-alignment-g3-cycle-explicit-drop-ssot.md`
  - G-RC-4 は Decision promotion + rollback note を lock 済み。
  - SSOT: `docs/development/current/main/design/rc-gc-alignment-g4-decision-promotion-ssot.md`
- Progress note (2026-02-14):
  - G-RC-5 は GC mode semantics invariance gate（`rc+cycle` vs `off`）で lock 済み。
  - SSOT: `docs/development/current/main/design/rc-gc-alignment-g5-gc-mode-semantics-invariance-ssot.md`
  - G-RC-2 matrix は `g3_cycle_timing_matrix` + `g5_gc_mode_semantics_invariance` を milestone replay として含む。
  - Phase 29y optional GC lane entry は single-entry gate（`phase29y_optional_gc_lane_entry_vm.sh`）で固定した。
  - Phase 29y lane gate（`phase29y_lane_gate_vm.sh`）は当初 `core_contracts -> optional_entry` の2段で固定した（historical）。
  - 2026-02-18 時点の現行契約は `compiler pipeline parity -> no-compat -> core_contracts -> optional_entry` の4段（SSOT: `phase-29y/50-LANE-GATE-SSOT.md`）。
  - `phase29y_rc_insertion_overwrite_release_vm.sh` は現時点で standalone diagnostic gate として運用し、lane gate 必須stepからは分離する。
- Rollback note (accepted contract safety):
  - G-RC-1/G-RC-2/G-RC-3 のいずれかが連続 FAIL し、24時間以内に収束しない場合は status を `provisional` に戻す。
  - lifecycle boundary drift（MIR intent / runtime ownership）検出時は同様に rollback する。
  - rollback 手順の正本: `docs/development/current/main/design/rc-gc-alignment-g4-decision-promotion-ssot.md`
- SSOT:
  - `docs/reference/language/lifecycle.md` (Section 10: RC responsibility split and retirement policy)
  - `CURRENT_TASK.md` (RC / GC Alignment, 2026-02-13)

2026‑02‑05
- Stage‑3 exceptions（try/throw/catch/cleanup）の JSON v0 bridge lowering は Result‑mode を採用し、gate/selfhost では `NYASH_TRY_RESULT_MODE=1` を固定する。
  - legacy の MIR Throw/Catch 経路は “動くが pin しない” 扱いにする（挙動揺れ防止）。
  - SSOT: `docs/guides/exceptions-stage3.md` / `docs/reference/architecture/parser_mvp_stage3.md`

2026‑02‑05
- strict/dev(+planner_required) の gate が依存する “sentinel タグ” は、`NYASH_RING0_LOG_LEVEL` や debug フラグに依存させない。
  - 方針: **prefix-free 1行**を **stderr** に出す（`ring0.io.stderr_write`）。
  - 対象例: `[joinir/planner_first ...]`, `[joinir/no_plan ...]`, `[phase132/gate] StepTree root ...`,
    `[plan/pattern2/promotion_hint:*]`, `[flowbox/adopt ...]`
  - 理由: hermetic gate（`HAKO_JOINIR_DEBUG=0` / `NYASH_CLI_VERBOSE=0`）でも `grep -qF` で安定検証できるようにする。

2026‑02‑04
- TODO/FIXME/HACK の上位3ファイル（`src/mir/join_ir/lowering/loop_patterns/mod.rs`,
  `src/mir/join_ir/lowering/loop_patterns/nested_minimal.rs`,
  `src/mir/loop_pattern_detection/mod.rs`）は **Issue/SSOTへ移送**してから削除する。
  - 新規 TODO には必ず Issue/SSOT 参照を付ける（単独 TODO を禁止）。
  - 進捗SSOT: `docs/development/current/main/investigations/todo-fixme-hack-inventory.md`
- TODO/FIXME/HACK の上位20ファイルをトリアージし、Decision/Issue 方針を確定。
  - 進捗SSOT: `docs/development/current/main/investigations/todo-fixme-hack-inventory.md`

2026‑01‑26
- Entry coherence（候補の一意性）を設計SSOTとして固定する。
  - strict/dev(+planner_required) では、複数候補が同時成立したら `entry_ambiguous` で freeze（順序依存は禁止）。
  - “特定パターン優先” の優先順位表/数値スコアは導入しない。必要なら guard を狭めて重なりを消す（支配関係で一意化）。
  - SSOT: `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`
- planner_first の命名は `rule=` を安定IDとして維持し、可読性は `label=` で補う（テストは label に依存しない）。
  - SSOT: `docs/development/current/main/design/entry-name-map-ssot.md`

2026‑01‑25
- VerifiedRecipe は PortSig（出口署名）を持ち、Lower/Parts は PortSig に従って配線するだけの全域関数へ収束する（silent wrong 禁止）。
  - SSOT（design-only）: `docs/development/current/main/design/verified-recipe-port-sig-ssot.md`
- then-only if の join は「pre で存在していた変数は identity 入力を許可 / then 側導入の local は freeze」をポリシーとして固定する（IfJoin completeness）。
  - SSOT（design-only）: `docs/development/current/main/design/verified-recipe-port-sig-ssot.md`
- Loop-carried completeness は Option A（VerifiedRecipe が明示で carrier list を持つ）を採用する。
  - SSOT（design-only）: `docs/development/current/main/design/verified-recipe-port-sig-ssot.md`

2026‑01‑24
- generic_loop_v1 の受理は “ShapeId 列挙” を真実にしない。Recipe/Verifier を受理のSSOTとし、ShapeId は hint-only に降格する。
  - 理由: Recipe は再帰構造であり、合成閉包を壊す ShapeId の組み合わせ増殖は運用破綻を招く。
  - SSOT: `docs/development/current/main/design/generic-loop-v1-acceptance-by-recipe-ssot.md` / `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- 実装順序は `compiler-task-map-ssot.md` を SSOT として固定する（観測→境界→受理→タグ→残骸掃除の順）。
  - 理由: selfhost ブロッカーに引きずられて “受理・タグ・残骸” を先に触る迷走を防ぐ。
  - SSOT: `docs/development/current/main/design/compiler-task-map-ssot.md` / `docs/development/current/main/10-Now.md`

2026‑01‑15
- B3 sugar（`if local x = f(); x > 0 { ... }` など）は v1 freeze の対象外として **実装を保留**し、v2 backlog へ送る（迷走防止）。
  - 理由: v1 は selfhost compiler / `.hako` mirbuilder 移植を “揺れなく” 進める境界であり、糖衣は surface churn を増やす。
  - 設計SSOT（design only）: `docs/development/current/main/design/block-expr-b3-sugar-decision.md`
  - v1 boundary SSOT: `docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md`
- Phase-0 の `.hako` mirbuilder は MIR JSON **v0** を出力する（出力形式の揺れを潰す）。
  - 理由: `--mir-json-file` の既存受理は v0 loader（`src/runner/mir_json_v0.rs`）が最小で、検証距離が短い。
  - v1 は Phase-1 以降（必要なら）で移行し、v0→v1 変換は Rust 側の bridge に寄せる（`.hako` 側に二重実装しない）。
  - 指示書SSOT: `docs/development/current/main/design/hako-mirbuilder-migration-phase0-entry-contract-ssot.md`

2026‑01‑16
- RecipeVerifier の **契約チェックは常時 fail-fast** にする（release でも有効）。
  - Debug/冗長チェックのみ dev/strict に残す。
  - 目的: 仕様と実装のズレを即発見し、デバッグ距離を短縮する。
  - SSOT: `src/mir/builder/control_flow/plan/parts/verify.rs` / `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`

2026‑01‑22
- Recipe-first 移行は **機能追加ではなく構造改革**として開始する（既定挙動は不変）。
  - 条件: planner_required/dev の並行パスのみ、1コミット=1受理形/1構造単位。
  - SSOT: `docs/development/current/main/design/recipe-first-migration-phased-plan-proposal.md`

2026‑01‑13 (superseded by 2026‑01‑16)
- Verifier（RecipeBlock mechanical checks）は dev/strict only で実行する（release では skip）。
  - 2026‑01‑16 の決定で **契約チェック常時ON** に更新済み（この決定は履歴として保持）。

2026‑01‑02
- legacy mode の function lowering（static/instance）では `type_ctx` を関数境界で必ず分離する（save/restore）。ValueId が関数ローカルである前提を破ると、callee 解決が揺れてフレークになる。
- selfhost（`.hako` 側の回避的対応）を優先せず、compiler 側（Facts/Normalize/CorePlan）の表現力を先に増やす（方針SSOT: `docs/development/current/main/design/compiler-expressivity-first-policy.md`）。
2026‑01‑25
- ObligationState を Verifier 返却として固定（design-only）。
- PortSig 合成は Verifier で行い、Parts は検査のみ。
- Return obligation freeze の適用範囲は strict/dev(+planner_required) に限定（design-only）。

2026‑01‑01
- CompareOperator.apply の adopt (`NYASH_OPERATOR_BOX_COMPARE_ADOPT`) は既定OFFにする（意味論の SSOT は VM 側、CompareOperator は observe-only で Void を返す）。
- `--dev` は adopt/tolerate を暗黙で有効化しない（observe-only を維持し、必要な場合は明示的に env を設定する）。

2025‑12‑13
- JoinIR lowering の name-based 変数解決は、dev-only（`normalized_dev`）で BindingId-based に段階移行する（dual-path を維持）。
- promoted carriers（DigitPos/Trim などの synthetic name）は、`BindingId(original) → BindingId(promoted) → ValueId(join)` の鎖で接続し、by-name ルール分岐は導入しない。
- debug/観測は既存のフラグ（例: `NYASH_JOINIR_DEBUG`）に集約し、新しい環境変数のスパローは避ける。

2025‑12‑19
- return の表現力拡張は「パターン総当たり」ではなく、pure expression を扱う `NormalizedExprLowererBox`（AST walker）へ収束させる（Phase 140）。
- Call/MethodCall は effects + typing の論点が増えるため、pure とは分離して Phase 141+ で段階投入する。
- out-of-scope は `Ok(None)` で既存経路へフォールバックし、既定挙動不変を維持する（strict は “close-but-unsupported” のみ fail-fast）。

2025‑12‑20
- Phase 256 の詰まり（Jump/continuation/params/jump_args）を「暗黙 ABI の分裂」と捉え、契約を `JoinIR ABI/Contract` として明文化していく（SSOT を 1 箇所へ集約）。
- continuation の識別は ID を SSOT（String は debug/serialize 用）とし、`join_func_N` の legacy は alias で隔離する。
- `jump_args` は意味論の SSOT なので、最終的には MIR terminator operand に統合して DCE/CFG から自然に追える形へ収束させる（Phase 256 を緑に戻した後に段階導入）。
- 上記の収束先（north star）を “Join-Explicit CFG Construction” と命名し、段階移行（案1→案2→必要なら案3）で進める。
- 正規化（normalized）を **Semantic/Plumbing** に分離し、`NormalizeBox`（意味SSOT）/ `AbiBox`（役割SSOT）/ `EdgeArgsPlumbingBox`（配線SSOT）の最小セットで “推測禁止 + Fail-Fast” を維持する。
- spans は並行 Vec を最終的に廃止し、`Vec<Spanned<_>>` へ収束（段階導入: 編集APIの一本化 → 内部表現切替）。
- edge-args の参照 API は `Jump` だけでなく `Branch` を含むため、単発 `edge_args()` ではなく `out_edges()`/`edge_args_to(target)` のような “複数 edge” 前提の参照点を SSOT にする。

2025‑12‑21
- MIR 側の block-parameterized CFG を短い通称として **EdgeCFG** と呼ぶ（docs では “Block-Parameterized CFG（EdgeCFG）”）。
- EdgeCFG の P2（`BasicBlock.jump_args` 削除）まで到達し、edge-args は `Jump/Branch` の terminator operand を SSOT に一本化する（Return は `return_env` のみ例外）。
- 「pattern番号で推測分岐」は長期的に消したい。Structured→CFG lowering の中心概念を **ExitKind + Frag（fragment）**へ移し、pattern は “Extractor/Plan の薄い層” に縮退させる（設計SSOT: `docs/development/current/main/design/edgecfg-fragments.md`）。

2025‑09‑08
- ループ制御は既存命令（Branch/Jump/Phi）で表現し、新命令は導入しない。
- Builder に loop_ctx（{head, exit}）を導入し、continue/break を分岐で降ろす。
- Verifier の支配関係/SSA を崩さないよう、単一 exit と post‑terminated 後の emit 禁止を徹底。
- VInvoke（vector 経路）の戻り値は、短期は「既知メソッドの整数返り」を特例扱いで保持し、
  中期は nyash.toml の戻り型ヒント or NyRT シムの期待フラグで正道化。
