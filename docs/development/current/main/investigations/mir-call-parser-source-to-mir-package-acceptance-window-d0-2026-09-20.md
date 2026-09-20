---
Status: design_stop__2026-09-20__ParserSourceToMirPackageAcceptanceWindow
Task: MIR-CALL-PARSER-SOURCE-TO-MIR-PACKAGE-ACCEPTANCE-WINDOW-D0
Current execution row: MIR-CALL-PARSER-LOOPCOND-SOURCE-HANDOFF-I0__PackageAcceptanceWindowD0
Date: 2026-09-20
Parent: mir-call-parser-loopcond-source-handoff-i0-2026-09-19.md
Implementation permission: false; resolve the package acceptance owner before code or fixture changes
---

# Parser source-to-MIR package acceptance window D0

## Six-line brief

```text
Decision: keep the selected parser tuple at a design stop until its evidence
  can be observed through the installed normal semantic package without
  skipping or reclassifying another callable in that same package.
Source authority + canonical issuer: the existing
  VerifiedNormalCallableSemanticPackageV1, its installed
  NormalCallableSemanticPackagePortAdapterV1, and the resolver-issued
  `(ParserProgramBox.parse/2, source-site, ParserStringUtilsBox.starts_with/3)`
  relation from the same invocation.
Non-authority: a target-only lowerer, method-name/ordinal filtering, a cloned
  package, a subset work plan, LoopBreak fallback, GenericLoop retry, AST/MIR
  rescans, or a synthetic Cataloged/Selected receipt.
Fail-fast boundary: before catalog installation or Builder effects, the
  acceptance observation must co-seal caller, exact source site, target,
  signature/result/effect relation, and the explicit dependency terminal; any
  missing or foreign row rejects the package window.
Smallest next slice: design one existing-owner, source-keyed acceptance
  window that observes the selected tuple and records the
  `LoopBreakRecipe -> GenericLoopV1NotSelected` dependency terminal without
  lowering only a selected method. If no such owner exists, name the missing
  LoopBreak source consumer as a separate bounded design row.
Non-claims: no parser source-to-MIR success, publication, old-edge deletion,
  production switch, fallback, VM/AOT parity, or warning cleanup.
```

## Exact bounded tuple

```text
caller       = ParserProgramBox.parse/2
target       = ParserStringUtilsBox.starts_with/3
target site  = resolver-issued source site for parser_program_box.hako:102
route        = LoopCondBreakContinue, with the selected LoopTrue child handoff
result       = ExactI64, required ordinal `[1]`
dependency   = the same merged package's raw `[LoopBreakRecipe]` terminal
```

The line number is only a reviewed source witness. The acceptance window must
carry the resolver's exact source site, caller/target key, signature, result,
effect, and package brand from one invocation. It must not rediscover any of
those relations from Hako text, method names, arity, AST, MIR, or `ValueId`.

## Evidence for the current stop

The selected LoopCond and LoopTrue source handoffs now cross their focused
physical edges. The merged parser lifecycle still stops before catalog
installation at:

```text
[freeze:contract][callable-loop/route-not-front-selected]
GenericLoopV1NotSelected
raw front = [LoopBreakRecipe]
```

This is dependency evidence, not parser acceptance. A read-only owner audit
confirmed that `lower_program_root_work_plan_with_callable_port_v1` lowers all
immediate and deferred work, and that
`PreparedNonMainStaticBoxMethodBatchV1::lower_root_with_port_v1` iterates every
method in each static box. The installed package's selected-call APIs are
scoped loans; they do not complete one callable while preserving package
coverage.

## Owner and rejection matrix

| Concern | Existing authority | Required result |
| --- | --- | --- |
| source target/call relation | resolver-issued source target catalog | exact caller/site/target/signature/effect co-seal |
| package transport | `VerifiedNormalCallableSemanticPackageV1` and installed lowering port | one package-scoped observation; no clone or second ledger |
| root lowering | existing full work plan and method batches | preserve all-method coverage; no target-only filter |
| dependency terminal | existing LoopBreak/GenericLoop named reject | record explicitly; never reinterpret as success or fallback |
| missing/foreign selected row | existing source/package rejection boundary | reject before catalog/Builder effects |

The following are rejected in this D0:

- adding a target-only lowerer or filtering methods by name, ordinal, or arity;
- skipping the `LoopBreakRecipe` method to make the selected tuple appear green;
- cloning or partially installing the semantic package;
- converting the dependency terminal into `Ok(None)`, GenericLoop retry, VM
  compatibility, or another fallback;
- issuing a synthetic `Cataloged`/`Selected` receipt before the package owner
  and consumer are named.

## Acceptance for the next design decision

The design stop closes only when one existing owner is named for an
acceptance-window observation that:

1. consumes the resolver-issued selected tuple from the installed package;
2. preserves all other package methods and records the dependency terminal;
3. has a finite positive/negative matrix for exact site, target, signature,
   result/effect, foreign row, duplicate row, and package-brand drift; and
4. states whether the missing `LoopBreak` source consumer is a separate
   bounded design row, with its own source issuer, physical consumer, and
   exclusive old-edge delete-set.

Until then the parent LoopCond handoff card remains implemented through D2 but
does not claim parser source-to-MIR acceptance, publication, old-edge deletion,
or production switching.
