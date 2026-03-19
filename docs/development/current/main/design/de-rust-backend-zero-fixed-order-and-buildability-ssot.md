---
Status: SSOT
Decision: provisional
Date: 2026-03-19
Scope: backend-zero の active migration order を `current owner cutover -> compat keep reduction -> bootstrap keep reduction` に固定し、Rust ベースの buildability を gate contract として保つ。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/de-rust-zero-buildability-contract-ssot.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-provisional-inventory-ssot.md
  - docs/development/current/main/design/backend-legacy-preservation-and-archive-ssot.md
  - docs/development/current/main/design/backend-recipe-route-profile-ssot.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P1-NY-LLVMC-NATIVE-EMITTER-TASK-PACK.md
  - docs/development/current/main/phases/phase-29ck/P2-NATIVE-EMITTER-ACCEPTANCE-AND-REOPEN-RULE.md
  - docs/development/current/main/phases/phase-29cl/README.md
---

# De-Rust Backend-Zero Fixed Order And Buildability (SSOT)

## Purpose

- backend-zero は kernel migration の stop line の次に来る hard lane であり、境界いじりを続ける場所ではない。
- daily owner を `.hako -> thin backend boundary` に寄せる順番と、Rust ベースの build/bootstrap route を保持する順番を分離する。
- buildability は owner ではなく gate contract である。
- `0rust` は Rust meaning owner zero を意味するが、Rust の build/bootstrap route zero を意味しない。

## 1. Fixed Order

### 1. Current owner cutover

- `.hako` caller が daily route の owner になる。
- `BackendRecipeBox` / `LlvmBackendBox` の route/profile/pure-first policy は `.hako` 側が持つ。
- `native_driver.rs` は bootstrap seam only のままにする。
- `llvmlite` / `harness` / `native` keep lanes は explicit keep として残してよいが、daily owner にはしない。

### 2. Compat keep reduction

- old wrapper / compat route を thin floor まで縮める。
- C/Rust glue は export / marshal / diagnostics / loader の substrate に寄せる。
- compatibility lane を「まだ再生できる」状態から「daily route ではない」状態へ落とす。

### 3. Bootstrap keep reduction

- stage1 / JSON / module dispatch / native seam の keep bucket を 1 つずつ畳む。
- `program_json_v0` / `module_string_dispatch` / `native_driver.rs` は bootstrap-only / canary-only の範囲を超えない。
- bootstrap keep は、daily owner cutover と compat reduction が安定した後にだけ薄くする。

## 2. Buildability Gate

- migration slice を切っても、Rust から daily / compat / bootstrap build を再実行できる状態を保つ。
- buildability は workaround ではなく contract である。
- owner cutover と buildability cutover を同じ slice で壊さない。
- Rust buildability が壊れる slice は keep 条件を満たさない。

### Must remain buildable

- stage1 / bootstrap build paths
- compat / canary build paths
- `.hako` mainline を Rust から再構築する最小導線
- archive / preservation-first restore path

## 3. Boundary Roles

### `.hako` policy owner

- route selection
- compile recipe
- compatibility replay policy
- acceptance / evidence naming
- visible daily owner choice

### Rust substrate / bootstrap / compat

- payload decode
- symbol selection
- boundary glue
- bootstrap seams
- archive-compatible keep lanes

### C substrate

- export / marshal
- loader / process / path glue
- thin transport-only fallback

## 4. Stop Lines

- Do not add a new canonical ABI surface for backend-zero.
- Do not promote `native_driver.rs` to final owner.
- Do not mix backend-zero order changes with kernel migration refactors.
- Do not turn buildability into a separate authority that can override owner cutover.
- Do not silently delete Rust build routes while the migration is still in flight.

## 5. Practical Acceptance

1. daily mainline build remains reproducible from Rust-based build/bootstrap route
2. `.hako` caller can drive the thin backend boundary without re-owning policy in Rust
3. compat replay still works for current keep lanes
4. bootstrap seam remains canary-only and does not become final owner
5. acceptance / smoke rows remain named in `.hako`, not inferred from the transport shim

## 6. Relation To Current Phases

- `phase-29cm`:
  - kernel authority migration stop line / handoff source
- `phase-29ck`:
  - backend-zero execution lane and promotion gate
- `phase-29cl`:
  - by-name zero-caller closeout / compat archive maintenance

## 7. Done Shape

- backend-zero daily route is `.hako -> thin backend boundary -> object/exe`
- Rust is still a valid build/bootstrap route
- compat keeps are explicit, thin, and non-authoritative
- bootstrap seams are canary-only or retired
- no new semantic owner has leaked back into Rust
