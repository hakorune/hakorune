//! LOCAL-only allocation execution evidence; field initialization stays separate.
use super::*;
use crate::mir::function::ObjectDestructionDispositionV1;
use crate::mir::normal_callable_semantic_package::ordinary_new_coseal::OrdinaryNewConstructorDispositionV1;

#[derive(Debug, Clone)]
pub(crate) struct AllocationOnlyNewExecutionV1 {
    source: Rc<ConstructionSourceStructureV1>,
}
impl PartialEq for AllocationOnlyNewExecutionV1 {
    fn eq(&self, other: &Self) -> bool { Rc::ptr_eq(&self.source, &other.source) }
}
impl Eq for AllocationOnlyNewExecutionV1 {}
impl AllocationOnlyNewExecutionV1 {
    /// Called only for an original LOCAL candidate, never shared Result New resolution.
    pub(in crate::mir::normal_callable_semantic_package) fn issue_local(
        construction: &ConstructionEligibilityV1, source: &ConstructionSourceEligibilityV1,
        constructor: &OrdinaryNewConstructorDispositionV1, arity: usize,
        object: CanonicalObjectIdV1, destruction: ObjectDestructionDispositionV1,
    ) -> Option<Self> {
        let source = source.as_ref().ok()?;
        if construction != &Err(ConstructionUnavailableV1::InitializationContractMissing)
            || !matches!(constructor, OrdinaryNewConstructorDispositionV1::NoBirthZero)
            || arity != 0 || !destruction.has_no_hook()
            || source.object() != object || !source.supports_empty_allocation()
            || source.constructor().is_some() || !source.steps().is_empty() || !source.stores().is_empty() {
            return None;
        }
        Some(Self { source: Rc::clone(source) })
    }
    pub(crate) fn source(&self) -> &Rc<ConstructionSourceStructureV1> { &self.source }
    pub(crate) fn matches_source(&self, source: &ConstructionSourceEligibilityV1) -> bool {
        source.as_ref().is_ok_and(|source| Rc::ptr_eq(source, &self.source))
    }
}
#[derive(Debug)]
pub(crate) enum NewExecutionProfileRefV1<'a> {
    Initialized(&'a ConstructionPlanV1),
    AllocationOnly(&'a AllocationOnlyNewExecutionV1),
}
/// A view over existing initialized proof or the one LOCAL allocation lease.
/// No field-ready state, Return escape or fallback is created here.
pub(crate) fn new_execution_profile_v1<'a>(construction: &'a ConstructionEligibilityV1,
    allocation: Option<&'a AllocationOnlyNewExecutionV1>, object: CanonicalObjectIdV1,
) -> Result<NewExecutionProfileRefV1<'a>, ConstructionUnavailableV1> {
    match (construction, allocation) {
        (Ok(plan), None) if plan.object() == object => Ok(NewExecutionProfileRefV1::Initialized(plan)),
        (Err(ConstructionUnavailableV1::InitializationContractMissing), Some(lease))
            if lease.source().object() == object => Ok(NewExecutionProfileRefV1::AllocationOnly(lease)),
        (Err(reason), None) => Err(*reason),
        _ => Err(ConstructionUnavailableV1::SourceRelationMissing),
    }
}
