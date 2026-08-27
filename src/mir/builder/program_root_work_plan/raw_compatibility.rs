//! Explicit RawCompatibility work-plan terminals.
//!
//! The parent work-plan module owns classification and admission.  This child
//! only transports the already-issued compatibility symbol/arity shape to the
//! dedicated raw terminal; it never performs target resolution or source
//! recovery.

use super::super::instance_box_declaration_lifecycle::PreparedInstanceBoxDeclarationLifecycleV1;
use super::super::raw_compatibility_child_terminal::{
    RawCompatibilityCallableShapeV1, RawCompatibilityChildTerminalPortV1,
};
use super::super::recursive_child_lowering::RawInvocationChildPortV1;
use super::super::MirBuilder;
use super::{
    PreparedProgramDeferredStaticBoxWorkV1, PreparedProgramRootImmediateWorkV1,
    PreparedProgramRootInstanceBoxWorkV1, PreparedProgramRootTopLevelFunctionPartsV1,
    PreparedProgramRootTopLevelFunctionWorkV1, ProgramRootWorkPlanAdmissionV1,
};

impl PreparedProgramRootImmediateWorkV1 {
    pub(in crate::mir::builder) fn lower_raw_compat_with_port_v1(
        self,
        builder: &mut MirBuilder,
        port: &mut RawInvocationChildPortV1<'_, '_>,
    ) -> Result<(), String> {
        match self {
            Self::InstanceBox(work) => work.lower_raw_compat_with_port_v1(builder, port),
            Self::TopLevelFunction(work) => work.lower_raw_compat_with_port_v1(builder, port),
        }
    }
}

impl PreparedProgramRootInstanceBoxWorkV1 {
    pub(in crate::mir::builder) fn lower_raw_compat_with_port_v1(
        self,
        builder: &mut MirBuilder,
        port: &mut RawInvocationChildPortV1<'_, '_>,
    ) -> Result<(), String> {
        if self.admission != ProgramRootWorkPlanAdmissionV1::RawCompatibility {
            return Err(
                "[freeze:contract][mir/program-root-work-plan/raw-compat-admission-drift]"
                    .to_owned(),
            );
        }
        let constructor_shapes = self.constructors.issue_raw_compat_shapes();
        let lifecycle =
            PreparedInstanceBoxDeclarationLifecycleV1::prepare_with_constructor_batch_v1(
                &self.name,
                &self.methods,
                &self.fields,
                &self.field_decls,
                &self.init_fields,
                &self.weak_fields,
                self.constructors,
            );
        lifecycle.lower_raw_compat_with_port_v1(builder, port, constructor_shapes)
    }
}

impl PreparedProgramDeferredStaticBoxWorkV1 {
    pub(in crate::mir::builder) fn lower_raw_compat_with_port_v1(
        self,
        builder: &mut MirBuilder,
        port: &mut RawInvocationChildPortV1<'_, '_>,
    ) -> Result<(), String> {
        if self.admission != ProgramRootWorkPlanAdmissionV1::RawCompatibility {
            return Err(
                "[freeze:contract][mir/program-root-work-plan/raw-compat-admission-drift]"
                    .to_owned(),
            );
        }
        super::super::program_root_lowering::ProgramDeferredStaticBoxLifecycleV1::new(
            self.name,
            self.methods,
        )
        .lower_raw_compat_with_port_v1(builder, port)
    }
}

impl PreparedProgramRootTopLevelFunctionWorkV1 {
    pub(in crate::mir::builder) fn lower_raw_compat_with_port_v1(
        self,
        builder: &mut MirBuilder,
        port: &mut RawInvocationChildPortV1<'_, '_>,
    ) -> Result<(), String> {
        match self {
            Self::RawCompatibility(parts) => parts.lower_raw_compat_with_port_v1(builder, port),
            Self::SelectedNormal { .. } => Err(
                "[freeze:contract][mir/top-level-function-admission/raw-compat-port]".to_owned(),
            ),
        }
    }
}

impl PreparedProgramRootTopLevelFunctionPartsV1 {
    fn lower_raw_compat_with_port_v1(
        self,
        builder: &mut MirBuilder,
        port: &mut RawInvocationChildPortV1<'_, '_>,
    ) -> Result<(), String> {
        let shape = RawCompatibilityCallableShapeV1::issue(
            format!("{}/{}", self.name, self.params.len()),
            self.params.len(),
        );
        port.lower_raw_compat_static_child(
            builder,
            shape,
            self.params,
            self.param_decls,
            self.return_type_name,
            self.body,
            self.uses,
            self.attrs,
        )
    }
}
