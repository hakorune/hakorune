//! Conditional source contract for a field-resident named Array receiver.
//!
//! A `me.field = new ArrayBox()` residence lives on the owning Box's `birth`
//! while the `push` call may live in any method of that Box. This row carries
//! only exact source identities: the canonical field reference, the provider
//! caller key, the provider `new` site, and the bounded I64 argument site. It
//! selects no physical representation and grants no backend capability.
use super::body_shape::{
    BodyExpressionShapeV1, BodyMeReceiverV1, VerifiedResolvedBodyShapeInventoryV1,
};
use super::{
    BindingKindV1, BindingRefV1, CallableSemanticSourceLedgerView,
    CoreMethodHomeResultRelationV1, FunctionOwnerIdV1, ResolvedAssignmentTargetV1,
    ResolvedBinaryOperatorV1, ResolvedLexicalRefV1, ResolvedLiteralSourceV1,
    ResolvedMethodCallReceiverSourceV1, ResolvedUnaryOperatorV1, SourceExprSiteV1,
    SourceStmtSiteV1, VerifiedResolvedMethodCallSourceV1,
    VerifiedResolverCoreMethodCallableContractV1,
};
use hakorune_mir_defs::{CanonicalFieldRefV1, CanonicalSameModuleCallableKeyV1};
use std::collections::BTreeSet;

/// Fail-closed boundary for one field-resident Array/I64 append contract.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum NamedArrayFieldResidenceIssueV1 {
    ForeignCall,
    DeclarationCollision,
    ForeignFieldOwner,
    WeakFieldResidence,
    ReassignedReceiver,
    ValueDemand,
    ArgumentShape,
    IntegerSourceMissing,
    ProviderMissing,
    ProviderShape,
    BodyShapeMissing,
}

/// Source-side residence claim detected from the exact alias initializer.
/// The package resolves the field declaration and provider before sealing.
#[derive(Debug, Clone)]
pub(crate) struct NamedArrayFieldResidenceClaimV1 {
    pub(crate) binding: BindingRefV1,
    pub(crate) call: SourceExprSiteV1,
    pub(crate) receiver_site: SourceExprSiteV1,
    pub(crate) argument: SourceExprSiteV1,
    pub(crate) field_site: SourceExprSiteV1,
    pub(crate) field_name: Box<str>,
    pub(crate) object: NamedArrayFieldResidenceObjectV1,
}

#[derive(Debug, Clone)]
pub(crate) enum NamedArrayFieldResidenceObjectV1 {
    /// `me.<field>` inside a method of the owning Box.
    Receiver,
    /// `<binding>.<field>` where the binding's declared type names a foreign
    /// ordinary Box; an unresolvable object stays outside this claim.
    ForeignDeclared(Box<str>),
}

/// Must survive lowering until the actual backend discharges this
/// requirement. Owning this row alone does not authorize a physical Array
/// representation; the provider identities are source-owned, not MIR ids.
#[derive(Debug)]
pub(crate) struct NamedArrayFieldResidenceRequirementV1 {
    owner: FunctionOwnerIdV1,
    field: CanonicalFieldRefV1,
    provider_caller: CanonicalSameModuleCallableKeyV1,
    provider_site: SourceExprSiteV1,
    binding: BindingRefV1,
    call: SourceExprSiteV1,
    argument: SourceExprSiteV1,
}

impl NamedArrayFieldResidenceRequirementV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }
    pub(crate) const fn field(&self) -> CanonicalFieldRefV1 {
        self.field
    }
    pub(crate) fn provider_caller(&self) -> &CanonicalSameModuleCallableKeyV1 {
        &self.provider_caller
    }
    pub(crate) fn provider_site(&self) -> &SourceExprSiteV1 {
        &self.provider_site
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        self.binding
    }
    pub(crate) fn call(&self) -> &SourceExprSiteV1 {
        &self.call
    }
    pub(crate) fn argument(&self) -> &SourceExprSiteV1 {
        &self.argument
    }
}

/// One conditional named-Array obligation. A receiver is either constructed
/// by an in-function `new ArrayBox()` or resides on a canonical field of the
/// receiver's own Box; both arms carry the same call/argument identities so
/// the package transport and physical marker stay a single family.
#[derive(Debug)]
pub(crate) enum NamedArrayRequirementV1 {
    Construction(super::NamedArrayConstructionRequirementV1),
    FieldResidence(NamedArrayFieldResidenceRequirementV1),
}

impl NamedArrayRequirementV1 {
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 {
        match self {
            Self::Construction(requirement) => requirement.owner(),
            Self::FieldResidence(requirement) => requirement.owner(),
        }
    }
    pub(crate) fn call(&self) -> &SourceExprSiteV1 {
        match self {
            Self::Construction(requirement) => requirement.call(),
            Self::FieldResidence(requirement) => requirement.call(),
        }
    }
    pub(crate) fn argument(&self) -> &SourceExprSiteV1 {
        match self {
            Self::Construction(requirement) => requirement.argument(),
            Self::FieldResidence(requirement) => requirement.argument(),
        }
    }
    pub(crate) fn binding(&self) -> BindingRefV1 {
        match self {
            Self::Construction(requirement) => requirement.binding(),
            Self::FieldResidence(requirement) => requirement.binding(),
        }
    }
    /// `Some` only for the in-function `new ArrayBox()` arm.
    pub(crate) fn construction(&self) -> Option<&SourceExprSiteV1> {
        match self {
            Self::Construction(requirement) => Some(requirement.construction()),
            Self::FieldResidence(_) => None,
        }
    }
    pub(crate) fn residence(&self) -> Option<&NamedArrayFieldResidenceRequirementV1> {
        match self {
            Self::Construction(_) => None,
            Self::FieldResidence(requirement) => Some(requirement),
        }
    }
}

/// Detects whether one bounded Array `push` call reads a field-resident
/// receiver through a local alias: `local a = <object>.<field>; a.push(x)`.
/// `Ok(None)` leaves the call to the existing silent boundary; `Err` carries
/// a typed residence rejection.
pub(crate) fn detect_field_residence_claim(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    body_shape: &VerifiedResolvedBodyShapeInventoryV1,
    call: &VerifiedResolvedMethodCallSourceV1,
) -> Result<Option<NamedArrayFieldResidenceClaimV1>, NamedArrayFieldResidenceIssueV1> {
    use NamedArrayFieldResidenceIssueV1 as E;
    if call.owner() != ledger.owner()
        || !ledger
            .method_calls()
            .any(|(site, row)| site == call.site() && std::ptr::eq(row, call))
    {
        return Err(E::ForeignCall);
    }
    let ResolvedMethodCallReceiverSourceV1::Lexical(ResolvedLexicalRefV1::Local(binding)) =
        call.receiver()
    else {
        return Ok(None);
    };
    if binding.owner() != ledger.owner()
        || ledger.variable_ref(call.receiver_site())
            != Some(ResolvedLexicalRefV1::Local(binding))
    {
        return Ok(None);
    }
    let mut initializers = ledger
        .initializer_relations()
        .filter(|row| row.binding() == binding);
    let Some(initializer) = initializers.next() else {
        return Ok(None);
    };
    if initializers.next().is_some() {
        return Ok(None);
    }
    let Some(initializer_site) = initializer.initializer_site() else {
        return Ok(None);
    };
    let Some(BodyExpressionShapeV1::FieldAccess {
        object, field, ..
    }) = body_shape.expression_shape(initializer_site)
    else {
        return Ok(None);
    };
    let object = match body_shape.expression_shape(object) {
        Some(BodyExpressionShapeV1::Me {
            receiver: BodyMeReceiverV1::Lexical(me),
            ..
        }) if ledger
            .binding(*me)
            .is_some_and(|record| record.kind() == BindingKindV1::Receiver) =>
        {
            NamedArrayFieldResidenceObjectV1::Receiver
        }
        Some(BodyExpressionShapeV1::Variable { resolved, .. }) => {
            let ResolvedLexicalRefV1::Local(owner_binding) = resolved else {
                return Ok(None);
            };
            let Some(owner_initializer) = ledger
                .initializer_relations()
                .find(|row| row.binding() == *owner_binding)
            else {
                return Ok(None);
            };
            let Some(declared) = owner_initializer.declared_type_name() else {
                return Ok(None);
            };
            NamedArrayFieldResidenceObjectV1::ForeignDeclared(declared.into())
        }
        _ => return Ok(None),
    };
    Ok(Some(NamedArrayFieldResidenceClaimV1 {
        binding,
        call: call.site().clone(),
        receiver_site: call.receiver_site().clone(),
        argument: call
            .arguments()
            .first()
            .map(|argument| argument.site().clone())
            .unwrap_or_else(|| call.site().clone()),
        field_site: initializer_site.clone(),
        field_name: field.clone(),
        object,
    }))
}

/// Verifies the bounded receiver/argument relations once the package proved
/// the field is a declared ArrayBox field of the owning Box. Provider and
/// field identity are sealed separately; this owns only the caller-ledger
/// relations.
pub(crate) fn verify_field_residence_relations(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    body_shape: &VerifiedResolvedBodyShapeInventoryV1,
    call: &VerifiedResolvedMethodCallSourceV1,
    claim: &NamedArrayFieldResidenceClaimV1,
    integer_source: Option<&VerifiedResolverCoreMethodCallableContractV1>,
) -> Result<(), NamedArrayFieldResidenceIssueV1> {
    use NamedArrayFieldResidenceIssueV1 as E;
    if ledger.assignment_targets().any(|(_, target)| matches!(target, ResolvedAssignmentTargetV1::BindingRebind(actual) if *actual == claim.binding)) {
        return Err(E::ReassignedReceiver);
    }
    if !ledger
        .source_site_inventory()
        .contains_statement(&SourceStmtSiteV1::from_node(call.site().node().clone()))
    {
        return Err(E::ValueDemand);
    }
    let [argument] = call.arguments() else {
        return Err(E::ArgumentShape);
    };
    if call.arity() != 1 || argument.ordinal() != 0 || argument.site() != &claim.argument {
        return Err(E::ArgumentShape);
    }
    if !argument_is_integer_source(ledger, body_shape, &claim.argument, integer_source) {
        return Err(E::IntegerSourceMissing);
    }
    Ok(())
}

/// Seals the package-resolved residence identities into one requirement.
/// `provider_*` must come from the owning Box's `birth` ledger; the issuer
/// passes no MIR identity here.
pub(crate) fn seal_field_residence_requirement(
    owner: FunctionOwnerIdV1,
    field: CanonicalFieldRefV1,
    provider_caller: CanonicalSameModuleCallableKeyV1,
    provider_site: SourceExprSiteV1,
    claim: &NamedArrayFieldResidenceClaimV1,
) -> NamedArrayFieldResidenceRequirementV1 {
    NamedArrayFieldResidenceRequirementV1 {
        owner,
        field,
        provider_caller,
        provider_site,
        binding: claim.binding,
        call: claim.call.clone(),
        argument: claim.argument.clone(),
    }
}

/// Finds the single `me.<field> = new ArrayBox()` store inside the owning
/// Box's `birth` ledger. Anything else — no store, a non-`new` value, a
/// constructor call, or more than one store to the same field — is a typed
/// provider rejection, not a silent omission.
pub(crate) fn resolve_birth_provider(
    birth_ledger: &CallableSemanticSourceLedgerView<'_>,
    birth_body_shape: &VerifiedResolvedBodyShapeInventoryV1,
    field_name: &str,
) -> Result<SourceExprSiteV1, NamedArrayFieldResidenceIssueV1> {
    use NamedArrayFieldResidenceIssueV1 as E;
    let mut stores = Vec::new();
    for source in birth_body_shape.assignment_sources() {
        let Some(BodyExpressionShapeV1::FieldAccess {
            object, field, ..
        }) = birth_body_shape.expression_shape(source.target_site())
        else {
            continue;
        };
        if field.as_ref() != field_name {
            continue;
        }
        let receiver_is_owner = matches!(
            birth_body_shape.expression_shape(object),
            Some(BodyExpressionShapeV1::Me {
                receiver: BodyMeReceiverV1::Lexical(me),
                ..
            }) if birth_ledger
                .binding(*me)
                .is_some_and(|record| record.kind() == BindingKindV1::Receiver)
        );
        if !receiver_is_owner {
            continue;
        }
        stores.push(source);
    }
    let [store] = stores.as_slice() else {
        return Err(if stores.is_empty() {
            E::ProviderMissing
        } else {
            E::ProviderShape
        });
    };
    let construction = birth_ledger
        .construction_source(store.value_site())
        .ok_or(E::ProviderShape)?;
    if crate::runtime::CoreBoxId::from_name(construction.class())
        != Some(crate::runtime::CoreBoxId::Array)
        || !construction.arguments().is_empty()
        || !construction.field_initializers().is_empty()
    {
        return Err(E::ProviderShape);
    }
    Ok(store.value_site().clone())
}

const INTEGER_SOURCE_DEPTH: u32 = 8;

/// Bounded I64 argument proof over the exact resolver rows: integer literal,
/// local binding whose initializer and every rebind are integer-producing,
/// arithmetic over integer operands, unary minus, or a nested CoreMethod
/// contract whose result relation is `I64ToCaller`. Everything else —
/// strings, floats, bools, handles, unknown calls — is foreign to this
/// family and rejects instead of guessing.
fn argument_is_integer_source(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    body_shape: &VerifiedResolvedBodyShapeInventoryV1,
    site: &SourceExprSiteV1,
    integer_source: Option<&VerifiedResolverCoreMethodCallableContractV1>,
) -> bool {
    let mut visited = BTreeSet::new();
    integer_source_at(ledger, body_shape, site, integer_source, &mut visited, 0)
}

fn integer_source_at(
    ledger: &CallableSemanticSourceLedgerView<'_>,
    body_shape: &VerifiedResolvedBodyShapeInventoryV1,
    site: &SourceExprSiteV1,
    integer_source: Option<&VerifiedResolverCoreMethodCallableContractV1>,
    visited: &mut BTreeSet<BindingRefV1>,
    depth: u32,
) -> bool {
    if depth > INTEGER_SOURCE_DEPTH {
        return false;
    }
    if matches!(
        ledger.literal_source(site),
        Some(ResolvedLiteralSourceV1::Integer(_) | ResolvedLiteralSourceV1::TypedInteger { .. })
    ) {
        return true;
    }
    if let Some(contract) = integer_source {
        if contract.owner() == ledger.owner()
            && contract.call_site() == site
            && contract.result_site() == site
            && contract.target().result() == CoreMethodHomeResultRelationV1::I64ToCaller
        {
            return true;
        }
    }
    if let Some(source) = ledger.unary_source(site) {
        if source.operator() == ResolvedUnaryOperatorV1::Minus {
            return integer_source_at(
                ledger,
                body_shape,
                source.operand(),
                integer_source,
                visited,
                depth + 1,
            );
        }
        return false;
    }
    if let Some(source) = ledger.binary_source(site) {
        if !matches!(
            source.operator(),
            ResolvedBinaryOperatorV1::Add
                | ResolvedBinaryOperatorV1::Subtract
                | ResolvedBinaryOperatorV1::Multiply
                | ResolvedBinaryOperatorV1::Divide
                | ResolvedBinaryOperatorV1::Modulo
                | ResolvedBinaryOperatorV1::BitAnd
                | ResolvedBinaryOperatorV1::BitOr
                | ResolvedBinaryOperatorV1::BitXor
                | ResolvedBinaryOperatorV1::Shl
                | ResolvedBinaryOperatorV1::Shr
        ) {
            return false;
        }
        return integer_source_at(ledger, body_shape, source.lhs(), integer_source, visited, depth + 1)
            && integer_source_at(ledger, body_shape, source.rhs(), integer_source, visited, depth + 1);
    }
    let Some(ResolvedLexicalRefV1::Local(binding)) = ledger.variable_ref(site) else {
        return false;
    };
    if !visited.insert(binding) {
        return true;
    }
    let mut initializers = ledger
        .initializer_relations()
        .filter(|row| row.binding() == binding);
    let Some(initializer) = initializers.next() else {
        return false;
    };
    if initializers.next().is_some() {
        return false;
    }
    let Some(initializer_site) = initializer.initializer_site() else {
        return false;
    };
    if !integer_source_at(ledger, body_shape, initializer_site, integer_source, visited, depth + 1) {
        return false;
    }
    body_shape
        .assignment_sources()
        .iter()
        .filter(|source| {
            matches!(
                ledger.assignment_target(source.target_site()),
                Some(ResolvedAssignmentTargetV1::BindingRebind(actual)) if *actual == binding
            )
        })
        .all(|source| {
            integer_source_at(ledger, body_shape, source.value_site(), integer_source, visited, depth + 1)
        })
}
