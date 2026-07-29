//! Success-only outer state transaction for one raw non-Main static Box.
//!
//! This owner deliberately captures only the four states that the raw
//! dispatcher historically isolated around the complete sorted method set.
//! A successful Box restores its caller before the declaration Void is
//! emitted. A typed method failure does not restore: the outer candidate
//! session remains the sole owner of discarding unpublished Builder effects.

use std::collections::BTreeMap;

use hakorune_mir_builder::{BoxCompilationContext, TypeContextSnapshot};

use crate::mir::builder::MirBuilder;
use crate::mir::region::function_slot_registry::FunctionSlotRegistry;
use crate::mir::ValueId;

struct RawStaticBoxCallerCompilationStateV1 {
    variable_map: BTreeMap<String, ValueId>,
    type_context: TypeContextSnapshot,
    slot_registry: Option<FunctionSlotRegistry>,
    box_context: Option<BoxCompilationContext>,
}

pub(super) struct ActiveRawStaticBoxCompilationStateV1 {
    caller: RawStaticBoxCallerCompilationStateV1,
}

impl ActiveRawStaticBoxCompilationStateV1 {
    pub(super) fn begin(builder: &mut MirBuilder) -> Self {
        let caller = RawStaticBoxCallerCompilationStateV1 {
            variable_map: std::mem::take(&mut builder.function_state.variable_ctx.variable_map),
            type_context: builder.function_state.type_ctx.take_snapshot(),
            slot_registry: builder.comp_ctx.current_slot_registry.take(),
            box_context: builder.comp_ctx.compilation_context.take(),
        };
        builder.comp_ctx.compilation_context = Some(BoxCompilationContext::new());
        Self { caller }
    }

    pub(super) fn complete_success(self, builder: &mut MirBuilder) {
        builder.comp_ctx.compilation_context = self.caller.box_context;
        builder.function_state.variable_ctx.variable_map = self.caller.variable_map;
        builder
            .function_state
            .type_ctx
            .restore_snapshot(self.caller.type_context);
        builder.comp_ctx.current_slot_registry = self.caller.slot_registry;
    }

    pub(super) fn reject(self, error: String) -> RejectedRawStaticBoxCompilationStateV1 {
        RejectedRawStaticBoxCompilationStateV1 { error }
    }
}

pub(super) struct RejectedRawStaticBoxCompilationStateV1 {
    error: String,
}

impl RejectedRawStaticBoxCompilationStateV1 {
    pub(super) fn error(&self) -> &str {
        &self.error
    }

    pub(super) fn discard(self) {}
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::MirType;

    fn seeded_builder() -> MirBuilder {
        let mut builder = MirBuilder::new();
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("caller".to_string(), ValueId(41));
        builder
            .function_state
            .type_ctx
            .set_type(ValueId(42), MirType::Integer);
        let mut slots = FunctionSlotRegistry::new();
        slots.ensure_slot("caller", Some(MirType::Integer));
        builder.comp_ctx.current_slot_registry = Some(slots);
        let mut box_context = BoxCompilationContext::new();
        box_context
            .variable_map
            .insert("caller".to_string(), ValueId(43));
        builder.comp_ctx.compilation_context = Some(box_context);
        builder
    }

    fn assert_caller_restored(builder: &MirBuilder) {
        assert_eq!(
            builder
                .function_state
                .variable_ctx
                .variable_map
                .get("caller"),
            Some(&ValueId(41))
        );
        assert_eq!(
            builder.function_state.type_ctx.get_type(ValueId(42)),
            Some(&MirType::Integer)
        );
        assert!(builder
            .comp_ctx
            .current_slot_registry
            .as_ref()
            .is_some_and(|slots| slots.get_slot("caller").is_some()));
        assert_eq!(
            builder
                .comp_ctx
                .compilation_context
                .as_ref()
                .and_then(|context| context.variable_map.get("caller")),
            Some(&ValueId(43))
        );
    }

    #[test]
    fn success_restores_exact_four_state_caller_before_completion() {
        let mut builder = seeded_builder();
        let transaction = ActiveRawStaticBoxCompilationStateV1::begin(&mut builder);
        assert!(builder.function_state.variable_ctx.variable_map.is_empty());
        assert!(builder.function_state.type_ctx.value_types.is_empty());
        assert!(builder.comp_ctx.current_slot_registry.is_none());
        assert!(builder
            .comp_ctx
            .compilation_context
            .as_ref()
            .is_some_and(BoxCompilationContext::is_empty));

        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("inner".to_string(), ValueId(50));
        transaction.complete_success(&mut builder);
        assert_caller_restored(&builder);
        assert!(!builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key("inner"));
    }

    #[test]
    fn rejection_keeps_inner_state_without_restoring_caller() {
        let mut builder = seeded_builder();
        let transaction = ActiveRawStaticBoxCompilationStateV1::begin(&mut builder);
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("inner".to_string(), ValueId(50));

        let rejected = transaction.reject("primary".to_string());
        assert_eq!(rejected.error(), "primary");
        rejected.discard();

        assert_eq!(
            builder
                .function_state
                .variable_ctx
                .variable_map
                .get("inner"),
            Some(&ValueId(50))
        );
        assert!(!builder
            .function_state
            .variable_ctx
            .variable_map
            .contains_key("caller"));
        assert!(builder.comp_ctx.current_slot_registry.is_none());
        assert!(builder
            .comp_ctx
            .compilation_context
            .as_ref()
            .is_some_and(BoxCompilationContext::is_empty));
    }

    #[test]
    fn nested_success_restores_outer_then_original_caller() {
        let mut builder = seeded_builder();
        let outer = ActiveRawStaticBoxCompilationStateV1::begin(&mut builder);
        builder
            .function_state
            .variable_ctx
            .variable_map
            .insert("outer".to_string(), ValueId(60));

        let inner = ActiveRawStaticBoxCompilationStateV1::begin(&mut builder);
        inner.complete_success(&mut builder);
        assert_eq!(
            builder
                .function_state
                .variable_ctx
                .variable_map
                .get("outer"),
            Some(&ValueId(60))
        );

        outer.complete_success(&mut builder);
        assert_caller_restored(&builder);
    }
}
