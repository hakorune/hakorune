//! Field-assignment admission and physical emission; read lowering stays in the parent.

use crate::ast::ASTNode;
use crate::mir::builder::MirBuilder;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RawAstChildLoweringPortV1, RawLegacyChildLoweringPortV1,
};
use crate::mir::builder::weak_field_write_route::{
    prepare_field_write_route_v1, PreparedFieldWriteRouteV1,
};
use crate::mir::instruction::FastMemRegionId;
use crate::mir::ValueId;
use super::store_post_success::PreparedOrdinaryFieldStoreAccessSiteV1;

/// Ordinary Field-assignment source and record-target admission prepared once.
pub(in crate::mir::builder) struct PreparedRawFieldAssignmentV1 {
    object: ASTNode,
    field: String,
    value: ASTNode,
}

impl PreparedRawFieldAssignmentV1 {
    pub(in crate::mir::builder) fn prepare(
        builder: &MirBuilder,
        object: ASTNode,
        field: String,
        value: ASTNode,
    ) -> Result<Self, String> {
        builder.fail_if_record_field_assignment_target(&object, &field)?;
        Ok(Self {
            object,
            field,
            value,
        })
    }
}

pub(in crate::mir::builder) fn lower_prepared_raw_field_assignment_with_port_v1<Port>(
    builder: &mut MirBuilder,
    port: &mut Port,
    prepared: PreparedRawFieldAssignmentV1,
) -> Result<ValueId, String>
where
    Port: RawAstChildLoweringPortV1,
{
    let PreparedRawFieldAssignmentV1 {
        object,
        field,
        value,
    } = prepared;
    let object_value = drive_legacy_expression_v1(builder, port, object)?;
    let object_value = builder.local_field_base(object_value);
    builder.build_field_assignment_from_value_with_port_v1(port, object_value, field, value)
}

impl MirBuilder {
    pub(in crate::mir::builder) fn build_field_assignment_from_value(
        &mut self,
        object_value: ValueId,
        field: String,
        value: ASTNode,
    ) -> Result<ValueId, String> {
        let mut port = RawLegacyChildLoweringPortV1;
        self.build_field_assignment_from_value_with_port_v1(&mut port, object_value, field, value)
    }

    pub(super) fn build_field_assignment_from_value_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        object_value: ValueId,
        field: String,
        value: ASTNode,
    ) -> Result<ValueId, String>
    where
        Port: RawAstChildLoweringPortV1,
    {
        let mut value_result = drive_legacy_expression_v1(self, port, value)?;
        value_result = self.local_arg(value_result);
        self.build_field_assignment_from_value_id(
            self.current_fastmem_region(),
            object_value,
            field,
            value_result,
        )
    }

    pub(in crate::mir::builder) fn build_field_assignment_from_value_id(
        &mut self,
        region: Option<FastMemRegionId>,
        object_value: ValueId,
        field: String,
        value_result: ValueId,
    ) -> Result<ValueId, String> {
        let region = region.or_else(|| self.current_fastmem_region());
        let receiver_box_name = self
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&object_value)
            .cloned();
        let declared_type = self.declared_field_type_for_value(object_value, &field);

        let route = prepare_field_write_route_v1(
            region,
            object_value,
            &field,
            value_result,
            receiver_box_name.as_deref(),
            receiver_box_name
                .as_ref()
                .and_then(|owner| self.comp_ctx.user_box_field_decls.get(owner))
                .map(Vec::as_slice),
        );
        let contract_identity = self.declared_field_contract_identity(object_value, &field);
        let is_typed_array = contract_identity
            .as_ref()
            .and_then(|(_, _, declared)| declared.as_deref())
            .map(crate::typed_array_contract_spec::parse_annotation)
            .transpose()?
            .flatten()
            .is_some();
        let contract_identity = is_typed_array.then_some(contract_identity).flatten();
        let has_contract_identity = contract_identity.is_some();
        let mut is_known_weak = false;
        let ordinary_receipt = match route {
            PreparedFieldWriteRouteV1::KnownWeak(prepared) => {
                is_known_weak = true;
                self.record_field_access_site(
                    region,
                    object_value,
                    receiver_box_name.clone(),
                    field.clone(),
                    None,
                    "store",
                    if region.is_some() {
                        "verified_layout_field"
                    } else {
                        "none"
                    },
                    if region.is_some() {
                        "forbidden"
                    } else {
                        "allow_dynamic"
                    },
                )?;
                self.emit_prepared_known_weak_field_write(prepared)?;
                None
            }
            PreparedFieldWriteRouteV1::Ordinary(prepared) => {
                if region.is_none() && contract_identity.is_none() {
                    Some(PreparedOrdinaryFieldStoreAccessSiteV1::prepare(
                        self.metadata_ctx.current_span(),
                        prepared.base,
                        receiver_box_name.as_deref(),
                        &prepared.field,
                    ))
                } else {
                    self.record_field_access_site(
                        region,
                        object_value,
                        receiver_box_name.clone(),
                        field.clone(),
                        None,
                        "store",
                        if region.is_some() {
                            "verified_layout_field"
                        } else {
                            "none"
                        },
                        if region.is_some() {
                            "forbidden"
                        } else {
                            "allow_dynamic"
                        },
                    )?;
                    None
                }
            }
        };

        if let Some((box_name, field_index, declared_name)) = contract_identity.clone() {
            let function = self
                .function_state
                .current_function
                .as_mut()
                .ok_or_else(|| {
                    "[type/typed_array_contract_carrier_missing] function=<none>".to_string()
                })?;
            let contract_id = crate::mir::type_contracts::typed_array::register_instruction_source(
                function,
                crate::mir::function::TypedArrayContractBoundary::BoxFieldWrite,
                crate::mir::function::TypedArrayContractSourceIdentity::BoxField {
                    box_name,
                    field_index,
                },
                value_result,
                declared_name.as_deref(),
                &format!(
                    "box-field:{}:{}:{}",
                    object_value.as_u32(),
                    field,
                    value_result.as_u32()
                ),
            )?;
            if let Some(contract_id) = contract_id {
                self.emit_instruction(crate::mir::MirInstruction::ArrayStateContractClaim {
                    contract_id,
                    array: value_result,
                })?;
            }
        }

        if is_known_weak {
            // WeakFieldWrite owns validation, publication, and bookkeeping.
        } else if ordinary_receipt.is_none() && region.is_none() && !has_contract_identity {
            // The prepared ordinary receipt is committed below after FieldSet.
        } else if has_contract_identity {
            // The typed-array contract lane owns its existing FieldSet timing.
        } else if let Some(region) = region {
            self.emit_fastmem_memop(
                region,
                crate::mir::instruction::MemOpKind::FieldStore,
                None,
                vec![object_value, value_result],
                Some(crate::mir::instruction::MemOpAccess::field(field.clone())),
            )?;
        } else {
            self.emit_instruction(crate::mir::MirInstruction::FieldSet {
                base: object_value,
                field: field.clone(),
                value: value_result,
                declared_type,
            })?;
        }

        if let Some(receipt) = ordinary_receipt {
            receipt.commit(self)?;
        }

        // Record origin class for this field value if known
        if let Some(val_cls) = self
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&value_result)
            .cloned()
        {
            self.comp_ctx
                .field_origin_class
                .insert((object_value, field.clone()), val_cls.clone());
            // Also record class-level mapping if base object class is known
            if let Some(base_cls) = self
                .function_state
                .type_ctx
                .value_origin_newbox
                .get(&object_value)
                .cloned()
            {
                self.comp_ctx
                    .field_origin_by_box
                    .insert((base_cls, field.clone()), val_cls);
            }
        }

        Ok(value_result)
    }
}
