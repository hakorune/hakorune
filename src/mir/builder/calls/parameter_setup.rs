//! Function-entry parameter identity owner.
//!
//! Static parameters, instance receivers, and instance parameters all publish
//! their ValueId and BindingId through one API. `function_param_names` remains
//! observation inventory; it is never assignment authority.

use crate::mir::builder::{MirBuilder, MirType};
use crate::mir::resolved_semantics::{BindingKindV1, SourceBindingSiteV1};
use hakorune_mir_core::{BindingId, MirValueKind, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FunctionParameterKind {
    Receiver,
    Explicit,
}

impl MirBuilder {
    pub(super) fn setup_function_params(&mut self, params: &[String]) -> Result<(), String> {
        self.scope_ctx.function_param_names.clear();
        let entries = {
            let Some(function) = self.scope_ctx.current_function.as_mut() else {
                return Err("[type/parameter_binding_identity_missing] function=<none>".to_string());
            };
            let receiver_offset = usize::from(function.params.len() > params.len());
            let param_types = function.signature.params.clone();
            params
                .iter()
                .enumerate()
                .map(|(source_index, name)| {
                    let formal_index = receiver_offset + source_index;
                    let value = if formal_index < function.params.len() {
                        function.params[formal_index]
                    } else {
                        let value = function.next_value_id();
                        function.params.push(value);
                        value
                    };
                    (
                        name.clone(),
                        value,
                        formal_index,
                        param_types.get(formal_index).cloned(),
                    )
                })
                .collect::<Vec<_>>()
        };

        for (name, value, formal_index, ty) in entries {
            self.declare_function_parameter(
                &name,
                value,
                formal_index,
                ty,
                FunctionParameterKind::Explicit,
                None,
            )?;
        }
        Ok(())
    }

    pub(super) fn setup_method_params(
        &mut self,
        box_name: &str,
        params: &[String],
    ) -> Result<(), String> {
        self.scope_ctx.function_param_names.clear();
        let me_type = MirType::Box(box_name.to_string());
        let entries = {
            let Some(function) = self.scope_ctx.current_function.as_mut() else {
                return Err("[type/parameter_binding_identity_missing] function=<none>".to_string());
            };
            let required_count = params.len() + 1;
            while function.params.len() < required_count {
                let value = function.next_value_id();
                function.params.push(value);
            }
            let param_types = function.signature.params.clone();
            let mut entries = Vec::with_capacity(required_count);
            entries.push((
                "me".to_string(),
                function.params[0],
                0,
                Some(me_type.clone()),
                FunctionParameterKind::Receiver,
            ));
            for (source_index, name) in params.iter().enumerate() {
                let formal_index = source_index + 1;
                entries.push((
                    name.clone(),
                    function.params[formal_index],
                    formal_index,
                    param_types.get(formal_index).cloned(),
                    FunctionParameterKind::Explicit,
                ));
            }
            entries
        };

        for (name, value, formal_index, ty, kind) in entries {
            let receiver_box =
                matches!(kind, FunctionParameterKind::Receiver).then_some(box_name.to_string());
            self.declare_function_parameter(&name, value, formal_index, ty, kind, receiver_box)?;
        }
        Ok(())
    }

    fn declare_function_parameter(
        &mut self,
        name: &str,
        value: ValueId,
        formal_index: usize,
        ty: Option<MirType>,
        kind: FunctionParameterKind,
        receiver_box: Option<String>,
    ) -> Result<BindingId, String> {
        self.ensure_function_parameter_available(name, formal_index)?;
        let binding_id = self.allocate_binding_id();
        self.publish_function_parameter(
            name,
            value,
            binding_id,
            formal_index,
            ty,
            kind,
            receiver_box,
        )
    }

    #[allow(dead_code)]
    fn declare_resolved_function_parameter(
        &mut self,
        site: &SourceBindingSiteV1,
        resolved_kind: BindingKindV1,
        name: &str,
        value: ValueId,
        formal_index: usize,
        ty: Option<MirType>,
        kind: FunctionParameterKind,
        receiver_box: Option<String>,
    ) -> Result<BindingId, String> {
        self.ensure_function_parameter_available(name, formal_index)?;
        let claim = self
            .resolved_binding_state
            .claim_declaration(site, resolved_kind, name)?;
        let binding_id = self.publish_function_parameter(
            name,
            value,
            claim.binding_id(),
            formal_index,
            ty,
            kind,
            receiver_box,
        )?;
        self.resolved_binding_state
            .publish_declared_value(claim, value)?;
        Ok(binding_id)
    }

    fn ensure_function_parameter_available(
        &self,
        name: &str,
        formal_index: usize,
    ) -> Result<(), String> {
        if self.variable_ctx.variable_map.contains_key(name) || self.binding_ctx.contains(name) {
            return Err(format!(
                "[type/parameter_binding_identity_duplicate] name={} formal_index={}",
                name, formal_index
            ));
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    fn publish_function_parameter(
        &mut self,
        name: &str,
        value: ValueId,
        binding_id: BindingId,
        formal_index: usize,
        ty: Option<MirType>,
        kind: FunctionParameterKind,
        receiver_box: Option<String>,
    ) -> Result<BindingId, String> {
        self.variable_ctx
            .variable_map
            .insert(name.to_string(), value);
        self.binding_ctx.insert(name.to_string(), binding_id);
        self.scope_ctx.function_param_names.insert(name.to_string());
        self.register_value_kind(value, MirValueKind::Parameter(formal_index as u32));
        if let Some(ty) = ty.clone() {
            self.type_ctx.value_types.insert(value, ty);
        }
        if let Some(box_name) = receiver_box {
            self.type_ctx.value_origin_newbox.insert(value, box_name);
        }
        if let Some(registry) = self.comp_ctx.current_slot_registry.as_mut() {
            registry.ensure_slot(name, ty);
        }

        debug_assert_eq!(
            matches!(kind, FunctionParameterKind::Receiver),
            name == "me",
            "receiver identity must be published as me"
        );
        Ok(binding_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameter_publication_is_atomic() {
        let mut builder = MirBuilder::new();
        let binding = builder
            .declare_function_parameter(
                "arg",
                ValueId::new(4),
                0,
                Some(MirType::Integer),
                FunctionParameterKind::Explicit,
                None,
            )
            .expect("parameter declaration");

        assert_eq!(
            builder.variable_ctx.variable_map.get("arg"),
            Some(&ValueId::new(4))
        );
        assert_eq!(builder.binding_ctx.lookup("arg"), Some(binding));
        assert!(builder.scope_ctx.function_param_names.contains("arg"));
        assert_eq!(
            builder.get_value_kind(ValueId::new(4)),
            Some(MirValueKind::Parameter(0))
        );
    }

    #[test]
    fn receiver_uses_the_same_identity_owner() {
        let mut builder = MirBuilder::new();
        let binding = builder
            .declare_function_parameter(
                "me",
                ValueId::new(0),
                0,
                Some(MirType::Box("Counter".to_string())),
                FunctionParameterKind::Receiver,
                Some("Counter".to_string()),
            )
            .expect("receiver declaration");

        assert_eq!(builder.binding_ctx.lookup("me"), Some(binding));
        assert_eq!(
            builder.type_ctx.value_origin_newbox.get(&ValueId::new(0)),
            Some(&"Counter".to_string())
        );
    }

    #[test]
    fn duplicate_parameter_publication_fails() {
        let mut builder = MirBuilder::new();
        builder
            .declare_function_parameter(
                "arg",
                ValueId::new(0),
                0,
                None,
                FunctionParameterKind::Explicit,
                None,
            )
            .expect("first declaration");
        let error = builder
            .declare_function_parameter(
                "arg",
                ValueId::new(1),
                1,
                None,
                FunctionParameterKind::Explicit,
                None,
            )
            .expect_err("duplicate declaration must fail");
        assert!(error.starts_with("[type/parameter_binding_identity_duplicate]"));
    }
}
