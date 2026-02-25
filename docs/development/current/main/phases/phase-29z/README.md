# Phase 29z: RC insertion minimal（Phase 29y follow-up）

Status: P2 Closeout  
Scope: Phase 29y の SSOT（RC insertion / ABI / observability）を前提に、**RC insertion pass を最小動作**まで進める。  

Entry:
- SSOT: `docs/development/current/main/phases/phase-29y/20-RC-INSERTION-SSOT.md`
- 指示書: `docs/development/current/main/phases/phase-29z/P0-RC_INSERTION_MINIMAL-INSTRUCTIONS.md`

Opt-in:
- `rc-insertion-minimal` Cargo feature（既定OFF）

Verification:
- `cargo build --release` / `./tools/smokes/v2/run.sh --profile quick`
- `cargo run --bin rc_insertion_selfcheck --features rc-insertion-minimal`

Progress:
- P0: overwrite release（Store 上書き）
- P1: explicit drop（Store null）を最小対応
- P2: closeout（Return終端のcleanup追加、残課題を整理）

Next Phase:
- Phase 29aa（complete）: `docs/development/current/main/phases/phase-29aa/README.md`

Residuals（分類）:
- Design: CFG-aware RcPlan（block/edge 単位）の契約と Fail-Fast 方針、PHI/loop/early-exit の危険パターン整理、Branch/Jump 終端 cleanup 設計
- Implementation: null 伝搬の精度向上（copy以外の伝搬パターン追加）
- Verification: quick 154/154 PASS 維持（既定OFF）+ `rc_insertion_selfcheck` opt-in 確認
