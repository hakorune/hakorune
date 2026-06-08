use crate::ast::ASTNode;
use crate::mir::instruction::FastMemRegionId;
use crate::mir::instruction::MemOpKind;

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

    pub(super) fn build_index_access_from_values(
        &mut self,
        region: Option<FastMemRegionId>,
        target_val: ValueId,
        index_val: ValueId,
        target_label: Option<String>,
        access_kind: &'static str,
        store_value: Option<ValueId>,
    ) -> Result<ValueId, String> {
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
            target_label.clone(),
            None,
            access_kind,
            required_route,
            fallback_policy,
        )?;

        match class_hint.as_deref() {
            Some("ArrayBox") => {
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
            _ if region.is_some() && target_label.is_some() => {
                let slot = self.emit_fastmem_value_memop_with_access(
                    region.expect("region required"),
                    MemOpKind::TableIndex,
                    vec![target_val, index_val],
                    Some(crate::mir::instruction::MemOpAccess::table(
                        target_label.expect("table label required"),
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
        let index_val = self.build_expression(index)?;
        self.build_index_access_from_values(None, target_val, index_val, None, "load", None)
    }

    pub(super) fn build_index_assignment(
        &mut self,
        target: ASTNode,
        index: ASTNode,
        value: ASTNode,
    ) -> Result<ValueId, String> {
        let target_val = self.build_expression(target)?;
        let index_val = self.build_expression(index)?;
        let value_val = self.build_expression(value)?;
        self.build_index_access_from_values(
            None,
            target_val,
            index_val,
            None,
            "store",
            Some(value_val),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::super::MirBuilder;
    use super::MirType;
    use crate::ast::{ASTNode, LiteralValue, Span};

    fn span() -> Span {
        Span::unknown()
    }

    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: span(),
        }
    }

    fn int_lit(value: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(value),
            span: span(),
        }
    }

    fn index(target: ASTNode, idx: ASTNode) -> ASTNode {
        ASTNode::Index {
            target: Box::new(target),
            index: Box::new(idx),
            span: span(),
        }
    }

    fn assign(target: ASTNode, value: ASTNode) -> ASTNode {
        ASTNode::Assignment {
            target: Box::new(target),
            value: Box::new(value),
            span: span(),
        }
    }

    fn local(name: &str, value: ASTNode) -> ASTNode {
        ASTNode::Local {
            variables: vec![name.to_string()],
            initial_values: vec![Some(Box::new(value))],
            declared_type_names: Vec::new(),
            span: span(),
        }
    }

    #[test]
    fn ordinary_index_access_records_site_metadata() {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("ordinary_index_access/0".to_string());
        let page_table_id = builder.alloc_value_for_test();
        builder
            .variable_ctx
            .variable_map
            .insert("page_table".to_string(), page_table_id);
        builder
            .type_ctx
            .value_types
            .insert(page_table_id, MirType::Box("ArrayBox".to_string()));

        let body = vec![
            local("key", int_lit(3)),
            local("loaded", index(var("page_table"), var("key"))),
            assign(index(var("page_table"), var("key")), int_lit(42)),
        ];

        super::super::stmts::block_stmt::build_block(&mut builder, body).unwrap();
        let function = builder.scope_ctx.current_function.as_ref().unwrap();

        assert_eq!(function.metadata.fastmem_index_access_sites.len(), 2);
        assert!(function
            .metadata
            .fastmem_index_access_sites
            .iter()
            .all(|site| site.region.is_none()));
        assert_eq!(
            function.metadata.fastmem_index_access_sites[0].required_route,
            "none"
        );
        assert_eq!(
            function.metadata.fastmem_index_access_sites[0].fallback_policy,
            "allow_dynamic"
        );
        assert_eq!(
            function.metadata.fastmem_index_access_sites[0].access_kind,
            "load"
        );
        assert_eq!(
            function.metadata.fastmem_index_access_sites[1].access_kind,
            "store"
        );
    }
}
