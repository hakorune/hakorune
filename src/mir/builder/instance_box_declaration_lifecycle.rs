//! One declaration lifecycle shared by Program-root and raw instance Boxes.
//!
//! The effectful prefix is deliberately single-owner.  Root and raw lowering
//! diverge only after fields, metadata, and every constructor have completed.

use std::collections::HashMap;

use crate::ast::{ASTNode, FieldDecl};

use super::instance_box_constructor_batch::PreparedInstanceBoxConstructorBatchV1;
use super::instance_box_method_batch::PreparedInstanceBoxMethodBatchV1;
use super::module_lifecycle::RootCallableCapturePortV1;
use super::recursive_child_lowering::RawBoxMethodChildPortV1;
use super::MirBuilder;

pub(super) struct PreparedInstanceBoxDeclarationLifecycleV1<'source> {
    name: &'source str,
    methods: &'source HashMap<String, ASTNode>,
    fields: &'source [String],
    field_decls: &'source [FieldDecl],
    init_fields: &'source [String],
    weak_fields: &'source [String],
    constructors: PreparedInstanceBoxConstructorBatchV1,
    instance_methods: PreparedInstanceBoxMethodBatchV1,
}

impl<'source> PreparedInstanceBoxDeclarationLifecycleV1<'source> {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn prepare(
        name: &'source str,
        methods: &'source HashMap<String, ASTNode>,
        fields: &'source [String],
        field_decls: &'source [FieldDecl],
        constructors: &'source HashMap<String, ASTNode>,
        init_fields: &'source [String],
        weak_fields: &'source [String],
    ) -> Self {
        Self {
            name,
            methods,
            fields,
            field_decls,
            init_fields,
            weak_fields,
            constructors: PreparedInstanceBoxConstructorBatchV1::prepare(name, constructors),
            instance_methods: PreparedInstanceBoxMethodBatchV1::prepare(name, methods),
        }
    }

    pub(super) fn lower_root_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
    ) -> Result<(), String>
    where
        Port: RootCallableCapturePortV1,
    {
        let methods = self.lower_common_prefix_v1(builder, port)?;
        methods.lower_root_with_port_v1(builder, port)
    }

    pub(super) fn lower_raw_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
    ) -> Result<(), String>
    where
        Port: RawBoxMethodChildPortV1,
    {
        let methods = self.lower_common_prefix_v1(builder, port)?;
        methods.lower_raw_with_port_v1(builder, port)
    }

    fn lower_common_prefix_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
    ) -> Result<PreparedInstanceBoxMethodBatchV1, String>
    where
        Port: RawBoxMethodChildPortV1,
    {
        builder.comp_ctx.register_user_box_declared_fields(
            self.name.to_owned(),
            self.fields,
            self.field_decls,
            self.init_fields,
            self.weak_fields,
        );
        builder.build_box_declaration(
            self.name.to_owned(),
            self.methods.clone(),
            self.fields.to_vec(),
            self.weak_fields.to_vec(),
        )?;
        self.constructors.lower_with_port_v1(builder, port)?;
        Ok(self.instance_methods)
    }
}
