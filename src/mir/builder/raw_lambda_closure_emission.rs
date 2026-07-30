//! Source-neutral terminal for raw Lambda closure emission and body publication.

use super::{MirBuilder, MirInstruction, MirType, ValueId};
use crate::ast::ASTNode;
use crate::mir::function::ClosureBodyId;

pub(super) struct PreparedRawLambdaClosureEmissionV1 {
    params: Vec<String>,
    body: Vec<ASTNode>,
    captures: Vec<(String, ValueId)>,
    receiver: Option<ValueId>,
}

impl PreparedRawLambdaClosureEmissionV1 {
    pub(super) fn prepare(
        params: Vec<String>,
        body: Vec<ASTNode>,
        captures: Vec<(String, ValueId)>,
        receiver: Option<ValueId>,
    ) -> Self {
        Self {
            params,
            body,
            captures,
            receiver,
        }
    }

    pub(super) fn lower_with_builder_v1(self, builder: &mut MirBuilder) -> Result<ValueId, String> {
        let publication = PreparedClosureBodyPublicationV1::prepare(builder, self.body);
        match publication {
            PreparedClosureBodyPublicationV1::Inline { body } => {
                let dst = builder.next_value_id();
                builder.emit_instruction(MirInstruction::NewClosure {
                    dst,
                    params: self.params,
                    body_id: None,
                    body,
                    captures: self.captures,
                    me: self.receiver,
                })?;
                install_function_box_type(builder, dst);
                Ok(dst)
            }
            PreparedClosureBodyPublicationV1::External { expected_id, body } => {
                let dst = builder.next_value_id();
                builder.emit_instruction(MirInstruction::NewClosure {
                    dst,
                    params: self.params,
                    body_id: Some(expected_id),
                    body: Vec::new(),
                    captures: self.captures,
                    me: self.receiver,
                })?;
                builder
                    .current_module
                    .as_mut()
                    .expect("[freeze:contract][mir_builder/raw_lambda_missing_reserved_module]")
                    .commit_reserved_closure_body(expected_id, body);
                install_function_box_type(builder, dst);
                Ok(dst)
            }
        }
    }
}

fn install_function_box_type(builder: &mut MirBuilder, value: ValueId) {
    builder
        .function_state
        .type_ctx
        .value_types
        .insert(value, MirType::Box("FunctionBox".into()));
}

enum PreparedClosureBodyPublicationV1 {
    Inline {
        body: Vec<ASTNode>,
    },
    External {
        expected_id: ClosureBodyId,
        body: Vec<ASTNode>,
    },
}

impl PreparedClosureBodyPublicationV1 {
    fn prepare(builder: &MirBuilder, body: Vec<ASTNode>) -> Self {
        if body.is_empty() {
            return Self::Inline { body };
        }
        let Some(module) = builder.current_module.as_ref() else {
            return Self::Inline { body };
        };
        Self::External {
            expected_id: module.reserve_next_closure_body_id(),
            body,
        }
    }
}
