//! Checked field reads retain the original scalar facts, never a write-shaped row.
use crate::mir::builder::CanonicalSameModuleCallableKeyV1;
use crate::mir::named_array_obligation::{fault, NamedArrayAllocationRefV1};
use crate::mir::normal_callable_semantic_package::CheckedFieldArrayAccessSourceV1;
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, SourceExprSiteV1};
use crate::mir::{ArrayReadSiteId, MirInstruction, MirModule, MirType, ValueId};
use std::rc::Rc;

#[derive(Debug)]
pub(crate) struct FieldArrayReadRecipeV1 {
    source: Rc<CheckedFieldArrayAccessSourceV1>,
}
impl FieldArrayReadRecipeV1 {
    pub(in crate::mir::normal_callable_semantic_package) fn new(
        source: Rc<CheckedFieldArrayAccessSourceV1>,
    ) -> Self { Self { source } }
    pub(crate) fn source(&self) -> &Rc<CheckedFieldArrayAccessSourceV1> { &self.source }
    pub(crate) fn owner(&self) -> FunctionOwnerIdV1 { self.source.call().owner() }
    pub(crate) fn caller(&self) -> &CanonicalSameModuleCallableKeyV1 { self.source.caller() }
    pub(crate) fn call_site(&self) -> &SourceExprSiteV1 { self.source.call().site() }
    pub(crate) fn index_site(&self) -> Option<&SourceExprSiteV1> { self.source.index().map(|index| index.site()) }
    pub(crate) fn is_count(&self) -> bool { self.source.is_count() }
    pub(crate) fn physical_operation(&self, values: &[ValueId]) -> Result<crate::mir::instruction::ArrayReadOperationV1, String> {
        use crate::mir::instruction::ArrayReadOperationV1 as Operation;
        match (self.is_count(), values) {
            (true, []) => Ok(Operation::Count),
            (false, [index]) => Ok(Operation::Element { index: *index }),
            _ => Err(fault("read-argument-cardinality")),
        }
    }
    pub(crate) fn accepts_operation(&self, operation: crate::mir::instruction::ArrayReadOperationV1) -> bool {
        matches!((self.is_count(), operation), (true, crate::mir::instruction::ArrayReadOperationV1::Count)
            | (false, crate::mir::instruction::ArrayReadOperationV1::Element { .. }))
    }
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
    operation: crate::mir::instruction::ArrayReadOperationV1,
    dst: ValueId,
}
impl EmittedFieldArrayReadV1 {
    pub(crate) fn new(
        recipe: Rc<FieldArrayReadRecipeV1>,
        read: ArrayReadSiteId,
        receiver: ValueId,
        operation: crate::mir::instruction::ArrayReadOperationV1,
        dst: ValueId,
    ) -> Self {
        Self {
            recipe,
            read,
            receiver,
            operation,
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
        if !module.canonical_array_field(self.recipe.field())
            || module.canonical_field_definition(self.recipe.field())
            .is_none_or(|row| row.name != self.recipe.source.field_name()) {
            return Err(fault("read-field-foreign"));
        }
        self.validate_function(function)?;
        crate::mir::named_array_obligation::validate_array_allocation(
            module, function, &self.recipe.allocation(),
        )?;
        Ok(symbol)
    }

    pub(in crate::mir) fn count_binding_v1(&self, module: &MirModule,
        function: &crate::mir::MirFunction,
    ) -> Result<Option<(crate::mir::BasicBlockId, MirInstruction)>, String> {
        if !self.recipe.is_count() { return Ok(None); }
        let Some(symbol) = module.canonical_callable_definition_symbol(self.recipe.caller()) else {
            return Err(fault("read-caller-missing"));
        };
        if symbol != function.signature.name { return Ok(None); }
        if module.functions.get(symbol).is_none_or(|stored| !std::ptr::eq(stored, function)) {
            return Err(fault("read-function-membership"));
        }
        self.validate(module)?;
        let mut bindings = function.blocks.iter().flat_map(|(block, body)|
            body.all_instructions().map(move |row| (*block, row))).filter(|(_, row)|
                matches!(row, MirInstruction::ArrayRead { site_id, .. } if *site_id == self.read));
        let (block, row) = bindings.next().ok_or_else(|| fault("read-physical-cardinality"))?;
        if bindings.next().is_some() { return Err(fault("read-physical-cardinality")); }
        Ok(Some((block, row.clone())))
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
                    operation,
                } if *site_id == self.read => {
                    if *dst != Some(self.dst) || *receiver != self.receiver || *operation != self.operation
                        || !self.recipe.accepts_operation(*operation)
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
            || self.operation.used_values().any(|index| function.metadata.value_types.get(&index) != Some(&MirType::Integer))
        {
            return Err(fault("read-type-drift"));
        }
        let marker = crate::mir::named_array_obligation::NamedArrayReadMarkerV1 {
            owner: self.recipe.owner(),
            read: self.read,
            receiver: self.receiver,
            operation: self.operation,
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
            && self.receiver == other.receiver && self.operation == other.operation && self.dst == other.dst
    }
}
impl Eq for EmittedFieldArrayReadV1 {}
