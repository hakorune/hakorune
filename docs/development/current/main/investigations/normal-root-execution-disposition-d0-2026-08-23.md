# Normal root execution disposition

Status: design stop — total execution authority not yet frozen
Date: 2026-08-23
Decision: NORMAL-ROOT-EXECUTION-DISPOSITION-D0
Owner: parser source authority -> normal/default root lifecycle

## Six-line brief

Decision:
  Separate the total normal-root execution role from the narrow canonical
  Script-A/Main.main/0 admission products. Do not reinterpret their Outside
  states as Script execution.
Source authority + canonical issuer:
  The complete same-invocation parser Program authority plus callable source
  catalog; ParserNormalRootExecutionIssuerV1::issue_once, called only inside
  ParsedProgramWithCallableParameterSourceV1::new, is the candidate sole issuer.
Non-authority:
  ParserNormalRootSourceDispositionV1, CanonicalScriptCohortAdmissionV1,
  VerifiedRawRootExpansionV1, root_is_app_mode, AST/name/ordinal/pointer rescans,
  Builder state, Recipe, work-plan terminal, and MIR.
Fail-fast boundary:
  Source-backed execution disposition must be preserved through the exact final
  transform and consumed before declaration, resolver, target, module, catalog,
  work-plan, or Builder effects.
Smallest next slice:
  NORMAL-ROOT-EXECUTION-PARITY-D0: freeze every old raw classifier arm, every
  production consumer, and the exact App/ProgramRuntime source relation. No
  code, fixture, fallback, or new semantic receipt is authorized.
Non-claims:
  No lifecycle cutover, compatibility change, App-shape expansion, Script-A
  change, Recipe/lowering/publication work, raw retirement, or performance work.

## Why the previous order was wrong

The reverted S0 built the scoped loan before closing the consumer and state
closure. It treated:

~~~text
ParserNormalRootSourceDispositionV1::ScriptReady
~~~

as the total non-App execution role. It is not. That state is only the narrow
no-box canonical Script-A cohort.

The required order is now fixed:

~~~text
all production consumers
  -> all accepted/rejected source states
  -> total source authority
  -> exact final preservation
  -> scoped one-shot consumer
  -> atomic production cutover
  -> old authority retirement
~~~

Transport, HRTB, or a new Verified/Prepared product may not precede those first
three rows again.

## Current counterexamples

| Source | Existing production role | Narrow A-root state | Required total role |
| --- | --- | --- | --- |
| static box Scan { run(v) { ... } } | Script runtime | Outside(NonMainStaticBox) | ProgramRuntime |
| StringHelpers/parser_scan static providers | Script runtime | Outside(NonMainStaticBox) | ProgramRuntime |
| static box Main { main(value) { ... } } | App | Outside(NonZeroMainArity) | App |
| static box Main { main() {...} helper() {...} } | App | Outside/first-cohort mismatch | App |
| two top-level static Main boxes | reject | Outside or contradictory evidence is insufficient | IntegrityInvalid(DuplicateMain) |

Therefore neither `Outside => Script` nor `ScriptReady => total Script` is a
valid adapter.

## Production consumer closure

The old root meaning is currently reissued at three source-backed sites:

1. `normal_default_root_catalog_lifecycle.rs`
   - raw preflight scan;
   - converts the result to `preflight_is_app_mode: bool`.
2. `callable_declaration_catalog/source_backed.rs`
   - raw expansion scan;
   - uses AST pointer equality to skip root Main and identify Main static
     children.
3. `normal_default_root_catalog_lifecycle.rs`
   - retained-source raw scan after module/catalog effects;
   - compares bools and supplies the work-plan terminal.

The cutover is incomplete unless all three semantic uses are replaced. Removing
only the first and third scans still leaves a second source authority in the
callable catalog.

The borrowed expansion then fans out to these production consumers, all of
which belong to the same cutover closure:

~~~text
root role preflight
callable catalog role assignment
Main.main/N materialization target
ProgramRoot terminal schedule
Builder root execution mode
Script-runtime versus App terminal dispatch
App root-body lowering
Main static-child lowering
~~~

`raw_source_projection.rs` is a separate explicit raw/compatibility owner and
is not a source-backed caller to delete in this series. Test-only direct
constructors are likewise evidence, not production authority.

## Authority split

| Owner | Owns | Must not own |
| --- | --- | --- |
| Parser normal Program authority | complete paired Program coverage and parser invocation | App/ProgramRuntime policy |
| Existing narrow A products | Main.main/0 and canonical no-box Script-A eligibility | total execution role |
| ParserNormalRootExecutionIssuerV1 | total App versus ProgramRuntime role and exact callable-role relation | Builder route, Recipe, MIR |
| Exact transform preservation issuer | same token and same relation after the final source transform | reclassification or reconstruction |
| Prepared normal root consumer | one move-only scoped use of the preserved relation | source truth or a second loan |
| Callable catalog issuer | consume parser-issued callable roles | AST pointer pairing |
| Program root work-plan | project an admitted typed role to a schedule | bool classification |
| Compatibility raw owner | compatibility-only AST classification | fabricated parser receipt |

The new product is not a second Script-A authority. Its only meaning is normal
root execution.

## Candidate source model

Stored products are AST-free, non-Clone, constructor-private, and required
fields rather than parallel Options.

~~~rust
enum ParserNormalRootExecutionSourceDispositionV1 {
    Ready(ParserNormalRootExecutionSourceV1),
    SourceAuthorityUnavailable(ParserNormalRootExecutionUnavailableV1),
    Incomplete(ParserNormalRootExecutionIncompleteV1),
    IntegrityInvalid(ParserNormalRootExecutionIntegrityIssueV1),
}

enum ParserNormalRootExecutionSourceV1 {
    App(ParserNormalAppExecutionSourceV1),
    ProgramRuntime(ParserNormalProgramRuntimeSourceV1),
}
~~~

`ProgramRuntime` is deliberately not named canonical Script. It means only
that the existing normal root execution schedule is non-App.

App must co-seal at least:

~~~text
parser invocation witness
unique top-level static Main statement site
exact main callable identity and method site
ordered Main static-child identity/site relations
complete Program coverage relation
~~~

ProgramRuntime must co-seal at least:

~~~text
parser invocation witness
NoTopLevelStaticMain witness
complete Program coverage relation
~~~

The issuer may use source names while parsing the language construct. Names,
ordinals, spans, digests, and pointers do not leave the issuer as pairing
authority.

## Required finite state table

This table must be checked against the old raw classifier and production tests
before implementation is authorized.

| Source observation | Total execution result |
| --- | --- |
| zero top-level static Main declarations | Ready(ProgramRuntime) |
| one static Main with an exact static function main | Ready(App) |
| main arity greater than zero | Ready(App) |
| ordered static helper methods under Main | Ready(App) with child relations |
| more than one top-level static Main | IntegrityInvalid(DuplicateMain) |
| Main exists but main is missing | Incomplete(MainMethodMissing) |
| main or a required child is non-function/non-static | IntegrityInvalid |
| foreign, duplicate, stale, or incomplete callable relation | IntegrityInvalid/Incomplete |
| Program or callable source authority is absent | SourceAuthorityUnavailable |
| generated/AST-only compatibility | no source-backed execution product |
| narrow A Deferred/NonCandidate/Transported/Outside | no effect on execution role |

No `Outside`, wildcard, default, or bool state belongs in the total
source-backed execution disposition.

The parity audit must also preserve these current rules unless a separate
language Decision changes them:

~~~text
only a top-level static box named Main counts as App
a non-static Main box does not count and remains ProgramRuntime
top-level siblings and additional non-Main boxes are allowed
main arity is unrestricted
Main helper methods are allowed but must be static functions
missing main, non-function main, non-static main, and invalid helpers reject
duplicate top-level static Main declarations reject
~~~

The current raw classifier does not use the narrow Main.main/0/no-child
admission policy. The new total issuer must not silently tighten to that cohort.

## Final preservation and loan contract

The source product moves through the existing exact final-transform boundary.
The final preservation issuer verifies, but does not reissue, the relation.

The only production consumer shape is move-consuming:

~~~text
PreparedNormalDefaultProgramRootV1
  -> consume_source_backed_root_once(self, ...)
  -> HRTB ParserNormalRootExecutionLoanV1
  -> private lifecycle continuation
~~~

The App loan must provide:

~~~text
exact root callable syntax/body/contract
ordered top-level siblings with RootMain opaque in Program order
ordered callable-role relation:
  RootMain | AppStaticChild | Ordinary
~~~

The ProgramRuntime loan provides one paired Program statement cursor and
ordinary callable roles. Borrowed syntax cannot escape the callback. Parallel
AST and role arrays, a public getter, repeated `&self` reloan, Clone, and a
generic parts tuple are forbidden.

## Bounded task sequence

### D0-A — NORMAL-ROOT-EXECUTION-PARITY-D0

Change:
  Census every `VerifiedRawRootExpansionV1::from_program` production caller,
  every `root_is_app_mode` semantic reader, and every old accepted/rejected
  App shape.
Done:
  One table maps old observation -> required new state -> consumer -> retirement
  edge. Scan/StringHelpers/parser_scan, Main.main/N, static children, malformed
  Main, duplicate Main, non-static Main, mixed Program, entry materialization,
  terminal dispatch, and compatibility are explicit.
Stop:
  Any unmapped production state keeps design_stop.

### D0-B — NORMAL-ROOT-EXECUTION-AUTHORITY-FREEZE-D0

Change:
  Freeze the sole issuer, exact fields, typed errors, and relation with callable
  identities/final slots.
Done:
  Parser source authority and final preservation can issue every mapped state
  without Builder, pointer, raw fallback, or narrow A-state inference.
Stop:
  Missing source/callable relation is NoSafeSlice, not an empty/default row.

### P0 — NORMAL-ROOT-EXECUTION-SOURCE-P0

Change:
  Implement the total parser product and exact transform preservation only
  after D0-A/B are accepted.
Done:
  Sole issuer = 1; product is required, AST-free, non-Clone, and transported by
  move; focused positive/negative tests and one reusable guard are green.
Stop:
  No Builder caller or production claim. The immediately reserved successor is
  the consumer/cutover series or P0 is reverted.

### S0 — NORMAL-ROOT-EXECUTION-CONSUMER-S0

Change:
  Add the private move-consuming scoped consumer and role-bearing callable
  projection without changing the production route.
Done:
  No borrow escape, reloan, pointer pairing, bool role, or generic parts escape.
Stop:
  Must be immediately followed by C0 or reverted.

### C0 — NORMAL-ROOT-EXECUTION-CUTOVER-C0

Change:
  Atomically replace the lifecycle preflight, callable-catalog pointer pairing,
  retained raw expansion, and bool work-plan selection.
Done:
  Named source-backed consumer = 1; all three raw semantic callers = 0;
  Builder effect before consume = 0; typed App/ProgramRuntime reaches the
  existing schedule/lower path; fallback/retry = 0.
Stop:
  Compatibility remains on its explicit owner and receives no parser receipt.

### R0 — NORMAL-ROOT-RAW-AUTHORITY-RETIREMENT-R0

Change:
  Retire source-backed `root_is_app_mode`, bool work-plan APIs, pointer
  pairing, and the stale `VerifiedRawRootExpansionV1` authority comment.
Done:
  Raw expansion is compatibility/test-only or caller-zero; Builder root mode is
  typed and already admitted; old constructors/bypasses = 0.
Stop:
  Do not broaden language semantics or reorganize the Builder barrel.

## Guard packet

The eventual lane guard must prove:

~~~text
total execution issuer definition/caller                 = 1 / 1
narrow A product used as execution selector              = 0
source-backed raw root classifier callers                = 0 after C0
source-backed AST pointer role pairing                    = 0 after C0
root execution bool selector                             = 0 after R0
named move-consuming lifecycle consumer                  = 1
Builder effect before consume                            = 0
canonical reject -> raw/compatibility fallback or retry  = 0
compatibility receives parser source receipt             = 0
production source files                                  < 760 lines
~~~

Positive evidence covers ProgramRuntime providers, pure Script, Main.main/N,
Main static children, and Program-order siblings. Negative evidence covers
duplicate/missing/malformed Main, foreign parser invocation, duplicate/missing
callable relation, transform drift, borrow escape, and second consume.

## NoSafeSlice

Return to NoSafeSlice if:

- old raw semantics cannot be represented exhaustively;
- any App child/root role needs pointer, name, or ordinal pairing downstream;
- final transform cannot preserve the total relation exactly;
- one scoped consumer cannot serve lifecycle, callable catalog, and work-plan;
- one of the three raw semantic callers must remain after cutover;
- compatibility requires a fabricated source-backed product;
- a narrow A state must decide execution role;
- the production switch cannot remove fallback and old callers atomically.

## Ordered cleanup after the root cutover

CURRENT-STATE-COMPACT-POINTER-P0 is closed by this correction: the live pointer
now contains the current lane, blocker, authorized work, prohibitions, and
three landed entries rather than the accumulated implementation journal.

The remaining order is:

1. MIRBUILDER-README-STABLE-CONTRACT-R0
2. MIRBUILDER-BARREL-OWNER-CENSUS-D0

The barrel task is classification only: production, compatibility, caller-zero,
test-only, and facade/re-export. Retire caller-zero first; do not perform an
all-at-once directory reshuffle.
