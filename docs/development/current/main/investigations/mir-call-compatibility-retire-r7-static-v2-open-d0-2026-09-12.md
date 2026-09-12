---
Status: closed__DecisionAccepted__R7StaticV2Open__2026-09-12
Task: MIR-CALL-COMPATIBILITY-RETIRE-R7-STATIC-V2-OPEN-D0
Date: 2026-09-12
Parent: mir-call-compatibility-retire-r7-d0-2026-09-11.md
NextCard: MIR-CALL-COMPATIBILITY-RETIRE-R7-STATIC-V2-OPEN-I0
---

# R7 static V2 open ownership D0

## Six-line brief

```text
Decision: select the existing static V2 open/query/compile/close lifecycle as one R7 owner; retain harness, AOT compatibility, and public ABI.
Source authority + canonical issuer: PublishedMirBackendView/PublishedStaticMethodCFrameV2 issue the existing static request; Rust Opts and the accepted physical-options contract issue invocation options once.
Non-authority: ambient environment, names, static test output, harness compatibility, AOT public symbols, and C-side rediscovery do not issue static meaning or options.
Fail-fast boundary: static open validates the existing profile/options contract before C lowering or object publication; invalid/mismatched input terminates the invocation and preserves output cleanup.
Smallest next slice: pass the already-normalized physical contract into the existing static V2 open owner, remove only its Rust settings save/restore and duplicate option reads, and prove the existing lifecycle.
Non-claims: no harness/AOT/public-ABI retirement, no concurrent-compilation guarantee, no new receipt, no MIR change, and no whole-R7 completion.
```

Census boundary: `published_mir_object.rs` -> `static_invocation.rs` -> the
existing `hako_llvmc_static_open_v2` query/compile/close lifecycle; includes
the static V2 Rust caller, the C static V2 entry and its invocation owner, and
the static settings environment seam; excludes explicit harness, AOT v1/v2
compatibility callers, public ABI/dlsym, published Call-row globals, and MIR
semantic issuance.

## Accepted authority and caller

The real production caller is the existing published MIR object path:

```text
PublishedMirBackendView + PublishedStaticMethodCFrameV2
  -> static_invocation::open/query/compile/close
  -> hako_llvmc_static_open_v2_with_options
  -> existing HakoLlvmcInvocation + static C terminal
```

The published MIR view and static method frame remain the source/backend
request authority. The existing Rust `Opts` admission and versioned physical
options contract normalize recipe, replay, opt level, tool paths, and flags;
`HakoLlvmcInvocation` owns the C copy for the synchronous invocation. This
reuses the I1A contract and adds no semantic receipt or settings layer.

The existing `open -> query -> compile -> close` owner is retained. Its named
terminal is the current typed C error/status path before object publication;
failed compile leaves the existing output suppression and invocation cleanup.
The static V2 entry consumes the contract and must not read ambient options a
second time or reconstruct a target from MIR/JSON.

## Exclusive delete-set and state table

The same I0 series may delete only the static V2 Rust settings save/restore,
its duplicate option/environment reads, and the static-open caller's old
three-argument or environment-mutating handoff after the explicit contract is
consumed. It must not delete public symbols, link-only environment handling,
the explicit harness export, AOT compatibility, or published-row globals.

| state | owner | pre-effect terminal | next action |
| --- | --- | --- | --- |
| `ExplicitStaticV2` | Rust static open + C invocation | existing static query/compile result | consume one contract |
| `InvalidContract` | C contract validator | named validation error, no object | terminal reject |
| `CompileFailure` | existing static invocation | existing error and output cleanup | terminal failure |
| `CompatibilityOther` | harness/AOT/public ABI owners | existing compatibility behavior | retain outside this row |
| `Unclassified` | no owner | design stop | do not infer/default |

## Implementation contract

The implementation may touch only the existing static V2 Rust/C entry and
its focused tests/README/guard references. It must preserve query, compile,
close, error and cleanup ownership, and must demonstrate:

- one production static V2 open caller consumes the versioned contract;
- fake tool/path/flag propagation reaches the existing C invocation;
- unset, empty, and present ambient state are preserved on success and failure;
- invalid revision/size/profile/flags and option conflicts reject before C
  lowering or object publication; and
- the static V2 owner has no second environment/options snapshot or retry.

No test-only open path may substitute for the production caller. A missing
toolchain is environment evidence, not runtime success.

## Non-claims

This row does not retire harness, AOT, public ABI, dlsym, link-only state, or
published Call-row globals. It does not claim concurrency isolation, backend
parity, source-to-EXE success, LegacyCallV0 caller-zero, Loop production
selection, or whole-MIRBuilder completion.

## I0 closeout evidence (2026-09-12)

Passed:

```text
bash tools/build_hako_llvmc_ffi.sh
python3 lang/c-abi/tests/static_v2_session_test.py target/release/libhako_llvmc_ffi.so
bash tools/checks/llvm_compile_options_contract_smoke.sh
bash tools/checks/mir_call_static_v2_open_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
CARGO_BUILD_JOBS=1 CARGO_INCREMENTAL=0 RUST_MIN_STACK=16777216 \
  cargo check -p nyash-rust --features plugins --profile quick -j1
```

The static session test now proves the explicit Static V2 open/query/compile/
close lifecycle with fake tool/path/flag propagation, ambient recipe isolation,
and pre-effect revision/size/profile/flags rejection. The existing Boundary
compile-options smoke retains the generic/exact-seed propagation, environment,
and pre-effect contract evidence. The new static guard pins the explicit Rust
symbol, removes the old Rust environment save/restore seam, and keeps the
C/Rust sources below 800 lines.

The host has LLVM 14, so the legacy compatibility portion of the static
session test reports an explicit LLVM18 object-compile skip; this is environment
evidence, not a runtime or executable success claim. No static V2 public
symbol, harness/AOT compatibility owner, published-row global, or whole-R7
caller-zero claim is made.
