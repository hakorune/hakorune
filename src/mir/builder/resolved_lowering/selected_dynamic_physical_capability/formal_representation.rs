use crate::mir::a_prime_i64_formal_representation::APrimeI64FormalPhysicalRepresentationProjectionV1;
use crate::mir::compiler::dynamic_full_body_recipe::{
    DynamicAPrimeI64SourceRelationViewV1, DynamicFullLoopParameterClassV2,
};
use crate::mir::loop_recipe_contract::LoopValueKeyV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::ValueId;

use super::DynamicV2PhysicalRepresentationV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicV2APrimeFormalRepresentationRejectV1 {
    Ordinal,
    Binding,
    RecipeValue,
    Class,
    Representation,
    Owner,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct DynamicV2APrimeFormalRepresentationRowV1 {
    ordinal: u32,
    binding: BindingRefV1,
    recipe_value: LoopValueKeyV1,
    class: DynamicFullLoopParameterClassV2,
    representation: DynamicV2PhysicalRepresentationV1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) struct DynamicV2APrimeFormalRepresentationPairV1 {
    pos: DynamicV2APrimeFormalRepresentationRowV1,
    end: DynamicV2APrimeFormalRepresentationRowV1,
}

impl DynamicV2APrimeFormalRepresentationPairV1 {
    pub(in crate::mir) fn issue(
        relation: &DynamicAPrimeI64SourceRelationViewV1<'_>,
    ) -> Result<Self, DynamicV2APrimeFormalRepresentationRejectV1> {
        let rows = relation.formal_rows();
        let pos = rows[1];
        let end = rows[2];
        if pos.ordinal() != 1 || end.ordinal() != 2 {
            return Err(DynamicV2APrimeFormalRepresentationRejectV1::Ordinal);
        }
        if pos.binding().owner() != relation.owner() || end.binding().owner() != relation.owner() {
            return Err(DynamicV2APrimeFormalRepresentationRejectV1::Owner);
        }
        if pos.binding() != relation.pos_binding() || end.binding() != relation.end_binding() {
            return Err(DynamicV2APrimeFormalRepresentationRejectV1::Binding);
        }
        if pos.class() != DynamicFullLoopParameterClassV2::I64
            || end.class() != DynamicFullLoopParameterClassV2::I64
        {
            return Err(DynamicV2APrimeFormalRepresentationRejectV1::Class);
        }
        Ok(Self {
            pos: DynamicV2APrimeFormalRepresentationRowV1 {
                ordinal: pos.ordinal(),
                binding: pos.binding(),
                recipe_value: pos.recipe_value(),
                class: pos.class(),
                representation: DynamicV2PhysicalRepresentationV1::ImmediateI64,
            },
            end: DynamicV2APrimeFormalRepresentationRowV1 {
                ordinal: end.ordinal(),
                binding: end.binding(),
                recipe_value: end.recipe_value(),
                class: end.class(),
                representation: DynamicV2PhysicalRepresentationV1::ImmediateI64,
            },
        })
    }

    pub(in crate::mir) fn adopt_after_formals(
        self,
        relation: &DynamicAPrimeI64SourceRelationViewV1<'_>,
        values: [ValueId; 4],
        owner: FunctionOwnerIdV1,
    ) -> Result<
        APrimeI64FormalPhysicalRepresentationProjectionV1,
        DynamicV2APrimeFormalRepresentationRejectV1,
    > {
        let rows = relation.formal_rows();
        let expected = [(&self.pos, rows[1]), (&self.end, rows[2])];
        for (physical, source) in expected {
            if physical.ordinal != source.ordinal()
                || physical.binding != source.binding()
                || physical.recipe_value != source.recipe_value()
            {
                return Err(DynamicV2APrimeFormalRepresentationRejectV1::RecipeValue);
            }
            if physical.class != source.class()
                || physical.representation != DynamicV2PhysicalRepresentationV1::ImmediateI64
                || physical.binding.owner() != owner
            {
                return Err(DynamicV2APrimeFormalRepresentationRejectV1::Representation);
            }
        }
        Ok(
            APrimeI64FormalPhysicalRepresentationProjectionV1::from_adopted_exact_i64(
                owner,
                (
                    self.pos.ordinal,
                    self.pos.binding,
                    self.pos.recipe_value,
                    values[1],
                ),
                (
                    self.end.ordinal,
                    self.end.binding,
                    self.end.recipe_value,
                    values[2],
                ),
            ),
        )
    }
}
