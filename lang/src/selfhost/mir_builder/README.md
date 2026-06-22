Self‑Host MIR Builder Helpers (Scaffold) — Phase‑20.12b

Purpose
- Keep minimal helper scaffolds (`phi`/`verify`) for selfhost MIR contracts.
- Keep default MIR generation behavior unchanged. MIR generation SSOT is
  `lang/src/compiler/mirbuilder/`.
- Do not report scaffolded MIR verification as success.

Scope (this scaffold)
- Files: phi.hako, verify.hako, LAYER_GUARD.hako
- Behavior: exported placeholders fail-fast when called. They are registry
  surface only until real verification / PHI construction lands.

Non‑Goals (now)
- Replacing Rust MIR generation by default
- Emitting full MIR coverage (call/extern/boxcall/complex boxes)

Interfaces (subject to evolution)
- SelfhostMirVerify.verify(json_path) -> int
  - `0` means verified.
  - Current scaffold returns non-zero and prints
    `[selfhost/mir_builder/verify:unimplemented]`.
- SelfhostPhiBox helpers return `null` and print stable unimplemented tags.

Notes
- `builder.hako` scaffold was retired.
- Active selfhost MIR builder implementation lives under `lang/src/compiler/mirbuilder/`.

Gates (opt‑in)
- NYASH_USE_NY_COMPILER=1 → future: emit‑only builder path
- NYASH_JSON_ONLY=1 → future: sidecar JSON dump for parity check

Layer Guard
- See LAYER_GUARD.hako — allowed imports are restricted to shared JSON/MIR helpers. No VM/LLVM interaction here.

Rollback
- Folder is isolated under lang/src/selfhost/. Removing this directory reverts to current behavior.
