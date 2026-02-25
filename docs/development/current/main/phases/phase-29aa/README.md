# Phase 29aa: RC insertion safety expansion（CFG-aware design）

Status: P8 Complete (Null propagation across CFG; Copy-only)  
Scope: Phase 29z の単一block限定実装から、誤releaseを起こさない形で CFG-aware に拡張するための設計を固める。  

Entry:
- SSOT: `docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md`
- 指示書: `docs/development/current/main/phases/phase-29aa/P0-RC_INSERTION_CFG_AWARE_DESIGN-INSTRUCTIONS.md`

Non-goals:
- 既定挙動変更（feature `rc-insertion-minimal` 以外は no-op 維持）
- SSA last-use を根拠にした drop
- いきなり全ケース実装（設計＋最小プロトタイプを優先）

Deliverables (P0):
- RcPlan（解析→挿入の二段階）設計
- PHI/loop/early-exit の危険パターン整理と Fail-Fast 方針
- release 安全条件の契約
- P1 で実装する最小ターゲットを 1 個に絞る

Progress:
- P0: CFG-aware 設計の固定（RcPlan/危険パターン/安全条件の契約）
- P1: rc_insertion を RcPlan の Plan→Apply 2-stage へ分離（挙動不変）
- P2: Jump/Branch 終端では cleanup を入れない契約を SSOT 化（Fail-Fast guard）
- P3: Jump→Return（単一 predecessor）で state 伝播し ReturnCleanup を成立させる（P2維持）
- P4: Jump-chain（単一 predecessor 直列）で state 伝播し ReturnCleanup を成立させる（P2/P3 維持）
- P5: Multi-predecessor Return で incoming state が完全一致する場合のみ ReturnCleanup を成立させる（P2/P3/P4 維持）
- P6: Multi-predecessor Return で incoming state の「安全な共通部分（intersection）」のみ cleanup する（P2-P5 維持）
- P7: ReleaseStrong の values 順序を決定的にする（HashSet/HashMap 由来の非決定性を排除）
- P8: Null 伝播を CFG を跨いで扱う（最初は Copy-only + single-predecessor）

P3 SSOT:
- Contract:
  - cleanup は Return block の BeforeTerminator のみ（Jump/Branch block には入れない）
  - Jump→Return かつ predecessor が 1 つの場合のみ state 伝播を許可
  - 条件不一致で伝播/cleanup を試みたら debug_assert! で Fail-Fast
- Non-goals:
  - Branch/PHI/loop/early-exit の cleanup
  - multi-predecessor の合流（PHI 問題回避）
  - Jump block への release 挿入（P2維持）
- Acceptance:
  - quick 154/154 PASS 維持
  - `cargo run --bin rc_insertion_selfcheck --features rc-insertion-minimal` PASS
  - 既定OFF維持（featureなしは no-op）

P4 SSOT:
- Contract:
  - cleanup は Return block の BeforeTerminator のみ（Jump/Branch block には入れない）
  - Jump-chain（単一 predecessor の Jump 直列）のみ state 伝播を許可
  - cycle（loop）検出時は debug_assert! で Fail-Fast、伝播は停止
- Non-goals:
  - Branch/PHI/loop/early-exit の cleanup
  - multi-predecessor の合流（PHI 問題回避）
  - Jump block への release 挿入（P2維持）
- Acceptance:
  - quick 154/154 PASS 維持
  - `cargo run --bin rc_insertion_selfcheck --features rc-insertion-minimal` PASS
  - 既定OFF維持（featureなしは no-op）

P5 SSOT:
- Contract:
  - cleanup は Return block の BeforeTerminator のみ（Jump/Branch block には入れない）
  - Return の multi-predecessor 合流は次の条件を全部満たすときだけ許可:
    1. 対象 block の terminator は Return
    2. predecessor が 2 以上
    3. 全 predecessor の end_state が完全一致（同じ ptr→value map）
    4. 一致した state が非 empty
  - 条件を満たさない場合は initial_state を作らない（ReturnCleanup なし）
  - 条件NG時に古い initial_state が残らないよう、毎回再計算して remove する（落とし穴1対策）
  - initial_state を set/remove したら changed フラグを更新（落とし穴2対策）
- Non-goals:
  - 合流 state の "部分一致" / subset / merge（PHI相当なので禁止）
  - Branch/PHI/loop/early-exit の cleanup
  - Jump block への release 挿入（P2維持）
- Acceptance:
  - quick 154/154 PASS 維持
  - `cargo run --bin rc_insertion_selfcheck --features rc-insertion-minimal` PASS
  - selfcheck Case 3.7（state一致 → Return block に 1 cleanup）PASS
  - selfcheck Case 3.8（state不一致 → 全ブロック 0 cleanup）PASS
  - 既定OFF維持（featureなしは no-op）

P6 SSOT:
- Objective:
  - Return block が multi-predecessor のとき、incoming state が完全一致しない場合でも
    「全経路で必ず保持されている ptr→value」のみを ReturnCleanup で release する。
- Contract:
  - cleanup は Return block の BeforeTerminator のみ（Jump/Branch block には入れない）
  - join state は `intersection`（全 predecessor の end_state に同じ ptr が存在し、かつ value が同一のものだけ）
  - intersection が empty の場合は cleanup しない
  - subset/partial merge は許可するが、PHI 的な "値の合成" はしない（同一値のみ）
- Non-goals:
  - PHI/loop/early-exit の cleanup
  - value が一致しない ptr を release 対象に含めること
  - Jump block への release 挿入（P2維持）

P7 SSOT:
- Objective:
  - ReleaseStrong の `values: Vec<ValueId>` の順序を決定的にし、再現性と差分追跡性を上げる。
- Contract:
  - 生成する `values` は常に `ValueId` 昇順に sort する
  - dedup は “順序固定後に隣接重複を除去” で OK（または BTreeSet）
  - 挙動は不変（release される集合は同じ、順序だけを固定）
- Scope:
  - ReturnCleanup（単一block / P4 chain / P6 join）の `values` が対象
  - Overwrite/ExplicitNull の単一要素 vec はそのままで OK

P8 SSOT:
- Objective:
  - `Store null`（explicit drop）の追跡を CFG 越しに正しく扱い、誤 release を避ける。
- Contract:
  - 最初は Copy-only の null 伝播のみ（`Copy dst, src`）
  - single-predecessor（Jump-chain / single-pred Return）範囲に限定
  - multi-predecessor では null 伝播は合流しない（保守的に Unknown 扱い）
- Acceptance:
  - selfcheck: null を A で作って B で Store に使う（Jump で跨ぐ）ケースを追加し PASS
- Conservative:
  - intersection は「安全だが保守的」。value 不一致の ptr は release されない＝リーク方向
  - P5 でも同じく release できてなかったケースなので回帰にはならないが、完全なメモリ回収には将来の PHI 対応が必要
- Acceptance:
  - quick 154/154 PASS 維持
  - `cargo run --bin rc_insertion_selfcheck --features rc-insertion-minimal` PASS
  - selfcheck Case 3.11（values が昇順であることを検証）PASS
  - 既定OFF維持（featureなしは no-op）
