//! Complete raw lifecycle for one non-Main static Box declaration.
//!
//! Program-root deferred static Boxes intentionally keep different timing and
//! state semantics. This owner only replaces the raw dispatcher branch, where
//! app mode is a declaration no-op and non-App mode isolates one full method
//! batch in the existing four-state transaction.

use crate::ast::BoxMethodInventoryV1;
use crate::mir::builder::module_lifecycle::RootCallableCapturePortV1;
use crate::mir::builder::nonmain_static_box_method_batch::PreparedNonMainStaticBoxMethodBatchV1;
use crate::mir::builder::raw_expression_dispatch::static_box_state::ActiveRawStaticBoxCompilationStateV1;
use crate::mir::builder::recursive_child_lowering::RawBoxMethodChildPortV1;
use crate::mir::{MirBuilder, ValueId};

pub(in crate::mir::builder) struct PreparedRawNonMainStaticBoxLifecycleV1 {
    name: String,
    methods: PreparedNonMainStaticBoxMethodBatchV1,
}

impl PreparedRawNonMainStaticBoxLifecycleV1 {
    pub(in crate::mir::builder) fn prepare(name: String, methods: BoxMethodInventoryV1) -> Self {
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

    /// Reuses the raw static-Box outer transaction while admitting only its
    /// catalog-addressable methods through the selected Program-root port.
    ///
    /// This is deliberately not a raw-port extension: callers must already
    /// own the selected Program statement classification.
    pub(in crate::mir::builder) fn lower_normal_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
    ) -> Result<ValueId, String>
    where
        Port: RootCallableCapturePortV1,
    {
        if builder.root_is_app_mode.unwrap_or(false) {
            return crate::mir::builder::emission::constant::emit_void(builder);
        }

        builder.comp_ctx.register_user_box(self.name);
        let transaction = ActiveRawStaticBoxCompilationStateV1::begin(builder);
        match self.methods.lower_root_with_port_v1(builder, port) {
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
