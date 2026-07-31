//! Owned selected-Script block descent and exact statement-source installation.

use super::module_lifecycle::RootCallableCapturePortV1;
use super::normal_script_direct_statement_owner::{
    lower_direct_fastmem_region_v1, lower_direct_if_statement_v1,
    lower_direct_port_aware_expression_v1, lower_direct_print_v1,
    lower_direct_selected_unsupported_statement_v1,
    lower_direct_static_const_runtime_completion_v1,
};
use super::normal_script_runtime_work::{
    lower_cataloged_nonmain_static_box_v1, lower_instance_runtime_prefix_v1,
    lower_nonplain_instance_runtime_lifecycle_v1, lower_static_main_compatibility_v1,
    reject_sync_box_at_runtime_v1, LocatedNormalScriptRuntimeAdmissionV1,
    NormalScriptRuntimeStatementAdmissionV1,
};
use super::recursive_child_lowering::drive_legacy_statement_v1;
use super::stmts::block_driver::LegacyBlockDescentPortV1;
use super::MirBuilder;
use crate::ast::ASTNode;
use crate::mir::ValueId;

pub(super) struct NormalScriptRuntimeBlockPortV1<'port, Port> {
    statements: std::vec::IntoIter<ASTNode>,
    admissions: std::vec::IntoIter<LocatedNormalScriptRuntimeAdmissionV1>,
    port: &'port mut Port,
}

impl<'port, Port> NormalScriptRuntimeBlockPortV1<'port, Port> {
    pub(super) fn new(
        statements: Box<[ASTNode]>,
        admissions: Box<[LocatedNormalScriptRuntimeAdmissionV1]>,
        port: &'port mut Port,
    ) -> Self {
        Self {
            statements: statements.into_vec().into_iter(),
            admissions: admissions.into_vec().into_iter(),
            port,
        }
    }
}

impl<Port> LegacyBlockDescentPortV1 for NormalScriptRuntimeBlockPortV1<'_, Port>
where
    Port: RootCallableCapturePortV1,
{
    type SuffixInput<'a>
        = &'a [ASTNode]
    where
        Self: 'a;

    fn len(&self) -> usize {
        self.statements.len()
    }

    fn suffix_route_input(&self, _index: usize) -> Result<Option<Self::SuffixInput<'_>>, String> {
        debug_assert_eq!(self.statements.len(), self.admissions.len());
        Ok(Some(self.statements.as_slice()))
    }

    fn consume_suffix_prefix(&mut self, count: usize) {
        for _ in 0..count {
            let _ = self.statements.next();
            let _ = self.admissions.next();
        }
    }

    fn lower_statement(
        &mut self,
        builder: &mut MirBuilder,
        _index: usize,
    ) -> Result<ValueId, String> {
        let statement = self
            .statements
            .next()
            .expect("script runtime block index stays within owned statements");
        let admission = self
            .admissions
            .next()
            .expect("script runtime admission stays aligned with statements");
        let source_statement_index = admission.source_statement_index;
        match admission.admission {
            NormalScriptRuntimeStatementAdmissionV1::DirectPrint => {
                let source = self
                    .port
                    .prepare_body_statement_source_v1(&statement, source_statement_index)?;
                self.port.with_prepared_child_source_v1(source, |port| {
                    lower_direct_print_v1(builder, port, statement)
                })
            }
            NormalScriptRuntimeStatementAdmissionV1::DirectIfStatement => {
                let source = self
                    .port
                    .prepare_body_statement_source_v1(&statement, source_statement_index)?;
                self.port.with_prepared_child_source_v1(source, |port| {
                    lower_direct_if_statement_v1(builder, port, statement)
                })
            }
            NormalScriptRuntimeStatementAdmissionV1::DirectFastMemRegion => {
                let source = self
                    .port
                    .prepare_body_statement_source_v1(&statement, source_statement_index)?;
                self.port.with_prepared_child_source_v1(source, |port| {
                    lower_direct_fastmem_region_v1(builder, port, statement)
                })
            }
            NormalScriptRuntimeStatementAdmissionV1::DirectPortAwareExpression => {
                let source = self
                    .port
                    .prepare_body_statement_source_v1(&statement, source_statement_index)?;
                self.port.with_prepared_child_source_v1(source, |port| {
                    lower_direct_port_aware_expression_v1(builder, port, statement)
                })
            }
            NormalScriptRuntimeStatementAdmissionV1::DirectStaticConstRuntimeCompletion => {
                let source = self
                    .port
                    .prepare_body_statement_source_v1(&statement, source_statement_index)?;
                self.port.with_prepared_child_source_v1(source, |_port| {
                    lower_direct_static_const_runtime_completion_v1(builder, &statement)
                })
            }
            NormalScriptRuntimeStatementAdmissionV1::DirectSelectedUnsupportedStatement => {
                let source = self
                    .port
                    .prepare_body_statement_source_v1(&statement, source_statement_index)?;
                self.port.with_prepared_child_source_v1(source, |_port| {
                    lower_direct_selected_unsupported_statement_v1(builder, &statement)
                })
            }
            NormalScriptRuntimeStatementAdmissionV1::RawCompatibility => {
                drive_legacy_statement_v1(builder, self.port, statement)
            }
            NormalScriptRuntimeStatementAdmissionV1::CatalogedNonMainStaticBox => {
                lower_cataloged_nonmain_static_box_v1(builder, self.port, &statement)
            }
            NormalScriptRuntimeStatementAdmissionV1::StaticMainCompatibility => {
                lower_static_main_compatibility_v1(builder, self.port, &statement)
            }
            NormalScriptRuntimeStatementAdmissionV1::SyncBoxRejection => {
                reject_sync_box_at_runtime_v1(&statement)
            }
            NormalScriptRuntimeStatementAdmissionV1::InstancePrefixCompatibility {
                constructor_sources,
                constructor_batch,
            } => lower_instance_runtime_prefix_v1(
                builder,
                self.port,
                &statement,
                constructor_sources.as_ref(),
                constructor_batch.as_ref(),
            ),
            NormalScriptRuntimeStatementAdmissionV1::NonPlainInstanceFullLifecycle {
                constructor_sources,
                constructor_batch,
            } => lower_nonplain_instance_runtime_lifecycle_v1(
                builder,
                self.port,
                &statement,
                constructor_sources.as_ref(),
                constructor_batch.as_ref(),
            ),
        }
    }
}
