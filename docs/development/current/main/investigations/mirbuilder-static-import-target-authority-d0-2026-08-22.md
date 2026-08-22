Status: D0-A/B accepted after premise correction; next execution is one fast BoxShape
ClosedTask: SCRIPT-STATIC-IMPORT-TARGET-AUTHORITY-D0
NextTask: MIR-CALLABLE-RESOLVER-TYPED-DEFERRED-P0
Date: 2026-08-22
Priority: preserve the actual first production blocker before widening any resolver shape
Parent: MIR-LOOP-COMPARE-LIVE-PUBLICATION-CENSUS-D0
NextCard: this rolling card owns the bounded P0 brief
---

# Imported static authority census and callable Deferred correction

## Six-line brief

```text
Decision: revise and close the imported-target premise. Production imports are recursively text-merged before one parser invocation, so ParserCommonUtilsBox is already part of the source-backed callable catalog. The first live blocker is callable-batch ResolverDeferred before static lookup.
Source authority + canonical issuer: VerifiedFinalCallableProgramSourceV1 owns complete callable membership and opaque declaration identities. The selected-callable resolver batch issuer must co-seal every deferred cause/site with the exact identity supplied by that same parser loan.
Non-authority: the isolated direct-AST test, import alias strings, AST/name/arity, batch ordinal alone, Builder catalog state, TargetUnavailable, empty catalogs, and fallback.
Fail-fast boundary: typed Deferred evidence is issued at selected-callable owner-tree construction, before semantic-package completion, static lookup, Builder installation, or physical effects.
Smallest next slice: MIR-CALLABLE-RESOLVER-TYPED-DEFERRED-P0; replace bare Deferred with a non-empty identity-bound batch without accepting any new syntax.
Non-claims: no import handoff, resolver capability expansion, target/result redesign, StaticResultPublicationIngress change, Dynamic publication, fallback, production switch, backend, or performance work.
```

## Corrected production route

The live MIR runner does not parse the fixture in isolation.

```text
parser_scan_loop_box.hako
  -> prepare_source_with_imports
  -> recursive prelude text merge
  -> one merged source string
  -> one parser invocation
  -> VerifiedFinalCallableProgramSourceV1
  -> NormalCompileRequestV1
  -> normal callable semantic package attempt
  -> source-backed declaration catalog
  -> selected-callable resolver
  -> Batch(ResolverDeferred)                 current stop
  X  ScriptDirectStaticCallLookupIssuerV1
  X  result catalog / publication owner
  X  StaticResultPublicationIngress
```

Read-only production evidence on this HEAD:

```text
NYASH_RESOLVE_DUMP_MERGED=<temp>/merged.hako
NYASH_USING_AST=1 target/debug/hakorune --dev --backend mir \
  lang/src/compiler/parser/scan/parser_scan_loop_box.hako

merged source lines = 646
ParserCommonUtilsBox declaration = merged line 379
ParserScanLoopBox declaration = merged line 587
result = [mir/callable-semantic-package/issue] Batch(ResolverDeferred)
```

The current binary was built after the last production-code change affecting
this route. The same route was also observed with NYASH_USING_AST=0 by the
read-only audit.

The source chain is exact:

- source_hint.rs sends merge_prelude_text_with_imports output to normal
  callable materialization;
- merge.rs recursively expands dependencies, concatenates every prelude before
  the main source, and retains alias-to-owner rows;
- normal_callable.rs parses that merged string once and preserves one-read /
  one-parse lineage;
- the final parser source loan retains every callable anchor and declaration;
- source_backed.rs emits the callable declaration catalog from that same loan;
- normal_script_direct_static_lookup.rs already verifies aliases against that
  catalog, issues target/result catalogs, checks parser invocation, and issues
  the existing publication owner.

Therefore a second imported declaration catalog or imported-target handoff
would duplicate an existing source authority. It is rejected.

## Why the old target-unavailable evidence was not production evidence

normal_default_root_catalog_lifecycle_tests.rs directly include_str! parses
parser_scan_loop_box.hako. Its helper does not call the runner import loader,
does not merge ParserCommonUtilsBox, and opens a session without import rows.
For that deliberately different input, target-unavailable is the correct typed
reject.

The focused test remains valid for this narrower statement:

```text
unmerged direct-AST input with no imported declaration
  -> static-result-ingress/target-unavailable
```

It does not prove:

```text
production merged input
  -> imported declaration unavailable
```

The test passed unchanged during this census. It is retained as negative
boundary evidence, not as live reachability evidence.

## D0-A / D0-B decision

### D0-A — import authority census: Complete

```text
source owner:
  one recursively merged source string

parser authority:
  VerifiedFinalCallableProgramSourceV1
  + opaque callable declaration anchors
  + one parser/source lineage

declaration issuer:
  issue_source_backed_same_module_callable_catalog_v1

import relation:
  VerifiedStaticImportAliasViewV1
  accepted only when its canonical owner exists in the same catalog
```

ParserCommonUtilsBox.i2s/1 is inside the merged parser source and is eligible
for the existing canonical declaration key. The catalog is currently dropped
because the later callable-batch resolver defers; absence was not observed.

### D0-B — imported handoff contract: Rejected as redundant

If the semantic package becomes Complete, the existing
ScriptDirectStaticCallLookupIssuerV1 already scopes:

```text
parser source loan
+ exact declaration catalog
+ verified import aliases
+ whole-source static target inventory
+ same-module result catalog
+ static-result publication owner
```

No new imported handoff is needed. The current run never reaches this issuer.
Its result disposition must be observed after the callable Deferred is typed;
ParserCommonUtilsBox.i2s returning string syntax must not be guessed into an
ExactI64 result or relabeled as missing authority.

## Actual blocker

resolve_selected_callable_forests_with_body_shapes_and_brand_catalog currently
folds every deferrable ShadowResolveErrorV0 into a bool and returns a bare
Deferred. ResolvedCallableSemanticBatchIssueV1 then returns the cause-less
ResolverDeferred variant. Consequently the live source does not reveal:

```text
which parser callable deferred
which source cause was observed
which exact statement/expression/exit site caused it
whether more than one callable deferred
```

The resolver intentionally scans the whole input batch so a later integrity
error is not hidden by an earlier source deferral. The next slice must preserve
that precedence.

## Authority contract for typed Deferred

| Responsibility | Sole owner |
| --- | --- |
| complete callable membership | VerifiedFinalCallableProgramSourceV1 |
| exact callable identity | parser-issued CallableDeclarationIdentityV1 |
| deferral cause and source site | Shadow resolver kernel |
| identity + cause/site co-seal | selected-callable resolver batch issuer |
| package terminal | ResolvedCallableSemanticBatchIssueV1 |
| rollback / no publication | unpublished normal compilation session |

The selected resolver input must carry identity beside its borrowed
FunctionSyntaxViewV1. A returned Deferred row must nest that identity and its
cause/site; it must not expose a bare ordinal for a later join.

The constructor-semantic caller of the same resolver API must retain its own
parser-issued constructor source identity. It may not borrow a callable anchor
or silently discard the typed observation merely to compile.

Counterexample:

```text
parse P1 and parse P2 contain identical text, names, arities, and row order

P1 deferred cause + P2 batch ordinal/name
  -> looks equal under name/ordinal pairing
  -> is foreign under parser-issued identity
```

## Finite state table

| State | Evidence | Effect / terminal | Fallback |
| --- | --- | --- | --- |
| Complete | every tagged input produced one forest/body shape | semantic package may continue | none |
| DeferredNonEmpty | one or more identity-bound cause/site rows | package not issued; typed reject | none |
| IntegrityInvalid | any non-deferrable resolver or verification error | package not issued; hard reject | none |
| SourceIdentityInvalid | missing, foreign, duplicate, or unpaired input identity | package not issued; hard reject | none |

DeferredNonEmpty must be structurally non-empty, for example first + rest or a
private non-empty constructor. Vec::new(), Option::None, and default cannot
represent it.

All input rows are still scanned. If an early row defers and a later row has an
integrity error, IntegrityInvalid wins exactly as it does today. Multiple valid
deferrals are retained in deterministic parser-loan order.

## Next execution brief — MIR-CALLABLE-RESOLVER-TYPED-DEFERRED-P0

Change:

```text
replace ResolveSelectedCallableForestsWithBodyShapesOutcomeV1::Deferred
  with one non-empty identity-bound deferred batch

preserve each source-owned cause/site
  through callable-batch and constructor-batch terminal errors

delete the bool-only / cause-less production edge
```

Contract:

```text
BoxShape only; accepted syntax and resolver traversal stay unchanged
no AST reference escapes the HRTB loan
no name, arity, input ordinal, or source path becomes pairing authority
all-row scan and later-integrity precedence remain unchanged
Script typed Deferred behavior remains unchanged
no retry, fallback, Absent downgrade, or package publication on Deferred
```

Done:

```text
focused positive: complete batch remains Complete
focused negative: located and unlocated causes retain exact identity/site
multiple deferrals form one non-empty deterministic batch
later invariant still overrides an earlier deferral
constructor caller preserves its exact source identity
production parser_scan route reports typed cause/site instead of bare Deferred
cargo check, focused tests, current pointer guard, diff check, and source sizes pass
all touched production files remain below 760 lines; 800 is hard stop
```

Stop:

```text
the implementation needs an AST ref in the deferred product
the returned product exposes a bare index for caller-side re-pairing
the constructor caller has no exact source identity
any resolver syntax is newly accepted
any later invariant is hidden by an earlier deferred row
the typed terminal can retry static lookup, Builder, compatibility, or legacy
```

## Downstream boundary

After P0, rerun the real merged source and name the exact typed blocker. Only
then may a new design decide whether that source shape belongs to canonical
callable resolution or an explicit outside terminal. Static lookup, result
disposition, selected Dynamic publication, and the transaction-hardening card
remain downstream.

The parked transaction work remains recorded in:

```text
docs/development/current/main/investigations/
  mirbuilder-loop-compare-hardening-d0-2026-08-22.md
```

No code or fixture was changed while closing this D0.
