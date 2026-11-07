//! The one decision of how a function's ownership is analyzed (S3b route rules R1..R7):
//! which plain completions are issued without a walk, and whether the verified
//! walk or only the prefix issuer answers. The requests come from the operations
//! and contracts found in the function; no form of source picks a route.
use std::collections::BTreeSet;

/// One operation or contract of a function that needs the verified ownership walk.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum ScanRequestV1 {
    /// A static call whose callee has a typed contract.
    TypedStaticCall,
    /// A Map literal, as a value or as an actual.
    MapLiteral,
    /// An integer `return` that the borrowed source cohort needs a walk for.
    IntegerReturn,
    /// A read of a borrowed formal's field.
    FormalFieldRead,
    /// A Local initialized by a Dynamic invocation.
    DynamicLocalInitializer,
    /// A call that lends a borrowed formal.
    BorrowedFormalWalk,
    /// A call on a receiver Home.
    ReceiverHomeCall,
    /// A read of an object field of `me`.
    MeObjectFieldRead,
    /// A stored terminal.
    StoredTerminal,
    /// A field write of a scalar.
    FieldScalarWrite,
    /// A contract that requires a scalar source.
    RequiredScalarSource,
    /// A static source Local, or a current-owner i64 source.
    StaticSourceLocal,
    /// A `print` operand expression.
    PrintOperand,
    /// A condition expression.
    ConditionOperand,
    /// An explicit copy or an initializer expression.
    ValueExpression,
    /// A Text `return`.
    TextReturn,
    /// A named array residence contract.
    NamedArrayResidence,
    /// A direct call that holds an owner loan.
    DirectCallOwnerLoan,
    /// A call that returns an object.
    ObjectCallReturn,
}

impl ScanRequestV1 {
    /// S3b-R3b: whether this request alone needs the verified walk. A Map literal needs
    /// it only together with the facts the walk expression names.
    const fn needs_walk(self) -> bool {
        !matches!(self, Self::MapLiteral)
    }
    /// S3b-R3b: whether this request keeps the plain seed completion from being issued.
    /// A direct call's owner loan does not stop it in the app main.
    const fn blocks_plain_seed(self, is_app_main: bool) -> bool {
        !matches!(self, Self::DirectCallOwnerLoan) || !is_app_main
    }
}

/// The requests found in one function, collected in the order the queries run.
#[derive(Debug, Default)]
pub(super) struct ScanRequestsV1(BTreeSet<ScanRequestV1>);

impl ScanRequestsV1 {
    pub(super) fn require(&mut self, request: ScanRequestV1, found: bool) {
        if found {
            self.0.insert(request);
        }
    }
    pub(super) fn has(&self, request: ScanRequestV1) -> bool {
        self.0.contains(&request)
    }
}

/// The facts about a function that the decision reads besides the requests (S3b-R2d).
#[derive(Debug, Clone, Copy)]
pub(super) struct RouteFactsV1 {
    pub(super) seed_eligible: bool,
    pub(super) is_app_main: bool,
    pub(super) has_new_sites: bool,
    pub(super) has_result_sites: bool,
    pub(super) has_top_level_input: bool,
    /// The prefix probe found every selected `new` covered (S3b-R5).
    pub(super) readiness: bool,
}

/// What is issued without a walk. The two are independent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PlainIssuanceV1 {
    pub(super) seed: bool,
    pub(super) app_main_integer: bool,
}

/// How the function's ownership claims are produced after the plain issuance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ScanRouteV1 {
    /// The verified completion with its homes walk. An app main whose completion fails is
    /// answered by the prefix issuer, as before.
    VerifiedWalk,
    /// Only the prefix issuer.
    PrefixOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct OwnershipRouteV1 {
    pub(super) plain: PlainIssuanceV1,
    pub(super) scan: ScanRouteV1,
}

impl OwnershipRouteV1 {
    /// S3b-R2: the one place that decides the route of a function.
    /// `main_integer` answers the app main's plain integer result from the two shape facts
    /// `(no_sites, no_map_or_owner_loan)`.
    pub(super) fn decide(
        requests: &ScanRequestsV1,
        facts: RouteFactsV1,
        main_integer: impl FnOnce(bool, bool) -> bool,
    ) -> Self {
        use ScanRequestV1 as R;
        let child_new_ready = facts.seed_eligible && facts.has_new_sites && facts.readiness;
        let child_result_ready = facts.seed_eligible && facts.has_result_sites;
        let map_literal = requests.has(R::MapLiteral);
        let seed = (facts.seed_eligible
            || (facts.has_top_level_input && !facts.has_new_sites && !facts.has_result_sites))
            && !requests.0.iter().any(|request| request.blocks_plain_seed(facts.is_app_main))
            && !child_new_ready
            && !child_result_ready;
        let app_main_integer = main_integer(
            !facts.has_new_sites
                && !facts.has_result_sites
                && !requests.has(R::PrintOperand)
                && !requests.has(R::ConditionOperand)
                && !requests.has(R::ValueExpression)
                && !requests.has(R::TextReturn)
                && !requests.has(R::TypedStaticCall),
            !map_literal && !requests.has(R::DirectCallOwnerLoan),
        );
        let walk = requests.0.iter().any(|request| request.needs_walk())
            || (facts.is_app_main && (facts.has_new_sites || map_literal || facts.has_result_sites))
            || (facts.seed_eligible && (map_literal || child_new_ready || child_result_ready));
        Self {
            plain: PlainIssuanceV1 { seed, app_main_integer },
            scan: if walk { ScanRouteV1::VerifiedWalk } else { ScanRouteV1::PrefixOnly },
        }
    }
    pub(super) fn walks(&self) -> bool {
        self.scan == ScanRouteV1::VerifiedWalk
    }
}
