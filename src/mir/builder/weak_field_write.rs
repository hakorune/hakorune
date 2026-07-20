use super::weak_field_write_route::PreparedKnownWeakFieldWriteV1;
use super::{MirBuilder, MirInstruction};
use crate::mir::WeakFieldWriteSiteId;

impl MirBuilder {
    pub(super) fn emit_prepared_known_weak_field_write(
        &mut self,
        prepared: PreparedKnownWeakFieldWriteV1,
    ) -> Result<(), String> {
        if prepared.region.is_some() {
            return Err(format!(
                "[type/weak_field_contract_fastmem_unsupported] box={} field={}",
                prepared.box_name, prepared.field_name
            ));
        }
        let site_id = self.next_weak_field_write_site_id();
        let field_index = u32::try_from(prepared.field_index)
            .map_err(|_| "[freeze:contract][weak_field/field_index_overflow]".to_string())?;
        self.emit_instruction(MirInstruction::WeakFieldWrite {
            site_id,
            contract_id: prepared.contract_id(),
            base: prepared.base,
            field_index,
            value: prepared.value,
        })
    }

    fn next_weak_field_write_site_id(&self) -> WeakFieldWriteSiteId {
        let next = self
            .function_state
            .current_function
            .as_ref()
            .into_iter()
            .flat_map(|function| function.blocks.values())
            .flat_map(|block| block.instructions.iter())
            .filter_map(|instruction| match instruction {
                MirInstruction::WeakFieldWrite { site_id, .. } => Some(site_id.0),
                _ => None,
            })
            .max()
            .map_or(0, |site| site.saturating_add(1));
        WeakFieldWriteSiteId::new(next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::FieldDecl;
    use crate::mir::instruction::FastMemRegionId;
    use crate::mir::ValueId;

    fn weak_decl(name: &str) -> FieldDecl {
        FieldDecl {
            name: name.to_string(),
            declared_type_name: Some("MapBox".to_string()),
            is_weak: true,
            default_value: None,
        }
    }

    fn weak_builder() -> (MirBuilder, ValueId, ValueId) {
        let mut builder = MirBuilder::new();
        builder.enter_function_for_test("weak_field_write/0".to_string());
        builder
            .comp_ctx
            .user_box_field_decls
            .insert("Owner".to_string(), vec![weak_decl("slot")]);
        let base = builder.alloc_value_for_test();
        let value = builder.alloc_value_for_test();
        builder
            .function_state
            .type_ctx
            .value_origin_newbox
            .insert(base, "Owner".to_string());
        (builder, base, value)
    }

    #[test]
    fn weak_success_emits_one_physical_instruction() {
        let (mut builder, base, value) = weak_builder();
        let fields = builder.comp_ctx.user_box_field_decls.get("Owner").unwrap();
        let route = super::super::weak_field_write_route::prepare_field_write_route_v1(
            None,
            base,
            "slot",
            value,
            Some("Owner"),
            Some(fields.as_slice()),
        );
        let super::super::weak_field_write_route::PreparedFieldWriteRouteV1::KnownWeak(route) =
            route
        else {
            panic!("weak declaration must produce KnownWeak");
        };
        builder.emit_prepared_known_weak_field_write(route).unwrap();
        let function = builder.function_state.current_function.as_ref().unwrap();
        assert_eq!(
            function
                .blocks
                .values()
                .flat_map(|block| block.instructions.iter())
                .filter(|instruction| {
                    matches!(instruction, MirInstruction::WeakFieldWrite { .. })
                })
                .count(),
            1
        );
    }

    #[test]
    fn weak_fastmem_preserves_existing_error_boundary() {
        let (mut builder, base, value) = weak_builder();
        let fields = builder.comp_ctx.user_box_field_decls.get("Owner").unwrap();
        let route = super::super::weak_field_write_route::prepare_field_write_route_v1(
            Some(FastMemRegionId::new(1)),
            base,
            "slot",
            value,
            Some("Owner"),
            Some(fields.as_slice()),
        );
        let super::super::weak_field_write_route::PreparedFieldWriteRouteV1::KnownWeak(route) =
            route
        else {
            panic!("weak declaration must produce KnownWeak");
        };
        let error = builder
            .emit_prepared_known_weak_field_write(route)
            .unwrap_err();
        assert_eq!(
            error,
            "[type/weak_field_contract_fastmem_unsupported] box=Owner field=slot"
        );
        let function = builder.function_state.current_function.as_ref().unwrap();
        assert!(function
            .blocks
            .values()
            .flat_map(|block| block.instructions.iter())
            .all(|instruction| !matches!(instruction, MirInstruction::WeakFieldWrite { .. })));
    }
}
