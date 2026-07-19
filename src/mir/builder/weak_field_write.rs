use super::{MirBuilder, MirInstruction, ValueId};
use crate::mir::instruction::FastMemRegionId;
use crate::mir::{UserBoxFieldDecl, WeakFieldWriteSiteId};

impl MirBuilder {
    pub(super) fn emit_known_weak_field_write(
        &mut self,
        region: Option<FastMemRegionId>,
        base: ValueId,
        field_name: &str,
        value: ValueId,
    ) -> Result<bool, String> {
        let Some(box_name) = self
            .function_state
            .type_ctx
            .value_origin_newbox
            .get(&base)
            .cloned()
        else {
            return Ok(false);
        };
        let Some(fields) = self.comp_ctx.user_box_field_decls.get(&box_name) else {
            return Ok(false);
        };
        let Some((field_index, field)) = fields
            .iter()
            .enumerate()
            .find(|(_, field)| field.name == field_name)
        else {
            return Ok(false);
        };
        if !field.is_weak {
            return Ok(false);
        }
        if region.is_some() {
            return Err(format!(
                "[type/weak_field_contract_fastmem_unsupported] box={} field={}",
                box_name, field_name
            ));
        }
        let typed_fields = fields
            .iter()
            .map(|field| UserBoxFieldDecl {
                name: field.name.clone(),
                declared_type_name: field.declared_type_name.clone(),
                is_weak: field.is_weak,
            })
            .collect::<Vec<_>>();
        let fingerprint = crate::mir::type_contracts::weak_field::box_schema_fingerprint(
            &box_name,
            &typed_fields,
        );
        let site_id = self.next_weak_field_write_site_id();
        self.emit_instruction(MirInstruction::WeakFieldWrite {
            site_id,
            contract_id: format!("weak-field:{fingerprint}:{field_index}"),
            base,
            field_index: field_index as u32,
            value,
        })?;
        Ok(true)
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
