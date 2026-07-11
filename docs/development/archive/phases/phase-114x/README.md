# Phase 114x: execution surface wording closeout

- 目的: public/help surface を `mainline route` と `explicit keep/reference override` に分け、`--backend vm` を通常経路に見せない。
- 対象:
  - `src/cli/args.rs`
  - `src/runner/dispatch.rs`
  - `README.md`
  - `README.ja.md`
  - `docs/development/runtime/cli-hakorune-stage1.md`
- success:
  - `run.sh --runtime --runtime-route mainline` が selfhost mainline として読める
  - `--backend vm` / `--backend vm-hako` は explicit keep/reference override としてだけ読める
  - current pointers が `phase-114x` に揃う
