use crate::ast::ASTNode;
use crate::mir::builder::recursive_child_lowering::{
    drive_legacy_expression_v1, RecursiveChildLoweringPortV1,
};
use crate::mir::instruction::FastMemRegionId;
use crate::mir::instruction::MemOpKind;

use super::{EffectMask, MirInstruction, MirType, ValueId};

mod static_load_type;

use static_load_type::PreparedStaticU16LoadTypeV1;

/// One source-only Index-read route selected before child lowering or Builder
/// effects. The route vocabulary stays private so callers can only consume the
/// complete preparation through the Index owner.
pub(in crate::mir::builder) struct PreparedRawIndexReadV1 {
    route: PreparedRawIndexReadRouteV1,
}

enum PreparedRawIndexReadRouteV1 {
    Static {
        plan: crate::mir::function::StaticDataPlan,
        result_type: PreparedStaticU16LoadTypeV1,
        index: ASTNode,
    },
    Dynamic {
        target: ASTNode,
        index: ASTNode,
        target_label: Option<String>,
    },
}

/// Source-only Index-assignment snapshot prepared before child descent.
pub(in crate::mir::builder) struct PreparedRawIndexAssignmentV1 {
    target: ASTNode,
    index: ASTNode,
    value: ASTNode,
    target_label: Option<String>,
}

impl PreparedRawIndexAssignmentV1 {
    pub(in crate::mir::builder) fn prepare(
        target: ASTNode,
        index: ASTNode,
        value: ASTNode,
    ) -> Self {
        let target_label = match &target {
            ASTNode::Variable { name, .. } => Some(name.clone()),
            _ => None,
        };
        Self {
            target,
            index,
            value,
            target_label,
        }
    }
}

pub(in crate::mir::builder) fn lower_prepared_raw_index_assignment_with_port_v1<Port>(
    builder: &mut super::MirBuilder,
    port: &mut Port,
    prepared: PreparedRawIndexAssignmentV1,
) -> Result<ValueId, String>
where
    Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
{
    let PreparedRawIndexAssignmentV1 {
        target,
        index,
        value,
        target_label,
    } = prepared;
    let target_val = drive_legacy_expression_v1(builder, port, target)?;
    let index_val = drive_legacy_expression_v1(builder, port, index)?;
    let value_val = drive_legacy_expression_v1(builder, port, value)?;
    builder.build_index_access_from_values(
        None,
        target_val,
        index_val,
        target_label,
        "store",
        Some(value_val),
    )
}

impl PreparedRawIndexReadV1 {
    pub(in crate::mir::builder) fn prepare(
        builder: &super::MirBuilder,
        target: ASTNode,
        index: ASTNode,
    ) -> Result<Self, String> {
        let (static_plan, target_label) = match &target {
            ASTNode::Variable { name, .. } => {
                let static_plan = builder.current_module.as_ref().and_then(|module| {
                    crate::mir::static_data_plan::find_static_data_plan(
                        &module.metadata.static_data_plans,
                        name,
                    )
                    .cloned()
                });
                let target_label = static_plan.is_none().then(|| name.clone());
                (static_plan, target_label)
            }
            _ => (None, None),
        };

        let route = match static_plan {
            Some(plan) => {
                let result_type = PreparedStaticU16LoadTypeV1::prepare(&plan, None)
                    .map_err(|error| error.to_string())?;
                PreparedRawIndexReadRouteV1::Static {
                    plan,
                    result_type,
                    index,
                }
            }
            None => PreparedRawIndexReadRouteV1::Dynamic {
                target,
                index,
                target_label,
            },
        };
        Ok(Self { route })
    }
}

impl super::MirBuilder {
    pub(super) fn infer_index_target_class(&self, target_val: ValueId) -> Option<String> {
        if let Some(cls) = self
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&target_val)
        {
            return Some(cls.clone());
        }
        self.function_state
            .type_ctx
            .value_types
            .get(&target_val)
            .and_then(|ty| match ty {
                MirType::Box(name) => Some(name.clone()),
                MirType::String => Some("StringBox".to_string()),
                MirType::Integer => Some("Integer".to_string()),
                MirType::Float => Some("Float".to_string()),
                _ => None,
            })
    }

    fn format_index_target_kind(class_hint: Option<&String>) -> String {
        class_hint
            .map(|s| s.as_str())
            .filter(|s| !s.is_empty())
            .unwrap_or("unknown")
            .to_string()
    }

    pub(super) fn build_index_access_from_values(
        &mut self,
        region: Option<FastMemRegionId>,
        target_val: ValueId,
        index_val: ValueId,
        target_label: Option<String>,
        access_kind: &'static str,
        store_value: Option<ValueId>,
    ) -> Result<ValueId, String> {
        let region = region.or_else(|| self.current_fastmem_region());
        let table_id = target_label
            .clone()
            .or_else(|| region.map(|_| format!("value_{}", target_val.0)));
        let class_hint = self.infer_index_target_class(target_val);
        let required_route = if region.is_some() {
            "verified_table_index"
        } else {
            "none"
        };
        let fallback_policy = if region.is_some() {
            "forbidden"
        } else {
            "allow_dynamic"
        };
        self.record_index_access_site(
            region,
            target_val,
            index_val,
            table_id.clone(),
            None,
            access_kind,
            required_route,
            fallback_policy,
        )?;

        match class_hint.as_deref() {
            Some("ArrayBox") => {
                if let Some(value_id) = store_value {
                    self.emit_array_element_write(
                        None,
                        crate::mir::ArrayElementWriteKind::Set,
                        if access_kind == "compound_store" {
                            crate::mir::ArrayWriteProducerKind::CompoundIndexAssignment
                        } else {
                            crate::mir::ArrayWriteProducerKind::IndexAssignment
                        },
                        target_val,
                        Some(index_val),
                        value_id,
                    )?;
                    return Ok(value_id);
                }
                let dst = if store_value.is_some() {
                    None
                } else {
                    Some(self.next_value_id())
                };
                let value_id = match store_value {
                    Some(value_id) => value_id,
                    None => dst.expect("dst must exist for load"),
                };
                self.emit_box_or_plugin_call(
                    dst,
                    target_val,
                    if store_value.is_some() {
                        "set".to_string()
                    } else {
                        "get".to_string()
                    },
                    None,
                    if store_value.is_some() {
                        vec![index_val, value_id]
                    } else {
                        vec![index_val]
                    },
                    if store_value.is_some() {
                        EffectMask::MUT
                    } else {
                        EffectMask::READ
                    },
                )?;
                Ok(value_id)
            }
            Some("MapBox") => {
                let dst = if store_value.is_some() {
                    None
                } else {
                    Some(self.next_value_id())
                };
                let value_id = match store_value {
                    Some(value_id) => value_id,
                    None => dst.expect("dst must exist for load"),
                };
                self.emit_box_or_plugin_call(
                    dst,
                    target_val,
                    if store_value.is_some() {
                        "set".to_string()
                    } else {
                        "get".to_string()
                    },
                    None,
                    if store_value.is_some() {
                        vec![index_val, value_id]
                    } else {
                        vec![index_val]
                    },
                    if store_value.is_some() {
                        EffectMask::MUT
                    } else {
                        EffectMask::READ
                    },
                )?;
                Ok(value_id)
            }
            _ if region.is_some() => {
                let slot = self.emit_fastmem_value_memop_with_access(
                    region.expect("region required"),
                    MemOpKind::TableIndex,
                    vec![target_val, index_val],
                    Some(crate::mir::instruction::MemOpAccess::table(
                        table_id.unwrap_or_else(|| format!("value_{}", target_val.0)),
                    )),
                )?;
                if let Some(value_id) = store_value {
                    self.emit_fastmem_memop(
                        region.expect("region required"),
                        crate::mir::instruction::MemOpKind::FieldStore,
                        None,
                        vec![slot, value_id],
                        None,
                    )?;
                    Ok(value_id)
                } else {
                    Ok(slot)
                }
            }
            _ => Err(format!(
                "index operator is only supported for Array/Map (found {})",
                Self::format_index_target_kind(class_hint.as_ref())
            )),
        }
    }

    /// Consume one already selected Index-read route. Static-data validation
    /// has completed before this terminal opens either child descent.
    pub(in crate::mir::builder) fn lower_prepared_raw_index_read_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        prepared: PreparedRawIndexReadV1,
    ) -> Result<ValueId, String>
    where
        Port: RecursiveChildLoweringPortV1<ExpressionInput = ASTNode>,
    {
        match prepared.route {
            PreparedRawIndexReadRouteV1::Static {
                plan,
                result_type: prepared,
                index,
            } => {
                let index_val = drive_legacy_expression_v1(self, port, index)?;
                let dst = self.next_value_id();
                self.emit_instruction(MirInstruction::StaticDataLoad {
                    dst,
                    source_name: plan.source_name,
                    symbol: plan.symbol,
                    element: plan.element,
                    len: plan.values.len() as u32,
                    align: plan.align,
                    index: index_val,
                })?;
                prepared.commit(dst, &mut self.function_state.type_ctx);
                Ok(dst)
            }
            PreparedRawIndexReadRouteV1::Dynamic {
                target,
                index,
                target_label,
            } => {
                let target_val = drive_legacy_expression_v1(self, port, target)?;
                let index_val = drive_legacy_expression_v1(self, port, index)?;
                self.build_index_access_from_values(
                    None,
                    target_val,
                    index_val,
                    target_label,
                    "load",
                    None,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests;
