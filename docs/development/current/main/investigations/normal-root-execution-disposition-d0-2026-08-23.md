# Normal root execution disposition

Status: authority core accepted — parser-product transport closure reopened
Date: 2026-08-23
Decision: NORMAL-ROOT-EXECUTION-DISPOSITION-D0
Owner: parser source authority -> normal/default root lifecycle

This D0 closed parity before authority, and authority before substrate. It
selected only the parser-owned P0 recorded below. Before Rust work began, the
shared reference parser-product exit exposed two distinct route obligations.
`NORMAL-ROOT-EXECUTION-REFERENCE-ROUTE-CLOSURE-D0` must close that state before P0
resumes; the source authority decision itself remains unchanged.

## Six-line brief

Decision:
  Accept one total normal-root execution relation distinct from narrow
  Script-A/Main.main/0 admission. Never reinterpret narrow Outside states as
  ProgramRuntime.
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
  NORMAL-ROOT-EXECUTION-SOURCE-P0: implement only the parser relation, paired
  issuer input, required move chain, exact preservation, tests, docs, and one
  reusable guard.
Non-claims:
  No lifecycle cutover, compatibility change, App-shape expansion, Script-A
  change, Recipe/lowering/publication work, raw retirement, or performance work.

Census boundary: normal/default source-backed lifecycle entry -> root-role
catalog, work-plan, and lowering consumers; includes all three raw semantic
reclassification sites; excludes parser-product exits and all explicit
reference/Raw/compatibility owners. Those exits are now owned by
`NORMAL-ROOT-EXECUTION-REFERENCE-ROUTE-CLOSURE-D0`.

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

## D0-A parity census result

The old raw classifier has a finite policy. It examines only the top-level
Program and the method inventory of the unique top-level static Box named
`Main`:

| Raw observation | Raw result | Source-backed total result |
| --- | --- | --- |
| root is not Program | `RootMustBeProgram` | no product; never fabricate Program authority |
| no top-level `static box Main` | Script | Ready(ProgramRuntime) when exact initial source exists |
| one top-level `static box Main` | continue Main validation | continue total App issuance when exact initial source exists |
| more than one top-level `static box Main` | `DuplicateMainBox` | IntegrityInvalid(DuplicateMain) |
| Main has no `main` inventory entry | `MainMethodMissing` | Incomplete(MainMethodMissing) when source-backed reachable |
| Main `main` is not a function | `MainMethodMustBeFunction` | compatibility/raw only under current initial-source contract |
| Main `main` is not static | `StaticChildMustBeStatic(main)` | IntegrityInvalid(MainMethodNotStatic) when source-backed reachable |
| Main `main` has any arity | App | Ready(App) with exact identity/site/arity |
| every non-main inventory row is a static function | App | Ready(App) with complete child relation |
| helper is non-function | `StaticChildMustBeFunction` | compatibility/raw only under current initial-source contract |
| helper is non-static | `StaticChildMustBeStatic` | IntegrityInvalid(ChildNotStatic) when source-backed reachable |
| expected callable relation is missing | not a raw-classifier state | Incomplete(CallableRelationMissing) |
| relation is foreign/duplicate/contradictory | not a raw-classifier state | IntegrityInvalid(CallableRelation) |

Raw error vocabulary and source-backed execution vocabulary are deliberately
separate. Explicit parser inventory construction rejects some malformed method
shapes before an initial callable source can exist; compatibility ASTs may
still reach the historical raw errors. The total issuer must not import those
compatibility-only shapes merely to duplicate the raw enum.

The following are parity constraints, not new language claims:

- a non-static Box named `Main` is ignored by the role selector;
- one static Main plus any number of non-static Main declarations is App;
- nested Main, case-different names, and non-Box declarations named Main do not
  count as top-level static Main;
- a static Main may coexist with top-level executable statements, ordinary
  Boxes, top-level functions, and other static Boxes;
- fields, constructors, static initializers, and other Box flags do not select
  App versus ProgramRuntime in this classifier;
- Main static children currently execute in compatibility-name order. The new
  issuer must preserve an owned execution ordinal for parity, while identity
  and source site remain the pairing authority;
- App instance-Box and top-level-function siblings retain their immediate
  handling; non-Main static Boxes retain their App no-op/deferred behavior;
  App runtime-only statements remain outside the App terminal consumer;
- compatibility inventory may contain a map key and declaration name that do
  not agree. That historical raw behavior remains compatibility-only and is
  not a canonical source relation;
- an initial callable source that is unavailable because of BuildGate,
  delegate, interface/record, macro/import, or another explicit compatibility
  condition stays on the compatibility owner and receives no source-backed
  execution product.

`CompletedParserPostpassV1::initial_callable_source()` plus the complete
parameter catalog is the minimum admissible source boundary. The existing
single-parent static-Box seal is not total: it rejects additional static
parents and additional members that the old App policy accepts. It is therefore
non-authority for this row.

## D0-A consumer and retirement manifest

| Old semantic edge | Current use | Replacement input | Retirement condition |
| --- | --- | --- | --- |
| lifecycle raw scan #1 | pre-effect App bool | typed App/ProgramRuntime disposition | source-backed caller zero |
| `preflight_is_app_mode` | Script lookup and A-observation gate | typed root role projection | bool selector zero |
| callable-catalog raw scan | root Main exclusion and child role assignment | complete parser callable-role map | scan caller zero |
| catalog AST pointer comparisons | pair root/children with final callable rows | opaque identity + exact final slot/site | source-backed pointer pairing zero |
| catalog child ordinal lookup | feed Completion/signature/S6C/Dynamic roles | same role row carried through selected mapping | ordinal-only lookup zero |
| lifecycle raw scan #2 | retained-source drift check | exact transform-preserved disposition | second scan zero |
| entry materialization seal | optional `Main.main/N` target | admitted App root identity + canonical callable key | raw expansion input zero |
| bool work-plan API | statement schedule and terminal | admitted execution-mode enum | selected-normal bool parameter zero |
| work-plan index/name/arity pairing | pair statements with selected callables | paired Program cursor + callable identity map | selected-normal re-pairing zero |
| selected Main/name classifiers | hide Main from Script/runtime work | parser-issued RootMain statement role | source-backed Main name checks zero |
| installed-package child pointer check | pair selected child with source syntax | parser role identity + selected batch mapping | pointer pairing zero |
| `root_is_app_mode: Option<bool>` | non-Main static-Box runtime behavior | already-admitted execution mode carried by work | production field reader zero |
| terminal expansion match | Script runtime versus App root lowering | typed terminal carrying its exact projection | raw expansion parameter zero |
| `VerifiedMainExpansionV1` child/root views | App helper and root-body lowering | one scoped typed App loan | source-backed consumer zero |
| stale sole-authority comment | claims raw expansion is canonical | compatibility-only contract | comment corrected with R0 |

These edges are one atomic cutover closure. In particular, replacing both
lifecycle scans while retaining either catalog pointer pairing would leave a
second source authority alive.

Production cardinality before cutover is three raw classifications per
source-backed Normal compile: lifecycle preflight, callable-catalog issuance,
and retained-source verification. Normal compatibility retains its existing
two lifecycle classifications; explicit Raw retains its separate one. C0
changes only the source-backed set.

## D0-B authority freeze candidate

`ParserNormalRootExecutionIssuerV1::issue_once` is called once from
`ParsedProgramWithCallableParameterSourceV1::new`. It may inspect names and
syntax only inside that parser-owned call. Its accepted inputs are exactly:

~~~text
CompletedParserPostpassV1::initial_callable_source()
+ ParserNormalProgramSourceAuthorityV1
+ complete ParserCallableParameterSourceCatalogV1
~~~

The initial callable product already owns the complete `(source identity,
final slot, declaration)` relation. The issuer must consume a parser-private
loan over those paired rows; it must not zip independently exposed arrays.

The total relation covers every callable exactly once:

~~~text
AppRootMain
AppStaticChild { execution_ordinal }
Ordinary
~~~

For App, exactly one row is `AppRootMain`; every non-main method in the same
static Main is an `AppStaticChild`; every other callable is `Ordinary`. For
ProgramRuntime every callable is `Ordinary`, beside a source-backed
`NoTopLevelStaticMain` witness. The role row stores opaque callable identity,
exact final slot/source site, and its role. It stores no AST reference, encoded
symbol, Builder key, Recipe key, ValueId, or MIR type.

The relation also covers every Program statement exactly once. App marks the
unique static Main statement as `RootMain` and all siblings as `Ordinary`; a
ProgramRuntime marks every statement `Ordinary`. These are root-consumption
roles only. The parser does not issue immediate/deferred/runtime work-plan
meaning; existing declaration, Script-item, constructor, and Recipe owners
consume the paired statement cursor inside the named root consumer.

Main materialization symbol selection remains downstream of the source
relation. The parser carries main identity, final slot/site, and declared
arity; the canonical callable catalog supplies the key used to derive the
encoded symbol. An encoded backend or Builder symbol does not enter this
product.

The final transform moves the same disposition after the existing exact
Program/cardinality and callable-declaration checks. It may verify identity and
slot coverage, but may not rescan `Main`, re-sort methods, or reissue roles.

The accepted source model is nested by Program statement, so independent
statement/callable arrays cannot be re-paired:

~~~rust
struct ParserNormalRootExecutionSourceV1 {
    invocation: ParserInvocationWitnessV1,
    program: ParserNormalRootExecutionProgramV1,
}

enum ParserNormalRootExecutionProgramV1 {
    App(ParserNormalAppExecutionProgramV1),
    ProgramRuntime(ParserNormalProgramRuntimeExecutionProgramV1),
}

struct ParserNormalRootStatementRelationV1 {
    source: ParserNormalProgramBodySourceRowV1,
    role: ParserNormalRootStatementRoleV1,
}

enum ParserNormalRootStatementRoleV1 {
    Ordinary {
        callables: Box<[ParserNormalRootCallableRelationV1]>,
    },
    AppMain {
        root: ParserNormalRootCallableRelationV1,
        children: Box<[ParserNormalAppStaticChildRelationV1]>,
    },
}
~~~

Every callable relation owns its opaque parser identity, exact final slot and
source coordinate, declaration kind, and declared arity. A child additionally
owns a contiguous compatibility-name execution ordinal. Array position and
ordinal are schedule/coverage evidence only; opaque identity remains pairing
authority.

The closed source-backed errors are:

~~~text
SourceAuthorityUnavailable:
  initial callable source / complete parameter catalog / Program authority absent

Incomplete:
  Program body or Main main relation missing
  statement/callable row or representable ordinal missing

IntegrityInvalid:
  duplicate static Main
  non-static required Main method/helper when source-backed reachable
  foreign parser witness
  duplicate/contradictory identity, slot, site, role, or coverage
~~~

Compatibility-only malformed AST errors remain owned by raw expansion and are
not copied into this enum.

## Accepted paired issuer input

`VerifiedInitialCallableProgramSourceV1` gains one parser-private HRTB issuer
loan. Each callable row contains, as one item:

~~~text
opaque callable identity
+ PreparedCallableSourceV1 relation
+ InitialCallableFinalSlotV1
+ borrowed exact declaration
~~~

The same loan includes the paired Program statement cursor from
`ParserNormalProgramSourceAuthorityV1`. The loan constructor verifies the
initial source, Program authority, and parameter catalog share one parser
invocation before `ParserNormalRootExecutionIssuerV1::issue_once` runs. No raw
slice pair, `zip`, public constructor, or second static-parent authority is
exposed.

The issuer performs exactly one top-level Main classification and one Main
method inventory walk. It may sort child observations by diagnostic name only
to preserve the old schedule, then stores the resulting execution ordinal.
Names never leave as pairing keys.

## Accepted final preservation

The total disposition is a required, non-Clone field through:

~~~text
ParsedProgramWithCallableParameterSourceV1
-> PreparedNormalCallableProgramSourceV1
-> exact callable transform
-> VerifiedFinalCallableProgramSourceV1
-> PreparedNormalDefaultProgramRootV1
~~~

The existing exact Program cardinality/statement equality, callable slot, and
declaration equality checks run before the field moves into the final source.
Preservation may verify witness/coverage, but cannot classify Main, sort
children, or construct new role rows. A Ready relation that would enter
compatibility is a typed hard reject.

## Accepted one-shot consumer aggregate

The sole source-backed consumer is
`NormalRootExecutionConsumerV1::consume_once`, called at lifecycle entry before
`PreparedNormalProgramDeclarationFactsV1::collect`.

Conceptually it performs this affine transition:

~~~text
VerifiedFinalCallableProgramSourceV1
  -> one HRTB root-execution loan
       -> owned role-bound callable catalog
       -> owned typed Program-root work source
       -> owned Main materialization source relation
  -> ConsumedNormalRootCallableSourceV1
     (same final source, no root-execution getter)
  -> PreparedNormalRootExecutionConsumptionV1
~~~

The callback result cannot borrow from the source. After the callback returns,
the original AST/callable/source owners are moved into
`ConsumedNormalRootCallableSourceV1`, which is the only source type accepted by
the revised callable semantic-package issuer. This makes a second root-role
loan impossible without cloning or reconstructing authority.

`PreparedNormalRootExecutionConsumptionV1` is a private aggregate, not a parts
tuple. Its App/ProgramRuntime variant is the sole pre-effect gate. It carries
the preissued catalog and typed work/materialization source through named
methods. Later resolver/source loans may resolve other semantics, but cannot
issue or alter root roles.

The typed work source owns the current one-time AST lowering projection. App's
RootMain statement is opaque and nests exact root/children syntax; ordinary
siblings remain paired statement rows. ProgramRuntime carries every paired
statement row. Existing declaration, constructor, Script-item, Recipe, and
physical owners still decide their own meanings.

Current decision state is:

~~~text
D0-A parity/consumer closure = accepted
D0-B exact authority/API     = accepted
next implementation          = parser-only P0
~~~

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

Status: accepted 2026-08-23. The raw/compatibility state table is separated
from the source-backed reachable state table, and the complete production
retirement closure is recorded above.

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

Status: accepted 2026-08-23 after two independent read-only audits.

Change:
  Freeze the sole issuer, exact fields, typed errors, and relation with callable
  identities/final slots.
Done:
  Parser source authority and final preservation can issue every mapped state
  without Builder, pointer, raw fallback, or narrow A-state inference; one
  pre-effect consumer issues catalog roles and typed root work in the same
  scoped loan.
Stop:
  Missing source/callable relation is NoSafeSlice, not an empty/default row.

D0-B is executed in this order:

1. `B1-SOURCE-MODEL`
   - freeze App/ProgramRuntime statement rows, callable roles, helper execution
     order, and source-backed versus compatibility boundary;
   - done when every required field has exactly one existing parser owner.
2. `B2-ISSUER-INPUT`
   - freeze one parser-private paired loan from the initial callable source;
   - done when identity, declaration, and final slot cannot be independently
     zipped or rebuilt.
3. `B3-PRESERVATION`
   - freeze move-through validation at the exact final callable transform;
   - done when all statement/callable drift is typed and no role is reissued.
4. `B4-CONSUMER-AGGREGATE`
   - freeze one `PreparedNormalDefaultProgramRootV1` consuming facade that
     creates gate, catalog roles, and typed work/materialization siblings in
     one HRTB callback before declaration facts;
   - done when no AST reference, second getter, or later source reloan escapes.
5. `B5-CUTOVER-PROOF`
   - bind every retirement edge above to C0 and one reusable lane guard;
   - done when source-backed zeroes and compatibility/Raw preserved callers
     are mechanically distinguishable.

No B-cell authorized production Rust changes. D0-B acceptance selects only P0
as the next implementation slice; S0/C0 remain unauthorized until P0 closes.

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
explicit Raw classifier caller                           = 1 unchanged
Normal compatibility classifier caller set               = unchanged
source-backed AST pointer role pairing                    = 0 after C0
source-backed name/ordinal role pairing                   = 0 after C0
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
- complete Program statement coverage cannot be consumed without a second
  root-role scan;
- Main child role cannot flow through Completion, physical signature, S6C,
  Dynamic exclusion, and installed-package lookup by opaque identity;
- selected work-plan construction still needs index/name/arity or Main-name
  re-pairing;
- helper execution order is not frozen to the current compatibility-name order;
- disposition consumption occurs after declaration/package/module effects;
- one of the three raw semantic callers must remain after cutover;
- Normal compatibility or explicit Raw must receive a fabricated parser
  product or be changed in the same cutover;
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
