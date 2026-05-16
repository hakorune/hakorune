use crate::ast::ASTNode;
use crate::mir::builder::observe::types as type_trace;
use crate::mir::definitions::call_unified::TypeCertainty;
use crate::mir::ssot::method_call::runtime_method_call;

use super::{EffectMask, MirInstruction, MirType, ValueId};

impl super::MirBuilder {
    pub(super) fn build_array_literal(
        &mut self,
        elements: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        let arr_id = self.next_value_id();
        self.emit_instruction(MirInstruction::NewBox {
            dst: arr_id,
            box_type: "ArrayBox".to_string(),
            args: vec![],
        })?;
        self.emit_constructor_birth_marker(arr_id, "ArrayBox")?;
        self.type_ctx
            .value_origin_newbox
            .insert(arr_id, "ArrayBox".to_string());
        self.type_ctx
            .value_types
            .insert(arr_id, MirType::Box("ArrayBox".to_string()));
        self.comp_ctx
            .type_registry
            .record_newbox(arr_id, "ArrayBox".to_string());
        self.comp_ctx
            .type_registry
            .record_type(arr_id, MirType::Box("ArrayBox".to_string()));
        type_trace::origin("newbox:ArrayLiteral", arr_id, "ArrayBox");
        type_trace::ty(
            "newbox:ArrayLiteral",
            arr_id,
            &MirType::Box("ArrayBox".to_string()),
        );

        let mut element_types = Vec::new();
        for element in elements {
            let value = self.build_expression_impl(element)?;
            let element_type = self.type_ctx.value_types.get(&value).cloned().or_else(|| {
                self.type_ctx
                    .value_origin_newbox
                    .get(&value)
                    .map(|box_name| MirType::Box(box_name.clone()))
            });
            self.emit_instruction(runtime_method_call(
                None,
                arr_id,
                "ArrayBox",
                "push",
                vec![value],
                EffectMask::MUT,
                TypeCertainty::Known,
            ))?;
            element_types.push(element_type);
        }

        crate::mir::builder::types::array_element::record_array_literal_elements(
            self,
            arr_id,
            &element_types,
        );
        Ok(arr_id)
    }

    pub(super) fn build_map_literal(
        &mut self,
        entries: Vec<(String, ASTNode)>,
    ) -> Result<ValueId, String> {
        let map_id = self.next_value_id();
        self.emit_instruction(MirInstruction::NewBox {
            dst: map_id,
            box_type: "MapBox".to_string(),
            args: vec![],
        })?;
        self.emit_constructor_birth_marker(map_id, "MapBox")?;
        self.type_ctx
            .value_origin_newbox
            .insert(map_id, "MapBox".to_string());
        self.type_ctx
            .value_types
            .insert(map_id, MirType::Box("MapBox".to_string()));
        self.comp_ctx
            .type_registry
            .record_newbox(map_id, "MapBox".to_string());
        self.comp_ctx
            .type_registry
            .record_type(map_id, MirType::Box("MapBox".to_string()));
        type_trace::origin("newbox:MapLiteral", map_id, "MapBox");
        type_trace::ty(
            "newbox:MapLiteral",
            map_id,
            &MirType::Box("MapBox".to_string()),
        );

        for (key, expr) in entries {
            let key_id = crate::mir::builder::emission::constant::emit_string(self, key)?;
            let value_id = self.build_expression_impl(expr)?;
            self.emit_instruction(runtime_method_call(
                None,
                map_id,
                "MapBox",
                "set",
                vec![key_id, value_id],
                EffectMask::MUT,
                TypeCertainty::Known,
            ))?;
        }
        Ok(map_id)
    }
}
