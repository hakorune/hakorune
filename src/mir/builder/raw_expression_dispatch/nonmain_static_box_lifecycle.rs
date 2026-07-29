//! Complete raw lifecycle for one non-Main static Box declaration.
//!
//! Program-root deferred static Boxes intentionally keep different timing and
//! state semantics. This owner only replaces the raw dispatcher branch, where
//! app mode is a declaration no-op and non-App mode isolates one full method
//! batch in the existing four-state transaction.

use std::collections::HashMap;

use crate::ast::ASTNode;
use crate::mir::builder::nonmain_static_box_method_batch::PreparedNonMainStaticBoxMethodBatchV1;
use crate::mir::builder::raw_expression_dispatch::static_box_state::ActiveRawStaticBoxCompilationStateV1;
use crate::mir::builder::recursive_child_lowering::RawBoxMethodChildPortV1;
use crate::mir::{MirBuilder, ValueId};

pub(super) struct PreparedRawNonMainStaticBoxLifecycleV1 {
    name: String,
    methods: PreparedNonMainStaticBoxMethodBatchV1,
}

impl PreparedRawNonMainStaticBoxLifecycleV1 {
    pub(super) fn prepare(name: String, methods: HashMap<String, ASTNode>) -> Self {
        Self {
            methods: PreparedNonMainStaticBoxMethodBatchV1::prepare(name.clone(), methods),
            name,
        }
    }

    pub(super) fn lower_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
    ) -> Result<ValueId, String>
    where
        Port: RawBoxMethodChildPortV1,
    {
        if builder.root_is_app_mode.unwrap_or(false) {
            return crate::mir::builder::emission::constant::emit_void(builder);
        }

        builder.comp_ctx.register_user_box(self.name);
        let transaction = ActiveRawStaticBoxCompilationStateV1::begin(builder);
        match self.methods.lower_with_port_v1(builder, port) {
            Ok(()) => transaction.complete_success(builder),
            Err(error) => {
                let rejected = transaction.reject(error);
                let error = rejected.error().to_owned();
                rejected.discard();
                return Err(error);
            }
        }
        crate::mir::builder::emission::constant::emit_void(builder)
    }
}
