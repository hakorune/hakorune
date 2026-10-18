//! Checked field reads retain the original scalar facts, never a write-shaped row.
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::named_array_obligation::{fault, NamedArrayAllocationRefV1};
use crate::mir::normal_callable_semantic_package::CheckedFieldArrayGetSourceV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceExprSiteV1};
use crate::mir::{ArrayReadSiteId, MirInstruction, MirModule, MirType, ValueId};
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct FieldArrayReadRecipeV1 {
    source: Rc<CheckedFieldArrayGetSourceV1>,
}
impl FieldArrayReadRecipeV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn new(
        source: Rc<CheckedFieldArrayGetSourceV1>,
    ) -> Self { Self { source } }
    pub(crate) fn source(&self) -> &Rc<CheckedFieldArrayGetSourceV1> { &self.source }
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 { self.source.call().owner() }
    pub(crate) fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 { self.source.caller() }
    pub(crate) fn call_site(&self) -> &SourceExprSiteV1 { self.source.call().site() }
    pub(crate) fn index_site(&self) -> &SourceExprSiteV1 { self.source.index().site() }
    pub(crate) fn provider(&self) -> &CanonicalSameModuleCallableKeyV1 { self.source.provider() }
    pub(crate) fn provider_site(&self) -> &SourceExprSiteV1 { self.source.provider_site() }
    pub(in crate::mir::normal_callable_semantic_package) fn index_field(&self)
        -> Option<&(crate::mir::resolved_semantics::BindingRefV1, SourceExprSiteV1,
            Rc<crate::mir::normal_callable_semantic_package::LocalFieldReadSourceV1>)> { self.source.index_field() }
    pub(crate) fn field(&self) -> hakorune_mir_defs::CanonicalFieldRefV1 { self.source.field() }
    pub(crate) fn same_source(&self, other: &Self) -> bool { Rc::ptr_eq(&self.source, &other.source) }
    pub(crate) fn allocation(&self) -> NamedArrayAllocationRefV1 {
        NamedArrayAllocationRefV1::FieldResidence {
            field: self.field(),
            provider_caller: self.provider().clone(),
            provider_site: self.provider_site().clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct EmittedFieldArrayReadV1 {
    pub(super) recipe: Rc<FieldArrayReadRecipeV1>,
    pub(super) read: ArrayReadSiteId,
    receiver: ValueId,
    index: ValueId,
    dst: ValueId,
}
impl EmittedFieldArrayReadV1 {
    pub(crate) fn new(
        recipe: Rc<FieldArrayReadRecipeV1>,
        read: ArrayReadSiteId,
        receiver: ValueId,
        index: ValueId,
        dst: ValueId,
    ) -> Self {
        Self {
            recipe,
            read,
            receiver,
            index,
            dst,
        }
    }
    pub(super) fn validate<'m>(&self, module: &'m MirModule) -> Result<&'m str, String> {
        let symbol = module
            .canonical_callable_definition_symbol(self.recipe.caller())
            .ok_or_else(|| fault("read-caller-missing"))?;
        let function = module
            .functions
            .get(symbol)
            .ok_or_else(|| fault("read-function-missing"))?;
        if module.canonical_field_definition(self.recipe.field())
            .is_none_or(|row| row.name != self.recipe.source.field_name()) {
            return Err(fault("read-field-foreign"));
        }
        self.validate_function(function)?;
        crate::mir::named_array_obligation::validate_array_allocation(
            module, function, &self.recipe.allocation(),
        )?;
        Ok(symbol)
    }

    pub(crate) fn dst(&self) -> ValueId { self.dst }
    pub(crate) fn recipe(&self) -> &Rc<FieldArrayReadRecipeV1> { &self.recipe }
    pub(crate) fn validate_structure(&self, function: &crate::mir::MirFunction) -> Result<(), String> {
        if function.signature.name != self.recipe.caller().mir_symbol_projection() {
            return Err(fault("read-function-drift"));
        }
        let mut count = 0;
        let mut field_count = 0;
        // Finishing may copy the receiver. Normalize through the shared MIR owner,
        // retaining unique physical definitions rather than granting an alias by name.
        let definitions = crate::mir::value_origin::build_value_def_map(function);
        let is_self = |base| {
            let (root, chain) =
                crate::mir::value_origin::trace_value_origin(function, &definitions, base);
            Some(&root) == function.params.first()
                && chain.iter().all(|value| {
                    let count = function
                        .blocks
                        .values()
                        .flat_map(|block| block.all_instructions())
                        .filter(|instruction| instruction.dst_value() == Some(*value))
                        .count();
                    count == usize::from(*value != root)
                })
        };
        for instruction in function
            .blocks
            .values()
            .flat_map(|block| block.all_instructions())
        {
            match instruction {
                MirInstruction::ArrayRead {
                    site_id,
                    dst,
                    receiver,
                    operation: crate::mir::instruction::ArrayReadOperationV1::Element { index },
                } if *site_id == self.read => {
                    if *dst != Some(self.dst) || *receiver != self.receiver || *index != self.index
                    {
                        return Err(fault("read-operand-drift"));
                    }
                    count += 1;
                }
                MirInstruction::ObjectFieldGet { dst, base, field } if *dst == self.receiver => {
                    if *field != self.recipe.field() || !is_self(*base) {
                        return Err(fault("read-field-receiver-drift"));
                    }
                    field_count += 1;
                }
                MirInstruction::FieldGet {
                    dst, base, field, ..
                } if *dst == self.receiver => {
                    if field != self.recipe.source.field_name() || !is_self(*base) {
                        return Err(fault("read-field-receiver-drift"));
                    }
                    field_count += 1;
                }
                _ => {}
            }
        }
        if count != 1 || field_count != 1 {
            return Err(fault("read-physical-cardinality"));
        }
        Ok(())
    }
    pub(crate) fn validate_function(&self, function: &crate::mir::MirFunction) -> Result<(), String> {
        self.validate_structure(function)?;
        if function.metadata.value_types.get(&self.dst) != Some(&MirType::Integer)
            || function.metadata.value_types.get(&self.index) != Some(&MirType::Integer)
        {
            return Err(fault("read-type-drift"));
        }
        let marker = crate::mir::named_array_obligation::NamedArrayReadMarkerV1 {
            owner: self.recipe.owner(),
            read: self.read,
            receiver: self.receiver,
            index: self.index,
            dst: self.dst,
        };
        if function
            .metadata
            .named_array_read_obligations
            .iter()
            .filter(|row| **row == marker)
            .count()
            != 1
        {
            return Err(fault("read-marker-cardinality"));
        }
        Ok(())
    }
}

impl PartialEq for EmittedFieldArrayReadV1 {
    fn eq(&self, other: &Self) -> bool {
        self.recipe.same_source(&other.recipe) && self.read == other.read
            && self.receiver == other.receiver && self.index == other.index && self.dst == other.dst
    }
}
impl Eq for EmittedFieldArrayReadV1 {}
