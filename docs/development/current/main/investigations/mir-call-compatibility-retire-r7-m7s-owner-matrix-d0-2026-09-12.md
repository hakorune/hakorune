---
Status: closed__DecisionAccepted__R7M7SOwnerMatrix__2026-09-12
Task: MIR-CALL-COMPATIBILITY-RETIRE-R7-M7S-OWNER-MATRIX-D0
Date: 2026-09-12
Parent: mir-call-compatibility-retire-r7-d0-2026-09-11.md
---

# R7 remaining physical-options owner matrix

## Six-line brief

```text
Decision: stop before another compatibility implementation until the remaining physical-options owners and delete-sets are co-sealed.
Source authority + canonical issuer: each existing ingress contract and its named production caller; no cross-lane inference.
Non-authority: ambient environment readers, public symbol presence, static test output, and a shared helper's lexical reach do not prove caller-zero.
Fail-fast boundary: every selected owner must reject invalid recipe/replay/tool/contract input before child, lowering, or artifact effects.
Smallest next slice: read-only finite census of owner, caller, option snapshot, terminal, shared consumers, and exclusive retirement boundary.
Non-claims: no route switch, compatibility retirement, new receipt, fallback, warning cleanup, or whole-R7 completion.
```

Census boundary: `hako_llvmc_compile_json_with_options_v1` and its existing
physical-options helper callers -> the terminal C/AOT/public/link entrypoints.
Include Generic profile 0, Boundary profile 1, Static profile 2, Explicit
Harness profile 3, generic/v1 AOT compatibility, public ABI/dlsym, link-only
environment handling, and the Rust provider/llvmlite compatibility caller.
Exclude the already-closed Static V2 NULL correction, MIR semantic issuance,
Loop selection, warning census, and external backend parity.

## Design-stop questions

- Which existing caller is the canonical issuer for each remaining profile and
  compatibility entry, and what is its exact production effect boundary?
- Which environment/options snapshots are shared, duplicated, or ambient, and
  which can be removed without changing an external compatibility contract?
- Does each proposed retirement have caller-zero or an existing pre-effect Stop
  boundary, with no unconsumed sibling product or public re-entry?
- Where must typed diagnostics be promoted in `direct_accum`,
  `nested_predicate`, and pending/bridge paths without broadening this owner?
- Is `llvm_provider_flags.rs::backend_codegen_request_defaults` truly
  caller-zero, or does a finite guard need to cover its remaining tests/callers?

The audit must remain read-only: no code, fixture, route activation, fallback,
new semantic receipt, Cargo build, worktree, or git mutation. A worker report
is evidence for one Decision, not implementation permission.

## Read-only owner decision

McClintock's finite audit returned the following dispositions:

| owner | canonical caller / terminal | disposition |
|---|---|---|
| Generic 0 | public C/AOT v1 -> existing pre-effect options admission | Promote landed; shared core retained |
| Boundary 1 | Rust `OwnedPhysicalCompileContract` -> existing C lowering | design stop: llc policy unresolved |
| Static 2 | published static Rust -> Static V2 lifecycle | closed; NULL correction `85a16bccfa` |
| Harness 3 | named C/AOT harness -> existing child terminal | retain; shared/provider callers remain |
| Public/AOT/link | exported ABI, dlsym and link dispatcher | retain; no exclusive delete-set |
| Rust provider | explicit route -> llvmlite provider | retain; caller-zero absent |
| bridge errors | `direct_accum`/`nested_predicate`/pending | separate typed-diagnostic slice |
| ambient helper | `backend_codegen_request_defaults` | repo callers zero; external API unresolved |

Decision: `NoSafeSlice__M7SNoRemainingUnsharedDeleteSet`. The selected finite
boundary is exhausted for this matrix; Boundary profile1 llc policy is the
next design stop. No code, fixture, route, fallback, receipt or Cargo change
was made by the audit.
