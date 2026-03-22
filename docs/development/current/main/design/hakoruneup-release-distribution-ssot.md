---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `hakoruneup` を入口にした self-contained release bundle 配布形と、その上に package manager を載せる順序を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/de-rust-full-rust-zero-roadmap-ssot.md
  - docs/development/current/main/design/de-rust-backend-zero-boundary-lock-ssot.md
  - docs/development/current/main/design/hako-fullstack-host-abi-completion-ssot.md
  - docs/development/current/main/design/backend-legacy-preservation-and-archive-ssot.md
  - docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md
---

# Hakoruneup Self-Contained Release Distribution (SSOT)

## Purpose

- 配布の本命を `hakoruneup + self-contained release bundle` に固定する。
- package manager はその上に乗る層として扱い、配布の本体に toolchain semantics を埋め込まない。
- LLVM は一般ユーザー向けには同梱物として隠し、別インストール要求を原則なくす。
- system LLVM は開発者向けの explicit opt-in モードとしてだけ残す。
- 既存の kernel migration / thin backend / de-rust lanes と矛盾しない配布形を先に固定する。

## Final Shape

| layer | owner | responsibility | default posture |
| --- | --- | --- | --- |
| `hakoruneup` | launcher / bootstrapper | bundle discovery, install, update, channel selection | default entry |
| self-contained release bundle | distribution unit | compiler, runtime, bundled LLVM, checksums, release metadata | user-visible artifact |
| package manager | top layer | bundle selection, update coordination, integrity verification | above the bundle |
| system LLVM dev mode | explicit opt-in | developer-only host LLVM selection for debugging / advanced workflows | non-default |

## Bundle Contract

- default user path:
  - `hakoruneup` launches against a self-contained release bundle.
  - the bundle carries the LLVM toolchain artifacts needed for normal use.
  - the user does not install LLVM separately.
- package manager path:
  - package manager selects / installs / updates bundles.
  - it does not own compiler semantics.
  - it does not become the distribution truth for toolchain behavior.
- dev-only path:
  - system LLVM remains available as an explicit advanced mode.
  - the default path must not silently fall back to host LLVM.
  - host LLVM use must be visible in docs and smoke output.

## What Belongs in the Bundle

- `hakoruneup` launcher binary or launcher payload
- runtime binaries / shared runtime assets needed by the release
- compiler binaries or compiler-side executable payloads needed for local execution
- bundled LLVM toolchain artifacts used by the default route
- release manifest
- checksums / provenance metadata
- update-channel metadata if the package manager consumes the bundle directly

## What Stays Out of the Default Bundle Contract

- silent host LLVM discovery as a general-user requirement
- package manager semantics inside the compiler/runtime owner docs
- backend behavior changes that belong to kernel migration or backend-zero
- fallback ladders that make host LLVM appear as the implicit default

## Implementation Order

1. bundle inventory lock
   - enumerate what a release bundle contains
   - freeze the manifest shape and checksum/provenance rows
2. `hakoruneup` bootstrap contract
   - define how the launcher discovers and selects the bundle
   - keep the launcher path thin and explicit
3. bundled LLVM default route
   - make the bundled LLVM the normal path
   - keep host LLVM out of the default user experience
4. package manager integration
   - define how the package manager installs, updates, and pins bundles
   - ensure it sits on top of the bundle, not under it
5. system LLVM dev mode
   - add explicit advanced-mode selection for host LLVM
   - keep it opt-in and docs-visible
6. verification / release smoke
   - pin one smoke for default bundled LLVM
   - pin one smoke for explicit system LLVM dev mode
   - pin integrity / checksum validation for bundle resolution

## Non-Goals

- requiring a separate LLVM install for ordinary users
- making package manager the owner of compiler/runtime semantics
- hiding host LLVM fallback behind silent heuristics
- using bundle packaging as a shortcut to bypass kernel migration or backend boundary work
- changing current kernel capability ladder order

## Relation to Kernel Migration

- current `.hako` kernel migration is compatible with this distribution shape.
- the more the native keep shrinks, the easier it is to ship one bundle with a predictable toolchain surface.
- the distribution contract is about packaging and resolution, not about moving backend/runtime ownership in this doc.

## Acceptance

- the default user story is: install `hakoruneup`, fetch a self-contained release bundle, run without separate LLVM installation.
- the advanced user story is: explicitly opt into system LLVM and see that choice in docs / smoke evidence.
- the package manager story is: manage bundles above the bundle contract, without becoming a competing compiler owner.
- the docs story is: this repo can point to one SSOT for distribution shape without reusing backend-zero or kernel-lane docs as packaging policy.
