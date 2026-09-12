---
Status: closed__Fast__R7StaticV2ExplicitNull__2026-09-12
Task: MIR-CALL-COMPATIBILITY-RETIRE-R7-STATIC-V2-OPEN-NULL-I0
Date: 2026-09-12
Parent: mir-call-compatibility-retire-r7-static-v2-open-d0-2026-09-12.md
---

# R7 static V2 explicit-options NULL correction

## Six-line brief

```text
Decision: reject NULL at the explicit Static V2 options entry while retaining the three-argument compatibility entry.
Source authority + canonical issuer: the caller-selected explicit physical contract at hako_llvmc_static_open_v2_with_options.
Non-authority: the compatibility wrapper, ambient settings, parsed JSON, and an allocated invocation do not satisfy the explicit contract.
Fail-fast boundary: the explicit entry rejects before JSON parsing, invocation allocation, lowering, or artifact publication.
Smallest next slice: add the existing C entry guard, its production-session negative, and the owning API/guard wording.
Non-claims: no public-symbol retirement, compatibility change, new receipt, MIR change, or whole-R7 completion.
```

Census boundary: `hako_llvmc_static_open_v2_with_options` -> its existing
contract helper and `static_v2_session_test.py`; includes only the explicit
Static V2 NULL admission edge. The three-argument
`hako_llvmc_static_open_v2` compatibility path and all later compile paths are
excluded.

## Contract and acceptance

The explicit-options entry is a contract-bearing production API. A NULL
contract must return a named `[freeze:contract][static-v2/options-null]`
diagnostic with no handle, before the shared helper can take its compatibility
branch. The existing three-argument entry intentionally continues to pass NULL
to the internal helper as its compatibility behavior.

Acceptance is the existing production shared-library session test with a
NULL explicit contract and valid input, plus a malformed input ordering probe;
both must reject with no handle and preserve the environment. Existing
explicit profile/revision/flags negatives, fake tool propagation, query,
compile/cleanup, compatibility open, C build, static guard, pointer guard and
source-size limits remain required.

## Owner and delete set

Owner: `lang/c-abi/shims/published_mir/hako_llvmc_ffi_static_v2_compile.inc`
explicit wrapper. No shared options helper, invocation owner, Rust caller,
compatibility symbol, or lowering route is deleted. The old behavior being
removed is only the explicit wrapper's accidental NULL-to-compatibility
delegation.

No new authority or stored state is needed. The existing C error/status path
remains the sole reject terminal, and the existing `out = NULL` initialization
continues to protect callers on every rejected open.

## I0 closeout evidence

Passed after the final `out` initialization refinement:

```text
bash tools/build_hako_llvmc_ffi.sh
NYASH_NY_LLVM_OPT_TOOL=/usr/bin/opt-18 \
NYASH_NY_LLVM_LLC_TOOL=/usr/bin/llc-18 \
  python3 lang/c-abi/tests/static_v2_session_test.py target/release/libhako_llvmc_ffi.so
  -> static V2 session capture/error/cancel/compile: ok
bash tools/checks/mir_call_static_v2_open_contract_guard.sh
bash tools/checks/llvm_compile_options_contract_smoke.sh
bash tools/checks/llvm_codegen_route_identity_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The session test covers valid and malformed input with NULL explicit contracts,
non-NULL output-handle reset, existing profile/revision/flags negatives, fake
tool/flag propagation, compatibility open, query, compile/cleanup and ambient
preservation. The installed LLVM18 paths are explicit because the host's bare
`opt`/`llc` names resolve to LLVM14; this is environment selection evidence,
not a change to the API owner. No Cargo or MIR route changed.
