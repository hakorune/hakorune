Status: lexical Program-block I0 complete; source-bound hard reject P0 accepted as next BoxShape
ClosedTask: MIR-CALLABLE-RESOLVER-LEXICAL-PROGRAM-BLOCK-I0
NextTask: MIR-CALLABLE-RESOLVER-SOURCE-BOUND-REJECT-P0
Date: 2026-08-22
Priority: bind the next non-deferrable verifier failure to its exact parser source before diagnosis
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

## Typed Deferred P0 execution closeout

`MIR-CALLABLE-RESOLVER-TYPED-DEFERRED-P0` is complete.

```text
parser callable/constructor identity + borrowed FunctionSyntaxViewV1
  -> one all-row resolver kernel
  -> Complete
     or structurally non-empty identity-bound Deferred batch
  -> callable/constructor package terminal
```

The new Deferred product contains no AST reference and exposes no empty or
default state. Callable rows retain `CallableDeclarationIdentityV1`;
constructor rows retain `ConstructorSourceIdV1`. Multiple deferrals stay in
parser-loan order. A later non-deferrable resolver invariant still terminates
the issue attempt instead of being hidden by an earlier source deferral.

Production caller census after the change:

```text
source-bound resolver callers = 2
  callable semantic batch = 1
  constructor semantic batch = 1

cause-less resolver callers in those production owners = 0
legacy cause-less resolver API callers = test/caller-zero only
```

Focused evidence:

```text
callable semantic batch tests = 8 passed
  - Complete rows unchanged
  - two unresolved callables retain two identities, exact causes/sites, and order
  - same-scope redeclaration retains its unlocated typed cause

constructor Deferred test = 1 passed
  - exact parser ConstructorSourceIdV1 retained

later-invariant precedence test = 1 passed
Script Deferred conversion regression = 1 passed
cargo check --lib = passed
cargo build --bin hakorune = passed
current-state pointer guard = passed
diff check = passed
touched production source max = 581 lines
```

The rebuilt real merged-source probe now reports the exact next blocker:

```text
merged source lines = 646

StringHelpers.starts_with
  identity = exact parser callable anchor
  cause = UnsupportedStatement { kind: "Program" }
  site = Body(2)

StringHelpers.starts_with_kw
  identity = exact parser callable anchor
  cause = UnsupportedStatement { kind: "Program" }
  site = Body(0)
```

Both rows are standalone `{ ... }` debug blocks emitted by the parser as
nested `ASTNode::Program` statements. This is now observed evidence; no import,
target, result, or Builder authority is missing at this boundary.

## Next decision — lexical Program block I0

```text
Decision: accept one nested ASTNode::Program statement as the parser representation of a standalone bare lexical block. Traverse it under a new lexical region/scope; never flatten it into the enclosing function scope.
Source authority + canonical issuer: parse_standalone_block_statement is the sole syntax-shape issuer; VerifiedFinalCallableProgramSourceV1 retains the parser invocation/callable identity; the existing shadow resolver is the sole binding/scope/owner issuer.
Non-authority: top-level Program, Builder-created Program shells, braces/text/span, name or body ordinal, Lower behavior, Script root admission, and the current production fixture.
Fail-fast boundary: after the enclosing Program statement site is admitted and before any child is observed, enter one existing LexicalScope/LexicalBlock at ProgramBodyRoot; any child error unwinds that scope and returns the existing typed terminal before package or Builder effects.
Smallest next slice: MIR-CALLABLE-RESOLVER-LEXICAL-PROGRAM-BLOCK-I0; connect ProgramBody traversal for FullFunction/SelectedCallable profiles with exact lexical lifetime and source paths.
Non-claims: no transparent flattening, Script Program admission, Using/Import/StaticConst traversal, TryCatch/postfix compatibility, new resolver receipt, target/result/A/C, Recipe/Join, Builder/MIR, fallback, publication, backend, or performance work.
```

### Why the block is lexical, not transparent

The language invariant and legacy Lower agree that a standalone `{ ... }`
body owns a lexical lifetime. The parser emits that statement as
`ASTNode::Program`, while the existing block driver enters one lexical scope
before lowering its statements.

Flattening would be observably wrong:

```text
local x = 1
{
  local x = 2
}
return x

correct: inner x shadows and expires
flattened: false same-scope redeclaration or leaked inner binding
```

The accepted source coordinates are already canonical:

```text
outer statement             = Body(k)
lexical scope/region origin = Body(k) / ProgramBodyRoot
child statement i           = Body(k) / ProgramBody(i)
```

`ProgramBodyRoot` is a scope origin, not an extra item-prefix segment. The
existing `BodyChildRoleV1::ProgramBody` / `SourceBodyKindV1::Program` mapping
must issue both coordinates; callers must not build them by ordinal.

The lexical block adds no control target. A `break` or `continue` nested in the
block keeps the existing nearest enclosing Loop relation.

### Bounded implementation task

Change only one accepted statement shape:

```text
shadow/stmt.rs
  ASTNode::Program { statements, .. }
    -> record outer SequenceItem
    -> enter LexicalScope + LexicalBlock at ProgramBodyRoot
    -> resolve every child at ProgramBody(i)
    -> leave scope on success or error

shadow/vocabulary.rs
  Program: SemanticallyTransparentCandidate -> CurrentResolvedStatement
  Using/Import/StaticConst remain non-accepted candidates

focused tests
  exact lexical shadowing/lifetime
  exact ProgramBodyRoot and ProgramBody(i) coordinates
  same-scope duplicate inside one block rejects
  nested block inside Loop preserves Break/Continue target
  Script profile and unrelated candidate syntax remain unchanged
```

No new `Verified*` or `Prepared*` type is needed. This is one BoxCount using
the existing parser, scope, region, path, forest, and package authorities.

### Acceptance

```text
positive:
  natural parsed callable with one bare block resolves Complete
  local inside the block shadows an outer binding and is absent afterward
  outer Program statement and every child site appear exactly once
  nested block Break/Continue resolve to the enclosing Loop

negative:
  duplicate Local names inside the same Program block retain typed redeclaration
  unresolved child retains callable identity + ProgramBody child site
  Using/Import/StaticConst do not become resolved statements
  Script root admission does not widen

production observation:
  the two StringHelpers Program deferrals disappear
  any next stop remains a typed identity-bound cause/site
  no package/Builder effect occurs on a remaining Deferred

guards:
  Program flatten/rewrite = 0
  new scope/region vocabulary = 0
  new source-path vocabulary = 0
  new semantic receipt = 0
  fallback/retry = 0
  touched production files < 760 lines; 800 hard stop
```

### Stop conditions

Return to `design_stop` instead of widening the slice if:

```text
the nested Program cannot be distinguished from a synthetic/top-level shell
Lower does not give the same shape lexical lifetime
exact ProgramBody source coordinates require reconstruction
child traversal needs TryCatch/postfix compatibility
Script admission or target/Builder state is needed
the real probe requires a second newly accepted AST shape
```

## Lexical Program-block I0 execution closeout

`MIR-CALLABLE-RESOLVER-LEXICAL-PROGRAM-BLOCK-I0` is complete.

The existing statement resolver now consumes one nested parser-issued
`ASTNode::Program` as a `LexicalScope` / `LexicalBlock`. It uses only the
existing `ProgramBodyRoot` and `ProgramBody(i)` vocabulary, leaves the scope on
success or child error, and does not push a control target. Script profiles
still reject Program at their existing profile gate.

Evidence:

```text
scope-container tests = 5 passed
  - exact inner binding lifetime
  - exact scope/region origin and statement coverage
  - duplicate Local remains typed redeclaration
  - nested Break retains enclosing Loop target

vocabulary/profile tests = 5 passed
callable batch tests = 9 passed
  - natural parser bare block reaches Complete
  - unresolved child retains Body(0)/ProgramBody(0)/Value

cargo check --lib = passed
cargo build --bin hakorune = passed
Program flatten/rewrite = 0
new scope/region/path/receipt vocabulary = 0
touched production max = 556 lines
```

The rebuilt 646-line merged-source probe no longer reports either
`UnsupportedStatement { kind: "Program" }`. It advances to:

```text
Batch(Resolver(Function(Verification(
  IfRegion(ControlContractMismatch(
    RegionId { owner: compilation 1 / slot 9, slot: 3 }
  ))
))))
```

This is a hard resolver/verifier rejection, not a Deferred source shape. The
error still exposes only a resolver owner slot. Pairing that slot back to a
callable by source order or diagnostic name would violate the same identity
rule fixed for Deferred.

## Next decision — source-bound hard reject P0

```text
Decision: carry the exact selected callable or constructor parser identity beside every non-deferrable construction/forest-verification error emitted by the all-row resolver kernel. Preserve the existing first-hard-error terminal and do not repair the IfRegion mismatch in this slice.
Source authority + canonical issuer: the parser loan supplies CallableDeclarationIdentityV1 or ConstructorSourceIdV1 beside each FunctionSyntaxViewV1; the shadow/forest resolver remains the sole error issuer; the source-bound kernel alone co-seals identity + error.
Non-authority: FunctionOwnerIdV1 slot, resolver batch order, callable/Box name, arity, source path alone, AST pointer, merged line number, Builder state, and the current fixture.
Fail-fast boundary: bind a construction or forest-verification error before returning from the source-bound resolver API and before semantic-package completion, catalog installation, Builder effects, fallback, or publication.
Smallest next slice: MIR-CALLABLE-RESOLVER-SOURCE-BOUND-REJECT-P0; replace unbound production Resolver(error) terminals with one exact source-bound reject while leaving legacy caller-zero APIs unchanged.
Non-claims: no IfRegion repair, resolver syntax acceptance, control-contract redesign, owner-ID redesign, import/target/result work, Recipe/Join, Builder/MIR, fallback, publication, backend, or performance work.
```

### Bounded contract

```text
input identity + FunctionSyntaxView
  -> construction success
     -> retain identity beside pending owner tree
     -> forest verification success -> Complete row
     -> forest verification error   -> SourceBoundReject(identity, error)

  -> construction Deferred
     -> existing non-empty Deferred batch

  -> construction hard error
     -> SourceBoundReject(identity, error)
```

The kernel continues in parser-loan order. The first hard error remains
terminal exactly as today. A source-bound reject is one non-Clone terminal row,
not an optional parallel diagnostic and not a new semantic capability.

Counterexample:

```text
P1 and P2 contain identical callable names, arities, and batch positions
owner slot 9 is compilation-local and cannot recover either parser anchor

owner slot/name/order + verifier error
  -> ambiguous or foreign pairing

exact parser identity nested with verifier error
  -> one request-local reject, no re-pairing
```

### Acceptance

```text
positive:
  Complete and identity-bound Deferred behavior remain unchanged

negative:
  construction hard error retains exact callable identity
  forest verification error retains exact callable identity
  constructor hard error retains exact ConstructorSourceIdV1
  foreign parser identity cannot be substituted by name/order

production observation:
  real merged probe names the exact parser callable identity plus
  IfRegion(ControlContractMismatch)
  no package/Builder effect occurs

guards:
  production unbound Resolver(error) terminal = 0
  owner slot/name/ordinal re-pairing = 0
  AST ref in reject = 0
  new syntax acceptance = 0
  fallback/retry = 0
  touched production files < 760 lines; 800 hard stop
```

Stop and redesign if the verification error is session-global rather than
attributable to the one tree being sealed, or if retaining identity requires
an AST reference, a second resolver pass, or caller-side re-pairing.
