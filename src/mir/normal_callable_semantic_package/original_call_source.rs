//! Borrow context for the existing source collectors, never execution authority.
//! Constructor owners retain their real source input instead of a batch slot.
use super::instance_constructor_semantic::VerifiedInstanceConstructorSemanticRowV1;
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{OwnedExprSiteV1, VerifiedResolvedMethodCallSourceV1};
use hakorune_mir_defs::{CanonicalSameModuleCallableKeyV1, SameModuleCallableNamespaceV1};

#[derive(Clone, Copy)]
pub(in crate::mir::normal_callable_semantic_package) enum OriginalCallSourceCallerV1<'a> {
    Declaration { batch_slot: u32 },
    Constructor {
        row: &'a VerifiedInstanceConstructorSemanticRowV1,
        input: ResolvedFunctionLoweringInputV1<'a>,
    },
}
impl From<u32> for OriginalCallSourceCallerV1<'_> {
    fn from(batch_slot: u32) -> Self { Self::Declaration { batch_slot } }
}
impl<'a> OriginalCallSourceCallerV1<'a> {
    pub(super) fn declaration_slot(self) -> Option<u32> {
        match self { Self::Declaration { batch_slot } => Some(batch_slot), Self::Constructor { .. } => None }
    }
    /// Corroborate a genuine borrowed constructor source; no eligibility query.
    pub(super) fn constructor_key_for_call(
        self, site: &OwnedExprSiteV1, call: &VerifiedResolvedMethodCallSourceV1,
    ) -> Result<Option<&'a CanonicalSameModuleCallableKeyV1>, String> {
        let Self::Constructor { row, input } = self else { return Ok(None); };
        let reject = || "[freeze:contract][original-call/constructor-source-identity]".to_owned();
        let key = row.published_birth_key().ok_or_else(reject)?;
        if row.kind() != crate::parser::ConstructorSourceKindV1::Birth
            || key.namespace() != SameModuleCallableNamespaceV1::BirthConstructor
            || key.owner() != row.box_name() || key.arity() != row.source_arity()
            || row.forest().roots() != [site.owner()]
            || !std::ptr::eq(row.forest(), input.forest())
            || !row.forest().owner(site.owner()).is_some_and(|original| std::ptr::eq(original, input.function()))
            || input.owner() != site.owner() || call.owner() != site.owner() || call.site() != site.site()
            || input.function().method_call(site.site()) != Some(call)
        { return Err(reject()); }
        Ok(Some(key))
    }
}

/// Reborrow one original root input by its resolved owner, for existing source
/// consumers. This neither enumerates calls nor assigns constructor batch slots.
pub(super) fn with_original_source_input_v1<R>(
    batch: &crate::mir::callable_semantic_batch::VerifiedResolvedCallableSemanticBatchV1,
    constructors: &super::VerifiedInstanceConstructorSemanticBatchV1,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    callback: impl for<'source> FnOnce(OriginalCallSourceCallerV1<'source>, ResolvedFunctionLoweringInputV1<'source>) -> R,
) -> Result<R, String> {
    let reject = || "[freeze:contract][original-call/source-owner]".to_owned();
    let mut declarations = batch.declarations().filter(|row| row.owner() == owner);
    if let Some(declaration) = declarations.next() {
        if declarations.next().is_some()
            || constructors.rows().iter().any(|row| row.forest().roots() == [owner])
        { return Err(reject()); }
        let slot = declaration.batch_slot();
        return batch.with_lowering_input(slot, |input|
            callback(OriginalCallSourceCallerV1::Declaration { batch_slot: slot }, input))
            .map_err(|_| reject());
    }
    let mut rows = constructors.rows().iter().filter(|row| row.forest().roots() == [owner]);
    let row = rows.next().ok_or_else(reject)?;
    let input = row.lowering_input(batch.source_ast())?;
    let key = row.published_birth_key().ok_or_else(reject)?;
    if rows.next().is_some() || row.kind() != crate::parser::ConstructorSourceKindV1::Birth
        || key.namespace() != SameModuleCallableNamespaceV1::BirthConstructor
        || key.owner() != row.box_name() || key.arity() != row.source_arity()
        || !std::ptr::eq(row.forest(), input.forest()) || input.owner() != owner
    { return Err(reject()); }
    Ok(callback(OriginalCallSourceCallerV1::Constructor { row, input }, input))
}
