//! Function-entry parameter identity owner.
//!
//! Static parameters, instance receivers, and instance parameters all publish
//! their ValueId and BindingId through one API. `function_param_names` remains
//! observation inventory; it is never assignment authority.

use crate::mir::builder::{MirBuilder, MirType};
use hakorune_mir_builder::lowering_facts::{PreparedTypeFactPublicationV1, TypeFactDecisionV1};
use hakorune_mir_core::{BindingId, MirValueKind, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FunctionParameterKind {
    Receiver,
    Explicit,
}

/// Existing legacy facts and the prepared receiver fact share parameter
/// identity commit, but only the receiver path consumes a monotone decision.
enum FunctionParameterTypePublicationV1 {
    Legacy(Option<MirType>),
    Prepared(PreparedTypeFactPublicationV1),
}

impl FunctionParameterTypePublicationV1 {
    fn into_type_to_publish(self) -> Option<MirType> {
        match self {
            Self::Legacy(ty) => ty,
            Self::Prepared(PreparedTypeFactPublicationV1::Publish(ty)) => Some(ty),
            Self::Prepared(
                PreparedTypeFactPublicationV1::Idempotent(_)
                | PreparedTypeFactPublicationV1::PreserveExisting(_)
                | PreparedTypeFactPublicationV1::NoPublication,
            ) => None,
        }
    }
}

/// Prepared type publication for the one admitted receiver parameter shape.
///
/// This is intentionally local to method parameter setup. It is neither a
/// general TypeContext API nor an authority for explicit/static parameters.
struct PreparedInstanceReceiverParameterV1 {
    value: ValueId,
    receiver_type: MirType,
    owner: String,
    type_publication: PreparedTypeFactPublicationV1,
}

impl MirBuilder {
    pub(in crate::mir::builder) fn setup_function_params(
        &mut self,
        params: &[String],
    ) -> Result<(), String> {
        self.function_state.scope.function_param_names.clear();
        let entries = {
            let Some(function) = self.function_state.current_function.as_mut() else {
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

    pub(in crate::mir::builder) fn setup_method_params(
        &mut self,
        box_name: &str,
        params: &[String],
    ) -> Result<(), String> {
        let prepared_receiver = self.prepare_instance_receiver_parameter(box_name)?;
        let receiver_binding = self.allocate_binding_id()?;
        self.function_state.scope.function_param_names.clear();
        let entries = {
            let Some(function) = self.function_state.current_function.as_mut() else {
                return Err("[type/parameter_binding_identity_missing] function=<none>".to_string());
            };
            let required_count = params.len() + 1;
            while function.params.len() < required_count {
                let value = function.next_value_id();
                function.params.push(value);
            }
            let param_types = function.signature.params.clone();
            let mut entries = Vec::with_capacity(required_count);
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

        self.commit_instance_receiver_parameter(prepared_receiver, receiver_binding);
        for (name, value, formal_index, ty, _kind) in entries {
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

    fn prepare_instance_receiver_parameter(
        &self,
        box_name: &str,
    ) -> Result<PreparedInstanceReceiverParameterV1, String> {
        self.ensure_function_parameter_available("me", 0)?;
        let function = self
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| {
                "[freeze:contract][lowering_facts/receiver_parameter_preflight] function=<none>"
                    .to_string()
            })?;
        let value = *function.params.first().ok_or_else(|| {
            "[freeze:contract][lowering_facts/receiver_parameter_preflight] reason=missing_param0"
                .to_string()
        })?;
        let receiver_type = function.signature.params.first().cloned().ok_or_else(|| {
            "[freeze:contract][lowering_facts/receiver_parameter_preflight] reason=missing_signature_param0"
                .to_string()
        })?;
        let owner = match &receiver_type {
            MirType::Box(owner) => owner.clone(),
            _ => {
                return Err(
                    "[freeze:contract][lowering_facts/receiver_parameter_preflight] reason=signature_param0_not_box"
                        .to_string(),
                );
            }
        };
        if owner != box_name {
            return Err(format!(
                "[freeze:contract][lowering_facts/receiver_parameter_preflight] reason=owner_mismatch signature_owner={} setup_owner={}",
                owner, box_name
            ));
        }
        let type_publication = TypeFactDecisionV1::prepare(
            self.function_state.type_ctx.value_types.get(&value),
            Some(&receiver_type),
        )
        .map_err(|error| error.to_string())?;
        Ok(PreparedInstanceReceiverParameterV1 {
            value,
            receiver_type,
            owner,
            type_publication,
        })
    }

    fn commit_instance_receiver_parameter(
        &mut self,
        prepared: PreparedInstanceReceiverParameterV1,
        binding_id: BindingId,
    ) {
        self.publish_function_parameter(
            "me",
            prepared.value,
            binding_id,
            0,
            FunctionParameterTypePublicationV1::Prepared(prepared.type_publication),
            Some(prepared.receiver_type),
            FunctionParameterKind::Receiver,
            Some(prepared.owner),
        )
        .expect("receiver preflight makes parameter commit infallible");
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
        let binding_id = self.allocate_binding_id()?;
        self.publish_function_parameter(
            name,
            value,
            binding_id,
            formal_index,
            FunctionParameterTypePublicationV1::Legacy(ty.clone()),
            ty,
            kind,
            receiver_box,
        )
    }

    fn ensure_function_parameter_available(
        &self,
        name: &str,
        formal_index: usize,
    ) -> Result<(), String> {
        if self
            .function_state
            .variable_ctx
            .variable_map
            .contains_key(name)
            || self.function_state.binding_ctx.contains(name)
        {
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
        type_publication: FunctionParameterTypePublicationV1,
        slot_type: Option<MirType>,
        kind: FunctionParameterKind,
        receiver_box: Option<String>,
    ) -> Result<BindingId, String> {
        self.function_state
            .variable_ctx
            .variable_map
            .insert(name.to_string(), value);
        self.function_state
            .binding_ctx
            .insert(name.to_string(), binding_id);
        self.function_state
            .scope
            .function_param_names
            .insert(name.to_string());
        self.register_value_kind(value, MirValueKind::Parameter(formal_index as u32));
        if let Some(ty) = type_publication.into_type_to_publish() {
            self.function_state.type_ctx.value_types.insert(value, ty);
        }
        if let Some(box_name) = receiver_box {
            self.function_state
                .type_ctx
                .value_origin_newbox
                .insert(value, box_name);
        }
        if let Some(registry) = self.comp_ctx.current_slot_registry.as_mut() {
            registry.ensure_slot(name, slot_type);
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

    const RECEIVER_OWNER: &str = "Fact0ReceiverParameterV1";

    fn method_builder(owner: &str) -> MirBuilder {
        let mut builder = MirBuilder::new();
        builder
            .create_method_skeleton(format!("{owner}.probe/0"), owner, &[], &[])
            .unwrap();
        builder
    }

    fn receiver_value(builder: &MirBuilder) -> ValueId {
        builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .params[0]
    }

    fn assert_receiver_preflight_left_state_unchanged(
        builder: &MirBuilder,
        receiver: ValueId,
        binding_before: u32,
        params_before: &[ValueId],
        signature_before: &[MirType],
    ) {
        let function = builder.function_state.current_function.as_ref().unwrap();
        assert_eq!(builder.core_ctx.next_binding_id, binding_before);
        assert_eq!(function.params, params_before);
        assert_eq!(function.signature.params, signature_before);
        assert!(builder
            .function_state
            .variable_ctx
            .variable_map
            .get("me")
            .is_none());
        assert!(builder.function_state.binding_ctx.lookup("me").is_none());
        assert!(!builder
            .function_state
            .scope
            .function_param_names
            .contains("me"));
        assert_eq!(builder.get_value_kind(receiver), None);
        assert!(builder
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&receiver)
            .is_none());
        assert!(builder
            .comp_ctx
            .current_slot_registry
            .as_ref()
            .unwrap()
            .get_slot("me")
            .is_none());
    }

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
            builder.function_state.variable_ctx.variable_map.get("arg"),
            Some(&ValueId::new(4))
        );
        assert_eq!(
            builder.function_state.binding_ctx.lookup("arg"),
            Some(binding)
        );
        assert!(builder
            .function_state
            .scope
            .function_param_names
            .contains("arg"));
        assert_eq!(
            builder.get_value_kind(ValueId::new(4)),
            Some(MirValueKind::Parameter(0))
        );
    }

    #[test]
    fn receiver_uses_the_same_identity_owner() {
        let mut builder = method_builder("Counter");
        let receiver = receiver_value(&builder);
        builder.setup_method_params("Counter", &[]).unwrap();

        assert!(builder.function_state.binding_ctx.lookup("me").is_some());
        assert_eq!(
            builder
                .function_state
                .type_ctx
                .value_origin_newbox
                .get(&receiver),
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

    #[test]
    fn method_receiver_commits_the_existing_signature_box_fact() {
        let mut builder = method_builder(RECEIVER_OWNER);
        let receiver = receiver_value(&builder);

        builder.setup_method_params(RECEIVER_OWNER, &[]).unwrap();

        assert_eq!(
            builder.function_state.variable_ctx.variable_map.get("me"),
            Some(&receiver)
        );
        assert_eq!(
            builder.get_value_kind(receiver),
            Some(MirValueKind::Parameter(0))
        );
        assert_eq!(
            builder.function_state.type_ctx.value_types.get(&receiver),
            Some(&MirType::Box(RECEIVER_OWNER.to_string()))
        );
        assert_eq!(
            builder
                .function_state
                .type_ctx
                .value_origin_newbox
                .get(&receiver),
            Some(&RECEIVER_OWNER.to_string())
        );
        assert!(builder
            .comp_ctx
            .current_slot_registry
            .as_ref()
            .unwrap()
            .get_slot("me")
            .is_some());
    }

    #[test]
    fn method_receiver_same_exact_fact_is_idempotent() {
        let mut builder = method_builder(RECEIVER_OWNER);
        let receiver = receiver_value(&builder);
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(receiver, MirType::Box(RECEIVER_OWNER.to_string()));

        builder.setup_method_params(RECEIVER_OWNER, &[]).unwrap();

        assert_eq!(
            builder.function_state.type_ctx.value_types.get(&receiver),
            Some(&MirType::Box(RECEIVER_OWNER.to_string()))
        );
    }

    #[test]
    fn method_receiver_concrete_conflict_fails_before_receiver_state_mutation() {
        let mut builder = method_builder(RECEIVER_OWNER);
        let receiver = receiver_value(&builder);
        builder
            .function_state
            .type_ctx
            .value_types
            .insert(receiver, MirType::Integer);
        builder
            .function_state
            .scope
            .function_param_names
            .insert("sentinel".to_string());
        let binding_before = builder.core_ctx.next_binding_id;
        let params_before = builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .params
            .clone();
        let signature_before = builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .signature
            .params
            .clone();

        let error = builder
            .setup_method_params(RECEIVER_OWNER, &[])
            .expect_err("foreign concrete receiver fact must reject");

        assert!(error
            .starts_with("[freeze:contract][lowering_facts/type_decision/concrete_fact_conflict]"));
        assert_receiver_preflight_left_state_unchanged(
            &builder,
            receiver,
            binding_before,
            &params_before,
            &signature_before,
        );
        assert!(builder
            .function_state
            .scope
            .function_param_names
            .contains("sentinel"));
        assert_eq!(
            builder.function_state.type_ctx.value_types.get(&receiver),
            Some(&MirType::Integer)
        );
    }

    #[test]
    fn method_receiver_non_box_signature_rejects_before_receiver_state_mutation() {
        let mut builder = method_builder(RECEIVER_OWNER);
        let receiver = receiver_value(&builder);
        builder
            .function_state
            .current_function
            .as_mut()
            .unwrap()
            .signature
            .params[0] = MirType::Unknown;
        let binding_before = builder.core_ctx.next_binding_id;
        let params_before = builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .params
            .clone();
        let signature_before = builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .signature
            .params
            .clone();

        let error = builder
            .setup_method_params(RECEIVER_OWNER, &[])
            .expect_err("non-box signature must reject");

        assert!(error.ends_with("reason=signature_param0_not_box"));
        assert_receiver_preflight_left_state_unchanged(
            &builder,
            receiver,
            binding_before,
            &params_before,
            &signature_before,
        );
    }

    #[test]
    fn method_receiver_owner_mismatch_rejects_before_receiver_state_mutation() {
        let mut builder = method_builder(RECEIVER_OWNER);
        let receiver = receiver_value(&builder);
        let binding_before = builder.core_ctx.next_binding_id;
        let params_before = builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .params
            .clone();
        let signature_before = builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .signature
            .params
            .clone();

        let error = builder
            .setup_method_params("ForeignReceiverV1", &[])
            .expect_err("mismatched receiver owner must reject");

        assert!(error.contains("reason=owner_mismatch"));
        assert_receiver_preflight_left_state_unchanged(
            &builder,
            receiver,
            binding_before,
            &params_before,
            &signature_before,
        );
    }
}
