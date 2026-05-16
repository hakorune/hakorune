use crate::ast::ASTNode;

use super::{EffectMask, MirInstruction, MirType, ValueId};

impl super::MirBuilder {
    fn infer_index_target_class(&self, target_val: ValueId) -> Option<String> {
        if let Some(cls) = self.type_ctx.value_origin_newbox.get(&target_val) {
            return Some(cls.clone());
        }
        self.type_ctx
            .value_types
            .get(&target_val)
            .and_then(|ty| match ty {
                MirType::Box(name) => Some(name.clone()),
                MirType::String => Some("String".to_string()),
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

    pub(super) fn build_index_expression(
        &mut self,
        target: ASTNode,
        index: ASTNode,
    ) -> Result<ValueId, String> {
        if let ASTNode::Variable { name, .. } = &target {
            if let Some(plan) = self
                .current_module
                .as_ref()
                .and_then(|module| {
                    crate::mir::static_data_plan::find_static_data_plan(
                        &module.metadata.static_data_plans,
                        name,
                    )
                })
                .cloned()
            {
                if plan.element != "u16" {
                    return Err(format!(
                        "[static-const/load-unsupported-element] {} element={}",
                        plan.source_name, plan.element
                    ));
                }
                let index_val = self.build_expression(index)?;
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
                if let Some(func) = self.scope_ctx.current_function.as_mut() {
                    func.metadata.value_types.insert(dst, MirType::Integer);
                }
                self.type_ctx.value_types.insert(dst, MirType::Integer);
                return Ok(dst);
            }
        }

        let target_val = self.build_expression(target)?;
        let class_hint = self.infer_index_target_class(target_val);

        match class_hint.as_deref() {
            Some("ArrayBox") => {
                let index_val = self.build_expression(index)?;
                let dst = self.next_value_id();
                self.emit_box_or_plugin_call(
                    Some(dst),
                    target_val,
                    "get".to_string(),
                    None,
                    vec![index_val],
                    EffectMask::READ,
                )?;
                Ok(dst)
            }
            Some("MapBox") => {
                let index_val = self.build_expression(index)?;
                let dst = self.next_value_id();
                self.emit_box_or_plugin_call(
                    Some(dst),
                    target_val,
                    "get".to_string(),
                    None,
                    vec![index_val],
                    EffectMask::READ,
                )?;
                Ok(dst)
            }
            _ => Err(format!(
                "index operator is only supported for Array/Map (found {})",
                Self::format_index_target_kind(class_hint.as_ref())
            )),
        }
    }

    pub(super) fn build_index_assignment(
        &mut self,
        target: ASTNode,
        index: ASTNode,
        value: ASTNode,
    ) -> Result<ValueId, String> {
        let target_val = self.build_expression(target)?;
        let class_hint = self.infer_index_target_class(target_val);

        match class_hint.as_deref() {
            Some("ArrayBox") => {
                let index_val = self.build_expression(index)?;
                let value_val = self.build_expression(value)?;
                self.emit_box_or_plugin_call(
                    None,
                    target_val,
                    "set".to_string(),
                    None,
                    vec![index_val, value_val],
                    EffectMask::MUT,
                )?;
                Ok(value_val)
            }
            Some("MapBox") => {
                let index_val = self.build_expression(index)?;
                let value_val = self.build_expression(value)?;
                self.emit_box_or_plugin_call(
                    None,
                    target_val,
                    "set".to_string(),
                    None,
                    vec![index_val, value_val],
                    EffectMask::MUT,
                )?;
                Ok(value_val)
            }
            _ => Err(format!(
                "index assignment is only supported for Array/Map (found {})",
                Self::format_index_target_kind(class_hint.as_ref())
            )),
        }
    }
}
