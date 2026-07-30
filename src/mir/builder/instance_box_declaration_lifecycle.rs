//! One declaration lifecycle shared by Program-root and raw instance Boxes.
//!
//! The effectful prefix is deliberately single-owner. Root and raw lowering
//! share fields and metadata, then select their constructor/method terminals.

use std::collections::HashMap;

use crate::ast::{ASTNode, FieldDecl};

use super::instance_box_constructor_batch::PreparedInstanceBoxConstructorBatchV1;
use super::instance_box_declaration_metadata::PreparedInstanceBoxDeclarationMetadataV1;
use super::instance_box_method_batch::PreparedInstanceBoxMethodBatchV1;
use super::module_lifecycle::RootCallableCapturePortV1;
use super::normal_instance_constructor_admission::NormalInstanceConstructorSourceBatchV1;
use super::recursive_child_lowering::RawBoxMethodChildPortV1;
use super::MirBuilder;

pub(super) struct PreparedInstanceBoxDeclarationLifecycleV1<'source> {
    name: &'source str,
    fields: &'source [String],
    field_decls: &'source [FieldDecl],
    init_fields: &'source [String],
    weak_fields: &'source [String],
    metadata: PreparedInstanceBoxDeclarationMetadataV1,
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
        Self::prepare_with_constructor_batch_v1(
            name,
            methods,
            fields,
            field_decls,
            init_fields,
            weak_fields,
            PreparedInstanceBoxConstructorBatchV1::prepare(name, constructors),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn prepare_with_constructor_batch_v1(
        name: &'source str,
        methods: &'source HashMap<String, ASTNode>,
        fields: &'source [String],
        field_decls: &'source [FieldDecl],
        init_fields: &'source [String],
        weak_fields: &'source [String],
        constructors: PreparedInstanceBoxConstructorBatchV1,
    ) -> Self {
        Self {
            name,
            fields,
            field_decls,
            init_fields,
            weak_fields,
            metadata: PreparedInstanceBoxDeclarationMetadataV1::prepare(
                name,
                methods,
                fields,
                weak_fields,
            ),
            constructors,
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
        let (constructors, methods) = self.lower_declaration_prefix_v1(builder)?;
        constructors.lower_with_port_v1(builder, port)?;
        methods.lower_root_with_port_v1(builder, port)
    }

    pub(super) fn lower_normal_root_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
        constructor_sources: &NormalInstanceConstructorSourceBatchV1,
    ) -> Result<(), String>
    where
        Port: RootCallableCapturePortV1,
    {
        let (constructors, methods) = self.lower_declaration_prefix_v1(builder)?;
        constructors.lower_normal_with_port_v1(builder, port, constructor_sources)?;
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
        let (constructors, methods) = self.lower_declaration_prefix_v1(builder)?;
        constructors.lower_with_port_v1(builder, port)?;
        methods.lower_raw_with_port_v1(builder, port)
    }

    /// Preserves the runtime declaration prefix while leaving ordinary methods
    /// to the Program-root catalog admission that already owns them.
    pub(in crate::mir::builder) fn lower_runtime_prefix_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
    ) -> Result<(), String>
    where
        Port: RawBoxMethodChildPortV1,
    {
        let (constructors, _) = self.lower_declaration_prefix_v1(builder)?;
        constructors.lower_with_port_v1(builder, port)?;
        Ok(())
    }

    pub(in crate::mir::builder) fn lower_normal_runtime_prefix_with_port_v1<Port>(
        self,
        builder: &mut MirBuilder,
        port: &mut Port,
        constructor_sources: &NormalInstanceConstructorSourceBatchV1,
    ) -> Result<(), String>
    where
        Port: RootCallableCapturePortV1,
    {
        let (constructors, _) = self.lower_declaration_prefix_v1(builder)?;
        constructors.lower_normal_with_port_v1(builder, port, constructor_sources)
    }

    fn lower_declaration_prefix_v1(
        self,
        builder: &mut MirBuilder,
    ) -> Result<
        (
            PreparedInstanceBoxConstructorBatchV1,
            PreparedInstanceBoxMethodBatchV1,
        ),
        String,
    > {
        let Self {
            name,
            fields,
            field_decls,
            init_fields,
            weak_fields,
            metadata,
            constructors,
            instance_methods,
        } = self;
        builder.comp_ctx.register_user_box_declared_fields(
            name.to_owned(),
            fields,
            field_decls,
            init_fields,
            weak_fields,
        );
        metadata.lower_with_builder_v1(builder)?;
        Ok((constructors, instance_methods))
    }
}
