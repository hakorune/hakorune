# Published-row invocation ownership D0

Date: 2026-09-13  
Status: Decision implemented at `8fe3f00a6c`; retained ownership contract.

I0 is closed with WSL/Linux evidence. Current task selection belongs only to
`../CURRENT_STATE.toml` and its active card; the I0 tasks below document the
implemented contract and do not reopen implementation.

## Decision

The private published-call row ledger moves from process-global storage to one
`HakoLlvmcInvocation` owner. The ledger remains borrowed: it stores the Rust
row pointer and count, plus mode and `used[1024]`; it does not copy or extend
the Rust row lifetime. The existing Static V2 bind/begin boundary remains the
canonical production issuer.

The owner path is:

```text
static_v2_compile(invocation, frame)
  -> compile_static_v2_retained(invocation, frame)
  -> begin_v2(&invocation->published_call_rows, frame->calls, count)
  -> compile_doc_compat_pure(invocation, ...)
  -> prepass / emission / residual checks use the same owner pointer
  -> finish(&invocation->published_call_rows)
  -> artifact rename
  -> end(&invocation->published_call_rows) on every post-begin exit
```

`HakoLlvmcInvocation` is the authority; the row helper receives an explicit
`HakoLlvmcPublishedCallRows *` state. This keeps the helper independent of the
invocation definition while making every read, write, peek, take, finish, and
end owner-explicit. Non-published stack invocations retain an inactive zeroed
ledger.

## Issuer / terminal / consumer matrix

| Owner edge | Source-backed issuer | Consumer / terminal | Required route |
| --- | --- | --- | --- |
| Static V2 | `hako_llvmc_compile_static_v2_retained` after `static_v2_bind` | `compile_doc_compat_pure`; finish before rename; end after cleanup | pass invocation ledger |
| generic prepass | `compile_doc_compat_pure(invocation)` | intrinsic and i64 site `peek` | pass same ledger |
| same-module prepass | same invocation | i64 site `peek` | pass same ledger |
| same-module call emit | same invocation | i64 row `take` | pass same ledger |
| generic MIR call dispatch | same invocation | i64 row `take` | pass same ledger |
| generic op dispatch | same invocation | array-write row `take` | pass same ledger |
| typed field RMW | same invocation | array-write row `take` | pass same ledger |
| generic lowering residual | same invocation | `active`, `finish`, fallback suppression | pass same ledger |
| V1 test helper | test-owned local state only | focused pre-artifact tests | migrate to explicit state; no production issuer |

The complete consumer path is lexical through `compile_doc_compat_pure` and
its existing `invocation` context. No MIR, Recipe, public C ABI, row kind, or
coordinate authority is added. Typed kind/payload validation, exact-site
matching, duplicate coordinate rejection, duplicate take rejection, borrowed
row lifetime, and zero-row V2 behavior remain unchanged.

## Re-entry and overlap

- A second begin on the same invocation ledger rejects with the existing
  `rows already active` diagnostic. The first active ledger is not cleared;
  its caller must finish or end it.
- Different invocation ledgers may overlap. Each sees only its own rows and
  `used` bits; ending A cannot clear B. This is row-ledger isolation only, not
  a general backend, LLVM, or process-wide concurrency guarantee.
- A caller must keep each borrowed Rust row array alive until its owning
  invocation reaches end. C does not retain the rows after end and does not
  make an unsafe lifetime promise for overlapping foreign callers.

## Exact old-edge delete set

Delete the anonymous `hako_llvmc_published_call_rows` object and its direct
reads/writes in `published_mir/hako_llvmc_ffi_published_static_method.inc`.
Delete its zero-argument `active()` production calls in
`hako_llvmc_ffi_pure_compile.inc` and
`hako_llvmc_ffi_pure_compile_generic_lowering.inc`. Update the six consumer
families, static V2 compile, and the intrinsic/same-module prescan helpers to
pass the owner state. Migrate direct global assertions and test-only V1 begin
calls in `published_rows_preartifact_test.c`, `named_query_driver.c`,
`static_v2_execution_driver.c`, and `pure_document_lifetime_driver.c` to local
state or an invocation owner.

Retain the row/frame structs, exported Static V2 ABI, validation diagnostics,
the public/static ingress contract, and the existing cleanup stages. No public
ABI deletion or global-to-MIR redesign is part of I0.

## I0 acceptance and bounded task

Implement the named state and invocation field, route the explicit pointer
through all listed consumers, and preserve the current Static V2 cleanup rule:
begin failure does not end another owner; every post-begin pre-artifact exit
ends this owner before returning. Add focused negatives for:

1. same-owner re-entry rejection while the first owner remains active;
2. two-owner overlap isolation and independent end ordering;
3. duplicate take rejection and unfinished-row finish rejection;
4. V2 `(NULL, 0)` begin/finish/end success;
5. Static V2 pre-artifact failure leaves no object and an inactive owner;
6. existing positive typed rows and exact-site/coordinate validation.

Run the focused C tests, the row/route/pointer guards, `git diff --check`, and
the changed-source size guard. Classify unrelated named-llvmlite or real-LLVM
environment reds separately. Do not change public ABI, add fallback, alter
MIR layers, claim backend parity, or widen the task to aggregate R7 closure.
