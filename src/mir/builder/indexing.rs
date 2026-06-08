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
                self.record_index_access_site(
                    None,
                    target_val,
                    index_val,
                    None,
                    None,
                    "load",
                    "none",
                    "allow_dynamic",
                )?;
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
                self.record_index_access_site(
                    None,
                    target_val,
                    index_val,
                    None,
                    None,
                    "load",
                    "none",
                    "allow_dynamic",
                )?;
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
                self.record_index_access_site(
                    None,
                    target_val,
                    index_val,
                    None,
                    None,
                    "store",
                    "none",
                    "allow_dynamic",
                )?;
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
                self.record_index_access_site(
                    None,
                    target_val,
                    index_val,
                    None,
                    None,
                    "store",
                    "none",
                    "allow_dynamic",
                )?;
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
