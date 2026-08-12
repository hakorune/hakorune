//! Session-local formal adoption and loop-header opening for the V2 canary.
//!
//! This module consumes the already verified A-prime formal relation. It does
//! not classify values, choose providers, or issue operation/cleanup meaning.

use crate::mir::builder::resolved_lowering::canonical_ssa::{
    CanonicalBindingReadReceiptV1, CanonicalSsaFunctionSessionV2,
};
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_emitter::{
    targets::DynamicV2PhysicalTargetSetV1, DynamicV2PhysicalSessionBrandV1,
};
use crate::mir::builder::MirBuilder;
use crate::mir::compiler::dynamic_full_body_recipe::{
    DynamicAPrimeI64SourceRelationViewV1, DynamicFullLoopParameterClassV2,
};
use crate::mir::resolved_semantics::BindingRefV1;
use crate::mir::ValueId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct DynamicV2FormalSeedV1 {
    ordinal: u32,
    binding: BindingRefV1,
    recipe_value: crate::mir::loop_recipe_contract::LoopValueKeyV1,
    value: ValueId,
}

#[derive(Debug)]
pub(super) struct DynamicV2OpenedFormalHeaderV1 {
    enter: crate::mir::BasicBlockId,
    header: crate::mir::BasicBlockId,
    formals: [DynamicV2FormalSeedV1; 4],
    header_current: CanonicalBindingReadReceiptV1,
}

impl DynamicV2OpenedFormalHeaderV1 {
    #[cfg(test)]
    pub(super) fn formals(&self) -> &[DynamicV2FormalSeedV1; 4] {
        &self.formals
    }

    #[cfg(test)]
    pub(super) const fn enter(&self) -> crate::mir::BasicBlockId {
        self.enter
    }

    #[cfg(test)]
    pub(super) const fn header(&self) -> crate::mir::BasicBlockId {
        self.header
    }

    #[cfg(test)]
    pub(super) const fn header_current(&self) -> CanonicalBindingReadReceiptV1 {
        self.header_current
    }
}

#[cfg(test)]
impl DynamicV2FormalSeedV1 {
    pub(super) const fn ordinal(self) -> u32 {
        self.ordinal
    }

    pub(super) const fn binding(self) -> BindingRefV1 {
        self.binding
    }

    pub(super) const fn recipe_value(
        self,
    ) -> crate::mir::loop_recipe_contract::LoopValueKeyV1 {
        self.recipe_value
    }

    pub(super) const fn value(self) -> ValueId {
        self.value
    }
}

pub(super) fn open(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    builder: &mut MirBuilder,
    relation: &DynamicAPrimeI64SourceRelationViewV1<'_>,
    targets: &DynamicV2PhysicalTargetSetV1,
    brand: &DynamicV2PhysicalSessionBrandV1,
) -> Result<DynamicV2OpenedFormalHeaderV1, String> {
    let rows = relation.formal_rows();
    for (index, row) in rows.iter().copied().enumerate() {
        let expected = u32::try_from(index)
            .map_err(|_| "[freeze:contract][formal_header/ordinal_overflow]".to_owned())?;
        if row.ordinal() != expected || row.binding().owner() != relation.owner() {
            return Err("[freeze:contract][formal_header/relation_drift]".to_owned());
        }
        let expected_class = match index {
            0 | 3 => DynamicFullLoopParameterClassV2::Dynamic,
            1 | 2 => DynamicFullLoopParameterClassV2::I64,
            _ => unreachable!(),
        };
        if row.class() != expected_class {
            return Err("[freeze:contract][formal_header/class_drift]".to_owned());
        }
    }

    let (enter_target, header_target) = targets.with_enter_header(|enter, header| (enter, header));
    if !enter_target.matches(brand) || !header_target.matches(brand) {
        return Err("[freeze:contract][formal_header/target_brand]".to_owned());
    }
    let enter = enter_target.block();
    let header = header_target.block();
    if canonical.entry_block(builder)? != enter
        || relation.entry_value() != rows[1].recipe_value()
    {
        return Err("[freeze:contract][formal_header/entry_drift]".to_owned());
    }

    let mut values = [ValueId::new(0); 4];
    for row in rows.iter().copied() {
        let index = usize::try_from(row.ordinal())
            .map_err(|_| "[freeze:contract][formal_header/ordinal_overflow]".to_owned())?;
        values[index] = canonical.adopt_exact_formal_parameter(
            builder,
            row.declaration(),
            row.binding(),
            row.ordinal(),
        )?;
    }

    canonical
        .identity
        .claim_variable_use_binding(relation.initializer(), relation.pos_binding())?;
    canonical.identity.publish_declaration_exact(
        relation.induction_declaration(),
        relation.induction_binding(),
        enter,
        values[1],
    )?;

    {
        let function = builder
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| "[freeze:contract][formal_header/function_missing]".to_owned())?;
        canonical
            .cfg
            .emit_jump(function, enter, header)
            .map_err(|error| error.to_string())?;
    }
    canonical
        .cfg
        .select_block(builder, header)
        .map_err(|error| error.to_string())?;
    let header_current = canonical
        .identity
        .read_entry_receipt(builder, &mut canonical.phis, header, relation.induction_binding())?;
    if header_current.owner() != relation.owner()
        || header_current.binding() != relation.induction_binding()
        || header_current.physical_block() != header
    {
        return Err("[freeze:contract][formal_header/header_receipt_drift]".to_owned());
    }

    let formals = std::array::from_fn(|index| DynamicV2FormalSeedV1 {
        ordinal: rows[index].ordinal(),
        binding: rows[index].binding(),
        recipe_value: rows[index].recipe_value(),
        value: values[index],
    });
    Ok(DynamicV2OpenedFormalHeaderV1 {
        enter,
        header,
        formals,
        header_current,
    })
}
