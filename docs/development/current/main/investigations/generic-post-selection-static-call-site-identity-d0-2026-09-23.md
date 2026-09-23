---
Status: closed__2026-09-23__RetiredVmCompatibilityRoute
Task: GENERIC-POST-SELECTION-STATIC-CALL-SITE-IDENTITY-D0
Date: 2026-09-23
Parent: generic-legacy-observation-front-g0-premise-recheck-2026-09-23.md
PreviousCard: generic-loop-source-facts-accessor-compile-repair-i0-2026-09-23.md
NextCard: generic-loop-mainline-source-front-d0-2026-09-23.md
Implementation permission: false; closed by route classification only. No source edit, diagnostic behavior change, fixture, receipt, route, caller switch, or fallback.
---

# Generic post-selection static-call source identity D0

## Six-line brief

```text
Decision: the pinned G0 failure belongs to explicit `--backend vm`, which dispatches to retired Legacy VM Keep and constructs a Compatibility AST root; close this caller/site D0 as a non-selected route, without attributing its terminal to GenericLoop.
Source authority + canonical issuer: the selected compiler front is MIR's existing parser/materializer outcome; only `SourceBacked` may enter the callable source lifecycle and its existing semantic owners.
Non-authority: the retired VM Keep terminal, target name/arity, planner tags, the unrelated `int_to_str` canary, AST/method-name reconstruction, or backend name alone.
Fail-fast boundary: retain `UnissuedStaticCallRetirementV1::GenericCompatibility` on VM Keep; do not add diagnostics, restore fallback, or publish semantic authority on that route.
Smallest next slice: design one exact mainline MIR front that proves its materializer outcome and GenericLoop terminal before any route repair.
Non-claims: no LoopRecipe/carrier defect, source caller/site identity, publication/result expansion, production cutover, VM restoration, fallback, backend parity, or legacy retirement.
```

The proposed caller/site diagnostic follow-up is superseded. Its unresolved
caller/site does not block the selected compiler lane: the pinned wrapper
selects the retired VM Keep override, whose post-macro request seals the AST as
`Compatibility`. The raw port and exact `Unavailable` subtype remain unknown
and are intentionally out of scope; do not infer or instrument them.

## Fixed observation boundary

```text
run SHA:        fa80c9ccc2461b58e99a3c6b2c1bb5d52e74e74f
run time:       2026-09-23T14:13:39+09:00
case:           generic_loop_continue_strict_shadow_vm
fixture:        apps/tests/phase29ca_generic_loop_continue_min.hako
command:        NYASH_BIN=target/quick/hakorune HAKO_BIN=target/quick/hakorune bash tools/smokes/v2/profiles/integration/joinir/generic_loop_continue_strict_shadow_vm.sh
first terminal: [freeze:contract][static-call/legacy-fallback-retired] owner=StringHelpers method=to_i64 arity=1
result:         exit 1; wrapper expected exit 4
```

The script hardcodes `--backend vm`. `dispatch.rs` labels that backend
"Legacy VM Keep/Debug Override (explicit only)"; `route_orchestrator.rs`
selects `BootstrapRustVmKeep`; `keep/vm.rs::compile_post_macro_program`
uses `NormalCompileRequestV1::for_vm_keep_post_macro`; that request seals its
AST with `PreparedNormalDefaultProgramRootV1::seal`, which creates
`Compatibility(ast)`. This is not the selected MIR source-backed lifecycle.

`LoopCondContinueOnly` and shadow-adoption tags appeared earlier, but the
terminal message has no caller or source site. Output order is not a
same-callable relation. Do not label this `InLoopTerminal` or join it to
`StringHelpers.int_to_str/1 Body(0).Initializer(0)` without identity evidence.
The VM terminal is retained as a historical retired-route observation only;
the old wrapper remains unaccepted and is not required to exit 4 for the
selected compiler lane.

## Finite result and handoff

| Result | Evidence | Consequence |
| --- | --- | --- |
| `RetiredVmCompatibilityRoute` | The fixed command selects VM Keep, and that request uses a Compatibility AST root. | Close this occurrence as non-selected. Keep its pre-effect stop; do not add diagnostics or repair VM. |

The exact raw-port subtype, caller/site, and relationship to the known
`int_to_str/1` row remain unknown. No semantic or engineering decision depends
on resolving them for this retired route. For any future selected MIR
source-backed occurrence, the former identity contract still applies:
exact caller, `SourceExprSiteV1`, target, and existing ingress/product must
match before reusing the existing publication owner.

## D0 acceptance and handoff

- Record the wrapper as a retired VM compatibility observation, not a Generic
  production acceptance or a Loop failure.
- Do not require the VM wrapper to exit 4, add a caller/site projection, or
  restore a compatibility fallback.
- Hand off only to
  [`GENERIC-LOOP-MAINLINE-SOURCE-FRONT-D0`](generic-loop-mainline-source-front-d0-2026-09-23.md),
  which must prove the MIR materializer outcome and selected GenericLoop
  terminal separately.
- No serial P1 route observation, S6E producer, source-to-Recipe expansion,
  production selection, or old-edge deletion is authorized by this closure.

## Worker audit

Read-only audits confirmed the diagnostic lacks caller/site and cannot be
joined to the printed Loop function or `int_to_str/1` row. They established
the route-level result: VM Keep is an explicit retired lane and its request
seals Compatibility AST. MIR is the selected entry, but its materializer has
both `SourceBacked` and `Compatibility` outcomes, so the backend name alone is
not proof. Workers made no edits and ran no Cargo/tests.
